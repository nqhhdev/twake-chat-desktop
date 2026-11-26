//! Calls (M5): the call buttons of the room header (web « Start a call », « Join the
//! call (n people) »), the call of this device over the room it is in, and its grid
//! (web `CallView`, `CallStage`). The media are in the engine (twake-call).

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use slint::{ComponentHandle, Model, VecModel};
use twake_core::{
    CallDeviceList, CallStatus, CallTile, Command, DeviceKind, IncomingCall, Picture,
};
use twake_domain::TimelineEntry;

use crate::view::{initials, tint};
use twake_domain::call_window::{self as cw, CallPlacement, WindowRect, WindowViewport};

use crate::{
    AppWindow, CallPopOut, CallTileRow, CallWindowRect, DeviceRow, DeviceSection, STATE, send,
};

/// Room between tiles (as in call.slint, web `TILE_GAP`).
const TILE_GAP: f32 = 8.0;

#[derive(Default)]
struct CallUi {
    /// How many people are in the call of each room (MatrixRTC).
    people: HashMap<String, usize>,
    /// The room of this device's call.
    room: Option<String>,
    tiles: Option<Rc<VecModel<CallTileRow>>>,
    microphone: bool,
    camera: bool,
    /// The calls that ring, and the rooms whose ring the user answered or declined here.
    ringing: Vec<IncomingCall>,
    attended: std::collections::HashSet<String>,
    /// The room of the ring the banner shows.
    shown_ring: Option<String>,
    /// The user shares a screen (their screen tile is in the call).
    sharing: bool,
    /// The thread of the call (web `useCallThread`), the one for which the user chose the
    /// channel instead, and the one last opened for them.
    call_thread: Option<String>,
    channel_shown_for: Option<String>,
    thread_opened_for: Option<String>,
    /// Where the call shows: in its room or in its window (web `callPlacement`, D571).
    placement: CallPlacement,
    /// Where the window of the call was left, kept for the next call.
    window_rect: Option<WindowRect>,
}

thread_local! {
    static CALL: RefCell<CallUi> = RefCell::new(CallUi::default());
    /// The call out of the app, in a window of its own (web `PipCall`).
    static POP_OUT: RefCell<Option<CallPopOut>> = const { RefCell::new(None) };
}

/// The width of the largest 16:9 tiles that fit `count` tiles in `width` × `height`,
/// trying every number of columns (web `tileWidth`).
pub fn tile_width(count: i32, width: f32, height: f32) -> f32 {
    let count = count.max(1);
    let mut best: f32 = 0.0;
    for columns in 1..=count {
        let rows = (count + columns - 1) / columns;
        let by_width = (width - TILE_GAP * (columns - 1) as f32) / columns as f32;
        let by_height = (height - TILE_GAP * (rows - 1) as f32) / rows as f32 * (16.0 / 9.0);
        best = best.max(by_width.min(by_height));
    }
    best.floor().max(0.0)
}

pub fn install(window: &AppWindow) {
    let tiles = Rc::new(VecModel::<CallTileRow>::default());
    window.set_call_tiles(slint::ModelRc::from(tiles.clone()));
    let kept = std::fs::read_to_string(window_file()).ok();
    CALL.with_borrow_mut(|c| {
        c.tiles = Some(tiles);
        c.window_rect = cw::parse_call_window(kept.as_deref());
    });
    window.on_call_tile_width(tile_width);
    // The window of the call follows the domain's rules (web `FloatingCall`)
    let weak = window.as_weak();
    window.on_call_window_moved(move |from, dx, dy| {
        let Some(w) = weak.upgrade() else { return from };
        to_slint(cw::move_window(
            to_domain(from),
            f64::from(dx),
            f64::from(dy),
            area(&w),
        ))
    });
    let weak = window.as_weak();
    window.on_call_window_resized(move |from, edge, dx, dy| {
        let Some(w) = weak.upgrade() else { return from };
        let (dx, dy) = (f64::from(dx), f64::from(dy));
        to_slint(cw::resize_window(to_domain(from), &edge, dx, dy, area(&w)))
    });
    let weak = window.as_weak();
    window.on_call_window_cornered(move |from, corner| {
        let Some(w) = weak.upgrade() else { return from };
        to_slint(cw::place_in_corner(to_domain(from), &corner, area(&w)))
    });
    // Whole in the app, as it is resized (web: on `resize`)
    let weak = window.as_weak();
    window.on_call_window_fitted(move |rect| {
        let Some(w) = weak.upgrade() else { return rect };
        to_slint(cw::fit_window(to_domain(rect), area(&w)))
    });
    let weak = window.as_weak();
    window.on_call_window_chosen(move |rect| {
        if let Some(w) = weak.upgrade() {
            choose_window(&w, to_domain(rect));
        }
    });
}

/// The file the window of the call is kept in (web `localStorage`).
fn window_file() -> std::path::PathBuf {
    twake_core::data_dir().join("call-window.json")
}

fn to_domain(rect: CallWindowRect) -> WindowRect {
    WindowRect {
        x: f64::from(rect.x),
        y: f64::from(rect.y),
        width: f64::from(rect.width),
        height: f64::from(rect.height),
    }
}

#[allow(clippy::cast_possible_truncation)]
fn to_slint(rect: WindowRect) -> CallWindowRect {
    CallWindowRect {
        x: rect.x as f32,
        y: rect.y as f32,
        width: rect.width as f32,
        height: rect.height as f32,
    }
}

/// The app's window, in logical pixels: the window of the call stays whole in it.
fn area(window: &AppWindow) -> WindowViewport {
    let w = window.window();
    let size = w.size().to_logical(w.scale_factor());
    WindowViewport {
        width: f64::from(size.width),
        height: f64::from(size.height),
    }
}

/// The user moved or resized the window: it stays so, for this call and the next.
fn choose_window(window: &AppWindow, rect: WindowRect) {
    CALL.with_borrow_mut(|c| c.window_rect = Some(rect));
    window.set_call_window(to_slint(rect));
    // No folder or no disk: the place lasts as long as the app (web). The numbers are
    // finite: fitted to the app's window
    let text = format!(
        r#"{{"x":{},"y":{},"width":{},"height":{}}}"#,
        rect.x, rect.y, rect.width, rect.height
    );
    let file = window_file();
    if let Some(folder) = file.parent() {
        let _ = std::fs::create_dir_all(folder);
    }
    let _ = std::fs::write(file, text);
}

/// What the screen shows now: leaving the room of the call detaches it, coming back does
/// not put it back (web `notePlacement`, `placeCall`).
pub fn note_placement(window: &AppWindow) {
    let shown = STATE.with_borrow(|s| s.current_room.clone());
    let shown = (!shown.is_empty()).then_some(shown);
    let floating = CALL.with_borrow_mut(|c| {
        c.placement = cw::place_call(&c.placement, shown.as_deref(), c.room.as_deref());
        cw::is_call_floating(&c.placement, c.room.as_deref())
    });
    set_floating(window, floating);
}

fn set_floating(window: &AppWindow, floating: bool) {
    if floating {
        // Where it was left, or its first place (web `readWindow`)
        let rect = CALL
            .with_borrow(|c| c.window_rect)
            .unwrap_or_else(|| cw::default_call_window(area(window)));
        window.set_call_window(to_slint(cw::fit_window(rect, area(window))));
    }
    window.set_call_floating(floating);
}

/// « Detach the call »: it floats while its room stays shown (web `detachCall`).
pub fn detach(window: &AppWindow) {
    CALL.with_borrow_mut(|c| c.placement.detached_room_id = c.room.clone());
    set_floating(window, true);
}

/// « Pop out of the app »: the call in a window of its own above the others, at the size
/// of its window in the app (web `popOutCall`, D618).
#[allow(clippy::cast_possible_truncation)]
pub fn pop_out(window: &AppWindow) {
    if POP_OUT.with_borrow(Option::is_some) {
        return;
    }
    let Ok(pop) = CallPopOut::new() else {
        return;
    };
    let rect = CALL
        .with_borrow(|c| c.window_rect)
        .unwrap_or_else(|| cw::default_call_window(area(window)));
    pop.window().set_size(slint::LogicalSize::new(
        rect.width as f32,
        rect.height as f32,
    ));
    pop.set_tiles(window.get_call_tiles());
    pop.on_tile_width(tile_width);
    let weak = window.as_weak();
    pop.on_toggle_microphone(move || {
        if let Some(w) = weak.upgrade() {
            toggle_microphone(&w);
        }
    });
    let weak = window.as_weak();
    pop.on_toggle_camera(move || {
        if let Some(w) = weak.upgrade() {
            toggle_camera(&w);
        }
    });
    let weak = window.as_weak();
    pop.on_toggle_screen(move || {
        if let Some(w) = weak.upgrade() {
            toggle_screen(&w);
        }
    });
    let weak = window.as_weak();
    pop.on_open_devices(move |which| {
        if let Some(w) = weak.upgrade() {
            open_devices(&w, &which);
        }
    });
    let weak = window.as_weak();
    pop.on_pick_device(move |kind, id| {
        if let Some(w) = weak.upgrade() {
            pick_device(&w, &kind, &id);
        }
    });
    let weak = window.as_weak();
    pop.on_leave(move || {
        if let Some(w) = weak.upgrade() {
            leave(&w);
        }
    });
    // Closed by the user: the call comes back, once out of the window's own handler
    let weak = window.as_weak();
    pop.window().on_close_requested(move || {
        let weak = weak.clone();
        slint::Timer::single_shot(std::time::Duration::ZERO, move || {
            if let Some(w) = weak.upgrade() {
                bring_back(&w);
            }
        });
        slint::CloseRequestResponse::HideWindow
    });
    if pop.show().is_err() {
        return;
    }
    POP_OUT.with_borrow_mut(|p| *p = Some(pop));
    window.set_call_popped_out(true);
    mirror(window);
}

/// The call back into the app: its window closed, the call ended, or « Expand ».
pub fn bring_back(window: &AppWindow) {
    if let Some(pop) = POP_OUT.with_borrow_mut(Option::take) {
        let _ = pop.hide();
    }
    window.set_call_popped_out(false);
}

/// The window out of the app shows what the app knows of the call; the tiles are shared.
pub fn mirror(window: &AppWindow) {
    POP_OUT.with_borrow(|pop| {
        let Some(pop) = pop else { return };
        pop.set_room_name(window.get_call_room_name());
        pop.set_status(window.get_call_status());
        pop.set_encrypted(window.get_call_encrypted());
        pop.set_microphone(window.get_call_microphone());
        pop.set_camera(window.get_call_camera());
        pop.set_sharing(window.get_call_sharing());
        pop.set_device_menu(window.get_call_device_menu());
        pop.set_device_sections(window.get_call_device_sections());
    });
}

/// « Full screen », « Expand »: the call back in its room, shown (web `expandCall`).
pub fn expand(window: &AppWindow) {
    bring_back(window);
    let room = CALL.with_borrow_mut(|c| {
        c.placement.detached_room_id = None;
        c.room.clone()
    });
    if let Some(room) = room {
        window.invoke_open_room(room.into());
    }
    note_placement(window);
}

/// The people of the call of a room changed.
pub fn members(window: &AppWindow, room_id: String, user_ids: &[String]) {
    CALL.with_borrow_mut(|c| {
        if user_ids.is_empty() {
            c.people.remove(&room_id);
        } else {
            let people = twake_domain::call::count_call_people(user_ids.iter().map(String::as_str));
            c.people.insert(room_id, people);
        }
    });
    refresh_header(window);
}

/// The header of the room shown: whether a call goes on in it.
pub fn refresh_header(window: &AppWindow) {
    let room = STATE.with_borrow(|s| s.current_room.clone());
    let people = CALL.with_borrow(|c| c.people.get(&room).copied().unwrap_or(0));
    window.set_call_people(i32::try_from(people).unwrap_or(i32::MAX));
}

fn room_name(room_id: &str) -> String {
    STATE.with_borrow(|s| {
        s.rooms
            .iter()
            .find(|r| r.id == room_id)
            .map(|r| r.name.clone())
            .unwrap_or_default()
    })
}

/// Joins (or starts) the call of the room shown.
pub fn start(window: &AppWindow, video: bool) {
    let room_id = STATE.with_borrow(|s| s.current_room.clone());
    if room_id.is_empty() {
        return;
    }
    CALL.with_borrow_mut(|c| {
        c.room = Some(room_id.clone());
        c.microphone = true;
        c.camera = video;
        if let Some(tiles) = &c.tiles {
            tiles.set_vec(Vec::new());
        }
    });
    window.set_call_room_id(room_id.clone().into());
    window.set_call_room_name(room_name(&room_id).into());
    window.set_call_status("Connecting…".into());
    window.set_call_encrypted(false);
    window.set_call_microphone(true);
    window.set_call_camera(video);
    window.set_in_call(true);
    note_placement(window);
    // The chat of a call started in a channel goes to a thread of its own (D105)
    let is_direct = STATE.with_borrow(|s| {
        s.rooms
            .iter()
            .any(|r| r.id == room_id && r.kind == twake_domain::RoomKind::Direct)
    });
    let thread_root = (!is_direct).then(|| crate::view::meeting_started(now_ms()));
    send(Command::JoinCall {
        room_id,
        video,
        thread_root,
    });
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX))
}

/// The words of a call error (web `call.error.*`).
fn error_text(code: &str) -> &'static str {
    match code {
        "no_focus" => "This server offers no calls.",
        "no_permission" => "You may not join a call in this conversation.",
        "room_not_found" => "This conversation is not loaded yet.",
        "token_failed" => "The call server refused the call. Try again in a moment.",
        "token_refused" => "The call server refused the call (error from the server).",
        "token_bad_response" => "The call server sent an answer that could not be read.",
        "media_failed" => "The microphone or the camera could not be opened.",
        _ => "The call server could not be reached.",
    }
}

pub fn status(window: &AppWindow, room_id: &str, status: CallStatus) {
    if CALL.with_borrow(|c| c.room.as_deref() != Some(room_id)) {
        return;
    }
    match status {
        CallStatus::Connecting => window.set_call_status("Connecting…".into()),
        CallStatus::Joined { encrypted } => {
            window.set_call_status("".into());
            window.set_call_encrypted(encrypted);
            refresh_thread(window);
        }
        // The stage stays, with why, until the user leaves it
        CallStatus::Failed(code) => {
            window.set_call_status(error_text(&code).into());
            if let Some(tiles) = CALL.with_borrow(|c| c.tiles.clone()) {
                tiles.set_vec(Vec::new());
            }
        }
        CallStatus::Declined => window.set_call_status("Call declined".into()),
        CallStatus::Left => clear(window),
    }
}

fn clear(window: &AppWindow) {
    CALL.with_borrow_mut(|c| {
        c.room = None;
        c.sharing = false;
        if let Some(tiles) = &c.tiles {
            tiles.set_vec(Vec::new());
        }
    });
    window.set_call_sharing(false);
    refresh_thread(window);
    window.set_in_call(false);
    window.set_call_room_id("".into());
    window.set_call_status("".into());
    bring_back(window);
    note_placement(window);
}

pub fn leave(window: &AppWindow) {
    send(Command::LeaveCall);
    window.set_call_device_menu("".into());
    clear(window);
}

/// Opens a device menu, read at each opening: one may have been plugged in since (web
/// `loadDeviceMenu`). "audio": microphones and speakers; "video": cameras; "": closes.
pub fn open_devices(window: &AppWindow, which: &str) {
    window.set_call_device_menu(which.into());
    window.set_call_device_sections(slint::ModelRc::default());
    mirror(window);
    let kinds = match which {
        "audio" => vec![DeviceKind::Microphone, DeviceKind::Speaker],
        "video" => vec![DeviceKind::Camera],
        _ => return,
    };
    send(Command::CallDevices(kinds));
}

/// The devices of the menu open, read by the engine.
pub fn devices(window: &AppWindow, lists: Vec<CallDeviceList>) {
    if window.get_call_device_menu().is_empty() {
        return;
    }
    let sections: Vec<DeviceSection> = lists.into_iter().filter_map(device_section).collect();
    window.set_call_device_sections(Rc::new(VecModel::from(sections)).into());
}

fn device_section(list: CallDeviceList) -> Option<DeviceSection> {
    let (kind, title) = match list.kind {
        DeviceKind::Microphone => ("microphone", "Microphone"),
        DeviceKind::Speaker => ("speaker", "Speaker"),
        DeviceKind::Camera => ("camera", "Camera"),
    };
    // A system that cannot choose its output says nothing about it (web)
    if list.devices.is_empty() && list.kind == DeviceKind::Speaker {
        return None;
    }
    let rows: Vec<DeviceRow> = list
        .devices
        .iter()
        .enumerate()
        .map(|(rank, (id, name))| DeviceRow {
            checked: list.active_id.as_deref() == Some(id.as_str()),
            // Without the permission a device has no name (web)
            label: if name.is_empty() {
                format!("{title} {}", rank + 1).into()
            } else {
                name.as_str().into()
            },
            id: id.as_str().into(),
        })
        .collect();
    Some(DeviceSection {
        kind: kind.into(),
        title: title.into(),
        rows: Rc::new(VecModel::from(rows)).into(),
    })
}

/// Uses a device of a menu from now on, and closes the menu.
pub fn pick_device(window: &AppWindow, kind: &str, id: &str) {
    window.set_call_device_menu("".into());
    mirror(window);
    let kind = match kind {
        "microphone" => DeviceKind::Microphone,
        "speaker" => DeviceKind::Speaker,
        "camera" => DeviceKind::Camera,
        _ => return,
    };
    send(Command::CallSwitchDevice(kind, id.to_string()));
}

pub fn toggle_microphone(window: &AppWindow) {
    let on = CALL.with_borrow_mut(|c| {
        c.microphone = !c.microphone;
        c.microphone
    });
    window.set_call_microphone(on);
    send(Command::CallMicrophone(on));
    mirror(window);
}

pub fn toggle_camera(window: &AppWindow) {
    let on = CALL.with_borrow_mut(|c| {
        c.camera = !c.camera;
        c.camera
    });
    window.set_call_camera(on);
    send(Command::CallCamera(on));
    mirror(window);
}

pub fn toggle_screen(window: &AppWindow) {
    let on = !CALL.with_borrow(|c| c.sharing);
    send(Command::CallScreen(on));
    // Shown as sharing once the screen tile is in the call (a cancelled pick says nothing)
    if !on {
        window.set_call_sharing(false);
    }
}

/// The thread of the call of the room shown, if its chat goes there (web `useCallThread`):
/// it opens beside the call in place of the channel, unless the user chose the channel.
pub fn refresh_thread(window: &AppWindow) {
    let (room, root, kind) = STATE.with_borrow(|s| {
        let kind = s
            .rooms
            .iter()
            .find(|r| r.id == s.current_room)
            .map(|r| r.kind);
        (s.current_room.clone(), s.thread_root.clone(), kind)
    });
    let in_call_here = CALL.with_borrow(|c| c.room.as_deref() == Some(room.as_str()));
    let is_direct = kind == Some(twake_domain::RoomKind::Direct);
    // ASSUMPTION: no voice rooms in this app yet, and the user may post (the root is only
    // sent by someone who may)
    let thread = if twake_domain::call::splits_call_chat(in_call_here, is_direct, false, true) {
        STATE.with_borrow(|s| {
            twake_domain::call::find_call_thread(
                s.entries.iter().filter_map(|e| match e {
                    TimelineEntry::Message(m) => Some(m),
                    _ => None,
                }),
                now_ms(),
            )
        })
    } else {
        None
    };
    let (was, shown, open) = CALL.with_borrow_mut(|c| {
        let was = c.call_thread.clone();
        c.call_thread = thread.clone();
        let shown = thread.is_some() && c.channel_shown_for != thread;
        // Opened once for each thread: a thread the user opened since stays
        let open = shown && c.thread_opened_for != thread && root != thread;
        if open {
            c.thread_opened_for = thread.clone();
        }
        (was, shown, open)
    });
    window.set_call_thread(thread.is_some());
    window.set_call_thread_shown(shown);
    if let Some(id) = thread.as_ref().filter(|_| open) {
        window.invoke_open_thread(id.as_str().into());
        window.set_side_title("Meeting thread".into());
    }
    // The call is over (or the thread changed): its thread goes away with it
    if was.is_some() && was != thread && root == was {
        window.invoke_close_thread();
    }
}

/// « Show the channel » / « Back to the meeting thread ».
pub fn toggle_thread(window: &AppWindow) {
    let (thread, shown) = CALL.with_borrow_mut(|c| {
        let shown = c.call_thread.is_some() && c.channel_shown_for != c.call_thread;
        c.channel_shown_for = if shown { c.call_thread.clone() } else { None };
        c.thread_opened_for = None;
        (c.call_thread.clone(), shown)
    });
    if shown && STATE.with_borrow(|s| s.thread_root == thread) {
        window.invoke_close_thread();
    }
    refresh_thread(window);
}

/// The calls that ring changed (web `useRingingRoom`).
pub fn ringing(window: &AppWindow, ringing: Vec<IncomingCall>) {
    CALL.with_borrow_mut(|c| {
        // A call that rings no more is forgotten: its next ring rings again
        c.attended
            .retain(|room| ringing.iter().any(|r| &r.room_id == room));
        c.ringing = ringing;
    });
    show_ring(window);
}

/// The banner shows the first call that rings, unless the user dealt with it here or is
/// in it (web `pickRing`, `stillRings`).
fn show_ring(window: &AppWindow) {
    let ring = CALL.with_borrow(|c| {
        c.ringing
            .iter()
            .find(|r| !c.attended.contains(&r.room_id) && c.room.as_ref() != Some(&r.room_id))
            .cloned()
    });
    let text = ring.as_ref().map_or(String::new(), |r| {
        let room = room_name(&r.room_id);
        if r.is_direct {
            format!("{room} is calling you")
        } else {
            format!("{} is calling you in {room}", name_of(&r.caller_id))
        }
    });
    CALL.with_borrow_mut(|c| c.shown_ring = ring.as_ref().map(|r| r.room_id.clone()));
    window.set_ring_video(ring.is_some_and(|r| r.is_video));
    window.set_ring_text(text.into());
}

/// « Answer »: the room of the call opens and the user joins it (microphone only, the
/// web's default choice).
pub fn answer(window: &AppWindow) {
    let Some(room) = CALL.with_borrow_mut(|c| {
        let room = c.shown_ring.take()?;
        c.attended.insert(room.clone());
        Some(room)
    }) else {
        return;
    };
    window.invoke_open_room(room.into());
    start(window, false);
    show_ring(window);
}

/// « Decline »: whoever rang is told (MSC4310).
pub fn decline(window: &AppWindow) {
    if let Some(room_id) = CALL.with_borrow_mut(|c| {
        let room = c.shown_ring.take()?;
        c.attended.insert(room.clone());
        Some(room)
    }) {
        send(Command::DeclineCall { room_id });
    }
    show_ring(window);
}

/// The display name of a user, as the messages of the open room show it.
fn name_of(user_id: &str) -> String {
    STATE
        .with_borrow(|s| {
            s.entries.iter().rev().find_map(|e| match e {
                TimelineEntry::Message(m) if m.sender_id == user_id => Some(m.sender_name.clone()),
                _ => None,
            })
        })
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| {
            user_id
                .trim_start_matches('@')
                .split(':')
                .next()
                .unwrap_or(user_id)
                .to_string()
        })
}

/// A shared screen takes the stage, the people a row under it: the first screen comes
/// first, the others keep their order (web `CallStage`).
fn stage_first(mut tiles: Vec<CallTile>) -> Vec<CallTile> {
    if let Some(i) = tiles.iter().position(|t| t.is_screen) {
        let screen = tiles.remove(i);
        tiles.insert(0, screen);
    }
    tiles
}

/// Who is in the call changed: the rows keep the last picture of each person.
pub fn tiles(window: &AppWindow, tiles: Vec<CallTile>) {
    let Some(model) = CALL.with_borrow(|c| c.tiles.clone()) else {
        return;
    };
    let old: HashMap<String, (slint::Image, bool)> = model
        .iter()
        .map(|r| (r.identity.to_string(), (r.frame.clone(), r.ready)))
        .collect();
    let sharing = tiles.iter().any(|t| t.is_local && t.is_screen);
    let rows: Vec<CallTileRow> = stage_first(tiles)
        .into_iter()
        .map(|t| {
            // The user's tile says « You », with their own initials (web `CallView`)
            let own_name = name_of(&t.user_id);
            let name = if t.is_screen {
                format!("Screen of {own_name}")
            } else if t.is_local {
                "You".to_string()
            } else {
                own_name.clone()
            };
            let (frame, ready) = old.get(&t.identity).cloned().unwrap_or_default();
            CallTileRow {
                identity: t.identity.into(),
                screen: t.is_screen,
                initials: initials(&own_name),
                tint: tint(&t.user_id),
                name: name.into(),
                local: t.is_local,
                has_video: t.has_video,
                muted: t.is_muted && !t.is_screen,
                speaking: t.is_speaking,
                frame,
                ready: ready && t.has_video,
            }
        })
        .collect();
    model.set_vec(rows);
    CALL.with_borrow_mut(|c| c.sharing = sharing);
    window.set_call_sharing(sharing);
}

/// A new picture of someone's video.
pub fn frame(identity: &str, picture: &Picture) {
    let Some(model) = CALL.with_borrow(|c| c.tiles.clone()) else {
        return;
    };
    let Some(index) = model.iter().position(|r| r.identity == identity) else {
        return;
    };
    let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
        &picture.rgba,
        picture.width,
        picture.height,
    );
    if let Some(mut row) = model.row_data(index) {
        row.frame = slint::Image::from_rgba8(buffer);
        row.ready = row.has_video;
        model.set_row_data(index, row);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fits_the_tiles_as_the_web() {
        // One tile: as wide as the box allows at 16:9
        assert_eq!(tile_width(1, 1600.0, 900.0), 1600.0);
        assert_eq!(tile_width(1, 1600.0, 450.0), 800.0);
        // Four in a wide box: two columns beat four or one
        assert_eq!(tile_width(4, 1000.0, 600.0), 496.0);
        assert_eq!(tile_width(0, 100.0, 100.0), 100.0);
    }

    fn tile(identity: &str, is_screen: bool) -> CallTile {
        CallTile {
            identity: identity.into(),
            user_id: String::new(),
            is_local: false,
            is_screen,
            has_video: is_screen,
            is_muted: false,
            is_speaking: false,
        }
    }

    fn order(tiles: Vec<CallTile>) -> Vec<String> {
        stage_first(tiles).into_iter().map(|t| t.identity).collect()
    }

    #[test]
    fn puts_the_first_screen_on_the_stage() {
        // No screen: the order stays
        assert_eq!(order(vec![tile("a", false), tile("b", false)]), ["a", "b"]);
        // The screen first, the people after it in their order
        assert_eq!(
            order(vec![
                tile("a", false),
                tile("b", false),
                tile("b:screen", true)
            ]),
            ["b:screen", "a", "b"]
        );
        // Two screens: only the first takes the stage
        assert_eq!(
            order(vec![
                tile("a", false),
                tile("a:screen", true),
                tile("b:screen", true)
            ]),
            ["a:screen", "a", "b:screen"]
        );
        assert_eq!(order(vec![]), Vec::<String>::new());
    }
}
