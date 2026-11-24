//! The room list: matrix-sdk-ui's `RoomListService` mapped to [`RoomSummary`] rows.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use futures_util::StreamExt;
use imbl::Vector;
use matrix_sdk::{Client, RoomState};
use matrix_sdk_ui::{
    room_list_service::{RoomListItem, RoomListService, filters},
    timeline::{LatestEventValue, RoomExt, TimelineDetails, TimelineItemContent},
};
use twake_domain::{
    PreviewKind, RoomKind, RoomPreview, RoomSummary, room_preview::to_preview_text,
};

use crate::engine::{Emit, Event};
use crate::timeline::{message_kind, profile_name};

/// Rooms fetched per page by the sliding sync list.
const PAGE_SIZE: usize = 500;

pub async fn run(client: Client, service: Arc<RoomListService>, emit: Emit) {
    let all = match service.all_rooms().await {
        Ok(list) => list,
        Err(e) => {
            emit(Event::Error(format!("room list: {e}")));
            return;
        }
    };
    let (stream, controller) = all.entries_with_dynamic_adapters(PAGE_SIZE);
    controller.set_filter(Box::new(filters::new_filter_all(vec![
        Box::new(filters::new_filter_non_left()),
        Box::new(filters::new_filter_not(Box::new(
            filters::new_filter_space(),
        ))),
    ])));
    futures_util::pin_mut!(stream);

    let mut rooms: Vector<RoomListItem> = Vector::new();
    let mut listened: HashSet<String> = HashSet::new();
    let mut notified: Option<HashMap<String, u64>> = None;
    while let Some(diffs) = stream.next().await {
        for diff in diffs {
            diff.apply(&mut rooms);
        }
        // Latest events are computed only for the rooms someone listens to.
        let latest = client.latest_events().await;
        for room in rooms.iter() {
            if listened.insert(room.room_id().to_string()) {
                let _ = latest.listen_to_room(room.room_id()).await;
            }
        }
        let mut summaries = Vec::with_capacity(rooms.len());
        for room in rooms.iter() {
            summaries.push(summarize(room).await);
        }
        notify_new(&rooms, &summaries, &mut notified, &emit);
        emit(Event::Rooms(summaries));
    }
}

/// Messages that notify: the client's count (read receipts) or the server's (push rules),
/// whichever knows more; the client's alone stays at 0 for rooms it has no events of.
fn notifying_count(room: &RoomListItem) -> u64 {
    let server = room.unread_notification_counts();
    room.num_unread_notifications()
        .max(room.num_unread_mentions())
        .max(server.notification_count)
        .max(server.highlight_count)
}

/// Emits [`Event::Notify`] for each room whose count of notifying messages went up since
/// the last pass (the server's push rules decide what notifies: mentions, direct messages,
/// muted rooms). Nothing on the first pass, nothing for one's own messages.
fn notify_new(
    rooms: &Vector<RoomListItem>,
    summaries: &[RoomSummary],
    notified: &mut Option<HashMap<String, u64>>,
    emit: &Emit,
) {
    let counts: HashMap<String, u64> = rooms
        .iter()
        .map(|r| (r.room_id().to_string(), notifying_count(r)))
        .collect();
    if let Some(previous) = notified.as_ref() {
        for summary in summaries {
            let now = counts.get(&summary.id).copied().unwrap_or(0);
            let before = previous.get(&summary.id).copied().unwrap_or(0);
            let Some(preview) = summary.preview.as_ref().filter(|p| !p.is_own) else {
                continue;
            };
            if now > before {
                let body = match summary.kind {
                    RoomKind::Direct => preview.text.clone(),
                    _ => format!("{}: {}", preview.sender_name, preview.text),
                };
                emit(Event::Notify {
                    room_id: summary.id.clone(),
                    title: summary.name.clone(),
                    body,
                });
            }
        }
    }
    *notified = Some(counts);
}

async fn summarize(room: &RoomListItem) -> RoomSummary {
    let name = room
        .cached_display_name()
        .map(|n| n.to_string())
        .or_else(|| room.name())
        .unwrap_or_else(|| room.room_id().to_string());
    let preview = to_room_preview(room.latest_event().await);
    RoomSummary {
        id: room.room_id().to_string(),
        name,
        kind: if room.direct_targets_length() > 0 {
            RoomKind::Direct
        } else {
            RoomKind::Discussion
        },
        // A direct message without a picture shows the other person's.
        avatar_url: room
            .avatar_url()
            .or_else(|| {
                (room.direct_targets_length() > 0)
                    .then(|| {
                        room.clone_info()
                            .heroes()
                            .first()
                            .and_then(|h| h.avatar_url.clone())
                    })
                    .flatten()
            })
            .map(|u| u.to_string()),
        is_encrypted: room.encryption_state().is_encrypted(),
        is_invite: room.state() == RoomState::Invited,
        unread_count: room.num_unread_messages(),
        mention_count: room.num_unread_mentions(),
        is_marked_unread: room.is_marked_unread(),
        last_activity: preview
            .as_ref()
            .map(|p| p.sent_at)
            .or_else(|| room.latest_event_timestamp().map(|t| i64::from(t.0))),
        preview,
    }
}

fn to_room_preview(value: LatestEventValue) -> Option<RoomPreview> {
    let (timestamp, sender, is_own, profile, content) = match value {
        LatestEventValue::None | LatestEventValue::RemoteInvite { .. } => return None,
        LatestEventValue::Remote {
            timestamp,
            sender,
            is_own,
            profile,
            content,
        } => (timestamp, sender, is_own, profile, content),
        LatestEventValue::Local {
            timestamp,
            sender,
            profile,
            content,
            ..
        } => (timestamp, sender, true, profile, content),
    };
    let (kind, text) = match &content {
        TimelineItemContent::MsgLike(msg_like) => message_kind(msg_like),
        _ => return None,
    };
    Some(RoomPreview {
        kind,
        text: if kind == PreviewKind::Text {
            to_preview_text(&text)
        } else {
            text
        },
        sender_name: profile_name(&profile_ready(&profile), sender.as_str()),
        is_own,
        sent_at: i64::from(timestamp.0),
    })
}

fn profile_ready(
    profile: &TimelineDetails<matrix_sdk_ui::timeline::Profile>,
) -> Option<matrix_sdk_ui::timeline::Profile> {
    match profile {
        TimelineDetails::Ready(p) => Some(p.clone()),
        _ => None,
    }
}
