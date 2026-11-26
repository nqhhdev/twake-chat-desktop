//! Sign-in, and the session kept in the OS keychain (ADR-004).

use std::path::PathBuf;

use matrix_sdk::{Client, authentication::matrix::MatrixSession};
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};

const KEYRING_SERVICE: &str = "app.twake.chat.desktop";
const KEYRING_USER: &str = "session";
pub const DEVICE_NAME: &str = "Twake Chat desktop";

/// What survives a restart. Lives in the keychain only, never on disk in clear.
#[derive(Clone, Serialize, Deserialize)]
pub struct StoredSession {
    pub homeserver: String,
    /// Passphrase of the encrypted SQLite store.
    pub passphrase: String,
    pub store_dir: PathBuf,
    pub session: MatrixSession,
}

/// `TWAKE_PROFILE` keeps separate sessions side by side (several test accounts).
fn profile() -> Option<String> {
    std::env::var("TWAKE_PROFILE")
        .ok()
        .filter(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'))
}

fn entry() -> Result<keyring::Entry, String> {
    let user = match profile() {
        Some(p) => format!("{KEYRING_USER}-{p}"),
        None => KEYRING_USER.to_string(),
    };
    keyring::Entry::new(KEYRING_SERVICE, &user).map_err(|e| format!("keychain: {e}"))
}

/// Development only (`TWAKE_DEV_SESSION_FILE`): the session in a file of the data folder
/// instead of the keychain, because macOS asks again for keychain access after every
/// rebuild of an unsigned binary. Never set in a release.
fn dev_file() -> Option<PathBuf> {
    std::env::var_os("TWAKE_DEV_SESSION_FILE").map(|_| data_dir().join("dev-session.json"))
}

pub fn load() -> Option<StoredSession> {
    let json = match dev_file() {
        Some(path) => std::fs::read_to_string(path).ok()?,
        None => entry().ok()?.get_password().ok()?,
    };
    serde_json::from_str(&json).ok()
}

pub fn save(stored: &StoredSession) -> Result<(), String> {
    let json = serde_json::to_string(stored).map_err(|e| e.to_string())?;
    match dev_file() {
        Some(path) => write_private(&path, &json),
        None => entry()?
            .set_password(&json)
            .map_err(|e| format!("keychain: {e}")),
    }
}

fn write_private(path: &std::path::Path, contents: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, contents).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
    }
    Ok(())
}

pub fn forget() {
    if let Some(path) = dev_file() {
        let _ = std::fs::remove_file(path);
        return;
    }
    if let Ok(entry) = entry() {
        let _ = entry.delete_credential();
    }
}

fn data_dir() -> PathBuf {
    let base = directories::ProjectDirs::from("app", "Linagora", "Twake Chat")
        .map(|dirs| dirs.data_local_dir().to_path_buf())
        .unwrap_or_else(|| std::env::temp_dir().join("twake-chat"));
    match profile() {
        Some(p) => base.join("profiles").join(p),
        None => base,
    }
}

/// A fresh store folder and passphrase for a new sign-in.
pub fn new_store() -> (PathBuf, String) {
    let id: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    let passphrase: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect();
    (data_dir().join("stores").join(id), passphrase)
}

pub async fn build_client(
    homeserver: &str,
    store_dir: &std::path::Path,
    passphrase: &str,
) -> Result<Client, String> {
    Client::builder()
        .server_name_or_homeserver_url(homeserver)
        .sqlite_store(store_dir, Some(passphrase))
        .handle_refresh_tokens()
        // Room keys come from the key backup when a message cannot be decrypted, and
        // backups are kept up to date from this device (as Element X does).
        .with_encryption_settings(matrix_sdk::encryption::EncryptionSettings {
            auto_enable_cross_signing: false,
            backup_download_strategy:
                matrix_sdk::encryption::BackupDownloadStrategy::AfterDecryptionFailure,
            auto_enable_backups: true,
        })
        .build()
        .await
        .map_err(|e| format!("cannot reach {homeserver}: {e}"))
}

/// Restores the session of the keychain, if any.
pub async fn restore() -> Option<Result<(Client, StoredSession), String>> {
    let stored = load()?;
    Some(
        async {
            let client =
                build_client(&stored.homeserver, &stored.store_dir, &stored.passphrase).await?;
            client
                .matrix_auth()
                .restore_session(
                    stored.session.clone(),
                    matrix_sdk::store::RoomLoadSettings::default(),
                )
                .await
                .map_err(|e| format!("cannot restore the session: {e}"))?;
            Ok((client, stored))
        }
        .await,
    )
}

/// Signs in with a password, then saves the session.
pub async fn sign_in_password(
    homeserver: &str,
    username: &str,
    password: &str,
) -> Result<Client, String> {
    let (store_dir, passphrase) = new_store();
    let client = build_client(homeserver, &store_dir, &passphrase).await?;
    client
        .matrix_auth()
        .login_username(username, password)
        .initial_device_display_name(DEVICE_NAME)
        .await
        .map_err(|e| format!("sign-in failed: {e}"))?;
    persist(&client, homeserver, store_dir, passphrase)?;
    Ok(client)
}

/// Signs in through SSO in the system browser, redirected to a loopback listener.
pub async fn sign_in_sso(homeserver: &str) -> Result<Client, String> {
    let (store_dir, passphrase) = new_store();
    let client = build_client(homeserver, &store_dir, &passphrase).await?;
    client
        .matrix_auth()
        .login_sso(|url| async move {
            open::that(&url).map_err(|e| matrix_sdk::Error::UnknownError(Box::new(e)))?;
            Ok(())
        })
        .initial_device_display_name(DEVICE_NAME)
        .await
        .map_err(|e| format!("SSO sign-in failed: {e}"))?;
    persist(&client, homeserver, store_dir, passphrase)?;
    Ok(client)
}

fn persist(
    client: &Client,
    homeserver: &str,
    store_dir: PathBuf,
    passphrase: String,
) -> Result<(), String> {
    let session = client
        .matrix_auth()
        .session()
        .ok_or("no session after sign-in")?;
    save(&StoredSession {
        homeserver: homeserver.to_string(),
        passphrase,
        store_dir,
        session,
    })
}

/// Keeps the keychain copy current when the server refreshes the tokens.
pub fn on_tokens_refreshed(client: &Client) {
    let Some(mut stored) = load() else { return };
    if let Some(session) = client.matrix_auth().session() {
        stored.session = session;
        let _ = save(&stored);
    }
}
