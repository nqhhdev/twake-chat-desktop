//! Chats deleted for the user alone (web `domain/deletedChats.ts`, D186): the account data
//! `app.twake.chat.deleted_chats` keeps, by room, the time before which its history is
//! hidden; the room leaves the list until something newer happens in it.

use std::collections::HashMap;

use crate::Message;

/// The account data that keeps them (web `sdkTypes`).
pub const DELETED_CHATS_TYPE: &str = "app.twake.chat.deleted_chats";

/// By room: the time (epoch milliseconds) up to which its history is hidden (`before_ts`).
pub type DeletedChats = HashMap<String, f64>;

/// What the account data holds, once read: entries without a numeric `before_ts` are
/// forgotten (web `readDeletedChats`).
pub fn read_deleted_chats(content: &serde_json::Value) -> DeletedChats {
    // `Object.entries`: an array's entries are keyed by their index
    let entries: Vec<(String, &serde_json::Value)> = match content {
        serde_json::Value::Object(o) => o.iter().map(|(k, v)| (k.clone(), v)).collect(),
        serde_json::Value::Array(a) => a
            .iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), v))
            .collect(),
        _ => return DeletedChats::new(),
    };
    entries
        .into_iter()
        .filter_map(|(room_id, entry)| {
            let before = entry.get("before_ts")?.as_f64()?;
            Some((room_id, before))
        })
        .collect()
}

/// The account data content for `deleted` (web `saveDeletedChat`): `before_ts` a whole
/// number of milliseconds when it is one, as `Date.now()` writes it.
pub fn deleted_chats_content(deleted: &DeletedChats) -> serde_json::Value {
    let entries: serde_json::Map<String, serde_json::Value> = deleted
        .iter()
        .map(|(room_id, &before)| {
            let before = if before.fract() == 0.0 && before.abs() < 9e15 {
                serde_json::json!(before as i64)
            } else {
                serde_json::json!(before)
            };
            (room_id.clone(), serde_json::json!({ "before_ts": before }))
        })
        .collect();
    serde_json::Value::Object(entries)
}

/// Deleted, and nothing newer since: the room stays out of the list (web `isChatHidden`).
pub fn is_chat_hidden(deleted: &DeletedChats, room_id: &str, last_activity_at: f64) -> bool {
    deleted
        .get(room_id)
        .is_some_and(|&before| last_activity_at <= before)
}

/// The messages the user did not delete (web `withoutDeleted`).
pub fn without_deleted<'a>(
    deleted: &DeletedChats,
    room_id: &str,
    messages: &'a [Message],
) -> Vec<&'a Message> {
    match deleted.get(room_id) {
        Some(&before) => messages
            .iter()
            .filter(|m| m.sent_at as f64 > before)
            .collect(),
        None => messages.iter().collect(),
    }
}

/// Something that happened at `at` in the room, newer than its deletion (web
/// `isAfterDeletion`).
pub fn is_after_deletion(deleted: &DeletedChats, room_id: &str, at: f64) -> bool {
    deleted.get(room_id).is_none_or(|&before| at > before)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_entries_with_a_time() {
        let read = read_deleted_chats(&serde_json::json!({
            "!a": { "before_ts": 1000 }, "!b": { "before_ts": "1000" }, "!c": 3
        }));
        assert_eq!(read, DeletedChats::from([("!a".to_string(), 1000.0)]));
        assert!(read_deleted_chats(&serde_json::json!([1, 2])).is_empty());
        assert_eq!(
            read_deleted_chats(&serde_json::json!([{ "before_ts": 5 }])).len(),
            1
        );
        // Written back as the web writes it
        assert_eq!(
            deleted_chats_content(&read),
            serde_json::json!({ "!a": { "before_ts": 1000 } })
        );
    }

    #[test]
    fn hides_the_room_until_something_newer() {
        let deleted = DeletedChats::from([("!a".to_string(), 1000.0)]);
        assert!(is_chat_hidden(&deleted, "!a", 1000.0));
        assert!(!is_chat_hidden(&deleted, "!a", 1001.0));
        assert!(!is_chat_hidden(&deleted, "!b", 0.0));
        assert!(!is_after_deletion(&deleted, "!a", 1000.0));
        assert!(is_after_deletion(&deleted, "!a", 1001.0));
        assert!(is_after_deletion(&deleted, "!b", 0.0));
    }

    #[test]
    fn drops_the_messages_up_to_the_deletion() {
        let message = |id: &str, sent_at: i64| Message {
            id: id.into(),
            sent_at,
            ..Message::default()
        };
        let messages = [message("old", 1000), message("new", 1001)];
        let deleted = DeletedChats::from([("!a".to_string(), 1000.0)]);
        let kept: Vec<&str> = without_deleted(&deleted, "!a", &messages)
            .iter()
            .map(|m| m.id.as_str())
            .collect();
        assert_eq!(kept, ["new"]);
        assert_eq!(without_deleted(&deleted, "!b", &messages).len(), 2);
    }
}
