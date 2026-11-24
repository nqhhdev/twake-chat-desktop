//! The PDF viewer (web `RoomPdfViewer`, ADR-017): a PDF of the conversation over the whole
//! window, its pages one under the other, drawn by PDFium in the engine. Only the pages on
//! screen or near it are drawn, at the width shown; pages far from it are dropped, so a
//! long document holds a few pages in memory. Save and Open in app beside it. Without
//! PDFium, « Open » hands the PDF to the system's viewer as before.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use slint::{ComponentHandle, Model, VecModel};
use twake_core::{Command, Picture};
use twake_domain::TimelineEntry;
use twake_domain::attachment::is_viewable_pdf;

use crate::{AppWindow, PdfPage, STATE, send};

/// Room between pages, in logical pixels (as in app.slint).
const GAP: f32 = 16.0;

#[derive(Default)]
struct Viewing {
    key: String,
    name: String,
    /// The size of each page, in points.
    sizes: Vec<(f32, f32)>,
    pages: Option<Rc<VecModel<PdfPage>>>,
    /// Page → the width in pixels it is drawn at.
    drawn: HashMap<u16, u32>,
    asked: HashSet<(u16, u32)>,
    generation: i32,
}

thread_local! {
    static VIEWING: RefCell<Viewing> = RefCell::new(Viewing::default());
}

/// Whether the attachment of `key`, among the messages on screen, opens in this viewer.
pub fn shows(key: &str) -> bool {
    let attachment = STATE.with_borrow(|s| {
        s.entries
            .iter()
            .chain(s.thread_entries.iter())
            .find_map(|e| match e {
                TimelineEntry::Message(m) => {
                    m.attachment.as_ref().filter(|a| a.source == key).cloned()
                }
                _ => None,
            })
    });
    attachment.is_some_and(|a| is_viewable_pdf(&a, twake_core::pdf_available()))
}

pub fn open(window: &AppWindow, key: &str, name: &str) {
    let pages = Rc::new(VecModel::<PdfPage>::default());
    window.set_pdf_pages(slint::ModelRc::from(pages.clone()));
    VIEWING.with_borrow_mut(|v| {
        let generation = v.generation;
        *v = Viewing {
            key: key.to_string(),
            name: name.to_string(),
            pages: Some(pages),
            generation,
            ..Viewing::default()
        };
    });
    window.set_pdf_name(name.into());
    window.set_pdf_status("Loading…".into());
    window.set_pdf_open(true);
    send(Command::OpenPdf {
        key: key.to_string(),
    });
}

pub fn close(window: &AppWindow) {
    window.set_pdf_open(false);
    window.set_pdf_pages(slint::ModelRc::default());
    VIEWING.with_borrow_mut(|v| {
        let generation = v.generation;
        *v = Viewing {
            generation,
            ..Viewing::default()
        };
    });
    send(Command::ClosePdf);
}

fn is_current(key: &str) -> bool {
    VIEWING.with_borrow(|v| !v.key.is_empty() && v.key == key)
}

pub fn opened(window: &AppWindow, key: &str, sizes: Vec<(f32, f32)>) {
    if !is_current(key) {
        return;
    }
    let rows: Vec<PdfPage> = sizes
        .iter()
        .map(|(w, h)| PdfPage {
            // A page without a size gets the shape of an A4 one
            ratio: if *w > 0.0 && *h > 0.0 { h / w } else { 1.414 },
            ..PdfPage::default()
        })
        .collect();
    let generation = VIEWING.with_borrow_mut(|v| {
        v.sizes = sizes;
        if let Some(pages) = &v.pages {
            pages.set_vec(rows);
        }
        v.generation += 1;
        v.generation
    });
    window.set_pdf_status("".into());
    // The list reports where it is scrolled once it holds the pages
    window.set_pdf_generation(generation);
}

pub fn failed(window: &AppWindow, key: &str, error: &str) {
    if !is_current(key) {
        return;
    }
    eprintln!("pdf not shown: {error}");
    let text = if error.contains("Password") {
        "This PDF is protected by a password: open it in another app."
    } else {
        "This PDF cannot be shown here: open it in another app."
    };
    window.set_pdf_status(text.into());
}

/// The list scrolled or changed size: the pages from `top` to `top + height` (logical
/// pixels), shown `page_width` wide, are drawn; a screen above and two below are drawn
/// ahead, pages farther than that dropped.
pub fn scrolled(window: &AppWindow, top: f32, height: f32, page_width: f32) {
    if page_width <= 0.0 {
        return;
    }
    let scale = window.window().scale_factor();
    // Widths in steps of 64 pixels: a small resize draws nothing again
    let pixels = ((page_width * scale / 64.0).ceil() as u32).max(1) * 64;
    let mut requests = Vec::new();
    VIEWING.with_borrow_mut(|v| {
        let Viewing {
            key,
            sizes,
            pages,
            drawn,
            asked,
            ..
        } = v;
        let Some(pages) = pages.as_ref() else { return };
        let mut y = 0.0;
        for (index, (w, h)) in sizes.iter().enumerate() {
            let Ok(page) = u16::try_from(index) else {
                break;
            };
            let ratio = if *w > 0.0 && *h > 0.0 { h / w } else { 1.414 };
            let start = y;
            y += page_width * ratio + GAP;
            let near = start < top + height * 3.0 && y > top - height;
            let keep = start < top + height * 5.0 && y > top - height * 3.0;
            if near && drawn.get(&page) != Some(&pixels) && asked.insert((page, pixels)) {
                requests.push(Command::RenderPdfPage {
                    key: key.clone(),
                    page,
                    width: pixels,
                });
            } else if !keep && drawn.remove(&page).is_some() {
                asked.retain(|(p, _)| *p != page);
                if let Some(mut row) = pages.row_data(index) {
                    row.image = slint::Image::default();
                    row.ready = false;
                    pages.set_row_data(index, row);
                }
            }
        }
    });
    requests.into_iter().for_each(send);
}

pub fn page_drawn(key: &str, page: u16, width: u32, picture: &Picture) {
    if !is_current(key) {
        return;
    }
    let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
        &picture.rgba,
        picture.width,
        picture.height,
    );
    VIEWING.with_borrow_mut(|v| {
        // Dropped meanwhile (scrolled far away): not kept
        if !v.asked.contains(&(page, width)) {
            return;
        }
        let index = usize::from(page);
        if let Some(pages) = &v.pages
            && let Some(mut row) = pages.row_data(index)
        {
            row.image = slint::Image::from_rgba8(buffer);
            row.ready = true;
            pages.set_row_data(index, row);
            v.drawn.insert(page, width);
        }
    });
}

pub fn save() {
    let (key, name) = VIEWING.with_borrow(|v| (v.key.clone(), v.name.clone()));
    send(Command::SaveAttachment { key, name });
}

pub fn open_external() {
    let (key, name) = VIEWING.with_borrow(|v| (v.key.clone(), v.name.clone()));
    send(Command::OpenAttachment { key, name });
}
