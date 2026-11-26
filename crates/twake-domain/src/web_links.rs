//! The web addresses of a text (web `domain/webLinks.ts`): `http(s)://` runs, `www.` at
//! the start of a word, and bare domains with a known ending (linagora.com/path), as
//! Slack and WhatsApp link them. The end of a sentence is not the end of its link.
//!
//! The web's pattern uses lookbehind, hence `fancy-regex`. Its classes are spelled out:
//! `\w` of JavaScript (no `u` flag) is ASCII, `\s` is the JavaScript set, and the `i`
//! flag folds ASCII only, so each letter of a fixed word is written `[aA]` (Unicode
//! folding would match the Kelvin sign for `k` and `ſ` for `s`).

use std::sync::LazyLock;

use fancy_regex::Regex;

/// Endings of a bare domain (a short list, not the public suffix list; endings that are
/// also file extensions stay out, so README.md or run.sh is no link).
const TLD: &str = "com|org|net|edu|gov|int|info|biz|io|app|dev|ai|co|me|tv|fm|ly|gg|xyz|\
online|site|tech|cloud|shop|store|blog|news|social|chat|space|\
fr|be|ch|ca|lu|mc|de|at|nl|uk|ie|es|pt|it|eu|se|no|dk|fi|is|cz|gr|ro|\
us|br|ar|mx|jp|cn|kr|in|au|nz|ru|ua|vn|sg|tn|ma|dz|sn|ci|cm";

/// The JavaScript `\s` set, inside a class.
const SPACE: &str = r"\t\n\x0B\x0C\r    -     　﻿";

/// `word` matching either case of its ASCII letters.
fn any_case(word: &str) -> String {
    word.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                format!("[{}{}]", c.to_ascii_lowercase(), c.to_ascii_uppercase())
            } else if c == '|' {
                "|".to_string()
            } else {
                regex_escape(c)
            }
        })
        .collect()
}

fn regex_escape(c: char) -> String {
    if r"\.+*?()|[]{}^$#&-~".contains(c) {
        format!("\\{c}")
    } else {
        c.to_string()
    }
}

static URL: LazyLock<Option<Regex>> = LazyLock::new(|| {
    let body = format!(r#"[^{SPACE}<>"']*[^{SPACE}<>"'.,;:!?)\]]"#);
    let http = any_case("http");
    let s = any_case("s");
    let pattern = format!(
        r#"(?:{http}{s}?://|(?<![A-Za-z0-9_@./-]){www}\.){body}|(?<![A-Za-z0-9_@./:-])(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?\.)+(?:{tld})(?![A-Za-z0-9_@-]|\.[A-Za-z0-9_-])(?:[/?#](?:{body})?)?"#,
        www = any_case("www"),
        tld = any_case(TLD),
    );
    Regex::new(&pattern).ok()
});

/// A web address of a text, as written, and the byte where it starts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebLink {
    pub index: usize,
    pub url: String,
}

/// `findWebLinks`: the web addresses of `text`.
pub fn find_web_links(text: &str) -> Vec<WebLink> {
    let Some(pattern) = URL.as_ref() else {
        return Vec::new();
    };
    let mut found = Vec::new();
    let mut end = 0;
    for m in pattern.find_iter(text) {
        let Ok(m) = m else { break };
        if m.start() < end {
            continue;
        }
        let mut url = m.as_str().to_string();
        // A closing parenthesis belongs to the address when it closes one in it
        let mut next = m.end();
        while text[next..].starts_with(')') && url.matches('(').count() > url.matches(')').count() {
            url.push(')');
            next += 1;
        }
        end = m.start() + url.len();
        found.push(WebLink {
            index: m.start(),
            url,
        });
    }
    found
}

/// `toHref`: what a browser opens for an address, `www.` or a bare domain getting https.
pub fn to_href(url: &str) -> String {
    let lower = url.get(..8).unwrap_or(url).to_ascii_lowercase();
    if lower.starts_with("https://") || lower.starts_with("http://") {
        url.to_string()
    } else {
        format!("https://{url}")
    }
}

/// `extractLinks`: the web links of a text, as the timeline draws them.
pub fn extract_links(text: &str) -> Vec<String> {
    find_web_links(text)
        .iter()
        .map(|l| to_href(&l.url))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn urls(text: &str) -> Vec<String> {
        find_web_links(text).into_iter().map(|l| l.url).collect()
    }

    #[test]
    fn builds_the_pattern() {
        assert!(URL.is_some());
    }

    #[test]
    fn finds_links_as_the_web() {
        assert_eq!(urls("see https://a.b/c."), ["https://a.b/c"]);
        assert_eq!(urls("www.linagora.com, now"), ["www.linagora.com"]);
        assert_eq!(urls("go to linagora.com/path?"), ["linagora.com/path"]);
        assert_eq!(urls("README.md and run.sh"), Vec::<String>::new());
        assert_eq!(urls("@bob:linagora.com"), Vec::<String>::new());
        assert_eq!(urls("facebook.community"), Vec::<String>::new());
        assert_eq!(
            urls("(https://en.wikipedia.org/wiki/A_(b))"),
            ["https://en.wikipedia.org/wiki/A_(b)"]
        );
        // The Kelvin sign is no ASCII k
        assert_eq!(urls("\u{212A}r.com x.\u{212A}r"), ["r.com"]);
    }

    #[test]
    fn hrefs() {
        assert_eq!(to_href("www.a.com"), "https://www.a.com");
        assert_eq!(to_href("HTTP://a.com"), "HTTP://a.com");
    }
}
