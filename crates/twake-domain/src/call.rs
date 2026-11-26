//! Calls (web `domain/call.ts`): rules of the call grid and of who is in a call.

use std::collections::HashSet;

/// The event the caller writes when a call ends (web `CALL_LINE_TYPE`).
pub const CALL_LINE_TYPE: &str = "app.twake.chat.call";
/// How long a ring lasts when this app starts one (web `RING_MS`, D579).
pub const RING_MS: i64 = 60_000;
/// A ring lasts at most this long, whatever its sender says (web `MAX_RING_MS`).
pub const MAX_RING_MS: i64 = 120_000;
/// The mark of the message that opens the thread of a call (web `CALL_THREAD_KEY`, D105).
pub const CALL_THREAD_KEY: &str = "app.twake.chat.call_thread";
/// A root older than this opened the thread of an earlier call (D105).
const CALL_THREAD_MAX_AGE_MS: i64 = 12 * 60 * 60 * 1000;
/// Roots this close were sent by two people starting the call at once.
const CALL_THREAD_RACE_MS: i64 = 30_000;

/// How a call went, for one reader (web `CallLine`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CallOutcome {
    /// From the answer to the end.
    Answered { duration_ms: f64 },
    /// Not answered, for the one rung.
    Missed,
    /// Not answered, for the caller.
    Canceled,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallLine {
    /// The reader rang (outgoing) or was rung (incoming).
    pub is_outgoing: bool,
    pub is_video: bool,
    pub outcome: CallOutcome,
}

/// The line of a call event's content, for a reader; None for anything else (web
/// `toCallLine`).
pub fn to_call_line(content: &serde_json::Value, is_own: bool) -> Option<CallLine> {
    let content = content.as_object()?;
    let is_video = match content.get("intent")?.as_str()? {
        "video" => true,
        "audio" => false,
        _ => return None,
    };
    let ended = content.get("ended_ts")?.as_f64()?;
    let outcome = match content.get("answered_ts") {
        None => {
            if is_own {
                CallOutcome::Canceled
            } else {
                CallOutcome::Missed
            }
        }
        Some(answered) => {
            let answered = answered.as_f64().filter(|a| *a <= ended)?;
            CallOutcome::Answered {
                duration_ms: ended - answered,
            }
        }
    };
    Some(CallLine {
        is_outgoing: is_own,
        is_video,
        outcome,
    })
}

/// « 1 hr 2 min 3 sec », the units that are not zero (web `formatCallDuration`, English).
pub fn format_call_duration(ms: f64) -> String {
    let total = (ms / 1000.0).floor().max(0.0) as u64;
    let parts = [
        (total / 3600, "hr"),
        (total / 60 % 60, "min"),
        (total % 60, "sec"),
    ];
    let shown: Vec<String> = parts
        .iter()
        .filter(|(value, _)| *value > 0)
        .map(|(value, unit)| format!("{value} {unit}"))
        .collect();
    if shown.is_empty() {
        "0 sec".into()
    } else {
        shown.join(" ")
    }
}

/// One device in a call, as the ring rules read it.
#[derive(Clone, Debug, PartialEq)]
pub struct RingMember {
    pub user_id: String,
    /// When it joined (ms), when known.
    pub joined_at: Option<f64>,
}

/// The ring of a call someone else started (MSC4075): declined by the user or not.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ring {
    pub is_declined: bool,
}

/// Whether a call still asks for the user: its ring decides; a direct call started by
/// a client that sends none asks for as long as a ring would last (web `ringAsks`).
pub fn ring_asks(
    ring: Option<Ring>,
    members: &[RingMember],
    own_user_id: &str,
    is_direct: bool,
    now: f64,
) -> bool {
    if let Some(ring) = ring {
        return !ring.is_declined;
    }
    is_direct
        && members
            .iter()
            .any(|m| m.user_id != own_user_id && m.joined_at.unwrap_or(now) + RING_MS as f64 > now)
}

/// The call rings for the user: someone else is in a call the user has not joined
/// (since they last left it), and it asks for them (`ring_asks`) (web `isRinging`).
pub fn is_ringing(
    members: &[RingMember],
    own_user_id: &str,
    asks: bool,
    is_joined: bool,
    left_at: Option<f64>,
) -> bool {
    asks && !is_joined
        && members.iter().any(|m| {
            m.user_id != own_user_id
                && left_at.is_none_or(|left| m.joined_at.unwrap_or(f64::INFINITY) > left)
        })
        && !members.iter().any(|m| m.user_id == own_user_id)
}

/// The own selfie camera shows as in a mirror; never the others, a shared screen or a
/// back camera (web `shouldMirror`, D309).
pub fn should_mirror(is_local: bool, is_screen: bool, faces_away: bool) -> bool {
    is_local && !is_screen && !faces_away
}

/// A microphone, speaker or camera of the system (web `MediaDevice`).
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
pub struct MediaDevice {
    pub id: String,
    /// Empty when the system gives no name.
    pub label: String,
}

/// The device in use. None known, or one unplugged: the first, which the system lists as
/// its default (web `currentDeviceId`).
pub fn current_device_id(devices: &[MediaDevice], active_id: Option<&str>) -> Option<String> {
    match active_id {
        Some(id) if devices.iter().any(|d| d.id == id) => Some(id.to_string()),
        _ => devices.first().map(|d| d.id.clone()),
    }
}

/// The people of a call, one per user, not per device: a user in the call from two
/// devices is one person in it (web `countCallPeople`).
pub fn count_call_people<'a>(user_ids: impl IntoIterator<Item = &'a str>) -> usize {
    user_ids.into_iter().collect::<HashSet<_>>().len()
}

/// The thread of the call going on in a channel (D105): the newest root of the main
/// timeline (oldest message first); when two people started the call at once, the first
/// of their roots. None without one: the call then chats in the channel (web
/// `findCallThread`).
pub fn find_call_thread<'a>(
    messages: impl IntoIterator<Item = &'a crate::Message>,
    now: i64,
) -> Option<String> {
    let roots: Vec<&crate::Message> = messages
        .into_iter()
        .filter(|m| m.is_call_thread_root && now - m.sent_at < CALL_THREAD_MAX_AGE_MS)
        .collect();
    let newest = roots.last()?;
    roots
        .iter()
        .find(|root| newest.sent_at - root.sent_at <= CALL_THREAD_RACE_MS)
        .map(|root| root.id.clone())
}

/// The chat of a call goes to a thread of its own: in a call of a channel the user may
/// post in; a direct call or a voice room chats where it is (web `splitsCallChat`).
pub fn splits_call_chat(is_in_call: bool, is_direct: bool, is_voice: bool, may_post: bool) -> bool {
    is_in_call && !is_direct && !is_voice && may_post
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirrors_only_the_own_selfie_camera() {
        assert!(should_mirror(true, false, false));
        assert!(!should_mirror(false, false, false));
        assert!(!should_mirror(true, true, false));
        assert!(!should_mirror(true, false, true));
    }

    #[test]
    fn reads_call_lines_as_the_web() {
        let line = |c: serde_json::Value, own| to_call_line(&c, own);
        let answered =
            serde_json::json!({"intent": "video", "answered_ts": 1000, "ended_ts": 66000});
        assert_eq!(
            line(answered, true),
            Some(CallLine {
                is_outgoing: true,
                is_video: true,
                outcome: CallOutcome::Answered {
                    duration_ms: 65000.0
                }
            })
        );
        let missed = serde_json::json!({"intent": "audio", "ended_ts": 5});
        assert_eq!(
            line(missed.clone(), false).map(|l| l.outcome),
            Some(CallOutcome::Missed)
        );
        assert_eq!(
            line(missed, true).map(|l| l.outcome),
            Some(CallOutcome::Canceled)
        );
        assert!(line(serde_json::json!({"intent": "audio"}), true).is_none());
        assert!(
            line(
                serde_json::json!({"intent": "audio", "answered_ts": 9, "ended_ts": 5}),
                true
            )
            .is_none()
        );
        assert_eq!(format_call_duration(65_000.0), "1 min 5 sec");
        assert_eq!(format_call_duration(3_600_000.0), "1 hr");
        assert_eq!(format_call_duration(400.0), "0 sec");
    }

    #[test]
    fn rings_as_the_web() {
        let m = |u: &str, at| RingMember {
            user_id: u.into(),
            joined_at: at,
        };
        let others = [m("@b:x", Some(1000.0))];
        // A ring decides; without one a direct call asks for a ring's time
        assert!(ring_asks(
            Some(Ring { is_declined: false }),
            &others,
            "@a:x",
            false,
            0.0
        ));
        assert!(!ring_asks(
            Some(Ring { is_declined: true }),
            &others,
            "@a:x",
            true,
            0.0
        ));
        assert!(ring_asks(None, &others, "@a:x", true, 1000.0 + 59_000.0));
        assert!(!ring_asks(None, &others, "@a:x", true, 1000.0 + 60_000.0));
        assert!(!ring_asks(None, &others, "@a:x", false, 1000.0));
        // In it already, joined, or the others were there before the user left
        assert!(is_ringing(&others, "@a:x", true, false, None));
        assert!(!is_ringing(&others, "@a:x", true, true, None));
        assert!(!is_ringing(
            &[m("@b:x", None), m("@a:x", None)],
            "@a:x",
            true,
            false,
            None
        ));
        assert!(!is_ringing(&others, "@a:x", true, false, Some(2000.0)));
        assert!(is_ringing(&others, "@a:x", true, false, Some(500.0)));
    }

    #[test]
    fn finds_the_call_thread_as_the_web() {
        let root = |id: &str, at: i64, is_root: bool| crate::Message {
            id: id.into(),
            sent_at: at,
            is_call_thread_root: is_root,
            ..crate::Message::default()
        };
        let hour = 60 * 60 * 1000;
        let now = 20 * hour;
        // The newest root; two at once: the first of them
        let messages = [
            root("old", now - 13 * hour, true),
            root("a", now - hour, true),
            root("text", now - hour + 5, false),
            root("b", now - hour + 10_000, true),
        ];
        assert_eq!(find_call_thread(&messages, now).as_deref(), Some("a"));
        let later = [
            root("a", now - hour, true),
            root("b", now - hour + 40_000, true),
        ];
        assert_eq!(find_call_thread(&later, now).as_deref(), Some("b"));
        // Older than 12 h, or none: the call chats in the channel
        assert_eq!(find_call_thread(&messages[..1], now), None);
        assert_eq!(find_call_thread(&messages[2..3], now), None);
        assert!(splits_call_chat(true, false, false, true));
        assert!(!splits_call_chat(true, true, false, true));
        assert!(!splits_call_chat(false, false, false, true));
        assert!(!splits_call_chat(true, false, false, false));
    }

    #[test]
    fn counts_people_not_devices() {
        assert_eq!(count_call_people(["@a:x", "@b:x", "@a:x"]), 2);
        assert_eq!(count_call_people([]), 0);
    }

    #[test]
    fn keeps_the_device_in_use_or_takes_the_first() {
        let device = |id: &str| MediaDevice {
            id: id.into(),
            label: String::new(),
        };
        let devices = [device("a"), device("b")];
        assert_eq!(current_device_id(&devices, Some("b")).as_deref(), Some("b"));
        // Unplugged, or none chosen: the system's default, the first
        assert_eq!(
            current_device_id(&devices, Some("gone")).as_deref(),
            Some("a")
        );
        assert_eq!(current_device_id(&devices, None).as_deref(), Some("a"));
        assert_eq!(current_device_id(&[], Some("a")), None);
    }
}
