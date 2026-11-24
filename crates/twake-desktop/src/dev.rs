//! Development hooks, driven by environment variables, to run the app unattended
//! against the local Synapse (`dev/up.sh`). All are off unless set.
//!
//! - `TWAKE_DEV_HOMESERVER`, `TWAKE_DEV_USER`, `TWAKE_DEV_PASSWORD`: sign in at start.
//! - `TWAKE_DEV_ROOM`: open the first room whose name contains this text.
//! - `TWAKE_DEV_SEND`: once that room is open, send this message.
//! - `TWAKE_DEV_SNAPSHOT`: save a PNG of the window to this path, `TWAKE_DEV_SNAPSHOT_AFTER`
//!   seconds (default 6) after the room opened, then quit when `TWAKE_DEV_QUIT` is set.
//! - `TWAKE_DEV_STATS`: print timing and resident memory at each step.
//! - `TWAKE_DEV_TYPE`: 3 s after the room opened, type this in the composer (its
//!   suggestions show); `TWAKE_DEV_PICK`: then take the first suggestion;
//!   `TWAKE_DEV_SUBMIT`: then send what the composer holds.

use std::time::{Duration, Instant};

use slint::ComponentHandle;

use crate::AppWindow;

fn var(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.is_empty())
}

pub struct Dev {
    started: Instant,
    signed_in_requested: bool,
    room_opened: bool,
    first_rooms: bool,
    all_rooms: bool,
}

impl Dev {
    pub fn new() -> Self {
        Dev {
            started: Instant::now(),
            signed_in_requested: false,
            room_opened: false,
            first_rooms: false,
            all_rooms: false,
        }
    }

    pub fn stat(&self, step: &str) {
        if var("TWAKE_DEV_STATS").is_some() {
            println!(
                "[dev] {step}: {} ms, resident {:.1} MB",
                self.started.elapsed().as_millis(),
                resident_mb()
            );
        }
    }

    /// Called when the sign-in page shows.
    pub fn on_signed_out(&mut self, window: &AppWindow) {
        if self.signed_in_requested {
            return;
        }
        if let (Some(user), Some(password)) = (var("TWAKE_DEV_USER"), var("TWAKE_DEV_PASSWORD")) {
            self.signed_in_requested = true;
            let homeserver =
                var("TWAKE_DEV_HOMESERVER").unwrap_or_else(|| window.get_homeserver().to_string());
            window.invoke_sign_in(homeserver.into(), user.into(), password.into());
        }
    }

    /// Called with each room list: opens the dev room once.
    pub fn on_rooms(&mut self, window: &AppWindow, rooms: &[twake_domain::RoomSummary]) {
        if !self.first_rooms && !rooms.is_empty() {
            self.first_rooms = true;
            self.stat(&format!("room list shown ({} rooms)", rooms.len()));
        }
        // `TWAKE_DEV_EXPECT_ROOMS`: note when the whole account is listed.
        let expected = var("TWAKE_DEV_EXPECT_ROOMS").and_then(|n| n.parse::<usize>().ok());
        if let Some(expected) = expected.filter(|n| !self.all_rooms && rooms.len() >= *n) {
            self.all_rooms = true;
            self.stat(&format!("all {expected} rooms listed"));
        }
        if self.room_opened {
            return;
        }
        let Some(wanted) = var("TWAKE_DEV_ROOM") else {
            return;
        };
        let Some(room) = rooms
            .iter()
            .find(|r| r.name.to_lowercase().contains(&wanted.to_lowercase()))
        else {
            return;
        };
        self.room_opened = true;
        self.stat(&format!("opening {} ({})", room.name, room.id));
        window.invoke_open_room(room.id.as_str().into());
        if let Some(text) = var("TWAKE_DEV_SEND") {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(2), move || {
                if let Some(w) = weak.upgrade() {
                    w.invoke_send(text.into());
                }
            });
        }
        // `TWAKE_DEV_STAGE`: put this file above the composer.
        if let Some(file) = var("TWAKE_DEV_STAGE") {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(2), move || {
                if let Some(w) = weak.upgrade() {
                    crate::staged::stage(&w, vec![file.into()], false);
                }
            });
        }
        // `TWAKE_DEV_VIEW_IMAGE`: open the newest image of the room in the viewer.
        if var("TWAKE_DEV_VIEW_IMAGE").is_some() {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(3), move || {
                let Some(w) = weak.upgrade() else { return };
                let image = crate::STATE.with_borrow(|s| {
                    s.entries.iter().rev().find_map(|e| match e {
                        twake_domain::TimelineEntry::Message(m) => m
                            .attachment
                            .as_ref()
                            .filter(|a| a.kind == twake_domain::attachment::AttachmentKind::Image)
                            .map(|a| (a.source.clone(), a.name.clone())),
                        _ => None,
                    })
                });
                if let Some((key, name)) = image {
                    crate::viewer::open(&w, &key, &name);
                }
            });
        }
        // `TWAKE_DEV_VIEW_PDF`: open the newest PDF of the room as « Open » does.
        if var("TWAKE_DEV_VIEW_PDF").is_some() {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(3), move || {
                let Some(w) = weak.upgrade() else { return };
                let pdf = crate::STATE.with_borrow(|s| {
                    s.entries.iter().rev().find_map(|e| match e {
                        twake_domain::TimelineEntry::Message(m) => m
                            .attachment
                            .as_ref()
                            .filter(|a| a.name.to_lowercase().ends_with(".pdf"))
                            .map(|a| (a.source.clone(), a.name.clone())),
                        _ => None,
                    })
                });
                if let Some((key, name)) = pdf {
                    w.invoke_open_attachment(key.into(), name.into());
                }
            });
        }
        if let Some(text) = var("TWAKE_DEV_TYPE") {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(3), move || {
                let Some(w) = weak.upgrade() else { return };
                w.set_compose_text(text.as_str().into());
                crate::composer::edited(&w, false, &text, text.len());
                if var("TWAKE_DEV_PICK").is_some() {
                    crate::composer::pick(&w, false, 0);
                }
                // `TWAKE_DEV_FORMAT`: `format:start:end[,…]` applied in turn, the bar shown
                if let Some(formats) = var("TWAKE_DEV_FORMAT") {
                    w.set_formats_open(true);
                    for step in formats.split(',') {
                        let mut parts = step.split(':');
                        let name = parts.next().unwrap_or("");
                        let mut num = || parts.next().and_then(|n| n.parse().ok()).unwrap_or(0);
                        let (start, end) = (num(), num());
                        let change =
                            crate::formats::format(&w.get_compose_text(), name, start, end);
                        if change.ok {
                            w.set_compose_text(change.text);
                        }
                    }
                }
                if var("TWAKE_DEV_SUBMIT").is_some() {
                    let typed = w.get_compose_text();
                    w.invoke_send(typed);
                }
            });
        }
        // `TWAKE_DEV_SEARCH`: search the room for this, `TWAKE_DEV_OPEN_HIT`: then open the
        // first result in context.
        if let Some(query) = var("TWAKE_DEV_SEARCH") {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(3), move || {
                let Some(w) = weak.upgrade() else { return };
                crate::search::open(&w);
                w.set_search_query(query.as_str().into());
                crate::search::search(&w, &query, false, false);
            });
            if var("TWAKE_DEV_OPEN_HIT").is_some() {
                let weak = window.as_weak();
                slint::Timer::single_shot(Duration::from_secs(5), move || {
                    let Some(w) = weak.upgrade() else { return };
                    use slint::Model;
                    if let Some(hit) = w.get_search_hits().row_data(0) {
                        crate::search::open_hit(&w, &hit.id);
                    }
                });
            }
        }
        // `TWAKE_DEV_JUMP`: go to this date (YYYY-MM-DD, or `start`), `TWAKE_DEV_DATE_PICKER`:
        // show the date picker.
        if let Some(date) = var("TWAKE_DEV_JUMP") {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(3), move || {
                let Some(w) = weak.upgrade() else { return };
                let mut parts = date.split('-').filter_map(|p| p.parse::<i32>().ok());
                match (parts.next(), parts.next(), parts.next()) {
                    (Some(y), Some(m), Some(d)) => w.invoke_jump_to_date(y, m, d),
                    _ => w.invoke_jump_to_beginning(),
                }
            });
        }
        if var("TWAKE_DEV_DATE_PICKER").is_some() {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(3), move || {
                if let Some(w) = weak.upgrade() {
                    w.invoke_open_date_picker();
                }
            });
        }
        // `TWAKE_DEV_VOICE`: send a voice message of this many seconds, a generated tone (no
        // microphone); `TWAKE_DEV_RECORDING_BAR`: show the recording bar as it looks.
        if let Some(seconds) = var("TWAKE_DEV_VOICE").and_then(|s| s.parse::<u32>().ok()) {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(3), move || {
                let Some(w) = weak.upgrade() else { return };
                let rate = 44_100u32;
                let tone: Vec<f32> = (0..rate * seconds)
                    .map(|i| {
                        let t = i as f32 / rate as f32;
                        // Syllables: the loudness rises and falls three times a second
                        let level = (t * 3.0 * std::f32::consts::PI).sin().abs() * 0.4;
                        (t * 220.0 * std::f32::consts::TAU).sin() * level
                    })
                    .collect();
                crate::voice::send_samples(&w, &tone, rate, u64::from(seconds) * 1000, false, None);
            });
        }
        if var("TWAKE_DEV_RECORDING_BAR").is_some() {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(3), move || {
                if let Some(w) = weak.upgrade() {
                    w.set_recording_in_thread(false);
                    w.set_recording_time("0:07".into());
                    w.set_recording(true);
                }
            });
        }
        // `TWAKE_DEV_ACTIVITY`: show the Activity.
        // `TWAKE_DEV_ACTIVITY`: show the Activity, with this chip when not `1` (`saved`…).
        if let Some(chip) = var("TWAKE_DEV_ACTIVITY") {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(5), move || {
                if let Some(w) = weak.upgrade() {
                    crate::activity::open(&w);
                    if chip != "1" {
                        crate::activity::pick_filter(&w, &chip);
                    }
                }
            });
        }
        // `TWAKE_DEV_SAVE`: save the first message of the room whose text holds this.
        if let Some(wanted) = var("TWAKE_DEV_SAVE") {
            slint::Timer::single_shot(Duration::from_secs(3), move || {
                let id = crate::STATE.with_borrow(|s| {
                    s.entries.iter().find_map(|e| match e {
                        twake_domain::TimelineEntry::Message(m) if m.text.contains(&wanted) => {
                            Some(m.id.clone())
                        }
                        _ => None,
                    })
                });
                if let Some(event_id) = id {
                    crate::send(twake_core::Command::SaveMessage {
                        event_id,
                        saved: true,
                    });
                }
            });
        }
        // `TWAKE_DEV_SWITCHER`: open the quick switcher with this query.
        if let Some(query) = var("TWAKE_DEV_SWITCHER") {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(3), move || {
                let Some(w) = weak.upgrade() else { return };
                crate::switcher::open(&w);
                w.set_switcher_query(query.as_str().into());
                crate::switcher::search(&w, &query);
            });
        }
        if var("TWAKE_DEV_OPEN_INFO").is_some() {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(2), move || {
                if let Some(w) = weak.upgrade() {
                    w.invoke_open_info();
                }
            });
        }
        if let Some(file) = var("TWAKE_DEV_SEND_FILE") {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(2), move || {
                if let Some(w) = weak.upgrade() {
                    crate::send_files(&w, vec![file.into()], false);
                }
            });
        }
        // `TWAKE_DEV_CALL`: `audio` or `video`, 3 s after the room opened, join its call
        // as the header's buttons do
        if let Some(kind) = var("TWAKE_DEV_CALL") {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(3), move || {
                if let Some(w) = weak.upgrade() {
                    w.invoke_start_call(kind == "video");
                }
            });
        }
        // `TWAKE_DEV_CALL_LEAVE`: leave the call this many seconds after the room opened
        if let Some(secs) = var("TWAKE_DEV_CALL_LEAVE").and_then(|s| s.parse().ok()) {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(secs), move || {
                if let Some(w) = weak.upgrade() {
                    w.invoke_leave_call();
                }
            });
        }
        // `TWAKE_DEV_SHARE`: press « Share the screen » this many seconds after the room opened
        if let Some(secs) = var("TWAKE_DEV_SHARE").and_then(|s| s.parse().ok()) {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(secs), move || {
                if let Some(w) = weak.upgrade() {
                    w.invoke_call_toggle_screen();
                }
            });
        }
        // `TWAKE_DEV_CALL_CHANNEL`: press « Show the channel » this many seconds after the
        // room opened
        if let Some(secs) = var("TWAKE_DEV_CALL_CHANNEL").and_then(|s| s.parse().ok()) {
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(secs), move || {
                if let Some(w) = weak.upgrade() {
                    w.invoke_call_thread_toggle();
                }
            });
        }
        // `TWAKE_DEV_RING`: `answer` or `decline` the call that rings, 5 s after it shows
        if let Some(action) = var("TWAKE_DEV_RING") {
            let weak = window.as_weak();
            let shown = std::rc::Rc::new(std::cell::Cell::new(None::<Instant>));
            let timer = slint::Timer::default();
            timer.start(
                slint::TimerMode::Repeated,
                Duration::from_millis(500),
                move || {
                    let Some(w) = weak.upgrade() else { return };
                    if w.get_ring_text().is_empty() {
                        return;
                    }
                    let since = shown.get().unwrap_or_else(Instant::now);
                    shown.set(Some(since));
                    if since.elapsed() >= Duration::from_secs(5) {
                        shown.set(None);
                        println!("[dev] ring {action}: {}", w.get_ring_text());
                        if action == "answer" {
                            w.invoke_ring_answer();
                        } else {
                            w.invoke_ring_decline();
                        }
                    }
                },
            );
            // Dev only: the hook watches for the whole run
            std::mem::forget(timer);
        }
        if let Some(path) = var("TWAKE_DEV_SNAPSHOT") {
            let after = var("TWAKE_DEV_SNAPSHOT_AFTER")
                .and_then(|s| s.parse().ok())
                .unwrap_or(6);
            let weak = window.as_weak();
            slint::Timer::single_shot(Duration::from_secs(after), move || {
                if let Some(w) = weak.upgrade() {
                    match save_snapshot(&w, &path) {
                        Ok(()) => println!("[dev] snapshot saved to {path}"),
                        Err(e) => eprintln!("[dev] snapshot failed: {e}"),
                    }
                    // `TWAKE_DEV_SNAPSHOT_AGAIN`: ms later, a second one (`-2.png`), to see
                    // what moved
                    if let Some(ms) = var("TWAKE_DEV_SNAPSHOT_AGAIN").and_then(|s| s.parse().ok()) {
                        let path = path.replace(".png", "-2.png");
                        let weak = w.as_weak();
                        slint::Timer::single_shot(Duration::from_millis(ms), move || {
                            if let Some(w) = weak.upgrade() {
                                let _ = save_snapshot(&w, &path);
                            }
                            if var("TWAKE_DEV_QUIT").is_some() {
                                let _ = slint::quit_event_loop();
                            }
                        });
                        return;
                    }
                    if var("TWAKE_DEV_STATS").is_some() {
                        println!("[dev] resident at snapshot: {:.1} MB", resident_mb());
                    }
                    if var("TWAKE_DEV_QUIT").is_some() {
                        let _ = slint::quit_event_loop();
                    }
                }
            });
        }
    }
}

fn save_snapshot(window: &AppWindow, path: &str) -> Result<(), String> {
    let buffer = window.window().take_snapshot().map_err(|e| e.to_string())?;
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut encoder = png::Encoder::new(
        std::io::BufWriter::new(file),
        buffer.width(),
        buffer.height(),
    );
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
    writer
        .write_image_data(buffer.as_bytes())
        .map_err(|e| e.to_string())
}

/// `TWAKE_DEV_OPEN_THREAD`: open, once, the thread of the first message whose text contains
/// this.
pub fn on_timeline(window: &AppWindow, entries: &[twake_domain::TimelineEntry]) {
    thread_local! { static DONE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
    let Some(wanted) = var("TWAKE_DEV_OPEN_THREAD") else {
        return;
    };
    if DONE.get() {
        return;
    }
    let root = entries.iter().find_map(|e| match e {
        twake_domain::TimelineEntry::Message(m)
            if m.thread.is_some() && m.text.contains(&wanted) =>
        {
            Some(m.id.clone())
        }
        _ => None,
    });
    if let Some(id) = root {
        DONE.set(true);
        println!("[dev] opening the thread of {id}");
        window.invoke_open_thread(id.into());
    }
}

/// `TWAKE_DEV_SPACE`: pick, once, the space whose name contains this.
pub fn on_spaces(window: &AppWindow, spaces: &[twake_domain::SpaceSummary]) {
    thread_local! { static DONE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
    let Some(wanted) = var("TWAKE_DEV_SPACE") else {
        return;
    };
    if DONE.get() {
        return;
    }
    if let Some(space) = spaces.iter().find(|s| s.name.contains(&wanted)) {
        DONE.set(true);
        println!("[dev] picking the space {}", space.name);
        window.invoke_pick_space(space.id.as_str().into());
    }
}

/// `TWAKE_DEV_RECOVERY_KEY_FILE`: enter the recovery key read from this file once the
/// app asks for it.
pub fn on_needs_recovery_key(window: &AppWindow) {
    thread_local! { static DONE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
    if DONE.get() {
        return;
    }
    let Some(path) = var("TWAKE_DEV_RECOVERY_KEY_FILE") else {
        return;
    };
    if let Ok(key) = std::fs::read_to_string(path) {
        DONE.set(true);
        println!("[dev] entering the recovery key");
        window.invoke_recover(key.trim().into());
    }
}

/// Resident memory of this process in MB (`ps`, macOS and Linux).
pub fn resident_mb() -> f64 {
    let pid = std::process::id().to_string();
    std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &pid])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse::<f64>().ok())
        .map_or(0.0, |kb| kb / 1024.0)
}
