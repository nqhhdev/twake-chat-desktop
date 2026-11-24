//! Port of `src/domain/search.ts`: folding and matching of the local search.

use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

use crate::js::is_js_space;

static DIACRITIC: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\p{Diacritic}").expect("valid pattern"));
static LETTER_OR_NUMBER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[\p{L}\p{N}]$").expect("valid pattern"));

/// `normalizeForSearch`: lower case, without accents (« Réunion » is « reunion »).
pub fn normalize_for_search(text: &str) -> String {
    let decomposed: String = text.nfd().collect();
    DIACRITIC.replace_all(&decomposed, "").to_lowercase()
}

/// `isClosedQuery`: the user closed the query with a space (D392).
pub fn is_closed_query(query: &str) -> bool {
    query.chars().next_back().is_some_and(is_js_space)
}

/// `[\p{L}\p{N}]` of a JavaScript `u` regex.
pub fn is_letter_or_number(c: char) -> bool {
    let mut buffer = [0; 4];
    LETTER_OR_NUMBER.is_match(c.encode_utf8(&mut buffer))
}

/// `holdsWord`: whether the folded `haystack` holds the folded `word`; `whole`: only
/// as a whole word, not inside a longer one (« ann » is not in « anna »).
pub fn holds_word(haystack: &str, word: &str, whole: bool) -> bool {
    if !whole {
        return haystack.contains(word);
    }
    let boundaries = haystack
        .char_indices()
        .map(|(index, _)| index)
        .chain(std::iter::once(haystack.len()));
    for start in boundaries {
        if !haystack[start..].starts_with(word) {
            continue;
        }
        let before = haystack[..start].chars().next_back();
        let after = haystack[start + word.len()..].chars().next();
        if !before.is_some_and(is_letter_or_number) && !after.is_some_and(is_letter_or_number) {
            return true;
        }
    }
    false
}

/// `IndexedMessage`: what the local search keeps of a message, never sent anywhere.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedMessage {
    pub room_id: String,
    pub message_id: String,
    pub sender_id: String,
    pub sender_name: String,
    /// The text, or the caption and the name of the file.
    pub text: String,
    pub sent_at: f64,
    pub has_file: bool,
}

/// `SearchFilter`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilter {
    /// Words that must all be in the text; empty: any text.
    pub query: String,
    pub sender_id: Option<String>,
    /// Epoch milliseconds; `None`: no bound.
    pub since: Option<f64>,
    pub has_file: bool,
}

/// `toIndexedMessage`: nothing to find in what cannot be read; a message not on the
/// server yet waits for its real id. Also leaves out what the web's feed leaves out
/// before calling it (`isShownMessage`): a deleted or unsupported message, a local echo.
pub fn to_indexed_message(room_id: &str, message: &crate::Message) -> Option<IndexedMessage> {
    if message.undecryptable.is_some() || message.notice.is_some() || message.id.starts_with('~') {
        return None;
    }
    if message
        .send_status
        .is_some_and(|s| s != crate::SendStatus::Sent)
    {
        return None;
    }
    let name = message.attachment.as_ref().map_or("", |a| a.name.as_str());
    Some(IndexedMessage {
        room_id: room_id.to_string(),
        message_id: message.id.clone(),
        sender_id: message.sender_id.clone(),
        sender_name: message.sender_name.clone(),
        text: crate::js::trim(&format!("{} {name}", message.text)).to_string(),
        sent_at: message.sent_at as f64,
        has_file: message.attachment.is_some(),
    })
}

/// `isLocalEchoId`.
pub fn is_local_echo_id(message_id: &str) -> bool {
    message_id.starts_with('~')
}

/// `isEmptyFilter`: a filter that asks for nothing finds nothing.
pub fn is_empty_filter(filter: &SearchFilter) -> bool {
    crate::js::trim(&filter.query).is_empty()
        && filter.sender_id.is_none()
        && filter.since.is_none()
        && !filter.has_file
}

/// `matchesFilter`.
pub fn matches_filter(entry: &IndexedMessage, filter: &SearchFilter) -> bool {
    if filter
        .sender_id
        .as_ref()
        .is_some_and(|sender| *sender != entry.sender_id)
    {
        return false;
    }
    if filter.since.is_some_and(|since| entry.sent_at < since) {
        return false;
    }
    if filter.has_file && !entry.has_file {
        return false;
    }
    let query = normalize_for_search(&filter.query);
    let words: Vec<&str> = query.split(is_js_space).filter(|w| !w.is_empty()).collect();
    let haystack = normalize_for_search(&entry.text);
    let closed = is_closed_query(&filter.query);
    words
        .iter()
        .enumerate()
        .all(|(i, word)| holds_word(&haystack, word, closed && i == words.len() - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_accents_and_case() {
        assert_eq!(normalize_for_search("Réunion"), "reunion");
    }

    #[test]
    fn whole_words_only_when_closed() {
        assert!(holds_word("anna", "ann", false));
        assert!(!holds_word("anna", "ann", true));
        assert!(holds_word("hi ann!", "ann", true));
        let entry = IndexedMessage {
            room_id: "!r".into(),
            message_id: "$e".into(),
            sender_id: "@a".into(),
            sender_name: "a".into(),
            text: "La réunion d'Anna".into(),
            sent_at: 10.0,
            has_file: false,
        };
        let filter = |query: &str| SearchFilter {
            query: query.into(),
            ..Default::default()
        };
        assert!(matches_filter(&entry, &filter("reunion ann")));
        assert!(!matches_filter(&entry, &filter("reunion ann ")));
    }
}
