//! Messages ticked to act on several at once (web `MessageSelectionBar`, D183): forward them,
//! download their files, copy them, pin or unpin them, delete the user's own.

use std::cell::RefCell;

use slint::Model;
use twake_core::Command;
use twake_domain::TimelineEntry;

use crate::{AppWindow, Row, STATE, send};

thread_local! {
    /// The ids ticked; None outside selection mode.
    static SELECTED: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
}

pub fn is_active() -> bool {
    SELECTED.with_borrow(Option::is_some)
}

pub fn is_selected(id: &str) -> bool {
    SELECTED.with_borrow(|s| s.as_ref().is_some_and(|ids| ids.iter().any(|i| i == id)))
}

/// « Select » in a message's menu starts the mode with it ticked; in the mode, a click on a
/// message ticks or unticks it.
pub fn select(window: &AppWindow, id: &str) {
    SELECTED.with_borrow_mut(|s| {
        let ids = s.get_or_insert_with(Vec::new);
        if let Some(i) = ids.iter().position(|x| x == id) {
            ids.remove(i);
        } else {
            ids.push(id.to_string());
        }
    });
    refresh(window);
}

/// The cross: the mode ends, nothing ticked.
pub fn close(window: &AppWindow) {
    SELECTED.with_borrow_mut(|s| *s = None);
    refresh(window);
}

/// Another room opens: the mode ends without drawing the old one again.
pub fn reset(window: &AppWindow) {
    SELECTED.with_borrow_mut(|s| *s = None);
    window.set_selecting(false);
}

/// The rows ticked, in timeline order.
fn chosen() -> Vec<Row> {
    STATE.with_borrow(|s| {
        s.rows_model
            .iter()
            .filter(|r| r.kind == 0 && is_selected(&r.id))
            .collect()
    })
}

fn plural(n: usize, one: &str, many: &str) -> String {
    if n == 1 {
        format!("1 {one}")
    } else {
        format!("{n} {many}")
    }
}

fn refresh(window: &AppWindow) {
    crate::redraw_messages(window);
    let chosen = chosen();
    let count = chosen.len();
    let files = chosen
        .iter()
        .filter(|r| !r.attachment_kind.is_empty())
        .count();
    // « 5 files » when every message ticked is one (web)
    let text = if count > 0 && files == count {
        plural(count, "file", "files")
    } else {
        plural(count, "message selected", "messages selected")
    };
    let can_pin = STATE.with_borrow(|s| s.rows_model.iter().any(|r| r.can_pin));
    window.set_selecting(is_active());
    window.set_selection_text(text.into());
    window.set_selection_count(i32::try_from(count).unwrap_or(i32::MAX));
    // A poll is never forwarded (web D413)
    window.set_selection_can_forward(count > 0 && !chosen.iter().any(|r| r.has_poll));
    window.set_selection_has_files(files > 0);
    window.set_selection_can_pin(can_pin);
    window.set_selection_unpin(count > 0 && chosen.iter().all(|r| r.pinned));
    window.set_selection_can_delete(chosen.iter().any(|r| r.can_delete));
}

pub fn forward(window: &AppWindow) {
    let ids: Vec<String> = chosen().iter().map(|r| r.id.to_string()).collect();
    close(window);
    crate::forward::open(window, ids);
}

pub fn download(window: &AppWindow) {
    for r in chosen().iter().filter(|r| !r.attachment_kind.is_empty()) {
        send(Command::SaveAttachment {
            key: r.attachment_key.to_string(),
            name: r.attachment_name.to_string(),
        });
    }
    close(window);
}

/// Author and time, then the text (or the file's name), in timeline order (web `copy`).
pub fn copy(window: &AppWindow) {
    let ids: Vec<String> = chosen().iter().map(|r| r.id.to_string()).collect();
    let now = chrono::Utc::now().timestamp_millis();
    let text = STATE.with_borrow(|s| {
        ids.iter()
            .filter_map(|id| {
                s.entries.iter().find_map(|e| match e {
                    TimelineEntry::Message(m) if &m.id == id => Some(m),
                    _ => None,
                })
            })
            .map(|m| {
                let body = if m.text.is_empty() {
                    m.attachment.as_ref().map_or("", |a| a.name.as_str())
                } else {
                    m.text.as_str()
                };
                format!(
                    "{}, {} {}\n{body}",
                    m.sender_name,
                    twake_domain::format::room_list_time(m.sent_at, now),
                    twake_domain::format::message_time(m.sent_at)
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    });
    crate::copy_to_clipboard(window, &text, "Messages copied");
    close(window);
}

/// Pin them all, or unpin them all when every one is pinned, in one update.
pub fn pin(window: &AppWindow) {
    let chosen = chosen();
    let unpin = !chosen.is_empty() && chosen.iter().all(|r| r.pinned);
    send(Command::Pin {
        ids: chosen.iter().map(|r| r.id.to_string()).collect(),
        pinned: !unpin,
    });
    close(window);
}

/// Deletes the user's own messages ticked, after one confirmation (the mode ends then).
pub fn delete(window: &AppWindow) {
    let deletable: Vec<Command> = chosen()
        .iter()
        .filter(|r| r.can_delete)
        .map(|r| Command::Delete {
            id: r.id.to_string(),
            thread: false,
            reason: None,
        })
        .collect();
    let n = deletable.len();
    let title = if n == 1 {
        "Delete 1 message?".to_string()
    } else {
        format!("Delete {n} messages?")
    };
    crate::ask_many(
        window,
        &format!(
            "{title}\n\nOnly your own messages are deleted, for everyone in the conversation."
        ),
        "Delete",
        deletable,
    );
}
