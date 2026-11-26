//! The image viewer (web `RoomImageViewer`): an image of the conversation over the whole
//! window, fetched at the size of the screen; Save and Open in app beside it.

use std::cell::RefCell;

use twake_core::Command;
use twake_domain::TimelineEntry;

use crate::{AppWindow, STATE, send};

/// The longest side the viewer asks for (the screen of a laptop, at density 2).
const MAX_SIDE: u32 = 2400;

#[derive(Default)]
struct Viewing {
    key: String,
    name: String,
    /// `key@WxH` of the picture shown.
    id: String,
    mime: String,
}

thread_local! {
    static VIEWING: RefCell<Viewing> = RefCell::new(Viewing::default());
}

/// The size of the image of `key`, fitted in `MAX_SIDE`, and its type, from the messages
/// on screen.
fn fitted_size(key: &str) -> (u32, u32, String) {
    let found = STATE.with_borrow(|s| {
        s.entries
            .iter()
            .chain(s.thread_entries.iter())
            .find_map(|e| match e {
                TimelineEntry::Message(m) => m
                    .attachment
                    .as_ref()
                    .filter(|a| a.source == key)
                    .map(|a| (a.dimensions, a.mime_type.clone())),
                _ => None,
            })
    });
    let (dims, mime) = found.unwrap_or_default();
    let (w, h) = dims.unwrap_or((MAX_SIDE, MAX_SIDE));
    let scale = (MAX_SIDE as f64 / w.max(h).max(1) as f64).min(1.0);
    (
        ((w as f64 * scale).round() as u32).max(1),
        ((h as f64 * scale).round() as u32).max(1),
        mime,
    )
}

pub fn open(window: &AppWindow, key: &str, name: &str) {
    let (width, height, mime) = fitted_size(key);
    VIEWING.with_borrow_mut(|v| {
        v.key = key.to_string();
        v.name = name.to_string();
        v.id = format!("{key}@{width}x{height}");
    });
    window.set_viewer_name(name.into());
    window.set_viewer_ready(false);
    window.set_viewer_open(true);
    let shown = STATE.with_borrow_mut(|s| {
        let id = format!("{key}@{width}x{height}");
        let image = s.pictures.get(&id).cloned();
        if image.is_none() && s.asked.insert(id) {
            send(Command::FetchImage {
                key: key.to_string(),
                width,
                height,
                crop: false,
            });
        }
        image
    });
    if let Some(image) = shown {
        window.set_viewer_image(image);
        window.set_viewer_ready(true);
    }
    VIEWING.with_borrow_mut(|v| v.mime = mime);
    refresh_animation(window);
}

/// A GIF plays in the viewer as in the timeline (asked for here, shown once decoded).
pub fn refresh_animation(window: &AppWindow) {
    if !window.get_viewer_open() {
        window.set_viewer_anim(0);
        return;
    }
    let (key, mime) = VIEWING.with_borrow(|v| (v.key.clone(), v.mime.clone()));
    let (width, height, _) = fitted_size(&key);
    let mut requests = Vec::new();
    window.set_viewer_anim(crate::gifs::slot_of(
        &key,
        &mime,
        width,
        height,
        &mut requests,
    ));
    requests.into_iter().for_each(send);
}

/// A picture arrived: the viewer shows it when it is the one asked for.
pub fn refresh(window: &AppWindow) {
    if !window.get_viewer_open() || window.get_viewer_ready() {
        return;
    }
    let id = VIEWING.with_borrow(|v| v.id.clone());
    if let Some(image) = STATE.with_borrow(|s| s.pictures.get(&id).cloned()) {
        window.set_viewer_image(image);
        window.set_viewer_ready(true);
    }
}

pub fn save() {
    let (key, name) = VIEWING.with_borrow(|v| (v.key.clone(), v.name.clone()));
    send(Command::SaveAttachment { key, name });
}

pub fn open_external() {
    let (key, name) = VIEWING.with_borrow(|v| (v.key.clone(), v.name.clone()));
    send(Command::OpenAttachment { key, name });
}
