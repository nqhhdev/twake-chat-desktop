//! Domain values to the rows the Slint components draw.

use slint::{Color, Image, ModelRc, SharedString, StyledText, VecModel};
use twake_domain::{
    Message, MessageNotice, PreviewKind, Reaction, RoomKind, RoomSummary, SendStatus,
    TimelineEntry,
    attachment::{AttachmentKind, format_size},
    format, rich_text,
};

use crate::{ReactionChip, RoomRow, Row};

/// Finds a decoded image by media key and pixel size, asking the engine for it when it is
/// not there yet (`crop`: fill the size, else fit in it).
pub type Pictures<'a> = &'a mut dyn FnMut(&str, u32, u32, bool) -> Option<Image>;

/// Pixels per logical pixel asked for images: sharp on high-density screens.
pub const DENSITY: u32 = 2;

fn avatar(mxc: Option<&str>, size: u32, pictures: Pictures<'_>) -> Option<Image> {
    let mxc = mxc.filter(|m| m.starts_with("mxc://"))?;
    pictures(
        &twake_core::mxc_key(mxc),
        size * DENSITY,
        size * DENSITY,
        true,
    )
}

/// Avatar tints, picked by a stable hash of the id.
const TINTS: [u32; 8] = [
    0x297EF2, 0x8135FE, 0xE5455C, 0x0F9D58, 0xF07A10, 0x00A0B0, 0xB666FA, 0x5C6BC0,
];

pub fn tint(id: &str) -> Color {
    let hash = id
        .bytes()
        .fold(0u32, |h, b| h.wrapping_mul(31).wrapping_add(u32::from(b)));
    Color::from_rgb_u8(
        (TINTS[(hash % 8) as usize] >> 16) as u8,
        (TINTS[(hash % 8) as usize] >> 8) as u8,
        TINTS[(hash % 8) as usize] as u8,
    )
}

pub fn initials(name: &str) -> SharedString {
    let letters: String = name
        .split_whitespace()
        .filter_map(|word| {
            word.trim_start_matches(['@', '#'])
                .chars()
                .find(|c| c.is_alphanumeric())
        })
        .take(2)
        .flat_map(char::to_uppercase)
        .collect();
    if letters.is_empty() {
        "?".into()
    } else {
        letters.into()
    }
}

pub fn room_row(room: &RoomSummary, now: i64, pictures: Pictures<'_>) -> RoomRow {
    let picture = avatar(room.avatar_url.as_deref(), 44, pictures);
    let preview = room.preview.as_ref().map_or(String::new(), |p| {
        let what = match p.kind {
            PreviewKind::Text => p.text.replace('\n', " "),
            PreviewKind::Image => "📷 Photo".into(),
            PreviewKind::Video => "🎬 Video".into(),
            PreviewKind::Audio | PreviewKind::Voice => "🎤 Audio".into(),
            PreviewKind::File => format!("📎 {}", p.text),
            PreviewKind::Poll => "📊 Poll".into(),
            PreviewKind::Encrypted => "🔒 Encrypted message".into(),
            PreviewKind::Deleted => "Message deleted".into(),
        };
        if p.is_own {
            format!("You: {what}")
        } else if room.kind != RoomKind::Direct && !p.sender_name.is_empty() {
            format!("{}: {what}", p.sender_name)
        } else {
            what
        }
    });
    RoomRow {
        id: room.id.as_str().into(),
        name: room.name.as_str().into(),
        initials: initials(&room.name),
        tint: tint(&room.id),
        preview: preview.into(),
        time: room
            .last_activity
            .map(|t| format::room_list_time(t, now))
            .unwrap_or_default()
            .into(),
        unread: room.unread_count.min(i32::MAX as u64) as i32,
        mentions: room.mention_count.min(i32::MAX as u64) as i32,
        encrypted: room.is_encrypted,
        invite: room.is_invite,
        has_avatar: picture.is_some(),
        avatar: picture.unwrap_or_default(),
    }
}

/// A run of messages by one sender within five minutes shares one header.
const GROUP_GAP_MS: i64 = 5 * 60 * 1000;

pub fn timeline_rows(entries: &[TimelineEntry], now: i64, pictures: Pictures<'_>) -> Vec<Row> {
    let mut rows = Vec::with_capacity(entries.len());
    let mut previous: Option<&Message> = None;
    for entry in entries {
        match entry {
            TimelineEntry::Message(m) => {
                let head = previous.is_none_or(|p| {
                    p.sender_id != m.sender_id || m.sent_at - p.sent_at > GROUP_GAP_MS
                });
                rows.push(message_row(m, head, pictures));
                previous = Some(m);
            }
            TimelineEntry::Day { at } => {
                rows.push(Row {
                    kind: 1,
                    id: format!("day-{at}").into(),
                    plain: format::day_label(*at, now).into(),
                    ..Row::default()
                });
                previous = None;
            }
            TimelineEntry::ReadMarker => {
                rows.push(Row {
                    kind: 2,
                    id: "read-marker".into(),
                    ..Row::default()
                });
            }
            TimelineEntry::Event { id, text, .. } => {
                rows.push(Row {
                    kind: 3,
                    id: id.as_str().into(),
                    plain: text.as_str().into(),
                    ..Row::default()
                });
                previous = None;
            }
        }
    }
    rows
}

/// Largest side of an image in the timeline, in logical pixels.
const MEDIA_MAX_SIDE: f64 = 320.0;

/// Bars of a voice message's waveform, heights from 0 to 1 (fitted to `VOICE_BARS`, as the
/// web fits another client's waveform with `computeWaveform`).
const VOICE_BARS: usize = 40;

fn waveform_bars(attachment: Option<&twake_domain::attachment::Attachment>) -> ModelRc<f32> {
    use twake_domain::attachment::{MAX_WAVEFORM_VALUE, compute_waveform};
    let Some(voice) = attachment.and_then(|a| a.voice.as_ref()) else {
        return ModelRc::default();
    };
    let levels: Vec<f32> = voice.waveform.iter().map(|v| *v as f32).collect();
    let bars: Vec<f32> = compute_waveform(&levels, VOICE_BARS)
        .into_iter()
        .map(|v| v as f32 / MAX_WAVEFORM_VALUE as f32)
        .collect();
    ModelRc::from(std::rc::Rc::new(VecModel::from(bars)))
}

fn message_row(m: &Message, head: bool, pictures: Pictures<'_>) -> Row {
    let sender_picture = if head {
        avatar(m.sender_avatar_url.as_deref(), 32, pictures)
    } else {
        None
    };
    let mut media = None;
    let (mut media_width, mut media_height) = (0, 0);
    if let Some(a) = m
        .attachment
        .as_ref()
        .filter(|a| a.kind == AttachmentKind::Image)
    {
        let dims = a.dimensions.map(|(w, h)| (f64::from(w), f64::from(h)));
        let (w, h) = format::fit_preview(dims, MEDIA_MAX_SIDE, 1.0);
        media_width = w as i32;
        media_height = h as i32;
        let key = a
            .thumbnail_source
            .as_deref()
            .filter(|_| !a.is_encrypted)
            .unwrap_or(&a.source);
        media = pictures(key, (w as u32) * DENSITY, (h as u32) * DENSITY, false);
    }
    let attachment_info = m.attachment.as_ref().map_or(String::new(), |a| {
        let mut parts = Vec::new();
        if let Some(d) = a.duration {
            parts.push(format::format_duration(d as f64));
        }
        if let Some(size) = a.size {
            parts.push(format_size(size));
        }
        parts.join(" · ")
    });
    let notice = if m.undecryptable.is_some() {
        "🔒 Waiting for the key to read this message"
    } else {
        match m.notice {
            Some(MessageNotice::Deleted) => "Message deleted",
            Some(MessageNotice::Unsupported) => "This message cannot be shown here yet",
            None => "",
        }
    };
    // The message that opens the thread of a call, from its own time in the reader's
    // zone (web `call.threadRoot`)
    let root_text = m.is_call_thread_root.then(|| meeting_started(m.sent_at));
    let text = root_text.as_deref().unwrap_or(&m.text);
    let mut markdown = match &m.html {
        Some(html) if root_text.is_none() => rich_text::html_to_markdown(html),
        _ => rich_text::plain_to_markdown(text),
    };
    if m.is_emote {
        // « * alice waves » (`describeEmote`), the body keeping its formatting
        let head = twake_domain::format::describe_emote(&m.sender_name, "");
        markdown = format!(
            "{} {markdown}",
            rich_text::plain_to_markdown(head.trim_end())
        );
    }
    let body =
        StyledText::from_markdown(&markdown).unwrap_or_else(|_| StyledText::from_plain_text(text));
    Row {
        kind: 0,
        id: m.id.as_str().into(),
        sender: m.sender_name.as_str().into(),
        initials: initials(&m.sender_name),
        tint: tint(&m.sender_id),
        body,
        plain: text.into(),
        time: format::message_time(m.sent_at).into(),
        own: m.is_own,
        head,
        edited: m.is_edited,
        status: match m.send_status {
            Some(SendStatus::Sending) => "sending".into(),
            Some(SendStatus::Failed) => "failed".into(),
            _ => SharedString::default(),
        },
        notice: notice.into(),
        emote: m.is_emote,
        has_avatar: sender_picture.is_some(),
        avatar: sender_picture.unwrap_or_default(),
        attachment_kind: m.attachment.as_ref().map_or("", |a| a.kind.as_str()).into(),
        attachment_key: m
            .attachment
            .as_ref()
            .map_or(String::new(), |a| a.source.clone())
            .into(),
        attachment_name: m
            .attachment
            .as_ref()
            .map_or(String::new(), |a| a.name.clone())
            .into(),
        attachment_info: attachment_info.into(),
        has_media: media.is_some(),
        media: media.unwrap_or_default(),
        media_width,
        media_height,
        reactions: reaction_chips(&m.id, &m.reactions),
        has_reply: m.reply_to.is_some(),
        reply_sender: m
            .reply_to
            .as_ref()
            .map_or(String::new(), |r| r.sender_name.clone())
            .into(),
        reply_text: m
            .reply_to
            .as_ref()
            .map_or(String::new(), |r| {
                if r.text.is_empty() {
                    "…".into()
                } else {
                    r.text.clone()
                }
            })
            .into(),
        thread_count: m
            .thread
            .as_ref()
            .map_or(0, |t| i32::try_from(t.reply_count).unwrap_or(i32::MAX)),
        thread_text: m
            .thread
            .as_ref()
            .map_or(String::new(), |t| {
                format!("{}: {}", t.latest_sender, t.latest_text)
            })
            .into(),
        can_act: m.id.starts_with('$'),
        can_edit: m.can_edit && m.attachment.is_none(),
        can_delete: m.is_own && m.id.starts_with('$') && m.notice != Some(MessageNotice::Deleted),
        has_poll: m.poll.is_some(),
        poll_answers: poll_answers(m.poll.as_ref()),
        poll_status: m.poll.as_ref().map_or(String::new(), poll_status).into(),
        poll_can_end: m.poll.as_ref().is_some_and(|p| p.can_end),
        poll_ended: m.poll.as_ref().is_some_and(|p| p.ended),
        pinned: m.is_pinned,
        highlight: false,
        can_pin: m.can_pin,
        playable_audio: m
            .attachment
            .as_ref()
            .is_some_and(|a| a.voice.is_some() || twake_domain::attachment::is_playable_audio(a)),
        is_voice: m.attachment.as_ref().is_some_and(|a| a.voice.is_some()),
        attachment_mime: m
            .attachment
            .as_ref()
            .map_or(String::new(), |a| a.mime_type.clone())
            .into(),
        duration_ms: m
            .attachment
            .as_ref()
            .and_then(|a| a.duration)
            .map_or(0, |d| i32::try_from(d).unwrap_or(i32::MAX)),
        waveform: waveform_bars(m.attachment.as_ref()),
        selectable: m.id.starts_with('$') && twake_domain::is_selectable(m, false),
        ..Row::default()
    }
}

/// Message id to its reactions and the chips model drawn for them.
type ChipCache = std::collections::HashMap<String, (Vec<Reaction>, ModelRc<ReactionChip>)>;

thread_local! {
    /// The chips of each message, kept while they do not change: the timeline model then
    /// sees the row unchanged and does not lay it out again.
    static CHIPS: std::cell::RefCell<ChipCache> = std::cell::RefCell::new(ChipCache::new());
}

fn reaction_chips(id: &str, reactions: &[Reaction]) -> ModelRc<ReactionChip> {
    CHIPS.with_borrow_mut(|cache| {
        if let Some((known, model)) = cache.get(id)
            && known.as_slice() == reactions
        {
            return model.clone();
        }
        let chips: Vec<ReactionChip> = reactions
            .iter()
            .map(|r| ReactionChip {
                key: r.key.as_str().into(),
                count: i32::try_from(r.count).unwrap_or(i32::MAX),
                own: r.is_own,
            })
            .collect();
        let model = ModelRc::new(VecModel::from(chips));
        cache.insert(id.to_string(), (reactions.to_vec(), model.clone()));
        model
    })
}

/// The info pane's header data.
pub fn room_info(
    details: &twake_domain::RoomDetails,
    pictures: Pictures<'_>,
) -> crate::RoomInfoData {
    let picture = avatar(details.avatar_url.as_deref(), 96, pictures);
    let joined = details.joined_count;
    crate::RoomInfoData {
        name: details.name.as_str().into(),
        topic: details.topic.clone().unwrap_or_default().into(),
        initials: initials(&details.name),
        tint: tint(&details.room_id),
        has_avatar: picture.is_some(),
        avatar: picture.unwrap_or_default(),
        encrypted: details.is_encrypted,
        direct: details.kind == RoomKind::Direct,
        count_text: if joined == 1 {
            "1 member".into()
        } else {
            format!("{joined} members").into()
        },
        can_edit: details.permissions.can_edit_settings,
        can_encrypt: details.permissions.can_encrypt,
        can_invite: details.permissions.can_invite,
    }
}

/// The members of the info pane, with what the user may do to each (web: `canManageMember`,
/// `assignableRoles`, `canBanMember`).
pub fn member_rows(
    details: &twake_domain::RoomDetails,
    pictures: Pictures<'_>,
) -> Vec<crate::MemberRow> {
    use twake_domain::room_management::{
        MemberRole, assignable_roles, can_ban_member, can_manage_member,
    };
    details
        .members
        .iter()
        .map(|m| {
            let is_self = m.user_id == details.own_user_id;
            let picture = avatar(m.avatar_url.as_deref(), 32, pictures);
            let role_label = if m.is_owner {
                "Owner"
            } else {
                match m.role {
                    MemberRole::Admin => "Admin",
                    MemberRole::Moderator => "Moderator",
                    MemberRole::Member => "",
                }
            };
            let roles: Vec<SharedString> = assignable_roles(m, &details.permissions, is_self)
                .into_iter()
                .map(|r| r.as_str().into())
                .collect();
            crate::MemberRow {
                user_id: m.user_id.as_str().into(),
                name: m.name.as_str().into(),
                initials: initials(&m.name),
                tint: tint(&m.user_id),
                has_avatar: picture.is_some(),
                avatar: picture.unwrap_or_default(),
                role_label: role_label.into(),
                invited: m.is_invited,
                is_self,
                can_remove: can_manage_member(m, &details.permissions, is_self),
                can_ban: can_ban_member(m, &details.permissions, is_self),
                roles: ModelRc::new(VecModel::from(roles)),
            }
        })
        .collect()
}

/// The answers of a poll with the share of each; hidden until one's vote (or the end) when
/// the poll keeps its results for later (web: `isDisclosed`).
fn poll_answers(poll: Option<&twake_domain::poll::PollView>) -> ModelRc<crate::PollAnswerRow> {
    let Some(poll) = poll else {
        return ModelRc::default();
    };
    let show = poll.ended || (poll.is_disclosed && poll.answers.iter().any(|a| a.own));
    let total = poll.answers.iter().map(|a| a.votes).sum::<u32>().max(1) as f32;
    let rows: Vec<crate::PollAnswerRow> = poll
        .answers
        .iter()
        .map(|a| crate::PollAnswerRow {
            id: a.id.as_str().into(),
            text: a.text.as_str().into(),
            votes: i32::try_from(a.votes).unwrap_or(i32::MAX),
            own: a.own,
            share: if show { a.votes as f32 / total } else { 0.0 },
        })
        .collect();
    ModelRc::new(VecModel::from(rows))
}

fn poll_status(poll: &twake_domain::poll::PollView) -> String {
    let votes = if poll.total_votes == 1 {
        "1 vote".to_string()
    } else {
        format!("{} votes", poll.total_votes)
    };
    let choice = if poll.max_selections > 1 {
        format!(" · up to {} answers", poll.max_selections)
    } else {
        String::new()
    };
    if poll.ended {
        format!("{votes} · ended")
    } else if !poll.is_disclosed {
        format!("{votes}{choice} · results at the end")
    } else {
        format!("{votes}{choice}")
    }
}

pub fn space_item(space: &twake_domain::SpaceSummary, pictures: Pictures<'_>) -> crate::SpaceItem {
    let picture = avatar(space.avatar_url.as_deref(), 40, pictures);
    crate::SpaceItem {
        id: space.id.as_str().into(),
        name: space.name.as_str().into(),
        initials: initials(&space.name),
        tint: tint(&space.id),
        has_avatar: picture.is_some(),
        avatar: picture.unwrap_or_default(),
    }
}

/// « Meeting started at 5:07 PM » (web `call.threadRoot`).
pub fn meeting_started(at: i64) -> String {
    format!("Meeting started at {}", format::message_time(at))
}
