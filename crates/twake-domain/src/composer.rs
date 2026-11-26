//! Port of the composer rules: `emojiShortcode.ts`, `botCommand.ts` and `jumboEmoji.ts`.

use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::js::{is_js_space, trim};
use crate::mention::{Mention, keep_present_mentions};

/// `EmojiShortcode`: an emoji and the name typed after « : » to find it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmojiShortcode {
    pub shortcode: &'static str,
    pub emoji: &'static str,
}

// The web's table, in its order (gemoji names for the emojis of the picker)
#[rustfmt::skip]
const SHORTCODES: &[(&str, &str)] = &[
    ("grinning", "😀"), ("smiley", "😃"), ("smile", "😄"), ("grin", "😁"),
    ("laughing", "😆"), ("sweat_smile", "😅"), ("joy", "😂"), ("rofl", "🤣"),
    ("slightly_smiling_face", "🙂"), ("upside_down_face", "🙃"),
    ("wink", "😉"), ("blush", "😊"), ("innocent", "😇"),
    ("heart_eyes", "😍"), ("kissing_heart", "😘"), ("sunglasses", "😎"),
    ("thinking", "🤔"), ("roll_eyes", "🙄"), ("open_mouth", "😮"),
    ("cry", "😢"), ("sob", "😭"), ("rage", "😡"), ("partying_face", "🥳"),
    ("sleeping", "😴"), ("exploding_head", "🤯"), ("grimacing", "😬"),
    ("hugs", "🤗"), ("saluting_face", "🫡"),
    ("thumbsup", "👍"), ("thumbsdown", "👎"), ("clap", "👏"),
    ("raised_hands", "🙌"), ("pray", "🙏"), ("muscle", "💪"), ("wave", "👋"),
    ("handshake", "🤝"), ("v", "✌️"), ("crossed_fingers", "🤞"),
    ("ok_hand", "👌"), ("eyes", "👀"), ("heart_hands", "🫶"),
    ("point_up", "☝️"), ("shrug", "🤷"), ("raising_hand", "🙋"),
    ("heart", "❤️"), ("orange_heart", "🧡"), ("yellow_heart", "💛"),
    ("green_heart", "💚"), ("blue_heart", "💙"), ("purple_heart", "💜"),
    ("fire", "🔥"), ("sparkles", "✨"), ("tada", "🎉"),
    ("white_check_mark", "✅"), ("x", "❌"), ("warning", "⚠️"),
    ("hundred", "💯"), ("star", "⭐"), ("rocket", "🚀"), ("bulb", "💡"),
    ("pushpin", "📌"), ("paperclip", "📎"), ("date", "📅"), ("coffee", "☕"),
    ("pizza", "🍕"), ("trophy", "🏆"), ("bug", "🐛"), ("lock", "🔒"),
];

/// The emojis of the table, for a picker.
pub fn all_emojis() -> impl Iterator<Item = EmojiShortcode> {
    SHORTCODES
        .iter()
        .map(|&(shortcode, emoji)| EmojiShortcode { shortcode, emoji })
}

/// `matchEmojis` (every emoji drawable): the one the query names first, then those
/// it starts, then the others, in the table's order; at most 8.
pub fn match_emojis(query: &str) -> Vec<EmojiShortcode> {
    let wanted = query.to_lowercase();
    if wanted.is_empty() {
        return Vec::new();
    }
    let rank = |s: &str| {
        if s == wanted {
            0
        } else if s.starts_with(&wanted) {
            1
        } else {
            2
        }
    };
    let mut found: Vec<EmojiShortcode> = all_emojis()
        .filter(|e| e.shortcode.contains(&wanted))
        .collect();
    found.sort_by_key(|e| rank(e.shortcode));
    found.truncate(8);
    found
}

/// `findEmojiQuery` on the text before the caret: the query after a « : » that starts
/// the text or follows a space, a letter then letters, digits, « _ », « + » or « - ».
/// The « : » is at `before.len() - query.len() - 1`.
pub fn find_emoji_query(before: &str) -> Option<&str> {
    let colon = before.rfind(':')?;
    let query = &before[colon + 1..];
    let mut chars = query.chars();
    if !chars.next().is_some_and(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    if !chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '+' | '-')) {
        return None;
    }
    let preceded = before[..colon].chars().next_back();
    (preceded.is_none() || preceded.is_some_and(is_js_space)).then_some(query)
}

/// The last word before the caret (after the last space).
fn last_word(before: &str) -> &str {
    before
        .rfind(is_js_space)
        .map(|i| &before[i + before[i..].chars().next().map_or(1, char::len_utf8)..])
        .unwrap_or(before)
}

/// `findMentionQuery` (web `ds/Composer`) on the text before the caret: « @query » at
/// the start of a word, the query an optional « @ » then no « @ ». The « @ » is at
/// `before.len() - query.len() - 1`.
pub fn find_mention_query(before: &str) -> Option<&str> {
    let query = last_word(before).strip_prefix('@')?;
    (!query.strip_prefix('@').unwrap_or(query).contains('@')).then_some(query)
}

/// `findRoomQuery` (web `ds/Composer`): « #query » at the start of a word.
pub fn find_room_query(before: &str) -> Option<&str> {
    let query = last_word(before).strip_prefix('#')?;
    (!query.contains('#')).then_some(query)
}

/// `BotCommand` (MSC4332): a command a bot of the room announces.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotCommand {
    pub bot_user_id: String,
    /// The first word of the syntax: `gif`.
    pub name: String,
    pub syntax: String,
    pub description: String,
    pub argument_names: Vec<String>,
    pub is_last_argument_variadic: bool,
}

/// `BOT_ID_PATTERN`: `@bot_…` or `@hermes_…`.
pub fn is_bot_id(user_id: &str) -> bool {
    user_id.starts_with("@bot_") || user_id.starts_with("@hermes_")
}

/// `findCommandQuery` on the text before the caret: « /query » at the very start.
pub fn find_command_query(before: &str) -> Option<&str> {
    let query = before.strip_prefix('/')?;
    (!query.chars().any(|c| is_js_space(c) || c == '/')).then_some(query)
}

/// `matchCommands`: the commands whose name starts with the query, in order.
pub fn match_commands<'a>(commands: &'a [BotCommand], query: &str) -> Vec<&'a BotCommand> {
    let wanted = query.to_lowercase();
    commands
        .iter()
        .filter(|c| c.name.to_lowercase().starts_with(&wanted))
        .collect()
}

/// `^\/([^\s/]+)(?:\s+|$)`: the command name and the rest after the spaces.
fn split_command(text: &str) -> Option<(&str, &str)> {
    let body = text.strip_prefix('/')?;
    let end = body
        .find(|c: char| is_js_space(c) || c == '/')
        .unwrap_or(body.len());
    if end == 0 {
        return None;
    }
    let (name, after) = body.split_at(end);
    let rest = after.trim_start_matches(is_js_space);
    if rest.len() == after.len() && !after.is_empty() {
        // A « / » right after the name: not a command
        return None;
    }
    Some((name, rest))
}

/// `LocalCommand`: a command that runs on this device and sends nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalCommand {
    DiscardSession,
}

/// `findLocalCommand`.
pub fn find_local_command(text: &str) -> Option<LocalCommand> {
    let (name, _) = split_command(trim(text))?;
    (name.to_lowercase() == "discardsession").then_some(LocalCommand::DiscardSession)
}

/// `PreparedText`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreparedText {
    pub text: String,
    pub command: Option<BotCommand>,
    /// `/me`: an action of the sender (`m.emote`).
    pub is_emote: bool,
    /// A « / » command nobody knows: not sent, the user is told.
    pub unknown_command: Option<String>,
}

const SHRUG: &str = "¯\\_(ツ)_/¯";

/// `prepareCommandText`: a known bot command goes out with « ! »; `/me` and `/shrug`
/// are the client's; another « /word » is a mistake; « // » sends one « / ».
pub fn prepare_command_text(text: &str, commands: &[BotCommand]) -> PreparedText {
    let message = |body: &str| PreparedText {
        text: body.to_string(),
        command: None,
        is_emote: false,
        unknown_command: None,
    };
    if let Some(rest) = text.strip_prefix("//") {
        return message(&format!("/{rest}"));
    }
    let Some((name, rest)) = split_command(text) else {
        return message(text);
    };
    let name = name.to_lowercase();
    if let Some(command) = commands.iter().find(|c| c.name.to_lowercase() == name) {
        return PreparedText {
            command: Some(command.clone()),
            ..message(&format!("!{}", &text[1..]))
        };
    }
    if name == "me" && !rest.is_empty() {
        return PreparedText {
            is_emote: true,
            ..message(rest)
        };
    }
    if name == "shrug" {
        return message(&if rest.is_empty() {
            SHRUG.to_string()
        } else {
            format!("{rest} {SHRUG}")
        });
    }
    PreparedText {
        unknown_command: Some(name),
        ..message(text)
    }
}

/// Why `prepareOutgoing` refuses a text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutgoingError {
    EmptyMessage,
    UnknownCommand,
}

/// `OutgoingMessage`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingMessage {
    pub error: Option<OutgoingError>,
    pub text: String,
    pub mentions: Vec<Mention>,
    pub command: Option<BotCommand>,
    pub is_emote: bool,
    pub unknown_command: Option<String>,
}

/// `prepareOutgoing`: trimmed, with its client command read and the mentions it
/// still names.
pub fn prepare_outgoing(
    text: &str,
    mentions: &[Mention],
    commands: &[BotCommand],
) -> OutgoingMessage {
    let trimmed = trim(text);
    let prepared = prepare_command_text(trimmed, commands);
    let error = if trimmed.is_empty() {
        Some(OutgoingError::EmptyMessage)
    } else if prepared.unknown_command.is_some() {
        Some(OutgoingError::UnknownCommand)
    } else {
        None
    };
    OutgoingMessage {
        error,
        mentions: keep_present_mentions(&prepared.text, mentions),
        text: prepared.text,
        command: prepared.command,
        is_emote: prepared.is_emote,
        unknown_command: prepared.unknown_command,
    }
}

static EMOJI_START: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\p{Extended_Pictographic}|\p{Emoji_Presentation}|[0-9#*]\x{FE0F}?\x{20E3})")
        .expect("valid pattern")
});

/// `isJumboEmoji`: one emoji and nothing else (spaces allowed), drawn big (D226).
pub fn is_jumbo_emoji(text: &str) -> bool {
    let mut graphemes = trim(text).graphemes(true);
    match (graphemes.next(), graphemes.next()) {
        (Some(first), None) => EMOJI_START.is_match(first),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emoji_queries() {
        assert_eq!(find_emoji_query("hi :sm"), Some("sm"));
        assert_eq!(find_emoji_query(":x"), Some("x"));
        assert_eq!(find_emoji_query("12:30"), None);
        assert_eq!(find_emoji_query("http://"), None);
        assert_eq!(find_emoji_query("a :"), None);
        assert_eq!(find_mention_query("hi @bo"), Some("bo"));
        assert_eq!(find_mention_query("@@bob:x"), Some("@bob:x"));
        assert_eq!(find_mention_query("a@b"), None);
        assert_eq!(find_mention_query("hi @"), Some(""));
        assert_eq!(find_room_query("see #gen"), Some("gen"));
        assert_eq!(find_room_query("x#gen"), None);
        let found: Vec<_> = match_emojis("smile").iter().map(|e| e.shortcode).collect();
        assert_eq!(found, vec!["smile", "smiley", "sweat_smile"]);
    }

    #[test]
    fn client_commands() {
        let p = prepare_command_text("/me waves", &[]);
        assert!(p.is_emote);
        assert_eq!(p.text, "waves");
        assert_eq!(
            prepare_command_text("/shrug ok", &[]).text,
            "ok ¯\\_(ツ)_/¯"
        );
        assert_eq!(prepare_command_text("//etc", &[]).text, "/etc");
        assert_eq!(
            prepare_command_text("/nope", &[])
                .unknown_command
                .as_deref(),
            Some("nope")
        );
        assert_eq!(prepare_command_text("/a/b", &[]).text, "/a/b");
        assert_eq!(
            find_local_command("  /discardSession now"),
            Some(LocalCommand::DiscardSession)
        );
        assert_eq!(find_command_query("/gi"), Some("gi"));
        assert_eq!(find_command_query("/gif x"), None);
    }

    #[test]
    fn jumbo() {
        assert!(is_jumbo_emoji(" 👍🏽 "));
        assert!(is_jumbo_emoji("👨‍👩‍👧"));
        assert!(!is_jumbo_emoji("👍 ok"));
        assert!(!is_jumbo_emoji("a"));
    }
}
