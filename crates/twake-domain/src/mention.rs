//! Port of `src/domain/mention.ts`: people and rooms named in a message.

use serde::{Deserialize, Serialize};

use crate::js::{is_js_space, locale_compare, trim};
use crate::search::normalize_for_search;

/// `Mention`: a person named in a message, or a room linked in it (`user_id` is then
/// its alias or id, see [`is_room_mention`]).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub user_id: String,
    /// As written in the text.
    pub name: String,
}

/// What `matchMembers` reads of a member.
pub trait Mentionable {
    fn user_id(&self) -> &str;
    fn name(&self) -> &str;
    fn matrix_name(&self) -> Option<&str>;
}

impl Mentionable for crate::room_management::RoomMemberDetail {
    fn user_id(&self) -> &str {
        &self.user_id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn matrix_name(&self) -> Option<&str> {
        self.matrix_name.as_deref()
    }
}

/// `text.split(/[\s:._-]+/)`, keeping the empty strings JavaScript keeps at the ends.
fn words(text: &str) -> Vec<&str> {
    let separator = |c: char| is_js_space(c) || matches!(c, ':' | '.' | '_' | '-');
    let mut out = Vec::new();
    let mut start = 0;
    let mut in_run = false;
    for (index, c) in text.char_indices() {
        if separator(c) {
            if !in_run {
                out.push(&text[start..index]);
                in_run = true;
            }
        } else if in_run {
            start = index;
            in_run = false;
        }
    }
    out.push(if in_run { "" } else { &text[start..] });
    out
}

/// 0 when a word starts with the query, 1 when one contains it.
fn score(words: &[&str], wanted: &str) -> Option<u8> {
    if words.iter().any(|w| w.starts_with(wanted)) {
        Some(0)
    } else if words.iter().any(|w| w.contains(wanted)) {
        Some(1)
    } else {
        None
    }
}

/// `matchMembers`: the members the user may mean by `@query`, best first. The user is
/// not offered to themselves.
pub fn match_members<'a, T: Mentionable>(
    members: &'a [T],
    query: &str,
    own_user_id: &str,
    limit: usize,
) -> Vec<&'a T> {
    let wanted = normalize_for_search(query);
    let mut scored: Vec<(u8, &T)> = members
        .iter()
        .filter(|m| m.user_id() != own_user_id)
        .filter_map(|member| {
            let folded = normalize_for_search(&format!(
                "{} {} {}",
                member.name(),
                member.matrix_name().unwrap_or(""),
                member.user_id().get(1..).unwrap_or("")
            ));
            let words = words(&folded);
            // A query holding « : » is a Matrix id, which the words cut apart
            let id = member.user_id().to_lowercase();
            let id_tail = id.get(1..).unwrap_or("");
            if words.iter().any(|w| w.starts_with(&wanted))
                || id.starts_with(&wanted)
                || id_tail.starts_with(&wanted)
            {
                Some((0, member))
            } else if words.iter().any(|w| w.contains(&wanted)) || id.contains(&wanted) {
                Some((1, member))
            } else {
                None
            }
        })
        .collect();
    scored.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| locale_compare(a.1.name(), b.1.name()))
    });
    scored.into_iter().take(limit).map(|(_, m)| m).collect()
}

/// `mentionText`: the Matrix name, never the name the user gave the person (D302).
pub fn mention_text<'a>(name: &'a str, matrix_name: Option<&'a str>) -> &'a str {
    matrix_name.unwrap_or(name)
}

/// `keepPresentMentions`: the mentions picked while writing that the text still
/// holds; each person once.
pub fn keep_present_mentions(text: &str, mentions: &[Mention]) -> Vec<Mention> {
    let mut kept: Vec<Mention> = Vec::new();
    for mention in mentions {
        if text.contains(&mention.name) && !kept.iter().any(|k| k.user_id == mention.user_id) {
            kept.push(mention.clone());
        }
    }
    kept
}

/// `LinkableRoom`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkableRoom {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub alias: Option<String>,
}

/// `matchRooms`: the rooms the user may mean by `#query`, best first.
pub fn match_rooms<'a>(
    rooms: &'a [LinkableRoom],
    query: &str,
    limit: usize,
) -> Vec<&'a LinkableRoom> {
    let wanted = normalize_for_search(query);
    let mut scored: Vec<(u8, &LinkableRoom)> = rooms
        .iter()
        .filter_map(|room| {
            let alias = room.alias.as_deref().and_then(|a| a.get(1..)).unwrap_or("");
            let folded = normalize_for_search(&format!("{} {}", room.name, alias));
            score(&words(&folded), &wanted).map(|s| (s, room))
        })
        .collect();
    scored.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| locale_compare(&a.1.name, &b.1.name))
    });
    scored.into_iter().take(limit).map(|(_, r)| r).collect()
}

/// `roomMentionRef`: the alias when there is one, else the id.
pub fn room_mention_ref(room: &LinkableRoom) -> &str {
    room.alias.as_deref().unwrap_or(&room.id)
}

/// `roomMentionText`: the alias, else « # » and the name.
pub fn room_mention_text(room: &LinkableRoom) -> String {
    room.alias
        .clone()
        .unwrap_or_else(|| format!("#{}", room.name))
}

/// `isRoomMention`.
pub fn is_room_mention(mention: &Mention) -> bool {
    mention.user_id.starts_with('#') || mention.user_id.starts_with('!')
}

/// `EVERYONE_MENTION` (`m.mentions.room`).
pub const EVERYONE: &str = "@room";

/// `isEveryoneMention`.
pub fn is_everyone_mention(mention: &Mention) -> bool {
    mention.user_id == EVERYONE
}

/// `offersEveryone`: « @room » first once the user types a word of it, « all » or
/// « everyone »; never for a bare « @ ».
pub fn offers_everyone(query: &str) -> bool {
    let wanted = normalize_for_search(query);
    !wanted.is_empty()
        && ["room", "all", "everyone"]
            .iter()
            .any(|w| w.starts_with(&wanted))
}

/// `withEveryone`: « @room » first when asked by name, last after a bare « @ ».
pub fn with_everyone<T: Clone>(
    people: &[T],
    everyone: T,
    query: &str,
    can_mention_everyone: bool,
) -> Vec<T> {
    if !can_mention_everyone {
        return people.to_vec();
    }
    if offers_everyone(query) {
        return std::iter::once(everyone)
            .chain(people.iter().cloned())
            .collect();
    }
    if trim(query).is_empty() && !people.is_empty() {
        return people
            .iter()
            .cloned()
            .chain(std::iter::once(everyone))
            .collect();
    }
    people.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_like_javascript() {
        assert_eq!(words("a b"), vec!["a", "b"]);
        assert_eq!(words(" a"), vec!["", "a"]);
        assert_eq!(words("a:"), vec!["a", ""]);
        assert_eq!(words(""), vec![""]);
    }

    #[test]
    fn everyone_first_by_name_last_after_bare_at() {
        assert_eq!(
            with_everyone(&["bob"], "@room", "ro", true),
            vec!["@room", "bob"]
        );
        assert_eq!(
            with_everyone(&["bob"], "@room", "", true),
            vec!["bob", "@room"]
        );
        assert_eq!(with_everyone(&["bob"], "@room", "b", true), vec!["bob"]);
    }
}
