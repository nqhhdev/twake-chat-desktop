//! The Media, Links and Files tabs of the room info (web `SharedContent`): read from the
//! messages of the open room, so they grow as its history is loaded.

use std::cell::RefCell;
use std::rc::Rc;

use slint::{ModelRc, VecModel};
use twake_domain::attachment::{AttachmentKind, format_size};
use twake_domain::shared_content::compute_shared_content;
use twake_domain::{Message, TimelineEntry};

use crate::{AppWindow, STATE, SharedFileRow, SharedLinkRow, SharedMediaRow, UiState, send};

/// At most this many pictures and links (web `SHOWN`).
const SHOWN: usize = 60;
/// The side of a tile of the grid, in logical pixels (three in the 360 px pane).
const TILE: u32 = 112;

#[derive(Default)]
struct Models {
    media: Rc<VecModel<SharedMediaRow>>,
    links: Rc<VecModel<SharedLinkRow>>,
    files: Rc<VecModel<SharedFileRow>>,
}

thread_local! {
    static MODELS: RefCell<Models> = RefCell::new(Models::default());
}

pub fn install(window: &AppWindow) {
    MODELS.with_borrow(|m| {
        window.set_info_media(ModelRc::from(m.media.clone()));
        window.set_info_links(ModelRc::from(m.links.clone()));
        window.set_info_files(ModelRc::from(m.files.clone()));
    });
}

/// The tabs of the open room's info, from `entries`, its timeline; nothing while the info is
/// closed.
pub fn show(window: &AppWindow, entries: &[TimelineEntry]) {
    if !window.get_info_open() {
        return;
    }
    let messages: Vec<Message> = entries
        .iter()
        .filter_map(|e| match e {
            TimelineEntry::Message(m) => Some(m.clone()),
            _ => None,
        })
        .collect();
    let content = compute_shared_content(&messages);
    let mut requests = Vec::new();
    let (media, links) = STATE.with_borrow_mut(|s| {
        let UiState {
            pictures,
            asked,
            link_previews,
            ..
        } = s;
        let mut find = crate::lookup(pictures, asked, &mut requests);
        let media: Vec<SharedMediaRow> = content
            .media
            .iter()
            .take(SHOWN)
            .filter_map(|m| media_row(m, &mut find))
            .collect();
        let links: Vec<SharedLinkRow> = content
            .links
            .iter()
            .take(SHOWN)
            .map(|link| SharedLinkRow {
                title: link_previews
                    .get(&link.url)
                    .and_then(|p| p.as_ref()?.title.clone())
                    .unwrap_or_default()
                    .into(),
                url: link.url.as_str().into(),
            })
            .collect();
        (media, links)
    });
    let files: Vec<SharedFileRow> = content.files.iter().filter_map(|m| file_row(m)).collect();
    requests.into_iter().for_each(send);
    MODELS.with_borrow(|m| {
        crate::sync_model(&m.media, media);
        crate::sync_model(&m.links, links);
        crate::sync_model(&m.files, files);
    });
}

/// A tile: the picture's thumbnail (the file itself when encrypted, as the timeline); a
/// video shows its mark, no thumbnail coming with it.
fn media_row(
    m: &Message,
    find: &mut impl FnMut(&str, u32, u32, bool) -> Option<slint::Image>,
) -> Option<SharedMediaRow> {
    let a = m.attachment.as_ref()?;
    let video = a.kind == AttachmentKind::Video;
    let picture = (!video)
        .then(|| {
            let key = a
                .thumbnail_source
                .as_deref()
                .filter(|_| !a.is_encrypted)
                .unwrap_or(&a.source);
            let side = TILE * crate::view::DENSITY;
            find(key, side, side, true)
        })
        .flatten();
    Some(SharedMediaRow {
        key: a.source.as_str().into(),
        name: a.name.as_str().into(),
        has_picture: picture.is_some(),
        picture: picture.unwrap_or_default(),
        video,
    })
}

/// A file: its name, then its size and day (web `formatShortDate`, dd/mm/yy).
fn file_row(m: &Message) -> Option<SharedFileRow> {
    let a = m.attachment.as_ref()?;
    let day = chrono::DateTime::from_timestamp_millis(m.sent_at).map(|t| {
        t.with_timezone(&chrono::Local)
            .format("%d/%m/%y")
            .to_string()
    });
    let info: Vec<String> = a.size.map(format_size).into_iter().chain(day).collect();
    Some(SharedFileRow {
        key: a.source.as_str().into(),
        name: a.name.as_str().into(),
        info: info.join(" · ").into(),
    })
}
