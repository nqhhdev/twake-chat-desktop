//! Twake Chat desktop: the Slint window, fed by the `twake-core` engine.

// No console window on Windows release builds.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod activity;
mod audio;
mod call;
mod composer;
mod deleted;
mod dev;
mod directory;
mod formats;
mod forward;
mod gifs;
mod icons;
mod pdf;
mod previews;
mod search;
mod selection;
mod settings;
mod shared;
mod staged;
mod switcher;
mod view;
mod viewer;
mod voice;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use slint::{ComponentHandle, Model, ModelRc, VecModel};
use twake_core::{Command, Engine, Event, RoomAction, SessionState};
use twake_domain::{RoomSummary, TimelineEntry};

slint::include_modules!();

/// What the window shows, kept on the UI thread.
struct UiState {
    rooms: Vec<RoomSummary>,
    filter: String,
    current_room: String,
    first_row_id: String,
    last_row_id: String,
    scroll_bump: i32,
    rooms_model: Rc<VecModel<RoomRow>>,
    rows_model: Rc<VecModel<Row>>,
    /// The open room's last timeline, drawn again when its images arrive.
    entries: Vec<TimelineEntry>,
    /// Decoded images by `key@WxH`, and the ones asked for or that failed.
    pictures: HashMap<String, slint::Image>,
    asked: HashSet<String>,
    /// Link previews by address (None: nothing to show), and the addresses asked for.
    link_previews: HashMap<String, Option<twake_domain::link_preview::LinkPreview>>,
    asked_previews: HashSet<String>,
    /// What the next send of the room's composer and of the thread's composer does.
    compose: Compose,
    thread_compose: Compose,
    /// The open thread: its root, rows and last entries.
    thread_root: Option<String>,
    thread_model: Rc<VecModel<Row>>,
    thread_entries: Vec<TimelineEntry>,
    thread_last_id: String,
    thread_scroll: i32,
    /// The details of the open room's info pane, and its members.
    details: Option<twake_domain::RoomDetails>,
    members_model: Rc<VecModel<MemberRow>>,
    /// The command a confirmation dialog runs on « yes ».
    pending: Option<Command>,
    /// The commands sent with `pending` once confirmed (several messages deleted at once).
    pending_more: Vec<Command>,
    /// The chats deleted for the user alone (D186), and the one « Undo » brings back.
    deleted: twake_domain::deleted_chats::DeletedChats,
    undo_room: Option<String>,
    /// Spaces, the one the room list shows ("" for all) and its filter chip.
    spaces: Vec<twake_domain::SpaceSummary>,
    spaces_model: Rc<VecModel<SpaceItem>>,
    current_space: String,
    room_filter: String,
}

/// The mode of a composer: a new message, a reply, or an edit.
#[derive(Clone, Debug, Default, PartialEq)]
enum Compose {
    #[default]
    New,
    Reply(String),
    Edit(String),
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            rooms: Vec::new(),
            filter: String::new(),
            current_room: String::new(),
            first_row_id: String::new(),
            last_row_id: String::new(),
            scroll_bump: 0,
            rooms_model: Rc::new(VecModel::default()),
            rows_model: Rc::new(VecModel::default()),
            entries: Vec::new(),
            pictures: HashMap::new(),
            asked: HashSet::new(),
            link_previews: HashMap::new(),
            asked_previews: HashSet::new(),
            compose: Compose::New,
            thread_compose: Compose::New,
            thread_root: None,
            thread_model: Rc::new(VecModel::default()),
            thread_entries: Vec::new(),
            thread_last_id: String::new(),
            thread_scroll: 0,
            details: None,
            members_model: Rc::new(VecModel::default()),
            pending: None,
            pending_more: Vec::new(),
            deleted: Default::default(),
            undo_room: None,
            spaces: Vec::new(),
            spaces_model: Rc::new(VecModel::default()),
            current_space: String::new(),
            room_filter: "all".into(),
        }
    }
}

/// The message `id` of the room's timeline (or of the open thread).
fn find_message(id: &str, thread: bool) -> Option<twake_domain::Message> {
    STATE.with_borrow(|s| {
        let entries = if thread {
            &s.thread_entries
        } else {
            &s.entries
        };
        entries.iter().find_map(|e| match e {
            TimelineEntry::Message(m) if m.id == id => Some(m.clone()),
            _ => None,
        })
    })
}

/// Puts a composer in `mode` and shows what it does.
fn set_compose(window: &AppWindow, thread: bool, mode: Compose) {
    let hint = match &mode {
        Compose::New => String::new(),
        Compose::Reply(id) => match find_message(id, thread) {
            Some(m) => {
                let text = if m.text.is_empty() {
                    m.attachment
                        .as_ref()
                        .map_or(String::new(), |a| a.name.clone())
                } else {
                    twake_domain::room_preview::to_preview_text(&m.text).replace('\n', " ")
                };
                format!("Replying to {}: {text}", m.sender_name)
            }
            None => "Replying".into(),
        },
        Compose::Edit(_) => "Editing your message".into(),
    };
    if let Compose::Edit(id) = &mode {
        let text = find_message(id, thread).map(|m| m.text).unwrap_or_default();
        if thread {
            window.set_thread_compose_text(text.into());
        } else {
            window.set_compose_text(text.into());
        }
    }
    if thread {
        window.set_thread_compose_hint(hint.into());
    } else {
        window.set_compose_hint(hint.into());
    }
    STATE.with_borrow_mut(|s| {
        if thread {
            s.thread_compose = mode;
        } else {
            s.compose = mode;
        }
    });
}

/// Sends what a composer holds, as a message, a reply or an edit, then empties it.
fn submit(window: &AppWindow, text: String, thread: bool) {
    let mode = STATE.with_borrow(|s| {
        if thread {
            s.thread_compose.clone()
        } else {
            s.compose.clone()
        }
    });
    send(Command::Typing { typing: false });
    // Staged files leave first, the text as their caption
    let files = staged::take(window, thread);
    if !files.is_empty() {
        let reply_to = match &mode {
            Compose::Reply(id) => Some(id.clone()),
            _ => None,
        };
        let caption = twake_domain::js::trim(&text).to_string();
        for (i, path) in files.into_iter().enumerate() {
            send(Command::SendFile {
                path,
                thread,
                reply_to: reply_to.clone(),
                caption: (i == 0 && !caption.is_empty()).then(|| caption.clone()),
            });
        }
        if thread {
            window.set_thread_compose_text("".into());
        } else {
            window.set_compose_text("".into());
            window.set_at_bottom(true);
        }
        set_compose(window, thread, Compose::New);
        return;
    }
    let is_new = mode == Compose::New;
    let reply_to = match mode {
        Compose::Edit(id) => {
            send(Command::Edit {
                id,
                text: text.clone(),
                thread,
            });
            None
        }
        Compose::Reply(id) => Some(id),
        Compose::New => None,
    };
    if reply_to.is_some() || is_new {
        let out = composer::outgoing(thread, &text);
        if let Some(name) = &out.unknown_command {
            window.set_notice(
                format!("Unknown command /{name}. Start with // to send it as text").into(),
            );
            return;
        }
        if out.error.is_some() {
            return;
        }
        if twake_domain::composer::find_local_command(&out.text).is_some() {
            send(Command::DiscardSession);
        } else {
            send(Command::Send {
                text: out.text,
                thread,
                reply_to,
                mentions: out.mentions,
                emote: out.is_emote,
                command: out.command,
            });
        }
    }
    if thread {
        window.set_thread_compose_text("".into());
    } else {
        window.set_compose_text("".into());
        window.set_at_bottom(true);
    }
    set_compose(window, thread, Compose::New);
}

fn close_thread(window: &AppWindow) {
    let (room, root) = STATE.with_borrow(|s| (s.current_room.clone(), s.thread_root.clone()));
    if let Some(root) = root {
        composer::leave_thread(window, &room, &root);
    }
    STATE.with_borrow_mut(|s| {
        s.thread_root = None;
        s.thread_model.clear();
        s.thread_entries.clear();
        s.thread_last_id.clear();
    });
    window.set_thread_open(false);
    window.set_side_pinned(false);
    set_compose(window, true, Compose::New);
}

thread_local! {
    static STATE: RefCell<UiState> = RefCell::new(UiState::default());
    static DEV: RefCell<dev::Dev> = RefCell::new(dev::Dev::new());
    static ENGINE: RefCell<Option<Rc<Engine>>> = const { RefCell::new(None) };
}

fn send(command: Command) {
    ENGINE.with_borrow(|e| {
        if let Some(engine) = e {
            engine.send(command);
        }
    });
}

fn main() -> Result<(), slint::PlatformError> {
    DEV.with_borrow(|d| d.stat("start"));
    let window = AppWindow::new()?;
    icons::load(&window);
    let weak = window.as_weak();

    STATE.with_borrow(|s| {
        window.set_rooms(ModelRc::from(s.rooms_model.clone()));
        window.set_rows(ModelRc::from(s.rows_model.clone()));
        window.set_thread_rows(ModelRc::from(s.thread_model.clone()));
    });

    let engine = Rc::new(Engine::start(move |event| {
        let weak = weak.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(window) = weak.upgrade() {
                apply(&window, event);
                call::mirror(&window);
            }
        });
    }));
    ENGINE.with_borrow_mut(|e| *e = Some(engine));

    window.on_sign_in(|homeserver, username, password| {
        send(Command::SignInPassword {
            homeserver: homeserver.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        });
    });
    window.on_sign_in_sso(|homeserver| {
        send(Command::SignInSso {
            homeserver: homeserver.to_string(),
        })
    });
    {
        let weak = window.as_weak();
        window.on_open_room(move |room_id| {
            if std::env::var("TWAKE_DEV_STATS").is_ok() {
                println!("[dev] open-room {room_id}");
            }
            if let Some(w) = weak.upgrade() {
                let old = STATE.with_borrow(|s| s.current_room.clone());
                composer::leave_room(&w, &old);
            }
            STATE.with_borrow_mut(|s| {
                s.current_room = room_id.to_string();
                s.first_row_id.clear();
                s.last_row_id.clear();
                s.rows_model.clear();
                s.entries.clear();
                s.details = None;
            });
            if let Some(w) = weak.upgrade() {
                selection::reset(&w);
                if !w.get_notice_action().is_empty() {
                    STATE.with_borrow_mut(|s| s.undo_room = None);
                    w.set_notice("".into());
                    w.set_notice_action("".into());
                }
                w.set_current_room_id(room_id.clone());
                w.set_history_complete(false);
                w.set_history_prepended(false);
                w.set_at_bottom(true);
                close_thread(&w);
                w.set_info_open(false);
                w.set_typing_text("".into());
                set_compose(&w, false, Compose::New);
                composer::enter_room(&w, &room_id);
                staged::clear(&w);
                audio::stop(&w);
                gifs::clear();
                call::refresh_header(&w);
                call::note_placement(&w);
                w.set_activity_open(false);
                search::reset(&w);
            }
            send(Command::OpenRoom {
                room_id: room_id.to_string(),
            });
        });
    }
    window.on_load_older(|| send(Command::LoadOlder));
    window.set_emojis(composer::emojis());
    staged::install_drop(&window);
    audio::install(&window);
    gifs::install(&window);
    call::install(&window);
    shared::install(&window);
    forward::install(&window);
    settings::install(&window);
    {
        let weak = window.as_weak();
        window.on_composer_unstage(move |thread, index| {
            if let Some(w) = weak.upgrade() {
                staged::unstage(&w, thread, index.max(0) as usize);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_composer_paste(move |thread| {
            if let Some(w) = weak.upgrade() {
                staged::paste(&w, thread);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_open_activity(move || {
            if let Some(w) = weak.upgrade() {
                activity::open(&w);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_activity_pick_filter(move |filter| {
            if let Some(w) = weak.upgrade() {
                activity::pick_filter(&w, &filter);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_activity_search(move |_| {
            if let Some(w) = weak.upgrade() {
                activity::refresh(&w);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_activity_choose(move |index| {
            if let Some(w) = weak.upgrade() {
                activity::choose(&w, index.max(0) as usize);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_open_search(move || {
            if let Some(w) = weak.upgrade() {
                search::open(&w);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_search(move |query, files_only| {
            if let Some(w) = weak.upgrade() {
                search::search(&w, &query, files_only, false);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_search_older(move || {
            if let Some(w) = weak.upgrade() {
                let (query, files) = (w.get_search_query(), w.get_search_files_only());
                search::search(&w, &query, files, true);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_open_hit(move |id| {
            if let Some(w) = weak.upgrade() {
                search::open_hit(&w, &id);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_open_switcher(move || {
            if let Some(w) = weak.upgrade() {
                switcher::open(&w);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_switcher_search(move |query| {
            if let Some(w) = weak.upgrade() {
                switcher::search(&w, &query);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_switcher_choose(move |index| {
            if let Some(w) = weak.upgrade() {
                switcher::choose(&w, index.max(0) as usize);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_jump_to_date(move |year, month, day| {
            let Some(w) = weak.upgrade() else { return };
            let start = chrono::NaiveDate::from_ymd_opt(year, month as u32, day as u32)
                .and_then(|d| d.and_hms_opt(0, 0, 0))
                .and_then(|d| d.and_local_timezone(chrono::Local).earliest());
            let Some(start) = start else { return };
            w.set_jump_title(start.format("From %e %B %Y").to_string().into());
            send(Command::JumpToDate {
                ts: start.timestamp_millis(),
            });
        });
    }
    {
        let weak = window.as_weak();
        window.on_play_audio(move |key, mime, ms| {
            if let Some(w) = weak.upgrade() {
                audio::toggle(&w, &key, &mime, ms);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_seek_audio(move |key, at| {
            if let Some(w) = weak.upgrade() {
                audio::seek(&w, &key, at);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_record_voice(move |thread| {
            if let Some(w) = weak.upgrade() {
                // A voice message answers the message the composer replies to
                let reply_to = STATE.with_borrow(|s| {
                    match if thread {
                        &s.thread_compose
                    } else {
                        &s.compose
                    } {
                        Compose::Reply(id) => Some(id.clone()),
                        _ => None,
                    }
                });
                voice::start(&w, thread, reply_to);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_stop_voice(move |sent| {
            if let Some(w) = weak.upgrade() {
                voice::finish(&w, sent);
            }
        });
    }
    window.on_text_format(|text, name, start, end| formats::format(&text, &name, start, end));
    window.on_paste_link(|text, start, end| formats::paste(&text, start, end));
    window.on_link_target(|text, start, end| formats::target(&text, start, end));
    window.on_link_apply(|text, from, to, label, url| {
        formats::apply_link(&text, from, to, &label, &url)
    });
    {
        let weak = window.as_weak();
        window.on_open_date_picker(move || {
            if let Some(w) = weak.upgrade() {
                use chrono::Datelike;
                let now = chrono::Local::now();
                w.set_today(Date {
                    year: now.year(),
                    month: now.month() as i32,
                    day: now.day() as i32,
                });
                w.invoke_show_date_picker();
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_jump_to_beginning(move || {
            if let Some(w) = weak.upgrade() {
                w.set_jump_title("Beginning of the conversation".into());
                send(Command::JumpToDate { ts: 0 });
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_show_about(move || {
            if let Some(w) = weak.upgrade() {
                w.set_notice(
                    format!(
                        "Twake Chat {} (desktop, AGPL-3.0)",
                        env!("CARGO_PKG_VERSION")
                    )
                    .into(),
                );
            }
        });
    }
    window.on_quit_app(|| {
        let _ = slint::quit_event_loop();
    });
    {
        let weak = window.as_weak();
        window.on_composer_edited(move |thread, text, caret| {
            if let Some(w) = weak.upgrade() {
                composer::edited(&w, thread, &text, caret.max(0) as usize);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_suggestion_pick(move |thread, index| {
            if let Some(w) = weak.upgrade() {
                composer::pick(&w, thread, index.max(0) as usize);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_suggestion_dismiss(move |thread| {
            if let Some(w) = weak.upgrade() {
                composer::dismiss(&w, thread);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_composer_insert(move |thread, text, caret| {
            if let Some(w) = weak.upgrade() {
                composer::insert(&w, thread, &text, caret.max(0) as usize);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_send(move |text| {
            if let Some(w) = weak.upgrade() {
                submit(&w, text.to_string(), false);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_send_in_thread(move |text| {
            if let Some(w) = weak.upgrade() {
                submit(&w, text.to_string(), true);
            }
        });
    }
    window.on_react(|id, key, thread| {
        send(Command::React {
            id: id.to_string(),
            key: key.to_string(),
            thread,
        });
    });
    {
        let weak = window.as_weak();
        window.on_reply(move |id, thread| {
            if let Some(w) = weak.upgrade() {
                set_compose(&w, thread, Compose::Reply(id.to_string()));
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_edit(move |id, thread| {
            if let Some(w) = weak.upgrade() {
                set_compose(&w, thread, Compose::Edit(id.to_string()));
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_delete(move |id, thread| {
            if let Some(w) = weak.upgrade() {
                ask(
                    &w,
                    "Delete this message for everyone?",
                    "Delete",
                    Command::Delete {
                        id: id.to_string(),
                        thread,
                        reason: None,
                    },
                );
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_attach(move |thread| {
            let Some(paths) = rfd::FileDialog::new().set_title("Send files").pick_files() else {
                return;
            };
            if let Some(w) = weak.upgrade() {
                staged::stage(&w, paths, thread);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_cancel_compose(move |thread| {
            if let Some(w) = weak.upgrade() {
                if STATE.with_borrow(|s| {
                    matches!(
                        if thread {
                            &s.thread_compose
                        } else {
                            &s.compose
                        },
                        Compose::Edit(_)
                    )
                }) {
                    if thread {
                        w.set_thread_compose_text("".into());
                    } else {
                        w.set_compose_text("".into());
                    }
                }
                set_compose(&w, thread, Compose::New);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_open_thread(move |root_id| {
            if let Some(w) = weak.upgrade() {
                close_thread(&w);
                w.set_side_pinned(false);
                w.set_info_open(false);
                STATE.with_borrow_mut(|s| s.thread_root = Some(root_id.to_string()));
                w.set_side_title("Thread".into());
                let room = STATE.with_borrow(|s| s.current_room.clone());
                composer::enter_thread(&w, &room, &root_id);
                w.set_thread_open(true);
            }
            send(Command::OpenThread {
                root_id: root_id.to_string(),
            });
        });
    }
    {
        let weak = window.as_weak();
        window.on_close_thread(move || {
            if let Some(w) = weak.upgrade() {
                close_thread(&w);
            }
            send(Command::CloseThread);
        });
    }
    {
        let weak = window.as_weak();
        window.on_recover(move |key| {
            send(Command::Recover {
                key: key.to_string(),
            });
            if let Some(w) = weak.upgrade() {
                w.set_recovery_key("".into());
            }
        });
    }
    window.on_sign_out(|| send(Command::SignOut));
    {
        let weak = window.as_weak();
        window.on_open_attachment(move |key, name| {
            // A PDF opens in the app's viewer when PDFium is there (web `isViewablePdf`)
            if pdf::shows(&key)
                && let Some(w) = weak.upgrade()
            {
                pdf::open(&w, &key, &name);
                return;
            }
            send(Command::OpenAttachment {
                key: key.to_string(),
                name: name.to_string(),
            });
        });
    }
    {
        let weak = window.as_weak();
        window.on_pdf_scrolled(move |top, height, width| {
            if let Some(w) = weak.upgrade() {
                pdf::scrolled(&w, top, height, width);
            }
        });
        let weak = window.as_weak();
        window.on_pdf_close(move || {
            if let Some(w) = weak.upgrade() {
                pdf::close(&w);
            }
        });
    }
    window.on_pdf_save(pdf::save);
    {
        let weak = window.as_weak();
        window.on_start_call(move |video| {
            if let Some(w) = weak.upgrade() {
                call::start(&w, video);
            }
        });
        let weak = window.as_weak();
        window.on_leave_call(move || {
            if let Some(w) = weak.upgrade() {
                call::leave(&w);
            }
        });
        let weak = window.as_weak();
        window.on_call_toggle_microphone(move || {
            if let Some(w) = weak.upgrade() {
                call::toggle_microphone(&w);
            }
        });
        let weak = window.as_weak();
        window.on_call_toggle_camera(move || {
            if let Some(w) = weak.upgrade() {
                call::toggle_camera(&w);
            }
        });
        let weak = window.as_weak();
        window.on_ring_answer(move || {
            if let Some(w) = weak.upgrade() {
                call::answer(&w);
            }
        });
        let weak = window.as_weak();
        window.on_ring_decline(move || {
            if let Some(w) = weak.upgrade() {
                call::decline(&w);
            }
        });
        let weak = window.as_weak();
        window.on_call_detach(move || {
            if let Some(w) = weak.upgrade() {
                call::detach(&w);
            }
        });
        let weak = window.as_weak();
        window.on_call_pop_out(move || {
            if let Some(w) = weak.upgrade() {
                call::pop_out(&w);
            }
        });
        let weak = window.as_weak();
        window.on_call_expand(move || {
            if let Some(w) = weak.upgrade() {
                call::expand(&w);
            }
        });
        let weak = window.as_weak();
        window.on_call_open_devices(move |which| {
            if let Some(w) = weak.upgrade() {
                call::open_devices(&w, &which);
            }
        });
        let weak = window.as_weak();
        window.on_call_pick_device(move |kind, id| {
            if let Some(w) = weak.upgrade() {
                call::pick_device(&w, &kind, &id);
            }
        });
        let weak = window.as_weak();
        window.on_call_toggle_screen(move || {
            if let Some(w) = weak.upgrade() {
                call::toggle_screen(&w);
            }
        });
        let weak = window.as_weak();
        window.on_call_thread_toggle(move || {
            if let Some(w) = weak.upgrade() {
                call::toggle_thread(&w);
            }
        });
    }
    window.on_pdf_external(pdf::open_external);
    {
        let weak = window.as_weak();
        window.on_view_image(move |key, name| {
            if let Some(w) = weak.upgrade() {
                viewer::open(&w, &key, &name);
            }
        });
    }
    window.on_viewer_save(viewer::save);
    window.on_viewer_external(viewer::open_external);
    {
        // The text of a message, or the link to it, to the clipboard (web `copyText`,
        // `copyLink`); the link is matrix.to's, the app having no address of its own (ADR-020)
        let weak = window.as_weak();
        window.on_copy_message(move |text| {
            if let Some(w) = weak.upgrade() {
                copy_to_clipboard(&w, &text, "Message copied");
            }
        });
        let weak = window.as_weak();
        window.on_forward_message(move |id| {
            if let Some(w) = weak.upgrade() {
                forward::open(&w, vec![id.to_string()]);
            }
        });
        let weak = window.as_weak();
        window.on_select_message(move |id| {
            if let Some(w) = weak.upgrade() {
                selection::select(&w, &id);
            }
        });
        let weak = window.as_weak();
        window.on_selection_close(move || {
            if let Some(w) = weak.upgrade() {
                selection::close(&w);
            }
        });
        let weak = window.as_weak();
        window.on_selection_forward(move || {
            if let Some(w) = weak.upgrade() {
                selection::forward(&w);
            }
        });
        let weak = window.as_weak();
        window.on_selection_download(move || {
            if let Some(w) = weak.upgrade() {
                selection::download(&w);
            }
        });
        let weak = window.as_weak();
        window.on_selection_copy(move || {
            if let Some(w) = weak.upgrade() {
                selection::copy(&w);
            }
        });
        let weak = window.as_weak();
        window.on_selection_pin(move || {
            if let Some(w) = weak.upgrade() {
                selection::pin(&w);
            }
        });
        let weak = window.as_weak();
        window.on_selection_delete(move || {
            if let Some(w) = weak.upgrade() {
                selection::delete(&w);
            }
        });
        let weak = window.as_weak();
        window.on_directory_open(move || {
            if let Some(w) = weak.upgrade() {
                directory::open(&w);
            }
        });
        let weak = window.as_weak();
        window.on_directory_search(move |_| {
            if let Some(w) = weak.upgrade() {
                directory::search(&w);
            }
        });
        let weak = window.as_weak();
        window.on_directory_choose(move |i| {
            if let Some(w) = weak.upgrade() {
                directory::choose(&w, usize::try_from(i).unwrap_or(0));
            }
        });
        let weak = window.as_weak();
        window.on_show_seen(move |id| {
            if let Some(w) = weak.upgrade() {
                show_seen(&w, &id);
            }
        });
        window.on_show_versions(|id| {
            send(Command::MessageVersions {
                event_id: id.to_string(),
            });
        });
        let weak = window.as_weak();
        window.on_forward_filter_edited(move |_| {
            if let Some(w) = weak.upgrade() {
                forward::filter_edited(&w);
            }
        });
        let weak = window.as_weak();
        window.on_forward_toggle(move |id| {
            if let Some(w) = weak.upgrade() {
                forward::toggle(&w, &id);
            }
        });
        let weak = window.as_weak();
        window.on_forward_send(move || {
            if let Some(w) = weak.upgrade() {
                forward::send_forward(&w);
            }
        });
        let weak = window.as_weak();
        window.on_forward_cancel(move || {
            if let Some(w) = weak.upgrade() {
                forward::cancel(&w);
            }
        });
        let weak = window.as_weak();
        window.on_copy_link(move |id| {
            if let Some(w) = weak.upgrade() {
                let link = twake_domain::permalink::message_permalink(&current_room(), &id);
                copy_to_clipboard(&w, &link, "Link copied");
            }
        });
    }
    window.on_save_attachment(|key, name| {
        send(Command::SaveAttachment {
            key: key.to_string(),
            name: name.to_string(),
        });
    });
    window.on_link_clicked(|url| {
        let lower = url.to_ascii_lowercase();
        if lower.starts_with("https://")
            || lower.starts_with("http://")
            || lower.starts_with("mailto:")
        {
            let _ = open::that(url.as_str());
        }
    });
    {
        let weak = window.as_weak();
        window.on_filter_changed(move |text| {
            STATE.with_borrow_mut(|s| s.filter = text.to_string());
            if let Some(w) = weak.upgrade() {
                show_rooms(&w);
            }
        });
    }

    wire_room_info(&window);
    wire_polls_and_pins(&window);
    STATE.with_borrow(|s| window.set_spaces(ModelRc::from(s.spaces_model.clone())));
    {
        let weak = window.as_weak();
        window.on_pick_space(move |id| {
            STATE.with_borrow_mut(|s| s.current_space = id.to_string());
            if let Some(w) = weak.upgrade() {
                w.set_current_space(id);
                w.set_activity_open(false);
                show_rooms(&w);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.on_pick_filter(move |chip| {
            STATE.with_borrow_mut(|s| s.room_filter = chip.to_string());
            if let Some(w) = weak.upgrade() {
                w.set_room_filter(chip);
                show_rooms(&w);
            }
        });
    }
    window.on_typing(|typing| send(Command::Typing { typing }));
    window.run()
}

fn apply(window: &AppWindow, event: Event) {
    match event {
        Event::Session(state) => match state {
            SessionState::Restoring => window.set_page(0),
            SessionState::SignedOut => {
                window.set_page(1);
                window.set_busy(false);
                window.set_current_room_id("".into());
                STATE.with_borrow_mut(|s| {
                    s.rooms.clear();
                    s.rooms_model.clear();
                    s.rows_model.clear();
                    s.current_room.clear();
                });
                DEV.with_borrow_mut(|d| d.on_signed_out(window));
            }
            SessionState::SigningIn => {
                window.set_busy(true);
                window.set_error("".into());
            }
            SessionState::SignedIn { user_id } => {
                window.set_busy(false);
                window.set_password("".into());
                window.set_user_id(user_id.into());
                window.set_page(2);
                DEV.with_borrow(|d| d.stat("signed in"));
            }
        },
        Event::Rooms(rooms) => {
            DEV.with_borrow_mut(|d| d.on_rooms(window, &rooms));
            STATE.with_borrow_mut(|s| s.rooms = rooms);
            show_rooms(window);
            if window.get_activity_open() {
                activity::refresh(window);
            }
        }
        Event::RoomOpened {
            room_id,
            name,
            is_encrypted,
        } => {
            if STATE.with_borrow(|s| s.current_room == room_id) {
                window.set_current_room_name(name.into());
                window.set_current_room_encrypted(is_encrypted);
                // A direct message says nothing of who read what (web `!isDirect`)
                let direct = STATE.with_borrow(|s| {
                    s.rooms
                        .iter()
                        .any(|r| r.id == room_id && r.kind == twake_domain::RoomKind::Direct)
                });
                window.set_current_room_direct(direct);
            }
        }
        Event::Timeline {
            room_id,
            entries,
            pinned_count,
        } => {
            if STATE.with_borrow(|s| s.current_room != room_id) {
                return;
            }
            window.set_pinned_count(i32::try_from(pinned_count).unwrap_or(i32::MAX));
            show_timeline(window, &entries);
            STATE.with_borrow_mut(|s| s.entries = entries.clone());
            call::refresh_thread(window);
            dev::on_timeline(window, &entries);
        }
        Event::Image {
            key,
            width,
            height,
            picture,
        } => {
            let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
                &picture.rgba,
                picture.width,
                picture.height,
            );
            STATE.with_borrow_mut(|s| {
                s.pictures.insert(
                    format!("{key}@{width}x{height}"),
                    slint::Image::from_rgba8(buffer),
                );
            });
            show_rooms(window);
            let entries = STATE.with_borrow_mut(|s| std::mem::take(&mut s.entries));
            show_timeline(window, &entries);
            STATE.with_borrow_mut(|s| s.entries = entries);
            let thread = STATE.with_borrow_mut(|s| std::mem::take(&mut s.thread_entries));
            show_thread(window, &thread);
            show_info(window);
            show_spaces();
            STATE.with_borrow_mut(|s| s.thread_entries = thread);
            viewer::refresh(window);
        }
        Event::ImageFailed { .. } => {}
        Event::FileData { key, bytes } => audio::file_arrived(window, &key, bytes),
        Event::PdfOpened { key, pages } => pdf::opened(window, &key, pages),
        Event::PdfFailed { key, error } => pdf::failed(window, &key, &error),
        Event::PdfPage {
            key,
            page,
            width,
            picture,
        } => pdf::page_drawn(&key, page, width, &picture),
        Event::Animation {
            key,
            width,
            height,
            frames,
        } => {
            if gifs::arrived(window, &key, width, height, frames) {
                redraw_messages(window);
                viewer::refresh_animation(window);
            }
        }
        Event::CallMembers { room_id, user_ids } => call::members(window, room_id, &user_ids),
        Event::CallStatus { room_id, status } => call::status(window, &room_id, status),
        Event::CallTiles(tiles) => call::tiles(window, tiles),
        Event::CallDevices(lists) => call::devices(window, lists),
        Event::Ringing(ringing) => call::ringing(window, ringing),
        Event::CallFrame { identity, picture } => call::frame(&identity, &picture),
        Event::Autoplay(enabled) => {
            // Never chosen: on, as the web when the system asks for no reduced motion
            window.set_autoplay(enabled != Some(false));
            if gifs::set_autoplay(enabled) {
                redraw_messages(window);
                viewer::refresh_animation(window);
            }
        }
        Event::LinkPreview { url, preview } => {
            STATE.with_borrow_mut(|s| s.link_previews.insert(url, preview));
            let entries = STATE.with_borrow_mut(|s| std::mem::take(&mut s.entries));
            show_timeline(window, &entries);
            STATE.with_borrow_mut(|s| s.entries = entries);
            let thread = STATE.with_borrow_mut(|s| std::mem::take(&mut s.thread_entries));
            show_thread(window, &thread);
            STATE.with_borrow_mut(|s| s.thread_entries = thread);
        }
        Event::Thread {
            room_id,
            root_id,
            entries,
        } => {
            let current = STATE.with_borrow(|s| {
                s.current_room == room_id && s.thread_root.as_deref() == Some(root_id.as_str())
            });
            if current {
                show_thread(window, &entries);
                STATE.with_borrow_mut(|s| s.thread_entries = entries);
            }
        }
        Event::History { loading, at_start } => {
            window.set_loading_history(loading);
            if at_start {
                window.set_history_complete(true);
            }
        }
        Event::Encryption(status) => {
            let needs_key = !status.verified && status.recovery_available;
            window.set_needs_recovery_key(needs_key);
            if needs_key {
                dev::on_needs_recovery_key(window);
            }
        }
        Event::Typing { room_id, names } => {
            if STATE.with_borrow(|s| s.current_room == room_id) {
                let text = match names.as_slice() {
                    [] => String::new(),
                    [one] => format!("{one} is typing…"),
                    [a, b] => format!("{a} and {b} are typing…"),
                    [a, rest @ ..] => format!("{a} and {} others are typing…", rest.len()),
                };
                window.set_typing_text(text.into());
            }
        }
        Event::Spaces(spaces) => {
            dev::on_spaces(window, &spaces);
            STATE.with_borrow_mut(|s| s.spaces = spaces);
            show_spaces();
            show_rooms(window);
        }
        Event::Activity(feed) => activity::update(window, feed),
        Event::Saved(entries) => {
            activity::saved(window, entries);
            // The 🔖 of the rows shown
            let entries = STATE.with_borrow_mut(|s| std::mem::take(&mut s.entries));
            show_timeline(window, &entries);
            STATE.with_borrow_mut(|s| s.entries = entries);
        }
        Event::SearchResults {
            room_id,
            query,
            results,
            is_complete,
        } => search::results(window, &room_id, &query, &results, is_complete),
        Event::DateFound { room_id, event_id } => {
            if window.get_current_room_id() == room_id.as_str() {
                let title = window.get_jump_title();
                search::open_in_context(window, &event_id, &title);
            }
        }
        Event::Composer {
            room_id,
            members,
            can_mention_everyone,
            commands,
        } => {
            composer::context(
                room_id,
                window.get_user_id().to_string(),
                members,
                can_mention_everyone,
                commands,
            );
        }
        Event::RoomInfo(details) => {
            if STATE.with_borrow(|s| s.current_room == details.room_id) {
                STATE.with_borrow_mut(|s| s.details = Some(details));
                show_info(window);
            }
        }
        Event::RoomCreated { room_id } => {
            window.set_create_mode("".into());
            window.invoke_open_room(room_id.into());
        }
        Event::RoomLeft { room_id } => {
            if STATE.with_borrow(|s| s.current_room == room_id) {
                STATE.with_borrow_mut(|s| {
                    s.current_room.clear();
                    s.details = None;
                    s.rows_model.clear();
                    s.entries.clear();
                });
                window.set_info_open(false);
                window.set_current_room_id("".into());
                close_thread(window);
                call::note_placement(window);
            }
        }
        Event::Notify {
            room_id,
            title,
            body,
        } => {
            // The room on screen needs no notification.
            if STATE.with_borrow(|s| s.current_room == room_id) {
                return;
            }
            DEV.with_borrow(|d| d.stat(&format!("notification: {title}")));
            // Off the UI thread: some platforms answer slowly.
            std::thread::spawn(move || {
                let _ = notify_rust::Notification::new()
                    .appname("Twake Chat")
                    .summary(&title)
                    .body(&body)
                    .show();
            });
        }
        Event::Forwarded { to_rooms } => forward::forwarded(window, &to_rooms),
        Event::MessageVersions { versions, .. } => show_versions(window, &versions),
        Event::PublicRooms(rooms) => directory::loaded(window, rooms),
        Event::Connection { online } => window.set_offline(!online),
        Event::Verification(phase) => show_verification(window, phase),
        Event::ComposerStyle(style) => window.set_composer_style(style.into()),
        Event::SpaceChoices { room_id, choices } => {
            if STATE.with_borrow(|s| s.current_room == room_id) {
                show_space_choices(window, &choices);
            }
        }
        Event::DeletedChats(deleted) => {
            STATE.with_borrow_mut(|s| s.deleted = deleted);
            show_rooms(window);
            redraw_messages(window);
        }
        Event::Notice(text) => {
            DEV.with_borrow(|d| d.stat(&format!("notice: {text}")));
            window.set_notice(text.into());
            window.set_error("".into());
        }
        Event::Error(text) => {
            eprintln!("twake-chat: {text}");
            if window.get_page() == 2 {
                window.set_notice(text.clone().into());
            }
            window.set_error(text.into());
        }
    }
}

/// Brings `model` to `rows` with as few row changes as possible, so the list keeps its
/// scroll position and only re-lays out what changed.
fn sync_model<T: Clone + PartialEq + 'static>(model: &VecModel<T>, rows: Vec<T>) {
    let old_len = model.row_count();
    let new_len = rows.len();
    let mut prefix = 0;
    while prefix < old_len
        && prefix < new_len
        && model.row_data(prefix).as_ref() == Some(&rows[prefix])
    {
        prefix += 1;
    }
    let mut suffix = 0;
    while suffix < old_len - prefix
        && suffix < new_len - prefix
        && model.row_data(old_len - 1 - suffix).as_ref() == Some(&rows[new_len - 1 - suffix])
    {
        suffix += 1;
    }
    let old_mid = old_len - prefix - suffix;
    let new_mid = new_len - prefix - suffix;
    let common = old_mid.min(new_mid);
    for i in 0..common {
        model.set_row_data(prefix + i, rows[prefix + i].clone());
    }
    if new_mid > old_mid {
        for i in common..new_mid {
            model.insert(prefix + i, rows[prefix + i].clone());
        }
    } else {
        for _ in common..old_mid {
            model.remove(prefix + common);
        }
    }
}

/// Looks images up in the cache; returns the requests to send for the missing ones.
fn lookup<'a>(
    pictures: &'a HashMap<String, slint::Image>,
    asked: &'a mut HashSet<String>,
    requests: &'a mut Vec<Command>,
) -> impl FnMut(&str, u32, u32, bool) -> Option<slint::Image> + 'a {
    move |key, width, height, crop| {
        let id = format!("{key}@{width}x{height}");
        if let Some(image) = pictures.get(&id) {
            return Some(image.clone());
        }
        if width > 0 && height > 0 && asked.insert(id) {
            requests.push(Command::FetchImage {
                key: key.to_string(),
                width,
                height,
                crop,
            });
        }
        None
    }
}

fn show_rooms(window: &AppWindow) {
    let _ = window;
    let now = chrono::Utc::now().timestamp_millis();
    let mut requests = Vec::new();
    STATE.with_borrow_mut(|s| {
        let filter = s.filter.to_lowercase();
        // The rooms of the picked space (all rooms when none is picked).
        let space: Option<HashSet<String>> = s
            .spaces
            .iter()
            .find(|sp| sp.id == s.current_space)
            .map(|sp| sp.rooms.iter().cloned().collect());
        let chip = s.room_filter.clone();
        let UiState {
            rooms,
            pictures,
            asked,
            rooms_model,
            deleted,
            ..
        } = s;
        let mut find = lookup(pictures, asked, &mut requests);
        let mut rows: Vec<RoomRow> = rooms
            .iter()
            // A deleted chat stays out of the list until something newer (web `isChatHidden`)
            .filter(|r| {
                let last = r.last_activity.unwrap_or(0) as f64;
                !twake_domain::deleted_chats::is_chat_hidden(deleted, &r.id, last)
            })
            .filter(|r| filter.is_empty() || r.name.to_lowercase().contains(&filter))
            .filter(|r| space.as_ref().is_none_or(|ids| ids.contains(&r.id)))
            .filter(|r| match chip.as_str() {
                "unread" => {
                    r.unread_count > 0 || r.mention_count > 0 || r.is_marked_unread || r.is_invite
                }
                "people" => r.kind == twake_domain::RoomKind::Direct,
                "rooms" => r.kind != twake_domain::RoomKind::Direct,
                _ => true,
            })
            .map(|r| view::room_row(r, now, &mut find))
            .collect();
        // Pinned rooms first, each group in its order (web « Pin »)
        rows.sort_by_key(|r| !r.favourite);
        sync_model(rooms_model, rows);
    });
    requests.into_iter().for_each(send);
}

fn show_timeline(window: &AppWindow, entries: &[TimelineEntry]) {
    // A chat deleted for the user: its history up to then is hidden (web `withoutDeleted`)
    let (entries, notice) = STATE.with_borrow(|s| {
        (
            deleted::visible(entries, &s.current_room, &s.deleted),
            deleted::notice(&s.current_room, &s.deleted),
        )
    });
    let entries = &entries[..];
    window.set_deleted_before(notice.into());
    let now = chrono::Utc::now().timestamp_millis();
    let encrypted = window.get_current_room_encrypted();
    let mut requests = Vec::new();
    let rows = STATE.with_borrow_mut(|s| {
        let UiState {
            pictures,
            asked,
            link_previews,
            asked_previews,
            ..
        } = s;
        let mut previews = Vec::new();
        let mut find = lookup(pictures, asked, &mut requests);
        let mut rows = view::timeline_rows(entries, now, &mut find);
        previews::fill(
            &mut rows,
            entries,
            link_previews,
            asked_previews,
            &mut previews,
            encrypted,
            &mut find,
        );
        drop(find);
        requests.extend(previews);
        rows
    });
    let rows: Vec<Row> = rows
        .into_iter()
        .map(|mut r| {
            r.saved = activity::is_saved(&r.id);
            r.selected = selection::is_selected(&r.id);
            r.anim = gifs::slot(&r, view::DENSITY, &mut requests);
            r
        })
        .collect();
    requests.into_iter().for_each(send);
    let first = rows.first().map(|r| r.id.to_string()).unwrap_or_default();
    let last = rows.last().map(|r| r.id.to_string()).unwrap_or_default();
    let last_is_own = rows.last().is_some_and(|r| r.own);
    let (prepended, new_end) = STATE.with_borrow_mut(|s| {
        let prepended =
            !s.first_row_id.is_empty() && s.first_row_id != first && s.last_row_id == last;
        let new_end = s.last_row_id != last;
        s.first_row_id = first;
        s.last_row_id = last;
        (prepended, new_end)
    });
    window.set_history_prepended(prepended);
    STATE.with_borrow(|s| sync_model(&s.rows_model, rows));
    if new_end && (window.get_at_bottom() || last_is_own) {
        let bump = STATE.with_borrow_mut(|s| {
            s.scroll_bump += 1;
            s.scroll_bump
        });
        window.set_scroll_to_bottom(bump);
        send(Command::MarkRead);
    }
    shared::show(window, entries);
}

/// The messages of the room and of the open thread, drawn again from what is kept.
fn redraw_messages(window: &AppWindow) {
    let entries = STATE.with_borrow_mut(|s| std::mem::take(&mut s.entries));
    show_timeline(window, &entries);
    STATE.with_borrow_mut(|s| s.entries = entries);
    let thread = STATE.with_borrow_mut(|s| std::mem::take(&mut s.thread_entries));
    show_thread(window, &thread);
    STATE.with_borrow_mut(|s| s.thread_entries = thread);
}

fn show_thread(window: &AppWindow, entries: &[TimelineEntry]) {
    if STATE.with_borrow(|s| s.thread_root.is_none()) {
        return;
    }
    let now = chrono::Utc::now().timestamp_millis();
    let encrypted = window.get_current_room_encrypted();
    let mut requests = Vec::new();
    let rows = STATE.with_borrow_mut(|s| {
        let UiState {
            pictures,
            asked,
            link_previews,
            asked_previews,
            ..
        } = s;
        let mut previews = Vec::new();
        let mut find = lookup(pictures, asked, &mut requests);
        let mut rows = view::timeline_rows(entries, now, &mut find);
        previews::fill(
            &mut rows,
            entries,
            link_previews,
            asked_previews,
            &mut previews,
            encrypted,
            &mut find,
        );
        drop(find);
        requests.extend(previews);
        rows
    });
    let rows: Vec<Row> = rows
        .into_iter()
        .map(|mut r| {
            r.highlight = search::is_target(&r.id);
            r.saved = activity::is_saved(&r.id);
            r.anim = gifs::slot(&r, view::DENSITY, &mut requests);
            r
        })
        .collect();
    requests.into_iter().for_each(send);
    let last = rows.last().map(|r| r.id.to_string()).unwrap_or_default();
    let (bump, new_end) = STATE.with_borrow_mut(|s| {
        sync_model(&s.thread_model, rows);
        let new_end = s.thread_last_id != last;
        if new_end {
            s.thread_last_id = last;
            s.thread_scroll += 1;
        }
        (s.thread_scroll, new_end)
    });
    window.set_thread_scroll_to_bottom(bump);
    // The thread is read to its newest reply (the engine sends a receipt only for a
    // thread, never for the pinned messages or a search result in context)
    if new_end && window.get_thread_open() {
        send(Command::MarkThreadRead);
    }
}

/// Sends files, as an answer when the composer is replying; the reply mode ends with them.
fn send_files(window: &AppWindow, paths: Vec<std::path::PathBuf>, thread: bool) {
    let mode = STATE.with_borrow(|s| {
        if thread {
            s.thread_compose.clone()
        } else {
            s.compose.clone()
        }
    });
    let reply_to = match mode {
        Compose::Reply(id) => Some(id),
        _ => None,
    };
    for path in paths {
        send(Command::SendFile {
            path,
            thread,
            reply_to: reply_to.clone(),
            caption: None,
        });
    }
    if reply_to.is_some() {
        set_compose(window, thread, Compose::New);
    }
    if !thread {
        window.set_at_bottom(true);
    }
}

/// Asks before running `command` (actions that cannot be undone).
fn ask(window: &AppWindow, message: &str, label: &str, command: Command) {
    // Deleting a message, removing or banning someone may say why (web `ReasonDialog`)
    let with_reason = matches!(
        command,
        Command::Delete { .. }
            | Command::Room {
                action: RoomAction::Remove(..)
                    | RoomAction::Ban(..)
                    | RoomAction::Report(_)
                    | RoomAction::ReportMessage(..),
                ..
            }
    );
    window.set_confirm_with_reason(with_reason);
    window.set_confirm_reason("".into());
    STATE.with_borrow_mut(|s| s.pending = Some(command));
    window.set_confirm_label(label.into());
    window.set_confirm_message(message.into());
}

/// A confirmation for several commands sent together (no reason asked).
fn ask_many(window: &AppWindow, message: &str, label: &str, mut commands: Vec<Command>) {
    if commands.is_empty() {
        return;
    }
    let first = commands.remove(0);
    window.set_confirm_with_reason(false);
    window.set_confirm_reason("".into());
    STATE.with_borrow_mut(|s| {
        s.pending = Some(first);
        s.pending_more = commands;
    });
    window.set_confirm_label(label.into());
    window.set_confirm_message(message.into());
}

/// The command of a confirmation, with the reason typed in it.
fn with_reason(command: Command, reason: Option<String>) -> Command {
    match command {
        Command::Delete { id, thread, .. } => Command::Delete { id, thread, reason },
        Command::Room {
            room_id,
            action: RoomAction::Remove(user, _),
        } => Command::Room {
            room_id,
            action: RoomAction::Remove(user, reason),
        },
        Command::Room {
            room_id,
            action: RoomAction::Ban(user, _),
        } => Command::Room {
            room_id,
            action: RoomAction::Ban(user, reason),
        },
        Command::Room {
            room_id,
            action: RoomAction::Report(_),
        } => Command::Room {
            room_id,
            action: RoomAction::Report(reason),
        },
        Command::Room {
            room_id,
            action: RoomAction::ReportMessage(id, _),
        } => Command::Room {
            room_id,
            action: RoomAction::ReportMessage(id, reason),
        },
        other => other,
    }
}

/// The verification of one's devices at its new phase (web `DeviceVerificationDialog`).
fn show_verification(window: &AppWindow, phase: twake_core::VerificationPhase) {
    use twake_core::VerificationPhase as P;
    let name = match &phase {
        P::Incoming => "incoming",
        P::Asking => "asking",
        P::Waiting => "waiting",
        P::Comparing { .. } => "comparing",
        P::Done => "done",
        P::Cancelled => "cancelled",
    };
    if let P::Comparing { emojis, decimals } = &phase {
        let rows: Vec<VerifyEmoji> = emojis
            .iter()
            .map(|(symbol, name)| VerifyEmoji {
                symbol: symbol.as_str().into(),
                name: name.as_str().into(),
            })
            .collect();
        window.set_verify_emojis(ModelRc::from(Rc::new(VecModel::from(rows))));
        let (a, b, c) = decimals;
        window.set_verify_decimals(format!("{a} {b} {c}").into());
    }
    window.set_verify_phase(name.into());
}

/// « Move to a Space »: « No space » first (checked when the room is in none), then the
/// spaces where the user may add rooms (web `MoveToSpaceDialog`).
fn show_space_choices(window: &AppWindow, choices: &[twake_domain::space_choices::SpaceChoice]) {
    let in_one = choices.iter().any(|c| c.is_current);
    let rows: Vec<SpaceChoiceRow> = std::iter::once(SpaceChoiceRow {
        id: "".into(),
        name: "No space".into(),
        depth: 0,
        current: !in_one,
    })
    .chain(choices.iter().map(|c| SpaceChoiceRow {
        id: c.id.as_str().into(),
        name: c.name.as_str().into(),
        depth: i32::try_from(c.depth).unwrap_or(0),
        current: c.is_current,
    }))
    .collect();
    window.set_space_choices(ModelRc::from(Rc::new(VecModel::from(rows))));
    window.set_space_dialog_open(true);
}

/// « Edit history », newest first, the newest marked current (web
/// `MessageVersionsDialog`); a history that could not be read says so instead.
fn show_versions(window: &AppWindow, versions: &[twake_core::MessageVersion]) {
    if versions.is_empty() {
        window.set_notice("The edit history could not be loaded.".into());
        return;
    }
    let now = chrono::Utc::now().timestamp_millis();
    let rows: Vec<VersionRow> = versions
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| {
            let when = format!(
                "{} {}",
                twake_domain::format::room_list_time(v.sent_at, now),
                twake_domain::format::message_time(v.sent_at)
            );
            VersionRow {
                text: v.text.as_str().into(),
                info: if i == 0 {
                    format!("{when} · current")
                } else {
                    when
                }
                .into(),
            }
        })
        .collect();
    window.set_versions(ModelRc::from(Rc::new(VecModel::from(rows))));
    window.set_versions_title("Edit history".into());
    window.set_versions_open(true);
}

/// « Seen by »: the others who read the message, the latest reader first, each with the
/// time of their receipt (web `message-seen`).
fn show_seen(window: &AppWindow, id: &str) {
    let mut readers = STATE.with_borrow(|s| {
        view::readers_by_message(&s.entries)
            .remove(id)
            .unwrap_or_default()
    });
    readers.sort_by_key(|r| std::cmp::Reverse(r.read_at));
    let now = chrono::Utc::now().timestamp_millis();
    let rows: Vec<VersionRow> = readers
        .iter()
        .map(|r| VersionRow {
            text: r.name.as_str().into(),
            info: r
                .read_at
                .map(|at| {
                    format!(
                        "{} {}",
                        twake_domain::format::room_list_time(at, now),
                        twake_domain::format::message_time(at)
                    )
                })
                .unwrap_or_default()
                .into(),
        })
        .collect();
    window.set_versions(ModelRc::from(Rc::new(VecModel::from(rows))));
    window.set_versions_title("Seen by".into());
    window.set_versions_open(true);
}

/// Puts `text` on the system's clipboard and says `done`, or that it could not.
fn copy_to_clipboard(window: &AppWindow, text: &str, done: &str) {
    let copied = arboard::Clipboard::new().and_then(|mut c| c.set_text(text));
    let notice = if copied.is_ok() {
        done.to_string()
    } else {
        "The system refused to copy.".to_string()
    };
    window.set_notice(notice.into());
}

fn current_room() -> String {
    STATE.with_borrow(|s| s.current_room.clone())
}

fn room_command(action: twake_core::RoomAction) {
    let room_id = current_room();
    if !room_id.is_empty() {
        send(Command::Room { room_id, action });
    }
}

fn member_name(user_id: &str) -> String {
    STATE.with_borrow(|s| {
        s.details
            .as_ref()
            .and_then(|d| d.members.iter().find(|m| m.user_id == user_id))
            .map_or_else(|| user_id.to_string(), |m| m.name.clone())
    })
}

fn show_info(window: &AppWindow) {
    let mut requests = Vec::new();
    STATE.with_borrow_mut(|s| {
        let UiState {
            pictures,
            asked,
            details,
            members_model,
            ..
        } = s;
        let Some(details) = details.as_ref() else {
            return;
        };
        let mut find = lookup(pictures, asked, &mut requests);
        window.set_info(view::room_info(details, &mut find));
        sync_model(members_model, view::member_rows(details, &mut find));
    });
    requests.into_iter().for_each(send);
    let entries = STATE.with_borrow_mut(|s| std::mem::take(&mut s.entries));
    shared::show(window, &entries);
    STATE.with_borrow_mut(|s| s.entries = entries);
}

/// The room info pane, invitations, the new room dialog and the confirmations.
fn wire_room_info(window: &AppWindow) {
    STATE.with_borrow(|s| window.set_info_members(ModelRc::from(s.members_model.clone())));
    let weak = window.as_weak();
    let with = move |f: &dyn Fn(&AppWindow)| {
        if let Some(w) = weak.upgrade() {
            f(&w);
        }
    };
    let w = with.clone();
    window.on_open_info(move || {
        w(&|win| {
            close_thread(win);
            win.set_info_open(true);
        });
        let room_id = current_room();
        if !room_id.is_empty() {
            send(Command::RoomInfo { room_id });
        }
    });
    let w = with.clone();
    window.on_close_info(move || w(&|win| win.set_info_open(false)));
    window.on_info_rename(|name| room_command(RoomAction::Rename(name.to_string())));
    window.on_info_set_topic(|topic| room_command(RoomAction::SetTopic(topic.to_string())));
    let w = with.clone();
    window.on_info_encrypt(move || {
        let room_id = current_room();
        w(&|win| {
            ask(
                win,
                "Turn on end-to-end encryption? It cannot be turned off afterwards.",
                "Turn on",
                Command::Room {
                    room_id: room_id.clone(),
                    action: RoomAction::Encrypt,
                },
            )
        });
    });
    let w = with.clone();
    window.on_info_invite(move |user| {
        room_command(RoomAction::Invite(user.to_string()));
        w(&|win| win.set_info_invite_text("".into()));
    });
    let w = with.clone();
    window.on_member_remove(move |user| {
        let (room_id, name) = (current_room(), member_name(&user));
        w(&|win| {
            ask(
                win,
                &format!("Remove {name} from the room?"),
                "Remove",
                Command::Room {
                    room_id: room_id.clone(),
                    action: RoomAction::Remove(user.to_string(), None),
                },
            )
        });
    });
    let w = with.clone();
    window.on_member_ban(move |user| {
        let (room_id, name) = (current_room(), member_name(&user));
        w(&|win| {
            ask(
                win,
                &format!("Ban {name}? They cannot come back until unbanned."),
                "Ban",
                Command::Room {
                    room_id: room_id.clone(),
                    action: RoomAction::Ban(user.to_string(), None),
                },
            )
        });
    });
    window.on_member_set_role(|user, role| {
        use twake_domain::room_management::MemberRole;
        let role = match role.as_str() {
            "admin" => MemberRole::Admin,
            "moderator" => MemberRole::Moderator,
            _ => MemberRole::Member,
        };
        room_command(RoomAction::SetRole(user.to_string(), role));
    });
    let w = with.clone();
    window.on_leave_room(move || {
        let room_id = current_room();
        let name = STATE.with_borrow(|s| {
            s.details
                .as_ref()
                .map_or_else(String::new, |d| d.name.clone())
        });
        w(&|win| {
            ask(
                win,
                &format!(
                    "Leave {}?",
                    if name.is_empty() { "this room" } else { &name }
                ),
                "Leave",
                Command::Room {
                    room_id: room_id.clone(),
                    action: RoomAction::Leave,
                },
            )
        });
    });
    let w = with.clone();
    window.on_notice_acted(move || {
        // « Undo »: the chat and its history come back (web `undeleteChat`)
        if let Some(room_id) = STATE.with_borrow_mut(|s| s.undo_room.take()) {
            send(Command::DeleteChat {
                room_id,
                before_ts: None,
            });
        }
        w(&|win| {
            win.set_notice("".into());
            win.set_notice_action("".into());
        });
    });
    window.on_info_set_join_rule(|rule| {
        use twake_domain::room_management::JoinRuleChoice;
        let rule = match rule.as_str() {
            "invite" => JoinRuleChoice::Invite,
            "knock" => JoinRuleChoice::Knock,
            "public" => JoinRuleChoice::Public,
            _ => return,
        };
        room_command(RoomAction::SetJoinRule(rule));
    });
    window.on_info_set_listed(|listed| room_command(RoomAction::SetListed(listed)));
    window.on_info_set_notification(|level| {
        use twake_domain::room_management::NotificationLevel;
        let level = match level.as_str() {
            "mentions" => NotificationLevel::Mentions,
            "none" => NotificationLevel::None,
            _ => NotificationLevel::All,
        };
        room_command(RoomAction::SetNotificationLevel(level));
    });
    // The menu of a room row (web `RoomRowMenu`)
    let w = with.clone();
    window.on_room_row_act(move |room_id, action| {
        use twake_domain::room_management::NotificationLevel;
        let room_id = room_id.to_string();
        let room = |action| Command::Room {
            room_id: room_id.clone(),
            action,
        };
        match action.as_str() {
            "read" => send(room(RoomAction::MarkRead)),
            "unread" => send(room(RoomAction::MarkUnread)),
            "mute" => send(room(RoomAction::SetNotificationLevel(NotificationLevel::None))),
            "unmute" => send(room(RoomAction::SetNotificationLevel(NotificationLevel::All))),
            "pin" => send(room(RoomAction::SetFavourite(true))),
            "unpin" => send(room(RoomAction::SetFavourite(false))),
            "leave" => w(&|win| ask(win, "Leave this room?", "Leave", room(RoomAction::Leave))),
            "delete" => {
                let last = STATE.with_borrow(|s| {
                    s.rooms
                        .iter()
                        .find(|r| r.id == room_id)
                        .and_then(|r| r.last_activity)
                        .unwrap_or(0)
                });
                let before_ts = chrono::Utc::now().timestamp_millis().max(last) as f64;
                w(&|win| {
                    ask(
                        win,
                        "Delete this chat?\n\nIt disappears from your list and its history is hidden for \
                         you; the other members keep it.",
                        "Delete chat",
                        Command::DeleteChat {
                            room_id: room_id.clone(),
                            before_ts: Some(before_ts),
                        },
                    )
                });
            }
            _ => {}
        }
    });
    window.on_info_move_to_space(|| {
        send(Command::SpaceChoices {
            room_id: current_room(),
        });
    });
    let w = with.clone();
    window.on_space_pick(move |id| {
        send(Command::MoveToSpace {
            room_id: current_room(),
            space_id: (!id.is_empty()).then(|| id.to_string()),
        });
        w(&|win| win.set_space_dialog_open(false));
    });
    let w = with.clone();
    window.on_report_room(move || {
        let room_id = current_room();
        w(&|win| {
            ask(
                win,
                "Report this room\n\nThe administrators of your server receive the room and your \
                 reason.",
                "Report",
                Command::Room {
                    room_id: room_id.clone(),
                    action: RoomAction::Report(None),
                },
            )
        });
    });
    let w = with.clone();
    window.on_report_message(move |id| {
        let room_id = current_room();
        w(&|win| {
            ask(
                win,
                "Report this message\n\nThe administrators of your server receive the message \
                 and your reason.",
                "Report",
                Command::Room {
                    room_id: room_id.clone(),
                    action: RoomAction::ReportMessage(id.to_string(), None),
                },
            )
        });
    });
    let w = with.clone();
    window.on_block_user(move || {
        // The other person of the direct message (web `peerId`)
        let peer = STATE.with_borrow(|s| {
            let details = s.details.as_ref()?;
            details
                .members
                .iter()
                .find(|m| m.user_id != details.own_user_id)
                .map(|m| (m.user_id.clone(), m.name.clone()))
        });
        let Some((user_id, name)) = peer else {
            return;
        };
        let room_id = current_room();
        w(&|win| {
            ask(
                win,
                &format!(
                    "Block {name}?\n\nYou will no longer receive their messages. They won't be \
                     notified."
                ),
                "Block",
                Command::Room {
                    room_id: room_id.clone(),
                    action: RoomAction::Block(user_id.clone()),
                },
            )
        });
    });
    let w = with.clone();
    window.on_delete_chat(move || {
        let room_id = current_room();
        // The clock of the server may run ahead of this one (web `deleteChat`)
        let last = STATE.with_borrow(|s| {
            s.rooms
                .iter()
                .find(|r| r.id == room_id)
                .and_then(|r| r.last_activity)
                .unwrap_or(0)
        });
        let before_ts = chrono::Utc::now().timestamp_millis().max(last) as f64;
        w(&|win| {
            ask(
                win,
                "Delete this chat?\n\nIt disappears from your list and its history is hidden for \
                 you; the other members keep it. You stay a member, so it comes back at the next \
                 message. To stop receiving it, leave the chat instead.",
                "Delete chat",
                Command::DeleteChat {
                    room_id: room_id.clone(),
                    before_ts: Some(before_ts),
                },
            )
        });
    });
    window.on_accept_invite(|room_id| {
        send(Command::Room {
            room_id: room_id.to_string(),
            action: RoomAction::Join,
        })
    });
    window.on_decline_invite(|room_id| {
        send(Command::Room {
            room_id: room_id.to_string(),
            action: RoomAction::Leave,
        })
    });
    let w = with.clone();
    window.on_show_create(move |mode| w(&|win| win.set_create_mode(mode.clone())));
    let w = with.clone();
    window.on_cancel_create(move || w(&|win| win.set_create_mode("".into())));
    window.on_retry_send(|id| send(Command::RetrySend { id: id.to_string() }));
    window.on_verify_device(|| send(Command::VerifyDevice));
    window.on_verify_accept(|| send(Command::AcceptVerification));
    window.on_verify_confirm(|matching| send(Command::ConfirmVerification(matching)));
    let w = with.clone();
    window.on_verify_cancel(move || {
        send(Command::CancelVerification);
        w(&|win| win.set_verify_phase("".into()));
    });
    let w = with.clone();
    window.on_verify_close(move || w(&|win| win.set_verify_phase("".into())));
    window.on_discard_send(|id| send(Command::DiscardSend { id: id.to_string() }));
    window.on_create_room(
        |kind, name, topic, people, public, encrypted, access, reactions, slug| {
            use twake_domain::room_management::{NewPage, NewRoom, NewRoomKind, PageAccess};
            let invite = people
                .split([',', ' ', ';', '\n'])
                .map(str::trim)
                .filter(|p| !p.is_empty())
                .map(str::to_string)
                .collect();
            let kind = match kind.as_str() {
                "feed" => NewRoomKind::Broadcast,
                "space" => NewRoomKind::Space,
                "voice" => NewRoomKind::Voice,
                "page" => NewRoomKind::Page,
                _ => NewRoomKind::Discussion,
            };
            let page = (kind == NewRoomKind::Page).then(|| NewPage {
                access: match access.as_str() {
                    "invite" => PageAccess::Invite,
                    "approval" => PageAccess::Approval,
                    _ => PageAccess::Open,
                },
                allows_reactions: reactions,
                slug: slug.to_string(),
            });
            send(Command::CreateRoom(NewRoom {
                name: name.to_string(),
                topic: topic.to_string(),
                kind,
                is_public: public && kind != NewRoomKind::Page,
                is_encrypted: encrypted,
                invite,
                page,
                ..NewRoom::default()
            }));
        },
    );
    window.on_suggest_slug(|name| twake_domain::room_management::suggest_slug(&name).into());
    window.on_valid_slug(|slug| twake_domain::room_management::is_valid_slug(&slug));
    window.on_open_direct(|user| {
        send(Command::OpenDirect {
            user_id: user.to_string(),
        })
    });
    let w = with.clone();
    window.on_confirm_yes(move || {
        let reason = std::cell::Cell::new(None);
        w(&|win| {
            let text = twake_domain::js::trim(&win.get_confirm_reason()).to_string();
            reason.set((!text.is_empty()).then_some(text));
        });
        if let Some(command) = STATE.with_borrow_mut(|s| s.pending.take()) {
            // A deleted chat says so, with Undo, and is read (web `announceDeletedChat`)
            if let Command::DeleteChat {
                room_id,
                before_ts: Some(_),
            } = &command
            {
                let room_id = room_id.clone();
                STATE.with_borrow_mut(|s| s.undo_room = Some(room_id));
                w(&|win| {
                    win.set_notice("Chat deleted".into());
                    win.set_notice_action("Undo".into());
                });
                send(Command::MarkRead);
            }
            send(with_reason(command, reason.take()));
            STATE
                .with_borrow_mut(|s| std::mem::take(&mut s.pending_more))
                .into_iter()
                .for_each(send);
            // Deleting the messages ticked ends the selection (web)
            if selection::is_active() {
                w(&selection::close);
            }
        }
        w(&|win| win.set_confirm_message("".into()));
    });
    let w = with;
    window.on_confirm_no(move || {
        STATE.with_borrow_mut(|s| {
            s.pending = None;
            s.pending_more.clear();
        });
        w(&|win| win.set_confirm_message("".into()));
    });
}

/// Polls (vote, end, create) and pins (pin, unpin, the pinned panel).
fn wire_polls_and_pins(window: &AppWindow) {
    window.on_vote(|id, answer, thread| {
        let Some(poll) = find_message(&id, thread).and_then(|m| m.poll) else {
            return;
        };
        let own = poll.own_answer_ids();
        let answers =
            twake_domain::poll::press_answer(f64::from(poll.max_selections), &own, &answer)
                .unwrap_or_default();
        send(Command::Vote {
            poll_id: id.to_string(),
            answers,
            thread,
        });
    });
    let weak = window.as_weak();
    window.on_end_poll(move |id, thread| {
        if let Some(w) = weak.upgrade() {
            ask(
                &w,
                "End this poll? Nobody can vote after.",
                "End poll",
                Command::EndPoll {
                    poll_id: id.to_string(),
                    thread,
                },
            );
        }
    });
    window.on_save_message(|id, saved| {
        send(Command::SaveMessage {
            event_id: id.to_string(),
            saved,
        })
    });
    window.on_pin(|id, pinned| {
        send(Command::Pin {
            ids: vec![id.to_string()],
            pinned,
        })
    });
    let weak = window.as_weak();
    window.on_open_pinned(move || {
        if let Some(w) = weak.upgrade() {
            close_thread(&w);
            w.set_info_open(false);
            STATE.with_borrow_mut(|s| s.thread_root = Some("pinned".into()));
            w.set_side_title("Pinned messages".into());
            w.set_side_pinned(true);
            w.set_thread_open(true);
        }
        send(Command::OpenPinned);
    });
    let weak = window.as_weak();
    window.on_show_poll(move |thread| {
        if let Some(w) = weak.upgrade() {
            w.set_poll_in_thread(thread);
            w.set_poll_open(true);
        }
    });
    let weak = window.as_weak();
    window.on_cancel_poll(move || {
        if let Some(w) = weak.upgrade() {
            w.set_poll_open(false);
        }
    });
    let weak = window.as_weak();
    window.on_create_poll(move |question, answers, multiple| {
        let Some(w) = weak.upgrade() else { return };
        let answers: Vec<String> = answers
            .lines()
            .map(str::trim)
            .filter(|a| !a.is_empty())
            .map(str::to_string)
            .collect();
        if !(twake_domain::poll::MIN_POLL_ANSWERS..=twake_domain::poll::MAX_POLL_ANSWERS)
            .contains(&answers.len())
        {
            w.set_notice("A poll needs 2 to 20 answers".into());
            return;
        }
        send(Command::CreatePoll {
            question: question.to_string(),
            answers,
            multiple,
            thread: w.get_poll_in_thread(),
        });
        w.set_poll_open(false);
        w.set_poll_question("".into());
        w.set_poll_answers("".into());
        w.set_poll_multiple(false);
    });
}

fn show_spaces() {
    let mut requests = Vec::new();
    STATE.with_borrow_mut(|s| {
        let UiState {
            spaces,
            pictures,
            asked,
            spaces_model,
            ..
        } = s;
        let mut find = lookup(pictures, asked, &mut requests);
        let rows: Vec<SpaceItem> = spaces
            .iter()
            .map(|sp| view::space_item(sp, &mut find))
            .collect();
        sync_model(spaces_model, rows);
    });
    requests.into_iter().for_each(send);
}
