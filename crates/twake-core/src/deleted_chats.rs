//! Chats deleted for the user alone (web `MatrixAccountPrefs`, D186): the account data
//! `app.twake.chat.deleted_chats`, shared with the web; each change, here or on another
//! device, reaches the window as [`Event::DeletedChats`].

use matrix_sdk::Client;
use matrix_sdk::ruma::events::{AnyGlobalAccountDataEvent, GlobalAccountDataEventType};
use matrix_sdk::ruma::serde::Raw;
use twake_domain::deleted_chats::{
    DELETED_CHATS_TYPE, DeletedChats, deleted_chats_content, read_deleted_chats,
};

use crate::engine::{Emit, Event};

fn kind() -> GlobalAccountDataEventType {
    GlobalAccountDataEventType::from(DELETED_CHATS_TYPE)
}

async fn current(client: &Client) -> DeletedChats {
    let raw = client
        .account()
        .account_data_raw(kind())
        .await
        .ok()
        .flatten();
    raw.and_then(|r| r.deserialize_as::<serde_json::Value>().ok())
        .map(|v| read_deleted_chats(&v))
        .unwrap_or_default()
}

/// Reports the deleted chats now and after each change.
pub async fn start(client: Client, emit: Emit) {
    emit(Event::DeletedChats(current(&client).await));
    let handler_emit = emit.clone();
    client.add_event_handler(move |raw: Raw<AnyGlobalAccountDataEvent>| {
        let emit = handler_emit.clone();
        async move {
            let Ok(value) = raw.deserialize_as::<serde_json::Value>() else {
                return;
            };
            if value.get("type").and_then(|t| t.as_str()) != Some(DELETED_CHATS_TYPE) {
                return;
            }
            let deleted = value
                .get("content")
                .map(read_deleted_chats)
                .unwrap_or_default();
            emit(Event::DeletedChats(deleted));
        }
    });
}

/// Deletes the chat up to `before_ts` (web `saveDeletedChat`), or brings it back with None
/// (web `removeDeletedChat`); the window hears it back as [`Event::DeletedChats`].
pub async fn set(
    client: &Client,
    emit: &Emit,
    room_id: &str,
    before_ts: Option<f64>,
) -> Result<(), String> {
    let mut deleted = current(client).await;
    match before_ts {
        Some(before) => deleted.insert(room_id.to_string(), before),
        None => deleted.remove(room_id),
    };
    let content = deleted_chats_content(&deleted);
    let raw = Raw::new(&content)
        .map_err(|e| e.to_string())?
        .cast_unchecked();
    client
        .account()
        .set_account_data_raw(kind(), raw)
        .await
        .map_err(|e| e.to_string())?;
    // Shown at once, before the sync echoes it
    emit(Event::DeletedChats(deleted));
    Ok(())
}
