//! The shapes the screens draw. Field names follow the web app's domain
//! (`room.ts`, `roomPreview.ts`, `message.ts`) so serde JSON matches its recordings.

use serde::{Deserialize, Serialize};

/// `RoomKind` of `room.ts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RoomKind {
    Direct,
    Discussion,
    Broadcast,
}

/// `RoomPreviewKind` of `roomPreview.ts` (the subset M1 draws).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PreviewKind {
    Text,
    Image,
    Video,
    Audio,
    File,
    Voice,
    Poll,
    Encrypted,
    Deleted,
}

/// `RoomPreview` of `roomPreview.ts`: the last thing said in a room.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomPreview {
    pub kind: PreviewKind,
    pub text: String,
    pub sender_name: String,
    pub is_own: bool,
    /// Epoch milliseconds.
    pub sent_at: i64,
}

/// One row of the room list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomSummary {
    pub id: String,
    pub name: String,
    pub kind: RoomKind,
    /// `mxc://` URI, if any.
    pub avatar_url: Option<String>,
    pub is_encrypted: bool,
    pub is_invite: bool,
    pub unread_count: u64,
    pub mention_count: u64,
    pub is_marked_unread: bool,
    pub preview: Option<RoomPreview>,
    /// Epoch milliseconds of the newest event, for ordering and the time column.
    pub last_activity: Option<i64>,
}

/// `sendStatus` of `message.ts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SendStatus {
    Sending,
    Sent,
    Failed,
}

/// `UndecryptableReason` of `encryption.ts` (the cases M1 tells apart).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Undecryptable {
    /// Keys not received yet, or not shared with this device.
    MissingKeys,
    /// Sent before this device joined, or withheld on purpose.
    Withheld,
    Other,
}

/// `MessageNotice` of `message.ts`: drawn as a line instead of a bubble.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageNotice {
    Deleted,
    Unsupported,
}

/// A message of the timeline: the subset of `Message` of `message.ts` M1 draws.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// Event id, or the transaction id of a local echo.
    pub id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub sender_avatar_url: Option<String>,
    /// Plain text body. Empty when unreadable.
    pub text: String,
    /// `org.matrix.custom.html`, untrusted: go through [`crate::rich_text`].
    pub html: Option<String>,
    pub undecryptable: Option<Undecryptable>,
    pub notice: Option<MessageNotice>,
    /// The file the message carries; `text` is then its caption, empty without one.
    pub attachment: Option<crate::attachment::Attachment>,
    pub is_emote: bool,
    /// Epoch milliseconds.
    pub sent_at: i64,
    pub is_own: bool,
    pub is_edited: bool,
    pub send_status: Option<SendStatus>,
    /// Reactions, in the order they were first given.
    pub reactions: Vec<Reaction>,
    /// The message this one answers (`replyTo` of `message.ts`).
    pub reply_to: Option<ReplyPreview>,
    /// The thread this message opens, when it has replies (`thread` of `message.ts`).
    pub thread: Option<ThreadSummary>,
    /// The signed-in user may edit it (own text, sent).
    pub can_edit: bool,
    /// A poll (`text` is then its question).
    pub poll: Option<crate::poll::PollView>,
    /// Pinned in the room.
    pub is_pinned: bool,
    /// The signed-in user may pin or unpin it.
    pub can_pin: bool,
    /// It opens the thread of a call (web `isCallThreadRoot`, D105).
    #[serde(default)]
    pub is_call_thread_root: bool,
    /// The others whose read receipt stops here (not the sender, not the user).
    #[serde(default)]
    pub read_receipts: Vec<ReadReceipt>,
}

/// One reaction key on a message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub key: String,
    pub count: u32,
    /// The signed-in user gave it: clicking it takes it back.
    pub is_own: bool,
}

/// The message a reply answers, as shown above the reply.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyPreview {
    pub event_id: String,
    /// Empty while the original is being fetched.
    pub sender_name: String,
    pub text: String,
}

/// The thread a message opens.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadSummary {
    pub reply_count: u32,
    pub latest_sender: String,
    pub latest_text: String,
}

/// One row of the timeline.
// Nearly every row is a message: boxing it would add an allocation per row for nothing.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TimelineEntry {
    Message(Message),
    /// The day divider; epoch milliseconds of the day.
    Day {
        at: i64,
    },
    /// Where the user stopped reading.
    ReadMarker,
    /// A membership or room change, drawn as a small centred line.
    Event {
        id: String,
        text: String,
        at: i64,
    },
}

/// Everything the room info pane shows (`InfoPanel` of the web).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomDetails {
    pub room_id: String,
    pub name: String,
    pub topic: Option<String>,
    pub avatar_url: Option<String>,
    pub kind: RoomKind,
    pub is_encrypted: bool,
    pub joined_count: u64,
    pub own_user_id: String,
    /// Joined and invited, owners and admins first, invited last.
    pub members: Vec<crate::room_management::RoomMemberDetail>,
    pub permissions: crate::room_management::RoomPermissions,
    /// Who may join (web `RoomAccess`), and whether the public directory lists it.
    #[serde(default)]
    pub join_rule: crate::room_management::JoinRuleChoice,
    #[serde(default)]
    pub is_listed: bool,
}

/// A message on the server and not deleted can be ticked to act on several at once (web
/// `isSelectable`). `is_call_line`: the line a call leaves (web `message.call`), which this
/// app draws apart from the messages. No send status: someone else's message, on the server.
pub fn is_selectable(message: &Message, is_call_line: bool) -> bool {
    message.send_status.is_none_or(|s| s == SendStatus::Sent)
        && message.notice != Some(MessageNotice::Deleted)
        && !is_call_line
}

/// Someone else whose read receipt stops at a message (web `MessageReader`): they read it
/// and everything before.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadReceipt {
    pub user_id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    /// When their receipt was sent (epoch milliseconds); None if unknown.
    pub read_at: Option<i64>,
}

/// A top-level space and every room below it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceSummary {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub rooms: Vec<String>,
}

#[cfg(test)]
mod selectable_tests {
    use super::*;

    #[test]
    fn ticks_only_messages_on_the_server() {
        let sent = Message::default();
        assert!(is_selectable(&sent, false));
        assert!(!is_selectable(&sent, true));
        let sending = Message {
            send_status: Some(SendStatus::Sending),
            ..Message::default()
        };
        assert!(!is_selectable(&sending, false));
        let deleted = Message {
            notice: Some(MessageNotice::Deleted),
            ..Message::default()
        };
        assert!(!is_selectable(&deleted, false));
    }
}
