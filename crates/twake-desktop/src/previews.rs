//! Link previews under messages (web `LinkPreview`): the first web link of a message, as
//! the homeserver previews it, asked once per address and kept. An encrypted room never
//! asks the homeserver (web D211), which would learn the links.

use twake_core::Command;
use twake_domain::TimelineEntry;
use twake_domain::link_preview::{LinkPreview, is_wide_picture};
use twake_domain::web_links::extract_links;

use crate::Row;
use crate::view::Pictures;

/// The side of the square picture beside the text, and the width of a banner, in logical
/// pixels (web `THUMBNAIL`, the 640 px banner at density 2 is 320).
const THUMBNAIL: u32 = 56;
const BANNER: u32 = 320;

/// The address a message previews: its first web link that is not a Matrix one.
fn previewed(entry: &TimelineEntry) -> Option<String> {
    let TimelineEntry::Message(m) = entry else {
        return None;
    };
    extract_links(&m.text)
        .into_iter()
        .find(|href| !href.starts_with("https://matrix.to/"))
}

/// Fills the preview of each message row (rows follow `entries` one to one); asks for the
/// previews not known yet.
pub fn fill(
    rows: &mut [Row],
    entries: &[TimelineEntry],
    known: &std::collections::HashMap<String, Option<LinkPreview>>,
    asked: &mut std::collections::HashSet<String>,
    requests: &mut Vec<Command>,
    encrypted: bool,
    pictures: Pictures<'_>,
) {
    if encrypted {
        return;
    }
    for (row, entry) in rows.iter_mut().zip(entries) {
        let Some(url) = previewed(entry) else {
            continue;
        };
        let Some(found) = known.get(&url) else {
            if asked.insert(url.clone()) {
                requests.push(Command::FetchLinkPreview { url });
            }
            continue;
        };
        let Some(preview) = found else { continue };
        let wide = is_wide_picture(preview.image_width, preview.image_height);
        let image = preview.image_mxc.as_deref().and_then(|mxc| {
            let key = twake_core::mxc_key(mxc);
            if wide {
                let height = preview
                    .image_height
                    .zip(preview.image_width)
                    .map_or(BANNER / 2, |(h, w)| (BANNER as f64 * h / w).round() as u32);
                pictures(&key, BANNER * 2, height.max(1) * 2, false)
            } else {
                pictures(&key, THUMBNAIL * 2, THUMBNAIL * 2, true)
            }
        });
        row.has_preview = true;
        row.preview_url = url.into();
        row.preview_title = preview.title.clone().unwrap_or_default().into();
        row.preview_text = preview.description.clone().unwrap_or_default().into();
        row.preview_site = preview.site_name.clone().unwrap_or_default().into();
        row.preview_wide = wide;
        row.has_preview_image = image.is_some();
        row.preview_image = image.unwrap_or_default();
    }
}
