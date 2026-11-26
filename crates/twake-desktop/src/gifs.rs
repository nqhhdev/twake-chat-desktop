//! Animated images of the timeline (web `MessageItem`, `useAutoplay`): a GIF plays where
//! its thumbnail was, unless the user turned « Autoplay animated images » off (then its
//! still thumbnail stays, as the web's first frame). Its frames are decoded by the engine
//! ([`Command::FetchAnimation`]) at the size shown; one timer moves all of them, and only
//! the frame changes in the window (the `Animations` global), never the rows. The frames
//! of a room are dropped when another opens.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::{Duration, Instant};

use slint::{ComponentHandle, Model, VecModel};
use twake_core::{Command, Picture};

use crate::{Animations, AppWindow, Row};

struct Animation {
    frames: Vec<slint::Image>,
    delays: Vec<Duration>,
    at: usize,
    next: Instant,
}

#[derive(Default)]
struct State {
    animations: Vec<Animation>,
    /// `key@WxH` → its index in `animations`.
    by_id: HashMap<String, usize>,
    asked: HashSet<String>,
    /// The user's choice; None until the account data is read (then it plays).
    autoplay: Option<bool>,
    shown: Option<Rc<VecModel<slint::Image>>>,
    timer: Option<slint::Timer>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn id(key: &str, width: u32, height: u32) -> String {
    format!("{key}@{width}x{height}")
}

/// Gives the window the frames shown.
pub fn install(window: &AppWindow) {
    let shown = Rc::new(VecModel::<slint::Image>::default());
    window
        .global::<Animations>()
        .set_frames(slint::ModelRc::from(shown.clone()));
    STATE.with_borrow_mut(|s| s.shown = Some(shown));
}

/// The animation of a row, 1-based (0: none, or not decoded yet: then it is asked for).
pub fn slot(row: &Row, density: u32, requests: &mut Vec<Command>) -> i32 {
    if row.attachment_kind != "image" || row.media_width <= 0 || row.media_height <= 0 {
        return 0;
    }
    slot_of(
        &row.attachment_key,
        &row.attachment_mime,
        row.media_width as u32 * density,
        row.media_height as u32 * density,
        requests,
    )
}

/// The animation of the image of `key` at this size in pixels, as [`slot`].
pub fn slot_of(key: &str, mime: &str, width: u32, height: u32, requests: &mut Vec<Command>) -> i32 {
    if !twake_domain::attachment::is_gif(mime) || width == 0 || height == 0 {
        return 0;
    }
    let id = id(key, width, height);
    STATE.with_borrow_mut(|s| {
        if s.autoplay == Some(false) {
            return 0;
        }
        if let Some(index) = s.by_id.get(&id) {
            return i32::try_from(*index + 1).unwrap_or(0);
        }
        if s.asked.insert(id) {
            requests.push(Command::FetchAnimation {
                key: key.to_string(),
                width,
                height,
            });
        }
        0
    })
}

/// The frames of an animation came (empty: it stays still). True when the rows must be
/// drawn again to show it.
pub fn arrived(
    window: &AppWindow,
    key: &str,
    width: u32,
    height: u32,
    frames: Vec<(Picture, u32)>,
) -> bool {
    let id = id(key, width, height);
    if frames.is_empty() || STATE.with_borrow(|s| !s.asked.contains(&id)) {
        return false;
    }
    let (images, delays): (Vec<_>, Vec<_>) = frames
        .into_iter()
        .map(|(p, ms)| {
            let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
                &p.rgba, p.width, p.height,
            );
            (
                slint::Image::from_rgba8(buffer),
                Duration::from_millis(u64::from(ms)),
            )
        })
        .unzip();
    let first = images[0].clone();
    STATE.with_borrow_mut(|s| {
        let index = s.animations.len();
        s.animations.push(Animation {
            next: Instant::now() + delays[0],
            frames: images,
            delays,
            at: 0,
        });
        s.by_id.insert(id, index);
        if let Some(shown) = &s.shown {
            shown.push(first);
        }
    });
    start_timer(window);
    true
}

fn start_timer(window: &AppWindow) {
    let running = STATE.with_borrow(|s| s.timer.as_ref().is_some_and(slint::Timer::running));
    if running {
        return;
    }
    let timer = slint::Timer::default();
    let weak = window.as_weak();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(10),
        move || {
            // Nothing moves in a window that is not shown
            if weak.upgrade().is_none_or(|w| !w.window().is_visible()) {
                return;
            }
            tick(Instant::now());
        },
    );
    STATE.with_borrow_mut(|s| s.timer = Some(timer));
}

/// Moves each animation whose frame has shown long enough to its next one.
fn tick(now: Instant) {
    STATE.with_borrow_mut(|s| {
        let Some(shown) = s.shown.clone() else { return };
        for (index, a) in s.animations.iter_mut().enumerate() {
            if now < a.next {
                continue;
            }
            a.at = (a.at + 1) % a.frames.len();
            // Late (a busy window): the next frame waits its whole time from now
            a.next = (a.next + a.delays[a.at]).max(now);
            if index < shown.row_count() {
                shown.set_row_data(index, a.frames[a.at].clone());
            }
        }
    });
}

/// Another room opened: its animations go.
pub fn clear() {
    STATE.with_borrow_mut(|s| {
        s.animations.clear();
        s.by_id.clear();
        s.asked.clear();
        if let Some(shown) = &s.shown {
            shown.set_vec(Vec::new());
        }
        if let Some(timer) = &s.timer {
            timer.stop();
        }
    });
}

/// The user's choice changed (here or on the web). True when the rows must be drawn
/// again.
pub fn set_autoplay(enabled: Option<bool>) -> bool {
    let before = STATE.with_borrow(|s| s.autoplay != Some(false));
    let after = enabled != Some(false);
    STATE.with_borrow_mut(|s| s.autoplay = enabled);
    if before && !after {
        clear();
    }
    before != after
}
