//! « Public groups » (web `PublicRoomsDialog`, D42): the rooms of the homeserver's directory
//! the user has not joined, found by name; a press joins and opens one, or asks to join one
//! on request.

use std::cell::RefCell;
use std::rc::Rc;

use slint::{ModelRc, VecModel};
use twake_core::{Command, PublicRoom};
use twake_domain::search::normalize_for_search;

use crate::{AppWindow, STATE, Suggestion, send};

#[derive(Default)]
struct Directory {
    rooms: Vec<PublicRoom>,
    /// The rooms listed now, in the order shown.
    shown: Vec<PublicRoom>,
    loaded: bool,
}

thread_local! {
    static DIRECTORY: RefCell<Directory> = RefCell::new(Directory::default());
}

/// The 🪐 button: the spotlight opens, the directory read again.
pub fn open(window: &AppWindow) {
    DIRECTORY.with_borrow_mut(|d| *d = Directory::default());
    window.set_directory_query("".into());
    window.set_directory_selected(0);
    refresh(window);
    window.set_directory_shown(true);
    send(Command::PublicRooms);
}

pub fn loaded(window: &AppWindow, rooms: Vec<PublicRoom>) {
    DIRECTORY.with_borrow_mut(|d| {
        d.rooms = rooms;
        d.loaded = true;
    });
    refresh(window);
}

pub fn search(window: &AppWindow) {
    window.set_directory_selected(0);
    refresh(window);
}

fn refresh(window: &AppWindow) {
    let needle = normalize_for_search(window.get_directory_query().trim());
    let joined: Vec<String> = STATE.with_borrow(|s| s.rooms.iter().map(|r| r.id.clone()).collect());
    let (shown, loaded) = DIRECTORY.with_borrow(|d| {
        let shown: Vec<PublicRoom> = d
            .rooms
            .iter()
            .filter(|r| !joined.contains(&r.room_id))
            .filter(|r| normalize_for_search(&r.name).contains(&needle))
            .cloned()
            .collect();
        (shown, d.loaded)
    });
    let items: Vec<Suggestion> = shown
        .iter()
        .map(|r| {
            let members = if r.member_count == 1 {
                "1 member".to_string()
            } else {
                format!("{} members", r.member_count)
            };
            Suggestion {
                icon: match r.kind {
                    "space" => "🪐",
                    "broadcast" => "📢",
                    _ => "#",
                }
                .into(),
                label: r.name.as_str().into(),
                detail: if r.on_approval {
                    format!("Ask to join · {members}")
                } else {
                    format!("Join · {members}")
                }
                .into(),
            }
        })
        .collect();
    window.set_directory_empty(
        if !loaded {
            "Loading public groups"
        } else if items.is_empty() {
            "No public group to join."
        } else {
            ""
        }
        .into(),
    );
    window.set_directory_items(ModelRc::from(Rc::new(VecModel::from(items))));
    DIRECTORY.with_borrow_mut(|d| d.shown = shown);
}

/// A press or Enter: joins and opens the room (the engine says once joined), or asks.
pub fn choose(window: &AppWindow, index: usize) {
    let Some(room) = DIRECTORY.with_borrow(|d| d.shown.get(index).cloned()) else {
        return;
    };
    send(Command::JoinPublic {
        room_id: room.room_id,
        on_approval: room.on_approval,
    });
    window.set_directory_shown(false);
}
