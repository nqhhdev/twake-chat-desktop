//! Seeds an end-to-end encrypted conversation on the local Synapse (dev/up.sh), for testing
//! the recovery key flow of the desktop app (M1). Local test accounts only.
//!
//! - alice gets a first device ("e2ee-seed") with cross-signing and recovery set up;
//!   her recovery key is written to `dev/.alice-recovery-key` (git-ignored).
//! - bob creates the encrypted room "Secret project" with alice; both send messages,
//!   and alice's device backs the room keys up.
//!
//! The desktop app, signed in as alice on a new device, then shows those messages as
//! unreadable until the recovery key is entered.
//!
//! Run: `cargo run -p twake-core --example e2ee_seed`

use std::time::Duration;

use matrix_sdk::{
    Client,
    config::SyncSettings,
    ruma::{
        OwnedUserId,
        api::client::{room::create_room::v3::Request as CreateRoom, uiaa},
        events::{
            InitialStateEvent,
            room::{encryption::RoomEncryptionEventContent, message::RoomMessageEventContent},
        },
    },
};

const HS: &str = "http://localhost:18008";

async fn login(user: &str, dir: &std::path::Path) -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .homeserver_url(HS)
        .sqlite_store(dir.join(user), None)
        .build()
        .await?;
    client
        .matrix_auth()
        .login_username(user, &format!("{user}-dev-password"))
        .initial_device_display_name("e2ee-seed")
        .await?;
    client.sync_once(SyncSettings::default()).await?;
    Ok(client)
}

async fn sync(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    client
        .sync_once(SyncSettings::default().timeout(Duration::from_secs(1)))
        .await?;
    Ok(())
}

async fn cross_sign(client: &Client, user: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = client.encryption().bootstrap_cross_signing(None).await {
        let Some(response) = e.as_uiaa_response() else {
            return Err(e.into());
        };
        let mut password = uiaa::Password::new(
            uiaa::UserIdentifier::Matrix(uiaa::MatrixUserIdentifier::new(user.to_owned())),
            format!("{user}-dev-password"),
        );
        password.session = response.session.clone();
        client
            .encryption()
            .bootstrap_cross_signing(Some(uiaa::AuthData::Password(password)))
            .await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::temp_dir().join(format!("twake-e2ee-seed-{}", std::process::id()));
    let alice = login("alice", &dir).await?;
    let bob = login("bob", &dir).await?;

    cross_sign(&alice, "alice").await?;
    cross_sign(&bob, "bob").await?;
    let key = alice
        .encryption()
        .recovery()
        .enable()
        .wait_for_backups_to_upload()
        .await?;
    let key_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dev/.alice-recovery-key");
    std::fs::write(&key_path, &key)?;
    println!("alice's recovery key written to {}", key_path.display());

    let alice_id: OwnedUserId = alice.user_id().ok_or("no user id")?.to_owned();
    let mut request = CreateRoom::new();
    request.name = Some("Secret project".to_owned());
    request.invite = vec![alice_id];
    request.is_direct = false;
    request.initial_state = vec![
        InitialStateEvent::with_empty_state_key(
            RoomEncryptionEventContent::with_recommended_defaults(),
        )
        .to_raw_any(),
    ];
    let room = bob.create_room(request).await?;
    sync(&alice).await?;
    alice.join_room_by_id(room.room_id()).await?;
    sync(&bob).await?;

    for text in [
        "This room is end-to-end encrypted.",
        "Only devices with the keys can read it.",
        "A new device needs the recovery key first.",
    ] {
        room.send(RoomMessageEventContent::text_plain(text)).await?;
    }
    for _ in 0..3 {
        sync(&alice).await?;
    }
    let alice_room = alice
        .get_room(room.room_id())
        .ok_or("alice cannot see the room")?;
    alice_room
        .send(RoomMessageEventContent::text_plain(
            "Got it, reading from my first device.",
        ))
        .await?;
    alice.encryption().backups().wait_for_steady_state().await?;
    println!(
        "Encrypted room {} seeded; alice's keys are backed up.",
        room.room_id()
    );
    let _ = std::fs::remove_dir_all(&dir);
    Ok(())
}
