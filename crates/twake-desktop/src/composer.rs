//! The composers' suggestions (emoji, @person, #room, /command), the mentions picked
//! while writing, and the drafts per conversation.
//!
//! Drafts stay in memory, never on the disk: the text of an encrypted room must not
//! outlive the session (web D16).

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use slint::{ModelRc, VecModel};
use twake_domain::composer::{
    BotCommand, OutgoingMessage, find_command_query, find_emoji_query, find_mention_query,
    find_room_query, match_commands, match_emojis, prepare_outgoing,
};
use twake_domain::mention::{
    EVERYONE, LinkableRoom, Mention, match_members, match_rooms, mention_text, room_mention_ref,
    room_mention_text, with_everyone,
};
use twake_domain::room_management::RoomMemberDetail;

use crate::{AppWindow, STATE, Suggestion};

/// What a suggestion puts in the text.
#[derive(Clone, Debug)]
enum Pick {
    Emoji(&'static str),
    Person(Mention),
    Command(String),
}

/// One composer: the mentions picked, the suggestions shown and the word they replace.
#[derive(Default)]
struct Pane {
    mentions: Vec<Mention>,
    picks: Vec<Pick>,
    /// Byte range of the word the suggestions replace.
    start: usize,
    end: usize,
    text: String,
}

#[derive(Default)]
struct ComposerState {
    room_id: String,
    members: Vec<RoomMemberDetail>,
    own_user_id: String,
    can_mention_everyone: bool,
    commands: Vec<BotCommand>,
    main: Pane,
    thread: Pane,
    /// Text and mentions not sent, by room id or `room|thread root`.
    drafts: HashMap<String, (String, Vec<Mention>)>,
}

impl ComposerState {
    fn pane(&mut self, thread: bool) -> &mut Pane {
        if thread {
            &mut self.thread
        } else {
            &mut self.main
        }
    }
}

thread_local! {
    static COMPOSER: RefCell<ComposerState> = RefCell::new(ComposerState::default());
}

/// The emojis of the picker.
pub fn emojis() -> ModelRc<slint::SharedString> {
    let all: Vec<slint::SharedString> = twake_domain::composer::all_emojis()
        .map(|e| e.emoji.into())
        .collect();
    ModelRc::from(Rc::new(VecModel::from(all)))
}

/// [`twake_core::Event::Composer`]: the members and commands of the open room.
pub fn context(
    room_id: String,
    own_user_id: String,
    members: Vec<RoomMemberDetail>,
    can_mention_everyone: bool,
    commands: Vec<BotCommand>,
) {
    COMPOSER.with_borrow_mut(|c| {
        if c.room_id != room_id {
            return;
        }
        c.members = members;
        c.own_user_id = own_user_id;
        c.can_mention_everyone = can_mention_everyone;
        c.commands = commands;
    });
}

fn draft_key(room_id: &str, thread_root: Option<&str>) -> String {
    match thread_root {
        Some(root) => format!("{room_id}|{root}"),
        None => room_id.to_string(),
    }
}

/// Keeps what the composer holds as the draft of `key`, clears the composer state.
fn keep_draft(key: String, text: String, thread: bool) {
    COMPOSER.with_borrow_mut(|c| {
        let pane = std::mem::take(c.pane(thread));
        if twake_domain::js::trim(&text).is_empty() {
            c.drafts.remove(&key);
        } else {
            c.drafts.insert(key, (text, pane.mentions));
        }
    });
}

/// The draft of `key`, now the composer's.
fn take_draft(key: &str, thread: bool) -> String {
    COMPOSER.with_borrow_mut(|c| {
        let (text, mentions) = c.drafts.get(key).cloned().unwrap_or_default();
        c.pane(thread).mentions = mentions;
        text
    })
}

/// Before the room changes: the main composer's text becomes its draft.
pub fn leave_room(window: &AppWindow, room_id: &str) {
    if !room_id.is_empty() {
        keep_draft(
            room_id.to_string(),
            window.get_compose_text().to_string(),
            false,
        );
    }
    clear_suggestions(window, false);
}

/// After the room changed: its draft, if any, comes back.
pub fn enter_room(window: &AppWindow, room_id: &str) {
    COMPOSER.with_borrow_mut(|c| {
        c.room_id = room_id.to_string();
        c.members.clear();
        c.commands.clear();
        c.can_mention_everyone = false;
    });
    window.set_compose_text(take_draft(room_id, false).into());
}

/// Before a thread closes: its composer's text becomes its draft.
pub fn leave_thread(window: &AppWindow, room_id: &str, root: &str) {
    keep_draft(
        draft_key(room_id, Some(root)),
        window.get_thread_compose_text().to_string(),
        true,
    );
    clear_suggestions(window, true);
}

/// After a thread opened: its draft comes back.
pub fn enter_thread(window: &AppWindow, room_id: &str, root: &str) {
    window.set_thread_compose_text(take_draft(&draft_key(room_id, Some(root)), true).into());
}

fn set_suggestions(window: &AppWindow, thread: bool, items: Vec<Suggestion>) {
    let model = ModelRc::from(Rc::new(VecModel::from(items)));
    if thread {
        window.set_thread_suggestions(model);
        window.set_thread_suggest_selected(0);
    } else {
        window.set_suggestions(model);
        window.set_suggest_selected(0);
    }
}

fn clear_suggestions(window: &AppWindow, thread: bool) {
    COMPOSER.with_borrow_mut(|c| c.pane(thread).picks.clear());
    set_suggestions(window, thread, Vec::new());
}

fn suggestion(icon: &str, label: &str, detail: &str) -> Suggestion {
    Suggestion {
        icon: icon.into(),
        label: label.into(),
        detail: detail.into(),
    }
}

/// The text changed: the suggestions for the word before the caret.
pub fn edited(window: &AppWindow, thread: bool, text: &str, caret: usize) {
    let caret = caret.min(text.len());
    let Some(before) = text.get(..caret) else {
        return clear_suggestions(window, thread);
    };
    let rooms: Vec<LinkableRoom> = STATE.with_borrow(|s| {
        s.rooms
            .iter()
            .map(|r| LinkableRoom {
                id: r.id.clone(),
                name: r.name.clone(),
                alias: None,
            })
            .collect()
    });
    let (items, picks, start) = COMPOSER.with_borrow(|c| {
        let mut items = Vec::new();
        let mut picks = Vec::new();
        let mut start = caret;
        if let Some(query) = find_command_query(before).filter(|_| !thread) {
            start = 0;
            for command in match_commands(&c.commands, query) {
                items.push(suggestion("/", &command.syntax, &command.description));
                picks.push(Pick::Command(command.name.clone()));
            }
            for (name, detail) in [("me", "an action: /me waves"), ("shrug", "¯\\_(ツ)_/¯")] {
                if name.starts_with(&query.to_lowercase()) {
                    items.push(suggestion("/", name, detail));
                    picks.push(Pick::Command(name.to_string()));
                }
            }
        } else if let Some(query) = find_emoji_query(before) {
            start = caret - query.len() - 1;
            for e in match_emojis(query) {
                items.push(suggestion(e.emoji, &format!(":{}:", e.shortcode), ""));
                picks.push(Pick::Emoji(e.emoji));
            }
        } else if let Some(query) = find_mention_query(before) {
            start = caret - query.len() - 1;
            let people: Vec<Option<&RoomMemberDetail>> =
                match_members(&c.members, query, &c.own_user_id, 8)
                    .into_iter()
                    .map(Some)
                    .collect();
            for person in with_everyone(&people, None, query, c.can_mention_everyone) {
                match person {
                    Some(m) => {
                        let text = mention_text(&m.name, m.matrix_name.as_deref());
                        items.push(suggestion("@", &m.name, &m.user_id));
                        picks.push(Pick::Person(Mention {
                            user_id: m.user_id.clone(),
                            name: text.to_string(),
                        }));
                    }
                    None => {
                        items.push(suggestion("📣", EVERYONE, "notify everyone in the room"));
                        picks.push(Pick::Person(Mention {
                            user_id: EVERYONE.into(),
                            name: EVERYONE.into(),
                        }));
                    }
                }
            }
        } else if let Some(query) = find_room_query(before) {
            start = caret - query.len() - 1;
            for room in match_rooms(&rooms, query, 8) {
                items.push(suggestion("#", &room.name, ""));
                picks.push(Pick::Person(Mention {
                    user_id: room_mention_ref(room).to_string(),
                    name: room_mention_text(room),
                }));
            }
        }
        (items, picks, start)
    });
    COMPOSER.with_borrow_mut(|c| {
        let pane = c.pane(thread);
        pane.picks = picks;
        pane.start = start;
        pane.end = caret;
        pane.text = text.to_string();
    });
    set_suggestions(window, thread, items);
}

fn replace(window: &AppWindow, thread: bool, text: String, caret: usize) {
    let caret = caret as i32;
    if thread {
        window.set_thread_compose_text(text.into());
        window.set_thread_caret_pos(caret);
        window.set_thread_caret_serial(window.get_thread_caret_serial() + 1);
    } else {
        window.set_compose_text(text.into());
        window.set_caret_pos(caret);
        window.set_caret_serial(window.get_caret_serial() + 1);
    }
}

/// A suggestion was taken: it replaces the word before the caret.
pub fn pick(window: &AppWindow, thread: bool, index: usize) {
    let picked = COMPOSER.with_borrow_mut(|c| {
        let pane = c.pane(thread);
        let pick = pane.picks.get(index).cloned()?;
        let insertion = match &pick {
            Pick::Emoji(e) => e.to_string(),
            Pick::Person(m) => format!("{} ", m.name),
            Pick::Command(name) => format!("/{name} "),
        };
        if let Pick::Person(m) = &pick {
            pane.mentions.push(m.clone());
        }
        let text = pane.text.get(..pane.start).unwrap_or("").to_string()
            + insertion.as_str()
            + pane.text.get(pane.end..).unwrap_or("");
        Some((text, pane.start + insertion.len()))
    });
    clear_suggestions(window, thread);
    if let Some((text, caret)) = picked {
        replace(window, thread, text, caret);
    }
}

/// Esc: the suggestions close.
pub fn dismiss(window: &AppWindow, thread: bool) {
    clear_suggestions(window, thread);
}

/// The emoji picker: `insertion` goes in at the caret.
pub fn insert(window: &AppWindow, thread: bool, insertion: &str, caret: usize) {
    let text = if thread {
        window.get_thread_compose_text()
    } else {
        window.get_compose_text()
    }
    .to_string();
    let mut caret = caret.min(text.len());
    while !text.is_char_boundary(caret) {
        caret -= 1;
    }
    let new_text = format!("{}{insertion}{}", &text[..caret], &text[caret..]);
    replace(window, thread, new_text, caret + insertion.len());
}

/// What the composer sends: `prepareOutgoing` with the mentions picked and the room's
/// commands. The picked mentions are dropped once read.
pub fn outgoing(thread: bool, text: &str) -> OutgoingMessage {
    COMPOSER.with_borrow_mut(|c| {
        let mentions = std::mem::take(&mut c.pane(thread).mentions);
        prepare_outgoing(text, &mentions, &c.commands)
    })
}
