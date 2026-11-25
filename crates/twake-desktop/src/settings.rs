//! Settings (web `SettingsScreen`): the theme is a choice of this device (web
//! `localStorage`, here `display.json` in the data folder); autoplay and the composer style
//! are kept in the account, shared with the web.

use slint::ComponentHandle;
use twake_core::Command;

use crate::{AppWindow, send};

fn file() -> std::path::PathBuf {
    twake_core::data_dir().join("display.json")
}

/// The theme kept on this device: "system", "light" or "dark".
fn kept_theme() -> String {
    std::fs::read_to_string(file())
        .ok()
        .and_then(|text| theme_in(&text))
        .unwrap_or_else(|| "system".to_string())
}

/// The theme of `{"theme":"dark"}`, the one key the file holds (the app has no JSON library).
fn theme_in(text: &str) -> Option<String> {
    ["system", "light", "dark"]
        .into_iter()
        .find(|mode| text.contains(&format!("\"theme\":\"{mode}\"")))
        .map(str::to_string)
}

pub fn install(window: &AppWindow) {
    window.set_theme_mode(kept_theme().into());
    window.on_open_settings({
        let weak = window.as_weak();
        move || {
            if let Some(w) = weak.upgrade() {
                w.set_settings_open(true);
            }
        }
    });
    window.on_set_theme({
        let weak = window.as_weak();
        move |mode| {
            if let Some(w) = weak.upgrade() {
                w.set_theme_mode(mode.clone());
            }
            let file = file();
            if let Some(folder) = file.parent() {
                let _ = std::fs::create_dir_all(folder);
            }
            let _ = std::fs::write(file, format!("{{\"theme\":\"{mode}\"}}"));
        }
    });
    window.on_set_autoplay(|on| send(Command::SetAutoplay(on)));
    window.on_set_composer(|style| send(Command::SetComposerStyle(style.to_string())));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_kept_theme() {
        assert_eq!(theme_in(r#"{"theme":"dark"}"#).as_deref(), Some("dark"));
        assert_eq!(theme_in("{}"), None);
    }
}
