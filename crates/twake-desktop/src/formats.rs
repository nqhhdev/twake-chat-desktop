//! The formats bar of the composers (web `FormatToolbar`, `LinkDialog`): the Markdown
//! changes of `twake_domain::text_format` on the field's text and selection.

use twake_domain::text_format::{
    FormatChange, TextFormat, apply_format, link_change, link_target, normalize_url, paste_link,
};

use crate::{LinkTarget, TextChange};

/// The selection `start`–`end` of `text` as byte offsets, None when it falls outside or
/// inside a character.
fn range(text: &str, start: i32, end: i32) -> Option<(usize, usize)> {
    let (start, end) = (usize::try_from(start).ok()?, usize::try_from(end).ok()?);
    (start <= end && text.is_char_boundary(start) && text.is_char_boundary(end))
        .then_some((start, end))
}

fn changed(text: &str, change: FormatChange) -> TextChange {
    TextChange {
        ok: true,
        text: change.apply(text).into(),
        from: change.selection.0 as i32,
        to: change.selection.1 as i32,
    }
}

pub fn format(text: &str, name: &str, start: i32, end: i32) -> TextChange {
    match (TextFormat::from_name(name), range(text, start, end)) {
        (Some(format), Some((start, end))) => changed(text, apply_format(text, start, end, format)),
        _ => TextChange::default(),
    }
}

/// Cmd/Ctrl+V over a selection: a single web address of the clipboard links it.
pub fn paste(text: &str, start: i32, end: i32) -> TextChange {
    let Some((start, end)) = range(text, start, end).filter(|(s, e)| s < e) else {
        return TextChange::default();
    };
    let pasted = arboard::Clipboard::new().and_then(|mut c| c.get_text());
    match pasted.ok().and_then(|p| paste_link(text, start, end, &p)) {
        Some(change) => changed(text, change),
        None => TextChange::default(),
    }
}

pub fn target(text: &str, start: i32, end: i32) -> LinkTarget {
    let Some((start, end)) = range(text, start, end) else {
        return LinkTarget::default();
    };
    let found = link_target(text, start, end);
    LinkTarget {
        from: found.from as i32,
        to: found.to as i32,
        text: found.text.into(),
        url: found.url.into(),
    }
}

/// The dialog's link written over `from`–`to`; not `ok` when the address is not a web one.
pub fn apply_link(text: &str, from: i32, to: i32, label: &str, url: &str) -> TextChange {
    match (range(text, from, to), normalize_url(url)) {
        (Some((from, to)), Some(url)) => changed(text, link_change(from, to, label, &url)),
        _ => TextChange::default(),
    }
}
