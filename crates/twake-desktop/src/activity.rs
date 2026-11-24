//! The Activity screen: the engine's [`twake_domain::activity::Activity`] and the unread
//! rooms of the list, as the web's feed (`buildFeed`, `filterFeed`, `searchFeed`,
//! `groupByDay`).

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use slint::{ComponentHandle, ModelRc, VecModel};
use twake_domain::activity::{
    Activity, Day, FeedFilter, FeedItem, FeedKind, FeedTarget, UnreadPreview, UnreadRoom,
    build_feed, filter_feed, group_by_day, search_feed,
};

use crate::{AppWindow, FeedLine, STATE};

#[derive(Default)]
struct ActivityState {
    activity: Activity,
    /// The saved messages ([`twake_core::Event::Saved`]), newest first.
    saved: Vec<twake_core::SavedEntry>,
    filter: Option<FeedFilter>,
    /// The items of the lines shown; `None` for a day header.
    shown: Vec<Option<FeedItem>>,
}

thread_local! {
    static ACTIVITY: RefCell<ActivityState> = RefCell::new(ActivityState::default());
}

/// [`twake_core::Event::Activity`].
pub fn update(window: &AppWindow, activity: Activity) {
    ACTIVITY.with_borrow_mut(|a| a.activity = activity);
    refresh(window);
}

/// [`twake_core::Event::Saved`].
pub fn saved(window: &AppWindow, entries: Vec<twake_core::SavedEntry>) {
    ACTIVITY.with_borrow_mut(|a| a.saved = entries);
    if window.get_activity_open() {
        refresh(window);
    }
}

/// Whether the message `id` is saved (the 🔖 of its action bar).
pub fn is_saved(id: &str) -> bool {
    ACTIVITY.with_borrow(|a| a.saved.iter().any(|s| s.event_id == id))
}

/// The saved messages with their room's name, for the feed.
fn saved_messages(saved: &[twake_core::SavedEntry]) -> Vec<twake_domain::saved::SavedMessage> {
    STATE.with_borrow(|s| {
        saved
            .iter()
            .map(|e| twake_domain::saved::SavedMessage {
                room_id: e.room_id.clone(),
                room_name: s
                    .rooms
                    .iter()
                    .find(|r| r.id == e.room_id)
                    .map_or(String::new(), |r| r.name.clone()),
                event_id: e.event_id.clone(),
                saved_at: e.saved_at,
                message: e
                    .quote
                    .as_ref()
                    .map(|(sender, text)| twake_domain::saved::SavedQuote {
                        sender_name: sender.clone(),
                        sender_avatar_url: None,
                        text: text.clone(),
                    }),
            })
            .collect()
    })
}

pub fn open(window: &AppWindow) {
    window.set_activity_open(true);
    refresh(window);
}

pub fn pick_filter(window: &AppWindow, filter: &str) {
    let filter = match filter {
        "unread" => FeedFilter::Unread,
        "mentions" => FeedFilter::Mentions,
        "replies" => FeedFilter::Replies,
        "saved" => FeedFilter::Saved,
        _ => FeedFilter::All,
    };
    ACTIVITY.with_borrow_mut(|a| a.filter = Some(filter));
    window.set_activity_filter(
        match filter {
            FeedFilter::Unread => "unread",
            FeedFilter::Mentions => "mentions",
            FeedFilter::Replies => "replies",
            FeedFilter::Saved => "saved",
            _ => "all",
        }
        .into(),
    );
    refresh(window);
}

/// The unread rooms of the room list (invitations aside).
fn unread_rooms() -> Vec<UnreadRoom> {
    STATE.with_borrow(|s| {
        s.rooms
            .iter()
            .filter(|r| !r.is_invite && (r.unread_count > 0 || r.is_marked_unread))
            .map(|r| UnreadRoom {
                room_id: r.id.clone(),
                room_name: r.name.clone(),
                unread_count: r.unread_count as f64,
                is_marked_unread: Some(r.is_marked_unread),
                last_message: r.preview.as_ref().map(|p| UnreadPreview {
                    sender_name: Some(p.sender_name.clone()),
                    text: p.text.clone(),
                    sent_at: p.sent_at as f64,
                }),
            })
            .collect()
    })
}

fn line_of(item: &FeedItem, now: i64) -> FeedLine {
    let actor = item.actor_name.clone().unwrap_or_default();
    let (icon, title) = match item.kind {
        FeedKind::Mention => ("@", format!("{actor} mentioned you in {}", item.room_name)),
        FeedKind::Reaction => (
            item.emoji.as_deref().unwrap_or("👍"),
            format!("{actor} reacted in {}", item.room_name),
        ),
        FeedKind::Thread => (
            "💬",
            format!(
                "{actor} replied in « {} » in {}",
                twake_domain::room_preview::to_preview_text(
                    item.thread_title.as_deref().unwrap_or("")
                ),
                item.room_name
            ),
        ),
        FeedKind::MissedCall => ("📞", format!("Missed call from {actor}")),
        FeedKind::Saved => (
            "🔖",
            if actor.is_empty() {
                format!("Saved in {}", item.room_name)
            } else {
                format!("{actor} in {}", item.room_name)
            },
        ),
        FeedKind::Unread => ("●", item.room_name.clone()),
    };
    let count = match item.kind {
        FeedKind::Unread | FeedKind::Thread if item.count > 0.0 => (item.count as u64).to_string(),
        _ => String::new(),
    };
    let quote = item.quote.clone().unwrap_or_default();
    let quote = match item.kind {
        FeedKind::Unread if !actor.is_empty() => format!("{actor}: {quote}"),
        _ => quote,
    };
    FeedLine {
        is_day: false,
        icon: icon.into(),
        title: title.into(),
        quote: twake_domain::room_preview::to_preview_text(&quote).into(),
        time: twake_domain::format::room_list_time(item.at as i64, now).into(),
        count: count.into(),
    }
}

fn day_title(day: Day, now: i64) -> String {
    match day {
        Day::Today => "Today".into(),
        Day::Yesterday => "Yesterday".into(),
        Day::Start(ms) => twake_domain::format::day_label(ms as i64, now),
    }
}

/// Draws the feed again (the activity, the room list, the filter or the query changed).
pub fn refresh(window: &AppWindow) {
    let unread = unread_rooms();
    let now = chrono::Utc::now().timestamp_millis();
    let query = window.get_activity_query().to_string();
    let (lines, shown, count) = ACTIVITY.with_borrow(|a| {
        let feed = build_feed(&a.activity, &unread, &saved_messages(&a.saved));
        let news = filter_feed(&feed, FeedFilter::All)
            .iter()
            .filter(|i| i.kind != FeedKind::Unread)
            .count();
        let feed = filter_feed(&feed, a.filter.unwrap_or(FeedFilter::All));
        let feed = search_feed(&feed, &query);
        let mut lines = Vec::new();
        let mut shown = Vec::new();
        for (day, items) in group_by_day(&feed, now as f64) {
            lines.push(FeedLine {
                is_day: true,
                title: day_title(day, now).into(),
                ..FeedLine::default()
            });
            shown.push(None);
            for item in items {
                lines.push(line_of(&item, now));
                shown.push(Some(item));
            }
        }
        (lines, shown, news)
    });
    ACTIVITY.with_borrow_mut(|a| a.shown = shown);
    window.set_activity_count(count as i32);
    window.set_activity_lines(ModelRc::from(Rc::new(VecModel::from(lines))));
}

/// A line was chosen: its room opens, then its thread or its message in context.
pub fn choose(window: &AppWindow, index: usize) {
    let Some(item) = ACTIVITY.with_borrow(|a| a.shown.get(index).cloned().flatten()) else {
        return;
    };
    window.set_activity_open(false);
    window.invoke_open_room(item.room_id.as_str().into());
    let target = item.target.clone();
    let weak = window.as_weak();
    // The room's timeline opens first
    slint::Timer::single_shot(Duration::from_millis(400), move || {
        let Some(w) = weak.upgrade() else { return };
        match target {
            Some(FeedTarget::Thread { thread }) => w.invoke_open_thread(thread.into()),
            Some(FeedTarget::Event { event }) => crate::search::open_hit(&w, &event),
            None => {}
        }
    });
}
