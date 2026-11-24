//! Saved messages (web `MatrixAccountPrefs`, D59): the list lives in the account data
//! `app.twake.chat.saved_messages`; each change, here or on another device, reaches the
//! window as [`Event::Saved`], the text of each message found in the search index (what
//! this device has read).

use std::sync::Arc;

use matrix_sdk::Client;
use matrix_sdk::ruma::events::{AnyGlobalAccountDataEvent, GlobalAccountDataEventType};
use matrix_sdk::ruma::serde::Raw;
use twake_domain::saved::{SAVED_MESSAGES_TYPE, SavedRef, read_saved_refs, with_saved};

use crate::engine::{Emit, Event, SavedEntry};
use crate::search_index::SearchIndex;

fn kind() -> GlobalAccountDataEventType {
    GlobalAccountDataEventType::from(SAVED_MESSAGES_TYPE)
}

async fn current(client: &Client) -> Vec<SavedRef> {
    let raw = client
        .account()
        .account_data_raw(kind())
        .await
        .ok()
        .flatten();
    raw.and_then(|r| r.deserialize_as::<serde_json::Value>().ok())
        .map(|v| read_saved_refs(&v))
        .unwrap_or_default()
}

async fn report(refs: Vec<SavedRef>, index: &SearchIndex, emit: &Emit) {
    let mut entries = Vec::with_capacity(refs.len());
    for r in refs {
        let found = index.entry(&r.room_id, &r.event_id).await;
        entries.push(SavedEntry {
            quote: found.map(|m| (m.sender_name, m.text)),
            room_id: r.room_id,
            event_id: r.event_id,
            saved_at: r.saved_at,
        });
    }
    emit(Event::Saved(entries));
}

/// Reports the saved messages now and after each change of the list.
pub async fn start(client: Client, index: Arc<SearchIndex>, emit: Emit) {
    report(current(&client).await, &index, &emit).await;
    let handler_index = index.clone();
    let handler_emit = emit.clone();
    client.add_event_handler(move |raw: Raw<AnyGlobalAccountDataEvent>| {
        let index = handler_index.clone();
        let emit = handler_emit.clone();
        async move {
            let Ok(value) = raw.deserialize_as::<serde_json::Value>() else {
                return;
            };
            if value.get("type").and_then(|t| t.as_str()) != Some(SAVED_MESSAGES_TYPE) {
                return;
            }
            let refs = value
                .get("content")
                .map(read_saved_refs)
                .unwrap_or_default();
            report(refs, &index, &emit).await;
        }
    });
}

/// Saves or forgets a message; the window hears it back as [`Event::Saved`].
pub async fn set(
    client: &Client,
    index: &SearchIndex,
    emit: &Emit,
    room_id: &str,
    event_id: &str,
    saved: bool,
) -> Result<(), String> {
    let now = chrono_now_ms();
    let refs = with_saved(&current(client).await, room_id, event_id, saved, now);
    // `savedAt` as a whole number of milliseconds, as the web's `Date.now()` writes it
    let messages: Vec<serde_json::Value> = refs
        .iter()
        .map(|r| {
            let at = if r.saved_at.fract() == 0.0 && r.saved_at.abs() < 9e15 {
                serde_json::json!(r.saved_at as i64)
            } else {
                serde_json::json!(r.saved_at)
            };
            serde_json::json!({ "roomId": r.room_id, "eventId": r.event_id, "savedAt": at })
        })
        .collect();
    let content = serde_json::json!({ "messages": messages });
    let raw = Raw::new(&content)
        .map_err(|e| e.to_string())?
        .cast_unchecked();
    client
        .account()
        .set_account_data_raw(kind(), raw)
        .await
        .map_err(|e| e.to_string())?;
    // Shown at once, before the sync echoes it
    report(refs, index, emit).await;
    Ok(())
}

fn chrono_now_ms() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0.0, |d| d.as_millis() as f64)
}
