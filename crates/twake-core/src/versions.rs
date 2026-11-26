//! The edit history of a message (web `readMessageVersions`): its first text, then each of
//! its author's corrections, oldest first.

use std::sync::Arc;

use matrix_sdk::Room;
use matrix_sdk::room::{IncludeRelations, RelationsOptions};
use matrix_sdk::ruma::events::relation::RelationType;
use matrix_sdk::ruma::{EventId, UInt};
use matrix_sdk_ui::timeline::Timeline;
use serde_json::Value;

use crate::forward::strip_reply_fallback;

/// One version of a message: its text and when it was written (web `MessageVersion`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageVersion {
    pub text: String,
    /// Epoch milliseconds.
    pub sent_at: i64,
}

/// The text of a version: its body, without a reply's quoted lines (web `readVersionText`).
fn version_text(content: Option<&Value>) -> String {
    let body = content
        .and_then(|c| c.get("body"))
        .and_then(Value::as_str)
        .unwrap_or("");
    strip_reply_fallback(body)
}

fn sent_at(event: &Value) -> i64 {
    event
        .get("origin_server_ts")
        .and_then(Value::as_i64)
        .unwrap_or(0)
}

/// The versions of `event_id`, found in the room's or the open thread's timeline; empty when
/// the message is not there.
pub async fn versions(
    room: &Room,
    timelines: &[Arc<Timeline>],
    event_id: &str,
) -> Vec<MessageVersion> {
    let Ok(event_id) = EventId::parse(event_id) else {
        return Vec::new();
    };
    let mut found = None;
    for timeline in timelines {
        if let Some(item) = timeline.item_by_event_id(&event_id).await {
            found = Some(item);
            break;
        }
    }
    let Some(item) = found else {
        return Vec::new();
    };
    let json = |raw: &matrix_sdk::ruma::serde::Raw<_>| raw.deserialize_as::<Value>().ok();
    let Some(original) = item.original_json().and_then(json) else {
        return Vec::new();
    };
    let sender = original.get("sender").cloned();
    let options = RelationsOptions {
        include_relations: IncludeRelations::RelationsOfType(RelationType::Replacement),
        limit: Some(UInt::from(100u32)),
        ..RelationsOptions::default()
    };
    let edits: Vec<Value> = match room.relations(event_id.clone(), options).await {
        Ok(relations) => relations
            .chunk
            .iter()
            .filter_map(|event| event.raw().deserialize_as::<Value>().ok())
            .collect(),
        // The edit the timeline holds stands in (web)
        Err(_) => item.latest_edit_json().and_then(json).into_iter().collect(),
    };
    let mut edits: Vec<MessageVersion> = edits
        .iter()
        .filter(|edit| edit.get("sender").cloned() == sender)
        .map(|edit| MessageVersion {
            text: version_text(edit.get("content").and_then(|c| c.get("m.new_content"))),
            sent_at: sent_at(edit),
        })
        .collect();
    edits.sort_by_key(|v| v.sent_at);
    let mut all = vec![MessageVersion {
        text: version_text(original.get("content")),
        sent_at: sent_at(&original),
    }];
    all.extend(edits);
    all
}
