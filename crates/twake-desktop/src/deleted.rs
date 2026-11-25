//! Chats deleted for the user alone (web `deleteChat`, D186): the timeline from the deletion
//! on, and the line that tells it.

use std::borrow::Cow;

use twake_domain::TimelineEntry;
use twake_domain::deleted_chats::{DeletedChats, is_after_deletion};

/// The entries of `room_id` after its deletion (web `withoutDeleted`, for the events too); a
/// day divider left with nothing under it goes. Borrowed when the chat was never deleted.
pub fn visible<'a>(
    entries: &'a [TimelineEntry],
    room_id: &str,
    deleted: &DeletedChats,
) -> Cow<'a, [TimelineEntry]> {
    if !deleted.contains_key(room_id) {
        return Cow::Borrowed(entries);
    }
    let kept: Vec<&TimelineEntry> = entries
        .iter()
        .filter(|e| match e {
            TimelineEntry::Message(m) => is_after_deletion(deleted, room_id, m.sent_at as f64),
            TimelineEntry::Event { at, .. } => is_after_deletion(deleted, room_id, *at as f64),
            TimelineEntry::Day { .. } | TimelineEntry::ReadMarker => true,
        })
        .collect();
    let shown = |i: usize| {
        kept[i + 1..]
            .iter()
            .take_while(|e| !matches!(e, TimelineEntry::Day { .. }))
            .any(|e| matches!(e, TimelineEntry::Message(_) | TimelineEntry::Event { .. }))
    };
    Cow::Owned(
        kept.iter()
            .enumerate()
            .filter(|(i, e)| !matches!(e, TimelineEntry::Day { .. }) || shown(*i))
            .map(|(_, e)| (*e).clone())
            .collect(),
    )
}

/// « You deleted the earlier messages on November 25, 2026 » (web `chat.deletedBefore`,
/// `dateStyle: 'long'`), or "" when the chat was never deleted.
pub fn notice(room_id: &str, deleted: &DeletedChats) -> String {
    deleted
        .get(room_id)
        .and_then(|&before| chrono::DateTime::from_timestamp_millis(before as i64))
        .map(|at| {
            let day = at.with_timezone(&chrono::Local).format("%B %-d, %Y");
            format!("You deleted the earlier messages on {day}")
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use twake_domain::Message;

    fn message(sent_at: i64) -> TimelineEntry {
        TimelineEntry::Message(Message {
            id: sent_at.to_string(),
            sent_at,
            ..Message::default()
        })
    }

    #[test]
    fn hides_the_history_up_to_the_deletion() {
        let entries = [
            TimelineEntry::Day { at: 0 },
            message(10),
            TimelineEntry::Day { at: 100 },
            message(110),
            message(120),
        ];
        let deleted = DeletedChats::from([("!a".to_string(), 110.0)]);
        let kept = visible(&entries, "!a", &deleted);
        // The first day has nothing left: it goes with its message
        assert_eq!(kept.len(), 2);
        assert!(matches!(kept[0], TimelineEntry::Day { at: 100 }));
        assert!(matches!(&kept[1], TimelineEntry::Message(m) if m.sent_at == 120));
        assert!(matches!(
            visible(&entries, "!b", &deleted),
            Cow::Borrowed(_)
        ));
    }
}
