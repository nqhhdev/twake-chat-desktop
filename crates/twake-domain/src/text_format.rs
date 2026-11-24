//! The formats of the composer bar (web `ds/Composer/applyFormat.ts`), as in Slack: the
//! Markdown stays in the text; a format writes its syntax around the selection and takes
//! it off when it is already there. Offsets are UTF-8 bytes of `text` (the web counts
//! UTF-16 units; the composer field of the desktop gives bytes).

use std::sync::LazyLock;

use regex::Regex;

use crate::js::{is_js_space, trim};

/// A format of the bar, in its order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextFormat {
    Bold,
    Italic,
    Underline,
    Strike,
    OrderedList,
    BulletList,
    Quote,
    Code,
    CodeBlock,
}

impl TextFormat {
    /// The name the web gives it (`bold`, `code_block`…).
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "bold" => Self::Bold,
            "italic" => Self::Italic,
            "underline" => Self::Underline,
            "strike" => Self::Strike,
            "ordered_list" => Self::OrderedList,
            "bullet_list" => Self::BulletList,
            "quote" => Self::Quote,
            "code" => Self::Code,
            "code_block" => Self::CodeBlock,
            _ => return None,
        })
    }
}

/// Replace `from`–`to` of the text by `insert`, then select `selection`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormatChange {
    pub from: usize,
    pub to: usize,
    pub insert: String,
    pub selection: (usize, usize),
}

impl FormatChange {
    /// The text once changed.
    pub fn apply(&self, text: &str) -> String {
        format!("{}{}{}", &text[..self.from], self.insert, &text[self.to..])
    }
}

const FENCE: &str = "```";

fn trim_start(s: &str) -> &str {
    s.trim_start_matches(is_js_space)
}

fn trim_end(s: &str) -> &str {
    s.trim_end_matches(is_js_space)
}

fn wrap(text: &str, start: usize, end: usize, open: &str, close: &str) -> FormatChange {
    // Spaces stay outside the marks: « **bold **» would not be bold
    let selected = &text[start..end];
    let is_blank = trim(selected).is_empty();
    let from = start + selected.len() - trim_start(selected).len();
    let to = if is_blank {
        from
    } else {
        end - selected.len() + trim_end(selected).len()
    };
    let inner = &text[from..to];
    if from >= open.len()
        && text.get(from - open.len()..from) == Some(open)
        && text.get(to..to + close.len()) == Some(close)
    {
        let at = from - open.len();
        return FormatChange {
            from: at,
            to: to + close.len(),
            insert: inner.to_string(),
            selection: (at, at + inner.len()),
        };
    }
    if inner.len() >= open.len() + close.len() && inner.starts_with(open) && inner.ends_with(close)
    {
        let bare = &inner[open.len()..inner.len() - close.len()];
        return FormatChange {
            from,
            to,
            insert: bare.to_string(),
            selection: (from, from + bare.len()),
        };
    }
    FormatChange {
        from,
        to,
        insert: format!("{open}{inner}{close}"),
        selection: (from + open.len(), to + open.len()),
    }
}

static QUOTE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^> ?").unwrap_or_else(|_| never()));
static BULLET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[-*] +").unwrap_or_else(|_| never()));
// `\d` of JavaScript is ASCII
static ORDERED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[0-9]+[.)] +").unwrap_or_else(|_| never()));

// INFALLIBLE: the patterns above are constant and valid (the tests build them).
fn never() -> Regex {
    unreachable!("constant pattern")
}

fn prefix_lines(
    text: &str,
    start: usize,
    end: usize,
    pattern: &Regex,
    prefix: impl Fn(usize) -> String,
) -> FormatChange {
    let from = if start == 0 {
        0
    } else {
        text[..start].rfind('\n').map_or(0, |i| i + 1)
    };
    // A selection that ends at the start of a line leaves that line alone
    let last = if end > start && text.as_bytes()[end - 1] == b'\n' {
        end - 1
    } else {
        end
    };
    let to = text[last..].find('\n').map_or(text.len(), |i| last + i);
    let lines: Vec<&str> = text[from..to].split('\n').collect();
    // Blank lines between the selected ones stay blank
    let is_skipped = |line: &str| lines.len() > 1 && trim(line).is_empty();
    let is_on = lines
        .iter()
        .all(|line| is_skipped(line) || pattern.is_match(line));
    let mut count = 0;
    let insert = lines
        .iter()
        .map(|line| {
            if is_skipped(line) {
                return line.to_string();
            }
            let bare = pattern.replace(line, "");
            if is_on {
                return bare.into_owned();
            }
            count += 1;
            format!("{}{bare}", prefix(count - 1))
        })
        .collect::<Vec<_>>()
        .join("\n");
    let block_end = from + insert.len();
    FormatChange {
        from,
        to,
        insert,
        selection: if start == end {
            (block_end, block_end)
        } else {
            (from, block_end)
        },
    }
}

fn code_block(text: &str, start: usize, end: usize) -> FormatChange {
    let selected = &text[start..end];
    let open = format!("{FENCE}\n");
    let close = format!("\n{FENCE}");
    if let Some(at) = start.checked_sub(open.len())
        && (at == 0 || text.as_bytes()[at - 1] == b'\n')
        && text.get(at..start) == Some(open.as_str())
        && text.get(end..end + close.len()) == Some(close.as_str())
    {
        return FormatChange {
            from: at,
            to: end + close.len(),
            insert: selected.to_string(),
            selection: (at, at + selected.len()),
        };
    }
    // The fences sit on lines of their own
    let before = if start > 0 && text.as_bytes()[start - 1] != b'\n' {
        "\n"
    } else {
        ""
    };
    let after = if end < text.len() && text.as_bytes()[end] != b'\n' {
        "\n"
    } else {
        ""
    };
    let code_start = start + before.len() + open.len();
    FormatChange {
        from: start,
        to: end,
        insert: format!("{before}{open}{selected}{close}{after}"),
        selection: (code_start, code_start + selected.len()),
    }
}

/// What `format` does to the text selected from `start` to `end`.
pub fn apply_format(text: &str, start: usize, end: usize, format: TextFormat) -> FormatChange {
    match format {
        TextFormat::Bold => wrap(text, start, end, "**", "**"),
        TextFormat::Italic => wrap(text, start, end, "_", "_"),
        TextFormat::Underline => wrap(text, start, end, "<u>", "</u>"),
        TextFormat::Strike => wrap(text, start, end, "~~", "~~"),
        TextFormat::Code => wrap(text, start, end, "`", "`"),
        TextFormat::CodeBlock => code_block(text, start, end),
        TextFormat::Quote => prefix_lines(text, start, end, &QUOTE, |_| "> ".into()),
        TextFormat::BulletList => prefix_lines(text, start, end, &BULLET, |_| "- ".into()),
        TextFormat::OrderedList => {
            prefix_lines(text, start, end, &ORDERED, |i| format!("{}. ", i + 1))
        }
    }
}

/// What the « Add a link » dialog starts from, and the range it replaces.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkTarget {
    pub from: usize,
    pub to: usize,
    pub text: String,
    pub url: String,
}

// `\s` of JavaScript spelled out (Rust's adds U+0085 and lacks U+FEFF)
static LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^\]\n]*)\]\(([^)\t\n\x0B\x0C\r    -     　﻿]*)\)").unwrap_or_else(|_| never())
});

/// The dialog of a link: the Markdown link `[t](u)` the selection touches (it is
/// replaced), else the selection itself as text.
pub fn link_target(text: &str, start: usize, end: usize) -> LinkTarget {
    for found in LINK.captures_iter(text) {
        let whole = found.get(0).map_or(0..0, |m| m.range());
        if start >= whole.start && end <= whole.end {
            return LinkTarget {
                from: whole.start,
                to: whole.end,
                text: found.get(1).map_or("", |m| m.as_str()).to_string(),
                url: found.get(2).map_or("", |m| m.as_str()).to_string(),
            };
        }
    }
    LinkTarget {
        from: start,
        to: end,
        text: text[start..end].to_string(),
        url: String::new(),
    }
}

/// The address to link to (`www.` gets https://), None when not a web one.
pub fn normalize_url(input: &str) -> Option<String> {
    let typed = trim(input);
    let url = if typed.len() >= 4 && typed[..4].eq_ignore_ascii_case("www.") {
        format!("https://{typed}")
    } else {
        typed.to_string()
    };
    if url.chars().any(|c| is_js_space(c) || c == '(' || c == ')') {
        return None;
    }
    let parsed = url::Url::parse(&url).ok()?;
    let is_web = matches!(parsed.scheme(), "http" | "https");
    (is_web && parsed.host_str().is_some_and(|h| !h.is_empty())).then_some(url)
}

/// The link written over `from`–`to`, the caret after it.
pub fn link_change(from: usize, to: usize, label: &str, url: &str) -> FormatChange {
    let text = if trim(label).is_empty() { url } else { label };
    let insert = format!("[{text}]({url})");
    let end = from + insert.len();
    FormatChange {
        from,
        to,
        insert,
        selection: (end, end),
    }
}

/// A single web address pasted over a selection makes it a link, as in Slack; None for
/// any other paste, which stays a plain one.
pub fn paste_link(text: &str, start: usize, end: usize, pasted: &str) -> Option<FormatChange> {
    let url = trim(pasted);
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    if start == end || rest.is_empty() || rest.chars().any(is_js_space) {
        return None;
    }
    let insert = format!("[{}]({url})", &text[start..end]);
    let caret = start + insert.len();
    Some(FormatChange {
        from: start,
        to: end,
        insert,
        selection: (caret, caret),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The web spec's helper: the text after the change, « [ » and « ] » around the
    /// selection.
    fn run(marked: &str, format: &str) -> String {
        let start = marked.find('[').unwrap_or(0);
        let end = marked.find(']').unwrap_or(0) - 1;
        let text = marked.replacen('[', "", 1).replacen(']', "", 1);
        let change = apply_format(&text, start, end, TextFormat::from_name(format).unwrap());
        let next = change.apply(&text);
        let (from, to) = change.selection;
        format!("{}[{}]{}", &next[..from], &next[from..to], &next[to..])
    }

    #[test]
    fn wraps_the_selection_spaces_left_outside() {
        assert_eq!(run("say [hello ]now", "bold"), "say **[hello]** now");
        assert_eq!(run("[it]", "italic"), "_[it]_");
        assert_eq!(run("[old]", "strike"), "~~[old]~~");
        assert_eq!(run("[x]", "underline"), "<u>[x]</u>");
        assert_eq!(run("[a < b]", "code"), "`[a < b]`");
        assert_eq!(run("a []b", "bold"), "a **[]**b");
    }

    #[test]
    fn takes_the_marks_off_inside_or_around() {
        assert_eq!(run("**[bold]**", "bold"), "[bold]");
        assert_eq!(run("[**bold**]", "bold"), "[bold]");
    }

    #[test]
    fn prefixes_lines() {
        assert_eq!(run("[one\ntwo]", "bullet_list"), "[- one\n- two]");
        assert_eq!(run("[one\n\ntwo]", "ordered_list"), "[1. one\n\n2. two]");
        assert_eq!(run("a\nqu[]ote", "quote"), "a\n> quote[]");
        assert_eq!(run("[> one\n> two]", "quote"), "[one\ntwo]");
        assert_eq!(run("[1. one\n2. two]", "ordered_list"), "[one\ntwo]");
    }

    #[test]
    fn code_blocks() {
        assert_eq!(
            run("see [if (a) {\n  b()\n}] now", "code_block"),
            "see \n```\n[if (a) {\n  b()\n}]\n```\n now"
        );
        assert_eq!(run("[]", "code_block"), "```\n[]\n```");
        assert_eq!(run("```\n[x = 1]\n```", "code_block"), "[x = 1]");
    }

    #[test]
    fn pasted_links() {
        assert_eq!(
            paste_link("see docs now", 4, 8, " https://a.b/c\n"),
            Some(FormatChange {
                from: 4,
                to: 8,
                insert: "[docs](https://a.b/c)".into(),
                selection: (25, 25)
            })
        );
        assert_eq!(paste_link("see docs", 4, 4, "https://a.b"), None);
        assert_eq!(paste_link("see docs", 4, 8, "read https://a.b"), None);
        assert_eq!(
            paste_link("see docs", 4, 8, "https://a.b https://c.d"),
            None
        );
        assert_eq!(paste_link("see docs", 4, 8, "javascript:alert(1)"), None);
    }

    #[test]
    fn link_targets() {
        assert_eq!(
            link_target("see docs now", 4, 8),
            LinkTarget {
                from: 4,
                to: 8,
                text: "docs".into(),
                url: String::new()
            }
        );
        let text = "a [docs](https://a.b) z";
        let found = LinkTarget {
            from: 2,
            to: 21,
            text: "docs".into(),
            url: "https://a.b".into(),
        };
        assert_eq!(link_target(text, 5, 5), found);
        assert_eq!(link_target(text, 3, 7), found);
        assert_eq!(link_target(text, 0, 0).url, "");
        assert_eq!(link_target(text, 1, 5).url, "");
    }

    #[test]
    fn normalizes_urls() {
        assert_eq!(
            normalize_url(" https://a.b/c ").as_deref(),
            Some("https://a.b/c")
        );
        assert_eq!(normalize_url("http://a.b").as_deref(), Some("http://a.b"));
        assert_eq!(normalize_url("www.a.b").as_deref(), Some("https://www.a.b"));
        for bad in [
            "",
            "a.b",
            "javascript:alert(1)",
            "https://",
            "https://a b",
            "https://a.b/)",
        ] {
            assert_eq!(normalize_url(bad), None, "{bad}");
        }
    }

    #[test]
    fn link_changes() {
        assert_eq!(
            link_change(4, 8, "docs", "https://a.b"),
            FormatChange {
                from: 4,
                to: 8,
                insert: "[docs](https://a.b)".into(),
                selection: (23, 23)
            }
        );
        assert_eq!(
            link_change(0, 0, " ", "https://a.b").insert,
            "[https://a.b](https://a.b)"
        );
    }
}
