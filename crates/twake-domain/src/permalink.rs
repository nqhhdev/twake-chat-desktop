//! Links to a person, a room or a message (web `domain/permalink.ts`, D157): the matrix.to
//! links any Matrix client opens.

/// The base of every matrix.to link.
const BASE: &str = "https://matrix.to/#/";

/// `encodeURIComponent`: every byte of UTF-8 escaped but letters, digits and `-_.!~*'()`.
pub fn encode_uri_component(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'-'
            | b'_'
            | b'.'
            | b'!'
            | b'~'
            | b'*'
            | b'\''
            | b'('
            | b')' => out.push(char::from(byte)),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

/// A link any Matrix client opens at a person (web `userPermalink`).
pub fn user_permalink(user_id: &str) -> String {
    format!("{BASE}{}", encode_uri_component(user_id))
}

/// A link any Matrix client opens at the room: its alias or its id (web `roomPermalink`).
pub fn room_permalink(alias_or_id: &str) -> String {
    format!("{BASE}{}", encode_uri_component(alias_or_id))
}

/// A link any Matrix client opens at a message. The web copies a link to itself (`messageLink`,
/// its own address); an app has none, so it copies the matrix.to link that the web's
/// `parsePermalink` reads back (ADR-020).
pub fn message_permalink(room_id: &str, event_id: &str) -> String {
    format!(
        "{}/{}",
        room_permalink(room_id),
        encode_uri_component(event_id)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_as_encode_uri_component() {
        assert_eq!(encode_uri_component("hello world"), "hello%20world");
        assert_eq!(encode_uri_component("!a:b.c"), "!a%3Ab.c");
        assert_eq!(encode_uri_component("é"), "%C3%A9");
        assert_eq!(encode_uri_component("-_.!~*'()"), "-_.!~*'()");
    }

    #[test]
    fn links_to_matrix_to() {
        assert_eq!(user_permalink("@a:b"), "https://matrix.to/#/%40a%3Ab");
        assert_eq!(
            message_permalink("!r:s", "$e"),
            "https://matrix.to/#/!r%3As/%24e"
        );
    }
}
