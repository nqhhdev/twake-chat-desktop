//! PDF pages drawn by PDFium (ADR-017), for the viewer of the web's `RoomPdfViewer`. The
//! library is loaded at run time: next to the app, or from `vendor/pdfium` in a debug
//! build, or `TWAKE_PDFIUM_DIR`. Without it the app opens PDFs in the system's viewer.
//!
//! PDFium is bound once per process and is not thread-safe: one worker thread owns it and
//! the one document open, and draws the pages asked for, in turn.

use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex, OnceLock};

use pdfium_render::prelude::{PdfDocument, PdfRenderConfig, Pdfium};

use crate::engine::{Emit, Event};
use crate::media::Picture;

/// The widest page drawn, in pixels (a 4K screen).
const MAX_WIDTH: u32 = 3840;

enum Job {
    Open {
        key: String,
        bytes: Vec<u8>,
        emit: Emit,
    },
    Render {
        key: String,
        page: u16,
        width: u32,
        emit: Emit,
    },
    Close,
}

/// The folder of PDFium in this repository for this computer (debug builds only).
fn vendor_dir() -> Option<PathBuf> {
    let platform = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => "mac-arm64",
        ("macos", "x86_64") => "mac-x64",
        ("linux", "x86_64") => "linux-x64",
        ("linux", "aarch64") => "linux-arm64",
        ("windows", "x86_64") => "win-x64",
        ("windows", "aarch64") => "win-arm64",
        _ => return None,
    };
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../vendor/pdfium")
        .join(platform);
    // The Windows archive keeps the DLL in bin/, the others in lib/
    Some(dir.join(if cfg!(windows) { "bin" } else { "lib" }))
}

/// Where the library is looked for, in turn.
fn library_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(dir) = std::env::var_os("TWAKE_PDFIUM_DIR") {
        dirs.push(PathBuf::from(dir));
    }
    if let Some(exe_dir) = std::env::current_exe()
        .ok()
        .and_then(|e| e.parent().map(PathBuf::from))
    {
        // macOS bundles keep libraries in Contents/Frameworks
        dirs.push(exe_dir.join("../Frameworks"));
        dirs.push(exe_dir.clone());
    }
    if cfg!(debug_assertions) {
        dirs.extend(vendor_dir());
    }
    dirs
}

fn bind() -> Result<Pdfium, String> {
    for dir in library_dirs() {
        let path = Pdfium::pdfium_platform_library_name_at_path(&dir);
        if path.exists() {
            return Pdfium::bind_to_library(&path)
                .map(Pdfium::new)
                .map_err(|e| format!("{}: {e}", path.display()));
        }
    }
    Pdfium::bind_to_system_library()
        .map(Pdfium::new)
        .map_err(|e| format!("PDFium not found: {e}"))
}

fn worker() -> Option<&'static Mutex<Sender<Job>>> {
    static WORKER: OnceLock<Option<Mutex<Sender<Job>>>> = OnceLock::new();
    WORKER
        .get_or_init(|| {
            let (jobs, queue) = channel();
            let (ready, bound) = channel();
            std::thread::Builder::new()
                .name("pdfium".into())
                .spawn(move || match bind() {
                    Ok(pdfium) => {
                        let _ = ready.send(true);
                        run(Box::leak(Box::new(pdfium)), &queue);
                    }
                    Err(e) => {
                        tracing::info!("no PDF viewer: {e}");
                        let _ = ready.send(false);
                    }
                })
                .ok()?;
            bound.recv().unwrap_or(false).then(|| Mutex::new(jobs))
        })
        .as_ref()
}

/// Whether PDFs can be shown in the app (the web's `navigator.pdfViewerEnabled`).
pub fn available() -> bool {
    worker().is_some()
}

fn submit(job: Job) -> bool {
    worker()
        .and_then(|w| w.lock().ok())
        .is_some_and(|w| w.send(job).is_ok())
}

/// Opens the PDF of `key` (closing the one open): [`Event::PdfOpened`] or
/// [`Event::PdfFailed`].
pub fn open(key: String, bytes: Vec<u8>, emit: Emit) {
    if !submit(Job::Open {
        key: key.clone(),
        bytes,
        emit: emit.clone(),
    }) {
        emit(Event::PdfFailed {
            key,
            error: "PDF viewer not available".into(),
        });
    }
}

/// Draws a page of the open PDF `width` pixels wide: [`Event::PdfPage`].
pub fn render(key: String, page: u16, width: u32, emit: Emit) {
    submit(Job::Render {
        key,
        page,
        width,
        emit,
    });
}

pub fn close() {
    submit(Job::Close);
}

/// The sizes of the pages, in points.
fn sizes(document: &PdfDocument<'_>) -> Vec<(f32, f32)> {
    document
        .pages()
        .iter()
        .map(|p| (p.width().value, p.height().value))
        .collect()
}

fn draw(document: &PdfDocument<'_>, page: u16, width: u32) -> Result<Picture, String> {
    let page = document
        .pages()
        .get(i32::from(page))
        .map_err(|e| e.to_string())?;
    let width = i32::try_from(width.clamp(1, MAX_WIDTH)).unwrap_or(1);
    let bitmap = page
        .render_with_config(
            &PdfRenderConfig::new()
                .set_target_width(width)
                .render_form_data(true),
        )
        .map_err(|e| e.to_string())?;
    Ok(Picture {
        width: u32::try_from(bitmap.width()).unwrap_or(0),
        height: u32::try_from(bitmap.height()).unwrap_or(0),
        rgba: Arc::new(bitmap.as_rgba_bytes()),
    })
}

fn run(pdfium: &'static Pdfium, queue: &Receiver<Job>) {
    let mut open: Option<(String, PdfDocument<'static>)> = None;
    while let Ok(job) = queue.recv() {
        match job {
            Job::Open { key, bytes, emit } => {
                open = None;
                match pdfium.load_pdf_from_byte_vec(bytes, None) {
                    Ok(document) => {
                        emit(Event::PdfOpened {
                            key: key.clone(),
                            pages: sizes(&document),
                        });
                        open = Some((key, document));
                    }
                    Err(e) => emit(Event::PdfFailed {
                        key,
                        error: e.to_string(),
                    }),
                }
            }
            Job::Render {
                key,
                page,
                width,
                emit,
            } => {
                // A page of a PDF closed since is not drawn
                let Some((_, document)) = open.as_ref().filter(|(k, _)| *k == key) else {
                    continue;
                };
                match draw(document, page, width) {
                    Ok(picture) => emit(Event::PdfPage {
                        key,
                        page,
                        width,
                        picture,
                    }),
                    Err(e) => tracing::debug!("pdf page {page}: {e}"),
                }
            }
            Job::Close => open = None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A one-page PDF, 200 × 100 points, a black square on white (PDFium mends the
    /// cross-reference table it has none of).
    const SQUARE: &[u8] = b"%PDF-1.4
1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj
2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj
3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 200 100] /Contents 4 0 R >> endobj
4 0 obj << /Length 24 >> stream
0 0 0 rg 0 0 50 50 re f
endstream endobj
trailer << /Root 1 0 R >>
%%EOF";

    #[test]
    fn draws_the_pages_of_a_pdf() {
        if !available() {
            eprintln!("PDFium not found (vendor/pdfium): skipped");
            return;
        }
        let (sent, heard) = channel();
        let sent = Mutex::new(sent);
        let emit: Emit = Arc::new(move |e| {
            if let Ok(s) = sent.lock() {
                let _ = s.send(e);
            }
        });
        open("k".into(), SQUARE.to_vec(), emit.clone());
        match heard.recv().unwrap() {
            Event::PdfOpened { pages, .. } => assert_eq!(pages, vec![(200.0, 100.0)]),
            _ => panic!("not opened"),
        }
        render("k".into(), 0, 400, emit.clone());
        let Event::PdfPage { picture, .. } = heard.recv().unwrap() else {
            panic!("not drawn")
        };
        assert_eq!((picture.width, picture.height), (400, 200));
        // Bottom left is black, top right white (RGBA)
        let at = |x: usize, y: usize| picture.rgba[(y * 400 + x) * 4];
        assert_eq!(at(10, 190), 0);
        assert_eq!(at(390, 10), 255);
        open("bad".into(), b"not a pdf".to_vec(), emit);
        assert!(matches!(heard.recv().unwrap(), Event::PdfFailed { .. }));
    }
}
