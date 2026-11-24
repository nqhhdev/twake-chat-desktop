//! Saved messages (web `domain/savedMessages.ts`, D59): ids in the account data
//! `app.twake.chat.saved_messages`, so they follow the user from device to device; the
//! text is looked up on the device.

use serde::{Deserialize, Serialize};

/// The account data type holding them.
pub const SAVED_MESSAGES_TYPE: &str = "app.twake.chat.saved_messages";
/// The oldest are dropped past this.
pub const MAX_SAVED: usize = 200;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedRef {
    pub room_id: String,
    pub event_id: String,
    pub saved_at: f64,
}

/// What the device knows of a saved message, for its line in the Activity.
#[derive(Clone, Debug, PartialEq)]
pub struct SavedQuote {
    pub sender_name: String,
    pub sender_avatar_url: Option<String>,
    /// Its text, or the name of its file.
    pub text: String,
}

/// A saved message with what the history held on this device.
#[derive(Clone, Debug, PartialEq)]
pub struct SavedMessage {
    pub room_id: String,
    pub room_name: String,
    pub event_id: String,
    pub saved_at: f64,
    /// None when the message is not in the history loaded here.
    pub message: Option<SavedQuote>,
}

fn is_ref(entry: &serde_json::Value) -> bool {
    entry.get("roomId").is_some_and(|v| v.is_string())
        && entry.get("eventId").is_some_and(|v| v.is_string())
        && entry.get("savedAt").is_some_and(|v| v.is_number())
}

/// `readSavedRefs`: what the account data holds, whatever a client wrote there.
pub fn read_saved_refs(content: &serde_json::Value) -> Vec<SavedRef> {
    content
        .get("messages")
        .and_then(|m| m.as_array())
        .map(|list| {
            list.iter()
                .filter(|e| is_ref(e))
                .filter_map(|e| serde_json::from_value(e.clone()).ok())
                .collect()
        })
        .unwrap_or_default()
}

/// `withSaved`: saves or forgets one message; the newest come first.
pub fn with_saved(
    refs: &[SavedRef],
    room_id: &str,
    event_id: &str,
    is_saved: bool,
    now: f64,
) -> Vec<SavedRef> {
    let others = refs
        .iter()
        .filter(|r| r.room_id != room_id || r.event_id != event_id)
        .cloned();
    let first = is_saved.then(|| SavedRef {
        room_id: room_id.to_string(),
        event_id: event_id.to_string(),
        saved_at: now,
    });
    first.into_iter().chain(others).take(MAX_SAVED).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_and_forgets() {
        let content = serde_json::json!({"messages": [
            {"roomId": "!a", "eventId": "$1", "savedAt": 1},
            {"roomId": "!a", "eventId": 2, "savedAt": 1}
        ]});
        let refs = read_saved_refs(&content);
        assert_eq!(refs.len(), 1);
        let more = with_saved(&refs, "!b", "$2", true, 5.0);
        assert_eq!(more[0].event_id, "$2");
        assert_eq!(with_saved(&more, "!a", "$1", false, 6.0).len(), 1);
    }
}
