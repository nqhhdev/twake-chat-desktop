//! « Forward to… » (web `ForwardDialog`, `forwardTargets`): the conversations a message may
//! go to, ticked in the order picked, and what is said once it went.

use std::cell::RefCell;
use std::rc::Rc;

use slint::{ModelRc, VecModel};
use twake_core::Command;
use twake_domain::RoomKind;
use twake_domain::search::normalize_for_search;

use crate::{AppWindow, ForwardRow, STATE, send};

/// At most this many conversations listed; typing narrows a long list (web).
const SHOWN: usize = 50;

#[derive(Default)]
struct Forward {
    /// The messages that go, in timeline order.
    event_ids: Vec<String>,
    /// The conversations ticked, in the order picked, with their label.
    picked: Vec<(String, String)>,
    rows: Rc<VecModel<ForwardRow>>,
}

thread_local! {
    static FORWARD: RefCell<Forward> = RefCell::new(Forward::default());
}

pub fn install(window: &AppWindow) {
    FORWARD.with_borrow(|f| window.set_forward_rows(ModelRc::from(f.rows.clone())));
}

/// The conversations of the list with their label: the space's name after a room's own
/// (web `roomLabelWithSpace`), not for a direct message.
fn labelled_rooms() -> Vec<(String, String)> {
    STATE.with_borrow(|s| {
        s.rooms
            .iter()
            .filter(|r| r.id != s.current_room && !r.is_invite)
            .map(|r| {
                let space = (r.kind != RoomKind::Direct)
                    .then(|| s.spaces.iter().find(|sp| sp.rooms.contains(&r.id)))
                    .flatten()
                    .filter(|sp| sp.name != r.name);
                let label = match space {
                    Some(sp) => format!("{} ({})", r.name, sp.name),
                    None => r.name.clone(),
                };
                (r.id.clone(), label)
            })
            .collect()
    })
}

fn refresh(window: &AppWindow) {
    let needle = normalize_for_search(window.get_forward_filter().trim());
    let rows: Vec<ForwardRow> = labelled_rooms()
        .into_iter()
        .filter(|(_, label)| normalize_for_search(label).contains(&needle))
        .take(SHOWN)
        .map(|(id, name)| ForwardRow {
            picked: FORWARD.with_borrow(|f| f.picked.iter().any(|(p, _)| *p == id)),
            id: id.into(),
            name: name.into(),
        })
        .collect();
    FORWARD.with_borrow(|f| {
        f.rows.set_vec(rows);
        window.set_forward_has_picked(!f.picked.is_empty());
    });
}

/// « Forward »: the dialog opens for the messages `event_ids`, nothing ticked.
pub fn open(window: &AppWindow, event_ids: Vec<String>) {
    FORWARD.with_borrow_mut(|f| {
        f.event_ids = event_ids;
        f.picked.clear();
    });
    window.set_forward_filter("".into());
    refresh(window);
    window.set_forward_open(true);
}

pub fn filter_edited(window: &AppWindow) {
    refresh(window);
}

/// A click on a conversation ticks or unticks it.
pub fn toggle(window: &AppWindow, room_id: &str) {
    let label = labelled_rooms()
        .into_iter()
        .find(|(id, _)| id == room_id)
        .map(|(_, label)| label);
    FORWARD.with_borrow_mut(|f| {
        if let Some(i) = f.picked.iter().position(|(id, _)| id == room_id) {
            f.picked.remove(i);
        } else if let Some(label) = label {
            f.picked.push((room_id.to_string(), label));
        }
    });
    refresh(window);
}

pub fn send_forward(window: &AppWindow) {
    let (event_ids, to_rooms) = FORWARD.with_borrow(|f| {
        (
            f.event_ids.clone(),
            f.picked
                .iter()
                .map(|(id, _)| id.clone())
                .collect::<Vec<_>>(),
        )
    });
    if !to_rooms.is_empty() {
        send(Command::Forward {
            event_ids,
            to_rooms,
        });
    }
    window.set_forward_open(false);
}

pub fn cancel(window: &AppWindow) {
    window.set_forward_open(false);
}

/// It went: « Forwarded to Design, Bob » (web `chat.forwarded`, the names in the order
/// picked).
pub fn forwarded(window: &AppWindow, to_rooms: &[String]) {
    let names: Vec<String> = FORWARD.with_borrow(|f| {
        f.picked
            .iter()
            .filter(|(id, _)| to_rooms.contains(id))
            .map(|(_, label)| label.clone())
            .collect()
    });
    window.set_notice(format!("Forwarded to {}", names.join(", ")).into());
}
