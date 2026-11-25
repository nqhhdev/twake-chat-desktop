//! Search in the open conversation: the query goes to the engine's local index (and to
//! the server in a room that is not encrypted); a result opens its messages around it
//! in the side panel.

use std::rc::Rc;

use slint::{ModelRc, VecModel};
use twake_core::Command;
use twake_domain::search::{IndexedMessage, SearchFilter};

use crate::{AppWindow, STATE, SearchHit, send};

thread_local! {
    /// The message the side panel shows in context.
    static TARGET: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

/// Whether this row is the message a search opened in context.
pub fn is_target(id: &str) -> bool {
    STATE.with_borrow(|s| s.thread_root.as_deref() == Some("around"))
        && TARGET.with_borrow(|t| t == id)
}

pub fn open(window: &AppWindow) {
    if window.get_thread_open() {
        crate::close_thread(window);
        send(Command::CloseThread);
    }
    window.set_info_open(false);
    window.set_search_open(true);
}

/// Clears the panel: a room was opened.
pub fn reset(window: &AppWindow) {
    window.set_search_open(false);
    window.set_search_query("".into());
    window.set_search_status("".into());
    window.set_search_can_older(false);
    window.set_search_hits(ModelRc::default());
}

fn filter(query: &str, files_only: bool) -> SearchFilter {
    SearchFilter {
        query: query.to_string(),
        has_file: files_only,
        ..Default::default()
    }
}

/// The query changed, or the « Files » filter.
pub fn search(window: &AppWindow, query: &str, files_only: bool, older: bool) {
    let filter = filter(query, files_only);
    if twake_domain::search::is_empty_filter(&filter) {
        window.set_search_hits(ModelRc::default());
        window.set_search_status("".into());
        window.set_search_can_older(false);
        return;
    }
    if older {
        window.set_search_status("Reading older messages…".into());
    }
    send(Command::Search { filter, older });
}

/// [`twake_core::Event::SearchResults`]: shown unless the query changed meanwhile.
pub fn results(
    window: &AppWindow,
    room_id: &str,
    query: &str,
    results: &[IndexedMessage],
    is_complete: bool,
) {
    let current = STATE.with_borrow(|s| s.current_room == room_id);
    if !current || window.get_search_query().as_str() != query {
        return;
    }
    let now = chrono::Utc::now().timestamp_millis();
    let deleted = STATE.with_borrow(|s| s.deleted.clone());
    let hits: Vec<SearchHit> = results
        .iter()
        // What the user deleted is not found again (web `isAfterDeletion`)
        .filter(|r| twake_domain::deleted_chats::is_after_deletion(&deleted, room_id, r.sent_at))
        .map(|r| {
            let at = r.sent_at as i64;
            SearchHit {
                id: r.message_id.as_str().into(),
                sender: r.sender_name.as_str().into(),
                time: format!(
                    "{} {}",
                    twake_domain::format::room_list_time(at, now),
                    twake_domain::format::message_time(at)
                )
                .into(),
                text: r.text.as_str().into(),
            }
        })
        .collect();
    let found = match hits.len() {
        0 => "No message found".to_string(),
        1 => "1 message".to_string(),
        n => format!("{n} messages"),
    };
    let status = if is_complete {
        found
    } else {
        format!("{found} in what this device has read so far")
    };
    window.set_search_status(status.into());
    window.set_search_can_older(!is_complete);
    window.set_search_hits(ModelRc::from(Rc::new(VecModel::from(hits))));
}

/// A result was chosen: the messages around it open in the side panel.
pub fn open_hit(window: &AppWindow, id: &str) {
    open_in_context(window, id, "In context");
}

/// The messages around `id` in the side panel, `id` outlined, under `title`.
pub fn open_in_context(window: &AppWindow, id: &str, title: &str) {
    crate::close_thread(window);
    window.set_info_open(false);
    STATE.with_borrow_mut(|s| s.thread_root = Some("around".into()));
    TARGET.with_borrow_mut(|t| *t = id.to_string());
    window.set_side_title(title.into());
    window.set_side_pinned(true);
    window.set_thread_open(true);
    send(Command::OpenAround {
        event_id: id.to_string(),
    });
}
