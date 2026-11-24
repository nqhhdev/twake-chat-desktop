//! The quick switcher (Cmd+K on macOS, Ctrl+K elsewhere): find a conversation by name.

use std::cell::RefCell;
use std::rc::Rc;

use slint::{ModelRc, VecModel};
use twake_domain::RoomKind;
use twake_domain::mention::{LinkableRoom, match_rooms};

use crate::{AppWindow, STATE, Suggestion};

/// Shown when nothing is typed: the most recent conversations.
const RECENT: usize = 8;

thread_local! {
    /// The room ids of the lines shown, in order.
    static SHOWN: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub fn open(window: &AppWindow) {
    window.set_switcher_query("".into());
    search(window, "");
    window.set_switcher_open(true);
}

/// The conversations whose name or a word of it matches `query` (`matchRooms`); the
/// recent ones, the open one left out, when the query is empty.
pub fn search(window: &AppWindow, query: &str) {
    let (items, ids) = STATE.with_borrow(|s| {
        let rooms: Vec<&twake_domain::RoomSummary> =
            s.rooms.iter().filter(|r| !r.is_invite).collect();
        let found: Vec<&twake_domain::RoomSummary> = if twake_domain::js::trim(query).is_empty() {
            rooms
                .iter()
                .filter(|r| r.id != s.current_room)
                .take(RECENT)
                .copied()
                .collect()
        } else {
            let linkable: Vec<LinkableRoom> = rooms
                .iter()
                .map(|r| LinkableRoom {
                    id: r.id.clone(),
                    name: r.name.clone(),
                    alias: None,
                })
                .collect();
            match_rooms(&linkable, query, RECENT)
                .into_iter()
                .filter_map(|l| rooms.iter().find(|r| r.id == l.id).copied())
                .collect()
        };
        let items: Vec<Suggestion> = found
            .iter()
            .map(|r| Suggestion {
                icon: match r.kind {
                    RoomKind::Direct => "@",
                    _ => "#",
                }
                .into(),
                label: r.name.as_str().into(),
                detail: if r.unread_count > 0 {
                    format!("{} unread", r.unread_count).into()
                } else {
                    "".into()
                },
            })
            .collect();
        (
            items,
            found.iter().map(|r| r.id.clone()).collect::<Vec<_>>(),
        )
    });
    SHOWN.with_borrow_mut(|shown| *shown = ids);
    window.set_switcher_items(ModelRc::from(Rc::new(VecModel::from(items))));
    window.set_switcher_selected(0);
}

/// A line was chosen: its conversation opens.
pub fn choose(window: &AppWindow, index: usize) {
    let id = SHOWN.with_borrow(|shown| shown.get(index).cloned());
    window.set_switcher_open(false);
    if let Some(id) = id {
        window.invoke_open_room(id.into());
    }
}
