//! Message bodies to the Markdown subset Slint's `StyledText` draws.
//!
//! `StyledText::from_markdown` (Slint 1.19) accepts paragraphs, line breaks, bold,
//! italics, strikethrough, inline code, links, lists, `<u>` and `<font color>`, and
//! rejects the rest (quotes, headings, code blocks, rules, other HTML). So every text
//! node is escaped (all ASCII punctuation gets a backslash, valid in CommonMark), and the
//! formatted body (`org.matrix.custom.html`, untrusted) is rebuilt from an allow-list:
//! quotes become a bar prefix, headings bold lines, code blocks inline code per line.
//! Anything else keeps only its text. The UI still falls back to plain text if Slint
//! rejects the result.

/// Plain text body to Markdown: escaped, web links made clickable, line breaks kept.
pub fn plain_to_markdown(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    for (index, line) in text.split('\n').enumerate() {
        if index > 0 {
            out.push('\n');
        }
        push_linkified(&mut out, line);
    }
    out
}

/// Formatted body (`org.matrix.custom.html`) to Markdown.
pub fn html_to_markdown(html: &str) -> String {
    let mut w = Writer {
        at_line_start: true,
        ..Writer::default()
    };
    for token in Tokenizer::new(html) {
        match token {
            Token::Text(text) => w.text(&decode_entities(text)),
            Token::Open {
                name,
                attrs,
                self_closing,
            } => {
                w.open(&name, &attrs);
                if self_closing || is_void(&name) {
                    w.close(&name);
                }
            }
            Token::Close(name) => w.close(&name),
        }
    }
    w.finish()
}

// ---------------------------------------------------------------------------
// Writer

#[derive(Default)]
struct Writer {
    /// Finished output.
    out: String,
    /// Inline spans being built (bold, italic, ...): their own buffers.
    spans: Vec<Span>,
    /// Depth inside elements whose content is dropped (`mx-reply`, `script`, `style`).
    skip: usize,
    lists: Vec<Option<u32>>,
    quote: usize,
    pre: usize,
    pre_text: String,
    at_line_start: bool,
}

struct Span {
    tag: String,
    kind: SpanKind,
    buf: String,
}

enum SpanKind {
    Marker(&'static str),
    Html(String, &'static str),
    Link(String),
    Code,
    Plain,
}

impl Writer {
    fn sink(&mut self) -> &mut String {
        match self.spans.last_mut() {
            Some(span) => &mut span.buf,
            None => &mut self.out,
        }
    }

    fn text(&mut self, text: &str) {
        if self.skip > 0 {
            return;
        }
        if self.pre > 0 {
            self.pre_text.push_str(text);
            return;
        }
        // HTML collapses white space, line breaks included.
        let mut collapsed = String::with_capacity(text.len());
        let mut in_space = false;
        for c in text.chars() {
            if c.is_ascii_whitespace() {
                if !in_space {
                    collapsed.push(' ');
                }
                in_space = true;
            } else {
                collapsed.push(c);
                in_space = false;
            }
        }
        if self.spans.is_empty() && self.at_line_start {
            let trimmed = collapsed.trim_start();
            if trimmed.is_empty() {
                return;
            }
            self.line_prefix();
            collapsed = trimmed.to_string();
        }
        let in_code = matches!(
            self.spans.last(),
            Some(Span {
                kind: SpanKind::Code,
                ..
            })
        );
        let in_link = self
            .spans
            .iter()
            .any(|s| matches!(s.kind, SpanKind::Link(_)));
        let sink = self.sink();
        if in_code {
            sink.push_str(&collapsed);
        } else if in_link {
            push_escaped(sink, &collapsed);
        } else {
            push_linkified(sink, &collapsed);
        }
    }

    fn line_prefix(&mut self) {
        self.at_line_start = false;
        if self.quote > 0 {
            for _ in 0..self.quote {
                self.out.push_str("▎ ");
            }
        }
    }

    /// Ends the current line, as a paragraph break when `blank`.
    fn break_line(&mut self, blank: bool) {
        self.flush_spans();
        if self.out.is_empty() {
            self.at_line_start = true;
            return;
        }
        let trimmed = self.out.trim_end_matches(' ').len();
        self.out.truncate(trimmed);
        if blank {
            if !self.out.ends_with("\n\n") {
                if !self.out.ends_with('\n') {
                    self.out.push('\n');
                }
                self.out.push('\n');
            }
        } else if !self.out.ends_with('\n') {
            self.out.push('\n');
        }
        self.at_line_start = true;
    }

    /// Writes open spans out before a block boundary: inline styles never cross lines.
    fn flush_spans(&mut self) {
        while !self.spans.is_empty() {
            let tag = self.spans.last().map(|s| s.tag.clone()).unwrap_or_default();
            self.close_span(&tag);
        }
    }

    fn open(&mut self, name: &str, attrs: &[(String, String)]) {
        let skipped = matches!(name, "mx-reply" | "script" | "style" | "head" | "title");
        if skipped {
            self.skip += 1;
        }
        if self.skip > 0 {
            return;
        }
        if self.pre > 0 {
            if name == "pre" {
                self.pre += 1;
            }
            return;
        }
        let attr = |key: &str| {
            attrs
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.as_str())
        };
        match name {
            "p" | "div" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "table" => {
                self.break_line(true);
                if name.starts_with('h') && name.len() == 2 {
                    self.push_span(name, SpanKind::Marker("**"));
                }
            }
            "tr" => self.break_line(false),
            "td" | "th" => {
                if !self.at_line_start {
                    self.sink().push_str(" \\| ");
                }
            }
            "br" => self.break_line(false),
            "hr" => {
                self.break_line(false);
                self.line_prefix();
                self.out.push_str("———");
                self.break_line(false);
            }
            "blockquote" => {
                self.break_line(true);
                self.quote += 1;
            }
            "ul" | "ol" => {
                self.break_line(self.lists.is_empty());
                let start = attr("start").and_then(|s| s.parse().ok()).unwrap_or(1);
                self.lists
                    .push(if name == "ol" { Some(start) } else { None });
            }
            "li" => {
                self.break_line(false);
                self.line_prefix();
                let depth = self.lists.len().saturating_sub(1);
                for _ in 0..depth {
                    self.out.push_str("   ");
                }
                match self.lists.last_mut() {
                    Some(Some(n)) => {
                        self.out.push_str(&format!("{n}. "));
                        *n += 1;
                    }
                    _ => self.out.push_str("- "),
                }
            }
            "pre" => {
                self.break_line(true);
                self.pre = 1;
                self.pre_text.clear();
            }
            "strong" | "b" => self.push_span(name, SpanKind::Marker("**")),
            "em" | "i" => self.push_span(name, SpanKind::Marker("*")),
            "del" | "s" | "strike" => self.push_span(name, SpanKind::Marker("~~")),
            "u" => self.push_span(name, SpanKind::Html("<u>".into(), "</u>")),
            "code" => self.push_span(name, SpanKind::Code),
            "font" | "span" => {
                let color = attr("data-mx-color")
                    .or_else(|| attr("color"))
                    .and_then(valid_color);
                match color {
                    Some(c) => self.push_span(
                        name,
                        SpanKind::Html(format!("<font color=\"{c}\">"), "</font>"),
                    ),
                    None => self.push_span(name, SpanKind::Plain),
                }
            }
            "a" => match attr("href").filter(|h| is_safe_link(h)) {
                Some(href) => self.push_span(name, SpanKind::Link(href.to_string())),
                None => self.push_span(name, SpanKind::Plain),
            },
            "img" => {
                let alt = attr("alt")
                    .or_else(|| attr("title"))
                    .unwrap_or("image")
                    .to_string();
                self.text(&alt);
            }
            _ => {}
        }
    }

    fn close(&mut self, name: &str) {
        if self.skip > 0 {
            if matches!(name, "mx-reply" | "script" | "style" | "head" | "title") {
                self.skip -= 1;
            }
            return;
        }
        if self.pre > 0 {
            if name == "pre" {
                self.pre -= 1;
                if self.pre == 0 {
                    self.write_code_block();
                }
            }
            return;
        }
        match name {
            "p" | "div" | "table" => self.break_line(true),
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                self.close_span(name);
                self.break_line(true);
            }
            "blockquote" => {
                self.break_line(true);
                self.quote = self.quote.saturating_sub(1);
            }
            "ul" | "ol" => {
                self.lists.pop();
                self.break_line(self.lists.is_empty());
            }
            "li" | "tr" => self.break_line(false),
            _ => self.close_span(name),
        }
    }

    fn push_span(&mut self, tag: &str, kind: SpanKind) {
        if self.spans.is_empty() && self.at_line_start {
            self.line_prefix();
        }
        self.spans.push(Span {
            tag: tag.to_string(),
            kind,
            buf: String::new(),
        });
    }

    fn close_span(&mut self, tag: &str) {
        let Some(position) = self.spans.iter().rposition(|s| s.tag == tag) else {
            return;
        };
        // Close the spans opened inside it too (mis-nested HTML).
        while self.spans.len() > position {
            let Some(span) = self.spans.pop() else { break };
            let rendered = render_span(span);
            self.sink().push_str(&rendered);
        }
    }

    fn write_code_block(&mut self) {
        let text = std::mem::take(&mut self.pre_text);
        let text = decode_entities(&text);
        for line in text.trim_end_matches('\n').split('\n') {
            self.line_prefix();
            if !line.trim().is_empty() {
                self.out.push_str(&inline_code(line));
            }
            self.out.push('\n');
            self.at_line_start = true;
        }
        self.break_line(true);
    }

    fn finish(mut self) -> String {
        self.flush_spans();
        let end = self.out.trim_end().len();
        self.out.truncate(end);
        self.out
    }
}

fn render_span(span: Span) -> String {
    let buf = span.buf;
    match span.kind {
        SpanKind::Plain => buf,
        SpanKind::Code => inline_code(&buf),
        SpanKind::Html(open, close) => {
            if buf.trim().is_empty() {
                buf
            } else {
                format!("{open}{buf}{close}")
            }
        }
        SpanKind::Marker(marker) => {
            // `**bold **` does not close in CommonMark: keep spaces outside markers.
            let core = buf.trim();
            if core.is_empty() {
                return buf;
            }
            let lead = &buf[..buf.len() - buf.trim_start().len()];
            let trail = &buf[buf.trim_end().len()..];
            format!("{lead}{marker}{core}{marker}{trail}")
        }
        SpanKind::Link(href) => {
            let label = if buf.trim().is_empty() {
                escape(&href)
            } else {
                buf
            };
            format!("[{label}]({})", link_destination(&href))
        }
    }
}

// ---------------------------------------------------------------------------
// Escaping and links

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    push_escaped(&mut out, text);
    out
}

/// Backslash before every ASCII punctuation: always valid CommonMark, and it keeps
/// text from turning into lists, headings, quotes or HTML that Slint would reject.
fn push_escaped(out: &mut String, text: &str) {
    for c in text.chars() {
        if c.is_ascii_punctuation() {
            out.push('\\');
        }
        out.push(c);
    }
}

/// Escapes `text` and turns its web addresses into links (`findWebLinks` of the web:
/// `http(s)://`, `www.` and bare domains with a known ending).
fn push_linkified(out: &mut String, text: &str) {
    let mut at = 0;
    for link in crate::web_links::find_web_links(text) {
        push_escaped(out, &text[at..link.index]);
        out.push('[');
        push_escaped(out, &link.url);
        out.push_str("](");
        out.push_str(&link_destination(&crate::web_links::to_href(&link.url)));
        out.push(')');
        at = link.index + link.url.len();
    }
    push_escaped(out, &text[at..]);
}

/// A link destination in angle brackets: spaces and parentheses need no escaping there.
fn link_destination(url: &str) -> String {
    let mut out = String::with_capacity(url.len() + 2);
    out.push('<');
    for c in url.chars() {
        match c {
            '<' => out.push_str("%3C"),
            '>' => out.push_str("%3E"),
            '\n' | '\r' => {}
            ' ' => out.push_str("%20"),
            '\\' => out.push_str("%5C"),
            _ => out.push(c),
        }
    }
    out.push('>');
    out
}

fn is_safe_link(href: &str) -> bool {
    let lower = href.trim().to_ascii_lowercase();
    ["https://", "http://", "mailto:", "matrix:"]
        .iter()
        .any(|scheme| lower.starts_with(scheme))
}

fn valid_color(value: &str) -> Option<String> {
    let v = value.trim();
    let hex = v.strip_prefix('#')?;
    (matches!(hex.len(), 3 | 6) && hex.chars().all(|c| c.is_ascii_hexdigit()))
        .then(|| format!("#{hex}"))
}

/// Inline code whose fence is longer than any backtick run inside it.
fn inline_code(text: &str) -> String {
    let longest = text.split(|c| c != '`').map(str::len).max().unwrap_or(0);
    let fence = "`".repeat(longest + 1);
    let pad = if text.starts_with('`') || text.ends_with('`') {
        " "
    } else {
        ""
    };
    format!("{fence}{pad}{text}{pad}{fence}")
}

// ---------------------------------------------------------------------------
// HTML tokenizer: enough for the allow-listed Matrix subset.

enum Token<'a> {
    Text(&'a str),
    Open {
        name: String,
        attrs: Vec<(String, String)>,
        self_closing: bool,
    },
    Close(String),
}

struct Tokenizer<'a> {
    rest: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn new(html: &'a str) -> Self {
        Self { rest: html }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        loop {
            if self.rest.is_empty() {
                return None;
            }
            if let Some(after) = self.rest.strip_prefix("<!--") {
                self.rest = after.find("-->").map_or("", |end| &after[end + 3..]);
                continue;
            }
            if self.rest.starts_with('<') {
                let looks_like_tag = self.rest[1..]
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphabetic() || c == '/');
                if looks_like_tag && let Some(end) = find_tag_end(self.rest) {
                    let inner = &self.rest[1..end];
                    self.rest = &self.rest[end + 1..];
                    return Some(parse_tag(inner));
                }
                // A lone `<`: text.
                let (text, rest) = self.rest.split_at(1);
                self.rest = rest;
                return Some(Token::Text(text));
            }
            let end = self.rest.find('<').unwrap_or(self.rest.len());
            let (text, rest) = self.rest.split_at(end);
            self.rest = rest;
            return Some(Token::Text(text));
        }
    }
}

/// Index of the `>` closing the tag that starts at 0, skipping quoted attribute values.
fn find_tag_end(s: &str) -> Option<usize> {
    let mut quote: Option<char> = None;
    for (i, c) in s.char_indices().skip(1) {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), _) => {}
            (None, '"' | '\'') => quote = Some(c),
            (None, '>') => return Some(i),
            _ => {}
        }
    }
    None
}

fn parse_tag(inner: &str) -> Token<'static> {
    if let Some(name) = inner.strip_prefix('/') {
        return Token::Close(name.trim().to_ascii_lowercase());
    }
    let self_closing = inner.trim_end().ends_with('/');
    let inner = inner.trim_end().trim_end_matches('/');
    let name_end = inner
        .find(|c: char| c.is_whitespace())
        .unwrap_or(inner.len());
    let name = inner[..name_end].to_ascii_lowercase();
    let mut attrs = Vec::new();
    let mut rest = inner[name_end..].trim_start();
    while !rest.is_empty() {
        let key_end = rest
            .find(|c: char| c == '=' || c.is_whitespace())
            .unwrap_or(rest.len());
        let key = rest[..key_end].to_ascii_lowercase();
        rest = rest[key_end..].trim_start();
        let mut value = String::new();
        if let Some(after) = rest.strip_prefix('=') {
            let after = after.trim_start();
            if let Some(q) = after.chars().next().filter(|c| *c == '"' || *c == '\'') {
                let body = &after[1..];
                let end = body.find(q).unwrap_or(body.len());
                value = decode_entities(&body[..end]);
                rest = body.get(end + 1..).unwrap_or("");
            } else {
                let end = after.find(char::is_whitespace).unwrap_or(after.len());
                value = decode_entities(&after[..end]);
                rest = &after[end..];
            }
        }
        if !key.is_empty() {
            attrs.push((key, value));
        }
        rest = rest.trim_start();
    }
    Token::Open {
        name,
        attrs,
        self_closing,
    }
}

fn is_void(name: &str) -> bool {
    matches!(
        name,
        "br" | "hr" | "img" | "input" | "meta" | "link" | "wbr"
    )
}

fn decode_entities(text: &str) -> String {
    if !text.contains('&') {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find('&') {
        out.push_str(&rest[..start]);
        let tail = &rest[start..];
        let decoded = tail.find(';').filter(|end| *end <= 10).and_then(|end| {
            let entity = &tail[1..end];
            let c = match entity {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "apos" | "#39" => Some('\''),
                "nbsp" => Some('\u{A0}'),
                _ => entity
                    .strip_prefix("#x")
                    .or_else(|| entity.strip_prefix("#X"))
                    .and_then(|hex| u32::from_str_radix(hex, 16).ok())
                    .or_else(|| entity.strip_prefix('#').and_then(|dec| dec.parse().ok()))
                    .and_then(char::from_u32),
            };
            c.map(|c| (c, end))
        });
        match decoded {
            Some((c, end)) => {
                out.push(c);
                rest = &tail[end + 1..];
            }
            None => {
                out.push('&');
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_is_escaped_and_linked() {
        assert_eq!(plain_to_markdown("# not a heading"), "\\# not a heading");
        assert_eq!(plain_to_markdown("1. x\n- y"), "1\\. x\n\\- y");
        assert_eq!(
            plain_to_markdown("see https://twake.app."),
            "see [https\\:\\/\\/twake\\.app](<https://twake.app>)\\."
        );
    }

    #[test]
    fn formats_the_allowed_tags() {
        assert_eq!(
            html_to_markdown("<b>bold </b>and <i>it</i>"),
            "**bold** and *it*"
        );
        assert_eq!(
            html_to_markdown("<del>no</del> <u>yes</u>"),
            "~~no~~ <u>yes</u>"
        );
        assert_eq!(html_to_markdown("<code>a*b</code>"), "`a*b`");
        assert_eq!(
            html_to_markdown("<a href=\"https://x.org/a b\">X</a>"),
            "[X](<https://x.org/a%20b>)"
        );
        assert_eq!(
            html_to_markdown("<a href=\"javascript:alert(1)\">X</a>"),
            "X"
        );
        assert_eq!(
            html_to_markdown("<font data-mx-color=\"#ff0000\">red</font>"),
            "<font color=\"#ff0000\">red</font>"
        );
    }

    #[test]
    fn blocks_become_supported_shapes() {
        assert_eq!(
            html_to_markdown("<ul><li>a</li><li>b</li></ul>"),
            "- a\n- b"
        );
        assert_eq!(
            html_to_markdown("<ol start=\"3\"><li>a</li><li>b</li></ol>"),
            "3. a\n4. b"
        );
        assert_eq!(
            html_to_markdown("<blockquote>q</blockquote>after"),
            "▎ q\n\nafter"
        );
        assert_eq!(html_to_markdown("<h1>Title</h1>"), "**Title**");
        assert_eq!(
            html_to_markdown("<pre><code>let a = `x`;\nb\n</code></pre>"),
            "``let a = `x`;``\n`b`"
        );
        assert_eq!(html_to_markdown("a<br>b<br/>c"), "a\nb\nc");
    }

    #[test]
    fn drops_reply_fallback_and_decodes_entities() {
        assert_eq!(
            html_to_markdown("<mx-reply><blockquote>old</blockquote></mx-reply>new &amp; &lt;3"),
            "new \\& \\<3"
        );
    }
}
