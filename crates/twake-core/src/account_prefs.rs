//! Choices kept in the account data, shared with the web (web `MatrixAccountPrefs`): the
//! composer style, and the writes of the « Autoplay animated images » switch.

use matrix_sdk::Client;
use matrix_sdk::ruma::events::{AnyGlobalAccountDataEvent, GlobalAccountDataEventType};
use matrix_sdk::ruma::serde::Raw;
use twake_domain::composer_style::{COMPOSER_STYLE_TYPE, read_composer_style};

use crate::engine::{Emit, Event};

/// Writes `content` as the account data `kind`.
pub async fn set(client: &Client, kind: &str, content: serde_json::Value) -> Result<(), String> {
    let raw = Raw::new(&content)
        .map_err(|e| e.to_string())?
        .cast_unchecked();
    client
        .account()
        .set_account_data_raw(GlobalAccountDataEventType::from(kind), raw)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Reports the composer style now and after each change of it (here or on the web).
pub async fn watch_composer_style(client: Client, emit: Emit) {
    let raw = client
        .account()
        .account_data_raw(GlobalAccountDataEventType::from(COMPOSER_STYLE_TYPE))
        .await
        .ok()
        .flatten();
    let now = raw
        .and_then(|r| r.deserialize_as::<serde_json::Value>().ok())
        .unwrap_or_default();
    emit(Event::ComposerStyle(read_composer_style(&now).to_string()));
    client.add_event_handler(move |raw: Raw<AnyGlobalAccountDataEvent>| {
        let emit = emit.clone();
        async move {
            let Ok(value) = raw.deserialize_as::<serde_json::Value>() else {
                return;
            };
            if value.get("type").and_then(|t| t.as_str()) == Some(COMPOSER_STYLE_TYPE) {
                let content = value.get("content").cloned().unwrap_or_default();
                emit(Event::ComposerStyle(
                    read_composer_style(&content).to_string(),
                ));
            }
        }
    });
}
