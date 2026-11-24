//! The open room's timeline: matrix-sdk-ui's `Timeline` mapped to [`TimelineEntry`] rows.

use std::sync::Arc;

use futures_util::StreamExt;
use imbl::Vector;
use std::collections::HashSet;

use matrix_sdk::ruma::{
    OwnedUserId, UserId,
    events::room::message::{MessageFormat, MessageType},
    events::{StateEventType, poll::start::PollKind},
};
use matrix_sdk_ui::timeline::{
    EmbeddedEvent, EventSendState, EventTimelineItem, MembershipChange, MsgLikeContent,
    MsgLikeKind, Profile, Timeline, TimelineDetails, TimelineItem, TimelineItemContent,
    TimelineItemKind, VirtualTimelineItem,
};
use twake_domain::poll::{PollAnswerView, PollView};
use twake_domain::{
    Message, MessageNotice, PreviewKind, Reaction, ReplyPreview, SendStatus, ThreadSummary,
    TimelineEntry, Undecryptable,
};

use crate::engine::{Emit, Event};

/// Which timeline of a room: the main one, or the thread a message opens.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pane {
    Main,
    Thread(String),
}

/// Follows the timeline and emits the whole list of rows on every batch of changes.
pub async fn run(
    room_id: String,
    pane: Pane,
    timeline: Arc<Timeline>,
    index: Option<Arc<crate::search_index::SearchIndex>>,
    emit: Emit,
) {
    let room = timeline.room().clone();
    let own = room.own_user_id().to_owned();
    // Pinning is a moderator's action (web: ROLE_ACTIONS.moderator).
    let can_pin = room
        .power_levels()
        .await
        .map(|levels| levels.user_can_send_state(&own, StateEventType::RoomPinnedEvents))
        .unwrap_or(false);
    let (mut items, stream) = timeline.subscribe().await;
    let send = |items: &Vector<Arc<TimelineItem>>| {
        let ctx = Ctx {
            own: own.clone(),
            pinned: room
                .pinned_event_ids()
                .unwrap_or_default()
                .into_iter()
                .map(|id| id.to_string())
                .collect(),
            can_pin,
        };
        let entries = map_items(items, &ctx);
        // What the room holds now joins the search index (history pages included)
        let indexed: Vec<twake_domain::search::IndexedMessage> = entries
            .iter()
            .filter_map(|entry| match entry {
                TimelineEntry::Message(m) => twake_domain::search::to_indexed_message(&room_id, m),
                _ => None,
            })
            .collect();
        let pinned_count = ctx.pinned.len();
        match &pane {
            Pane::Main => emit(Event::Timeline {
                room_id: room_id.clone(),
                entries,
                pinned_count,
            }),
            Pane::Thread(root) => emit(Event::Thread {
                room_id: room_id.clone(),
                root_id: root.clone(),
                entries,
            }),
        }
        indexed
    };
    let indexed = send(&items);
    if let Some(index) = &index {
        index.save(&room_id, indexed).await;
    }
    futures_util::pin_mut!(stream);
    while let Some(diffs) = stream.next().await {
        for diff in diffs {
            diff.apply(&mut items);
        }
        let indexed = send(&items);
        if let Some(index) = &index {
            index.save(&room_id, indexed).await;
        }
    }
}

/// What the mapping needs beyond the items.
pub struct Ctx {
    pub own: OwnedUserId,
    pub pinned: HashSet<String>,
    pub can_pin: bool,
}

pub fn map_items(items: &Vector<Arc<TimelineItem>>, ctx: &Ctx) -> Vec<TimelineEntry> {
    items
        .iter()
        .filter_map(|item| map_item(item, ctx))
        .collect()
}

fn map_item(item: &TimelineItem, ctx: &Ctx) -> Option<TimelineEntry> {
    match item.kind() {
        TimelineItemKind::Virtual(VirtualTimelineItem::DateDivider(at)) => {
            Some(TimelineEntry::Day {
                at: i64::from(at.0),
            })
        }
        TimelineItemKind::Virtual(VirtualTimelineItem::ReadMarker) => {
            Some(TimelineEntry::ReadMarker)
        }
        TimelineItemKind::Virtual(_) => None,
        TimelineItemKind::Event(event) => map_event(event, ctx),
    }
}

fn reactions(msg_like: &MsgLikeContent, own: &UserId) -> Vec<Reaction> {
    msg_like
        .reactions
        .iter()
        .filter(|(_, senders)| !senders.is_empty())
        .map(|(key, senders)| Reaction {
            key: key.clone(),
            count: u32::try_from(senders.len()).unwrap_or(u32::MAX),
            is_own: senders.contains_key(own),
        })
        .collect()
}

/// One line of what an embedded event (a replied-to message, a thread's latest) says.
fn embedded_line(details: &TimelineDetails<Box<EmbeddedEvent>>) -> (String, String) {
    match details {
        TimelineDetails::Ready(event) => {
            let name = profile_name(&ready(&event.sender_profile), event.sender.as_str());
            let text = match &event.content {
                TimelineItemContent::MsgLike(m) => {
                    let (kind, text) = message_kind(m);
                    match kind {
                        PreviewKind::Text => twake_domain::room_preview::to_preview_text(&text),
                        PreviewKind::Encrypted => "🔒 Encrypted message".into(),
                        PreviewKind::Deleted => "Message deleted".into(),
                        _ => text,
                    }
                }
                _ => String::new(),
            };
            (name, text)
        }
        _ => (String::new(), String::new()),
    }
}

fn reply_preview(msg_like: &MsgLikeContent) -> Option<ReplyPreview> {
    let details = msg_like.in_reply_to.as_ref()?;
    let (sender_name, text) = embedded_line(&details.event);
    Some(ReplyPreview {
        event_id: details.event_id.to_string(),
        sender_name,
        text,
    })
}

fn thread_summary(msg_like: &MsgLikeContent) -> Option<ThreadSummary> {
    let summary = msg_like
        .thread_summary
        .as_ref()
        .filter(|s| s.num_replies > 0)?;
    let (latest_sender, latest_text) = embedded_line(&summary.latest_event);
    Some(ThreadSummary {
        reply_count: summary.num_replies,
        latest_sender,
        latest_text,
    })
}

/// The events the conversation shows: the SDK's, and the line of a call (a type of the
/// web's own, which the SDK leaves out).
pub(crate) fn shown_event(
    event: &matrix_sdk::ruma::events::AnySyncTimelineEvent,
    rules: &matrix_sdk::ruma::room_version_rules::RoomVersionRules,
) -> bool {
    matrix_sdk_ui::timeline::default_event_filter(event, rules)
        || event.event_type().to_string() == twake_domain::call::CALL_LINE_TYPE
}

/// « 📞 Outgoing Call · 1 min 5 sec », « 🎥 Incoming Video Call · Missed » (web
/// `chat.call.*`).
/// The message opens the thread of a call: its content has the mark (web
/// `isCallThreadRoot`). The raw text is looked at first, most messages have no mark.
fn is_call_thread_root(event: &EventTimelineItem) -> bool {
    event.latest_json().is_some_and(|raw| {
        raw.json()
            .get()
            .contains(twake_domain::call::CALL_THREAD_KEY)
            && raw
                .get_field::<serde_json::Value>("content")
                .ok()
                .flatten()
                .is_some_and(|c| c.get(twake_domain::call::CALL_THREAD_KEY).is_some())
    })
}

fn call_line_text(line: &twake_domain::call::CallLine) -> String {
    use twake_domain::call::CallOutcome;
    let icon = if line.is_video { "🎥" } else { "📞" };
    let way = if line.is_outgoing {
        "Outgoing"
    } else {
        "Incoming"
    };
    let kind = if line.is_video { "Video Call" } else { "Call" };
    let status = match line.outcome {
        CallOutcome::Answered { duration_ms } => {
            twake_domain::call::format_call_duration(duration_ms)
        }
        CallOutcome::Missed => "Missed".into(),
        CallOutcome::Canceled => "Canceled".into(),
    };
    format!("{icon} {way} {kind} · {status}")
}

fn event_id(event: &EventTimelineItem) -> String {
    event
        .event_id()
        .map(|id| id.to_string())
        .or_else(|| event.transaction_id().map(|id| id.to_string()))
        .unwrap_or_default()
}

fn map_event(event: &EventTimelineItem, ctx: &Ctx) -> Option<TimelineEntry> {
    let own: &UserId = &ctx.own;
    let sender_name = profile_name(&ready(event.sender_profile()), event.sender().as_str());
    let at = i64::from(event.timestamp().0);
    match event.content() {
        // The line of a call, as its caller wrote it when it ended (web `CallMessage`)
        TimelineItemContent::MsgLike(msg_like)
            if matches!(&msg_like.kind, MsgLikeKind::Other(o)
                if o.event_type().to_string() == twake_domain::call::CALL_LINE_TYPE) =>
        {
            let content = event
                .latest_json()?
                .get_field::<serde_json::Value>("content")
                .ok()??;
            let line = twake_domain::call::to_call_line(&content, event.is_own())?;
            Some(TimelineEntry::Event {
                id: event_id(event),
                text: call_line_text(&line),
                at,
            })
        }
        TimelineItemContent::MsgLike(msg_like) => {
            let mut message = Message {
                id: event_id(event),
                sender_id: event.sender().to_string(),
                sender_name,
                sender_avatar_url: ready(event.sender_profile())
                    .and_then(|p| p.avatar_url.map(|u| u.to_string())),
                text: String::new(),
                html: None,
                undecryptable: None,
                notice: None,
                attachment: None,
                is_emote: false,
                sent_at: at,
                is_own: event.is_own(),
                is_edited: false,
                send_status: send_status(event),
                reactions: reactions(msg_like, own),
                reply_to: reply_preview(msg_like),
                thread: thread_summary(msg_like),
                can_edit: event.is_editable(),
                poll: None,
                is_pinned: false,
                can_pin: false,
                is_call_thread_root: false,
            };
            match &msg_like.kind {
                MsgLikeKind::Message(m) => {
                    message.is_call_thread_root = is_call_thread_root(event);
                    message.is_edited = m.is_edited();
                    message.text = m.body().to_string();
                    let (formatted, is_emote) = match m.msgtype() {
                        MessageType::Text(c) => (c.formatted.as_ref(), false),
                        MessageType::Notice(c) => (c.formatted.as_ref(), false),
                        MessageType::Emote(c) => (c.formatted.as_ref(), true),
                        other => {
                            message.attachment = crate::media::attachment_of(other);
                            // The body of a file is its caption, or its name when it has none.
                            if message
                                .attachment
                                .as_ref()
                                .is_some_and(|a| a.name == message.text)
                            {
                                message.text.clear();
                            }
                            (None, false)
                        }
                    };
                    message.is_emote = is_emote;
                    message.html = formatted
                        .filter(|f| f.format == MessageFormat::Html)
                        .map(|f| f.body.clone());
                }
                MsgLikeKind::Redacted => message.notice = Some(MessageNotice::Deleted),
                MsgLikeKind::UnableToDecrypt(_) => {
                    message.undecryptable = Some(Undecryptable::MissingKeys)
                }
                MsgLikeKind::Poll(state) => {
                    let results = state.results();
                    message.text = results.question.clone();
                    let own_id = own.as_str();
                    let ended = results.end_time.is_some();
                    let answers: Vec<PollAnswerView> = results
                        .answers
                        .iter()
                        .map(|a| {
                            let voters = results.votes.get(&a.id);
                            PollAnswerView {
                                id: a.id.clone(),
                                text: a.text.clone(),
                                votes: voters
                                    .map_or(0, |v| u32::try_from(v.len()).unwrap_or(u32::MAX)),
                                own: voters.is_some_and(|v| v.iter().any(|u| u == own_id)),
                            }
                        })
                        .collect();
                    let voters: HashSet<&String> = results.votes.values().flatten().collect();
                    message.poll = Some(PollView {
                        question: results.question.clone(),
                        answers,
                        max_selections: u32::try_from(results.max_selections).unwrap_or(1),
                        is_disclosed: matches!(results.kind, PollKind::Disclosed),
                        ended,
                        total_votes: u32::try_from(voters.len()).unwrap_or(u32::MAX),
                        can_end: event.is_own() && !ended,
                    });
                }
                _ => message.notice = Some(MessageNotice::Unsupported),
            }
            message.is_pinned = ctx.pinned.contains(&message.id);
            message.can_pin = ctx.can_pin && message.id.starts_with('$');
            Some(TimelineEntry::Message(message))
        }
        TimelineItemContent::MembershipChange(change) => {
            let who = change
                .display_name()
                .unwrap_or_else(|| change.user_id().to_string());
            let what = match change.change()? {
                MembershipChange::Joined | MembershipChange::InvitationAccepted => "joined",
                MembershipChange::Left => "left",
                MembershipChange::Invited => "was invited",
                MembershipChange::Kicked => "was removed",
                MembershipChange::Banned | MembershipChange::KickedAndBanned => "was banned",
                _ => return None,
            };
            Some(TimelineEntry::Event {
                id: event_id(event),
                text: format!("{who} {what}"),
                at,
            })
        }
        _ => None,
    }
}

fn send_status(event: &EventTimelineItem) -> Option<SendStatus> {
    if !event.is_own() {
        return None;
    }
    Some(match event.send_state() {
        Some(EventSendState::NotSentYet { .. }) => SendStatus::Sending,
        Some(EventSendState::SendingFailed { .. }) => SendStatus::Failed,
        _ => SendStatus::Sent,
    })
}

fn ready(profile: &TimelineDetails<Profile>) -> Option<Profile> {
    match profile {
        TimelineDetails::Ready(p) => Some(p.clone()),
        _ => None,
    }
}

/// The display name, else the localpart of the Matrix id.
pub fn profile_name(profile: &Option<Profile>, user_id: &str) -> String {
    profile
        .as_ref()
        .and_then(|p| p.display_name.clone())
        .filter(|n| !n.trim().is_empty())
        .unwrap_or_else(|| {
            user_id
                .trim_start_matches('@')
                .split(':')
                .next()
                .unwrap_or(user_id)
                .to_string()
        })
}

/// What a message-like event shows in a one-line preview.
pub fn message_kind(msg_like: &MsgLikeContent) -> (PreviewKind, String) {
    match &msg_like.kind {
        MsgLikeKind::Message(m) => match m.msgtype() {
            MessageType::Image(c) => (PreviewKind::Image, c.body.clone()),
            MessageType::Video(c) => (PreviewKind::Video, c.body.clone()),
            MessageType::Audio(c) => (PreviewKind::Audio, c.body.clone()),
            MessageType::File(c) => (PreviewKind::File, c.body.clone()),
            other => (PreviewKind::Text, other.body().to_string()),
        },
        MsgLikeKind::Poll(_) => (PreviewKind::Poll, String::new()),
        MsgLikeKind::UnableToDecrypt(_) => (PreviewKind::Encrypted, String::new()),
        MsgLikeKind::Redacted => (PreviewKind::Deleted, String::new()),
        _ => (PreviewKind::Text, String::new()),
    }
}
