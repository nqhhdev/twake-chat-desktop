//! « Autoplay animated images » (web `MatrixAccountPrefs.fetchAutoplay`): `{enabled}` in
//! the account data `app.twake.chat.autoplay`, so the choice made on the web holds here.
//! Each change reaches the window as [`Event::Autoplay`].

use matrix_sdk::Client;
use matrix_sdk::ruma::events::{AnyGlobalAccountDataEvent, GlobalAccountDataEventType};
use matrix_sdk::ruma::serde::Raw;

use crate::engine::{Emit, Event};

/// Web `domain/extensions.ts`, `AUTOPLAY_TYPE`.
const AUTOPLAY_TYPE: &str = "app.twake.chat.autoplay";

/// `enabled` when it is a boolean (the web reads nothing else).
fn enabled(content: &serde_json::Value) -> Option<bool> {
    content.get("enabled")?.as_bool()
}

/// Reports the choice now and after each change of it.
pub async fn start(client: Client, emit: Emit) {
    let raw = client
        .account()
        .account_data_raw(GlobalAccountDataEventType::from(AUTOPLAY_TYPE))
        .await
        .ok()
        .flatten();
    let now = raw
        .and_then(|r| r.deserialize_as::<serde_json::Value>().ok())
        .and_then(|v| enabled(&v));
    emit(Event::Autoplay(now));
    client.add_event_handler(move |raw: Raw<AnyGlobalAccountDataEvent>| {
        let emit = emit.clone();
        async move {
            let Ok(value) = raw.deserialize_as::<serde_json::Value>() else {
                return;
            };
            if value.get("type").and_then(|t| t.as_str()) == Some(AUTOPLAY_TYPE) {
                emit(Event::Autoplay(value.get("content").and_then(enabled)));
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_enabled_as_the_web() {
        assert_eq!(enabled(&serde_json::json!({"enabled": false})), Some(false));
        assert_eq!(enabled(&serde_json::json!({"enabled": "no"})), None);
        assert_eq!(enabled(&serde_json::json!({})), None);
    }
}
