//! Port of `src/domain/activity.ts` and `src/domain/activityFeed.ts`: what asks for the
//! user across rooms (mentions, reactions on their messages, followed threads, unread
//! rooms), as one feed grouped by day.

use chrono::{Local, TimeZone};
use serde::{Deserialize, Serialize};

use crate::search::normalize_for_search;

/// `ACTIVITY_LIMIT`: how many of each the view shows.
pub const ACTIVITY_LIMIT: usize = 50;

/// What a line of the Activity reads of a message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMessage {
    pub id: String,
    pub sender_name: String,
    #[serde(deserialize_with = "Option::deserialize")]
    pub sender_avatar_url: Option<String>,
    pub text: String,
    pub sent_at: f64,
}

/// `ActivityMention`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMention {
    pub room_id: String,
    pub room_name: String,
    pub message: ActivityMessage,
}

/// `ActivityReaction`: a reaction of someone else on a message of the user.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReaction {
    pub room_id: String,
    pub room_name: String,
    pub key: String,
    pub sender_name: String,
    pub reacted_at: f64,
    pub message: ActivityMessage,
}

/// The last reply of a thread.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastReply {
    pub sender_name: String,
    pub text: String,
    pub sent_at: f64,
}

/// What the Activity reads of a `ThreadSummary` (`thread.ts`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityThreadSummary {
    pub thread_id: String,
    pub title: String,
    pub is_participating: bool,
    #[serde(default)]
    pub is_resolved: Option<bool>,
    #[serde(default)]
    pub last_reply: Option<LastReply>,
    #[serde(default)]
    pub last_own_reply_at: Option<f64>,
    pub unread: ThreadUnread,
}

/// The unread replies of a thread.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadUnread {
    pub unread_count: f64,
}

/// `ActivityThread`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityThread {
    pub room_id: String,
    pub room_name: String,
    pub thread: ActivityThreadSummary,
}

/// `Activity` (missed calls come with M5).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Activity {
    pub mentions: Vec<ActivityMention>,
    pub threads: Vec<ActivityThread>,
    pub reactions: Vec<ActivityReaction>,
}

/// `items` newest first by `at` (a stable sort, as `Array.prototype.sort`), at most 50.
pub fn newest_first<T: Clone>(items: &[T], at: impl Fn(&T) -> f64) -> Vec<T> {
    let mut sorted = items.to_vec();
    sorted.sort_by(|a, b| {
        (at(b) - at(a))
            .partial_cmp(&0.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    sorted.truncate(ACTIVITY_LIMIT);
    sorted
}

/// A thread the user follows with something new: they take part, and the last reply is
/// not their own (web `buildActivity`).
pub fn is_followed(
    is_participating: bool,
    last_reply_at: Option<f64>,
    last_own_reply_at: Option<f64>,
) -> bool {
    is_participating
        && !last_reply_at.is_some_and(|reply| last_own_reply_at.unwrap_or(0.0) >= reply)
}

/// `buildActivity`: newest mentions and reactions first; followed threads, most
/// recently active first.
pub fn build_activity(
    mentions: &[ActivityMention],
    threads: &[ActivityThread],
    reactions: &[ActivityReaction],
) -> Activity {
    let followed: Vec<ActivityThread> = threads
        .iter()
        .filter(|t| {
            is_followed(
                t.thread.is_participating,
                t.thread.last_reply.as_ref().map(|r| r.sent_at),
                t.thread.last_own_reply_at,
            )
        })
        .cloned()
        .collect();
    Activity {
        mentions: newest_first(mentions, |m| m.message.sent_at),
        threads: newest_first(&followed, |t| {
            t.thread.last_reply.as_ref().map_or(0.0, |r| r.sent_at)
        }),
        reactions: newest_first(reactions, |r| r.reacted_at),
    }
}

/// `FeedKind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedKind {
    Unread,
    Mention,
    Reaction,
    Thread,
    Saved,
    MissedCall,
}

/// `FeedFilter`: the chips of the Activity (D179).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeedFilter {
    All,
    Unread,
    Mentions,
    Replies,
    Saved,
}

/// Where a line opens: a message or a thread of the room, the room itself otherwise.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FeedTarget {
    Event { event: String },
    Thread { thread: String },
}

fn is_false(value: &bool) -> bool {
    !value
}

/// `FeedItem`: one line of the Activity, whatever it comes from.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedItem {
    pub id: String,
    pub kind: FeedKind,
    pub room_id: String,
    pub room_name: String,
    pub at: f64,
    pub actor_name: Option<String>,
    pub actor_avatar_url: Option<String>,
    pub quote: Option<String>,
    pub count: f64,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_marked_unread: bool,
    pub emoji: Option<String>,
    pub thread_title: Option<String>,
    pub is_resolved: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub can_call_back: bool,
    pub target: Option<FeedTarget>,
}

/// The last message of an unread room, as its preview shows it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnreadPreview {
    pub sender_name: Option<String>,
    pub text: String,
    pub sent_at: f64,
}

/// `UnreadRoom`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnreadRoom {
    pub room_id: String,
    pub room_name: String,
    pub unread_count: f64,
    #[serde(default)]
    pub is_marked_unread: Option<bool>,
    pub last_message: Option<UnreadPreview>,
}

fn item(id: String, kind: FeedKind, room_id: &str, room_name: &str, at: f64) -> FeedItem {
    FeedItem {
        id,
        kind,
        room_id: room_id.to_string(),
        room_name: room_name.to_string(),
        at,
        actor_name: None,
        actor_avatar_url: None,
        quote: None,
        count: 0.0,
        is_marked_unread: false,
        emoji: None,
        thread_title: None,
        is_resolved: false,
        can_call_back: false,
        target: None,
    }
}

/// `buildFeed` (missed calls come with M5): newest first.
pub fn build_feed(
    activity: &Activity,
    unread: &[UnreadRoom],
    saved: &[crate::saved::SavedMessage],
) -> Vec<FeedItem> {
    let mut items = Vec::new();
    for entry in unread {
        let last = entry.last_message.as_ref();
        items.push(FeedItem {
            actor_name: last.and_then(|l| l.sender_name.clone()),
            quote: last.map(|l| l.text.clone()),
            count: entry.unread_count,
            is_marked_unread: entry.is_marked_unread == Some(true),
            ..item(
                format!("unread:{}", entry.room_id),
                FeedKind::Unread,
                &entry.room_id,
                &entry.room_name,
                last.map_or(0.0, |l| l.sent_at),
            )
        });
    }
    for m in &activity.mentions {
        items.push(FeedItem {
            actor_name: Some(m.message.sender_name.clone()),
            actor_avatar_url: m.message.sender_avatar_url.clone(),
            quote: Some(m.message.text.clone()),
            target: Some(FeedTarget::Event {
                event: m.message.id.clone(),
            }),
            ..item(
                format!("mention:{}", m.message.id),
                FeedKind::Mention,
                &m.room_id,
                &m.room_name,
                m.message.sent_at,
            )
        });
    }
    for r in &activity.reactions {
        items.push(FeedItem {
            actor_name: Some(r.sender_name.clone()),
            quote: Some(r.message.text.clone()),
            emoji: Some(r.key.clone()),
            target: Some(FeedTarget::Event {
                event: r.message.id.clone(),
            }),
            ..item(
                format!("reaction:{}{}{}", r.message.id, r.key, r.sender_name),
                FeedKind::Reaction,
                &r.room_id,
                &r.room_name,
                r.reacted_at,
            )
        });
    }
    for t in &activity.threads {
        let last = t.thread.last_reply.as_ref();
        items.push(FeedItem {
            actor_name: last.map(|l| l.sender_name.clone()),
            quote: last.map(|l| l.text.clone()),
            count: t.thread.unread.unread_count,
            thread_title: Some(t.thread.title.clone()),
            is_resolved: t.thread.is_resolved == Some(true),
            target: Some(FeedTarget::Thread {
                thread: t.thread.thread_id.clone(),
            }),
            ..item(
                format!("thread:{}", t.thread.thread_id),
                FeedKind::Thread,
                &t.room_id,
                &t.room_name,
                last.map_or(0.0, |l| l.sent_at),
            )
        });
    }
    for s in saved {
        let message = s.message.as_ref();
        items.push(FeedItem {
            actor_name: message.map(|m| m.sender_name.clone()),
            actor_avatar_url: message.and_then(|m| m.sender_avatar_url.clone()),
            quote: message.map(|m| m.text.clone()),
            target: Some(FeedTarget::Event {
                event: s.event_id.clone(),
            }),
            ..item(
                format!("saved:{}{}", s.room_id, s.event_id),
                FeedKind::Saved,
                &s.room_id,
                &s.room_name,
                s.saved_at,
            )
        });
    }
    items.sort_by(|a, b| {
        (b.at - a.at)
            .partial_cmp(&0.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    items
}

/// The kinds a chip shows.
pub fn kinds_of(filter: FeedFilter) -> &'static [FeedKind] {
    match filter {
        FeedFilter::All => &[
            FeedKind::Unread,
            FeedKind::Mention,
            FeedKind::Reaction,
            FeedKind::Thread,
            FeedKind::MissedCall,
        ],
        FeedFilter::Unread => &[FeedKind::Unread],
        FeedFilter::Mentions => &[FeedKind::Mention],
        // Replies = the threads the user follows
        FeedFilter::Replies => &[FeedKind::Thread],
        FeedFilter::Saved => &[FeedKind::Saved],
    }
}

/// `filterFeed`.
pub fn filter_feed(items: &[FeedItem], filter: FeedFilter) -> Vec<FeedItem> {
    let kinds = kinds_of(filter);
    items
        .iter()
        .filter(|i| kinds.contains(&i.kind))
        .cloned()
        .collect()
}

/// `searchFeed`: the items whose author, room, thread or quote holds every word.
pub fn search_feed(items: &[FeedItem], query: &str) -> Vec<FeedItem> {
    let folded = normalize_for_search(query);
    let words: Vec<&str> = folded
        .split(crate::js::is_js_space)
        .filter(|w| !w.is_empty())
        .collect();
    if words.is_empty() {
        return items.to_vec();
    }
    items
        .iter()
        .filter(|item| {
            // `[a, b, c, d].join(' ')`: null and undefined become empty strings
            let text = normalize_for_search(&format!(
                "{} {} {} {}",
                item.actor_name.as_deref().unwrap_or(""),
                item.room_name,
                item.thread_title.as_deref().unwrap_or(""),
                item.quote.as_deref().unwrap_or("")
            ));
            words.iter().all(|w| text.contains(w))
        })
        .cloned()
        .collect()
}

/// `FeedDay::day`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Day {
    Today,
    Yesterday,
    /// The start of the day, epoch milliseconds.
    Start(f64),
}

/// `new Date(ms).setHours(0, 0, 0, 0)` in the local time zone.
fn start_of_day(ms: f64) -> f64 {
    let Some(t) = Local.timestamp_millis_opt(ms as i64).single() else {
        return f64::NAN;
    };
    t.date_naive()
        .and_hms_opt(0, 0, 0)
        .and_then(|d| Local.from_local_datetime(&d).earliest())
        .map_or(f64::NAN, |d| d.timestamp_millis() as f64)
}

/// `groupByDay`: the items (newest first) under their day, in the local time zone.
pub fn group_by_day(items: &[FeedItem], now: f64) -> Vec<(Day, Vec<FeedItem>)> {
    let today = start_of_day(now);
    let yesterday = start_of_day(today - 1.0);
    let mut days: Vec<(Day, Vec<FeedItem>)> = Vec::new();
    for item in items {
        let start = start_of_day(item.at);
        let day = if start == today {
            Day::Today
        } else if start == yesterday {
            Day::Yesterday
        } else {
            Day::Start(start)
        };
        match days.last_mut() {
            Some((last, list)) if *last == day => list.push(item.clone()),
            _ => days.push((day, vec![item.clone()])),
        }
    }
    days
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_threads_with_news_only() {
        assert!(is_followed(true, Some(10.0), Some(5.0)));
        assert!(!is_followed(true, Some(10.0), Some(10.0)));
        assert!(!is_followed(false, Some(10.0), None));
        assert!(is_followed(true, None, None));
    }

    #[test]
    fn filters_and_searches_the_feed() {
        let a = FeedItem {
            actor_name: Some("Bob".into()),
            quote: Some("La réunion".into()),
            ..item("mention:1".into(), FeedKind::Mention, "!r", "Design", 2.0)
        };
        let b = item("unread:!s".into(), FeedKind::Unread, "!s", "General", 1.0);
        let items = vec![a.clone(), b.clone()];
        assert_eq!(filter_feed(&items, FeedFilter::Mentions), vec![a.clone()]);
        assert_eq!(search_feed(&items, "reunion bob"), vec![a]);
        assert_eq!(search_feed(&items, "  "), items);
    }
}
