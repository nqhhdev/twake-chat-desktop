//! Files waiting above a composer (web `useStagedFiles`): picked with « + », dropped on
//! the window or pasted. They leave with the next send, the composer's text as their
//! caption.

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use slint::winit_030::{EventResult, WinitWindowAccessor, winit};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::AppWindow;

#[derive(Default)]
struct Staged {
    main: Vec<PathBuf>,
    thread: Vec<PathBuf>,
    /// Files of a drop not handed over yet (winit sends them one by one).
    dropping: Vec<PathBuf>,
    /// The last pointer position, to tell a drop on the thread panel.
    cursor_x: f64,
}

thread_local! {
    static STAGED: RefCell<Staged> = RefCell::new(Staged::default());
}

fn show(window: &AppWindow, thread: bool) {
    let names: Vec<SharedString> = STAGED.with_borrow(|s| {
        let list = if thread { &s.thread } else { &s.main };
        list.iter()
            .map(|p| {
                p.file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default()
                    .into()
            })
            .collect()
    });
    let model = ModelRc::from(Rc::new(VecModel::from(names)));
    if thread {
        window.set_thread_staged(model);
    } else {
        window.set_staged(model);
    }
}

/// Adds files above the composer of the room, or of the open thread.
pub fn stage(window: &AppWindow, paths: Vec<PathBuf>, thread: bool) {
    let thread = thread && window.get_thread_open() && !window.get_side_pinned();
    STAGED.with_borrow_mut(|s| {
        let list = if thread { &mut s.thread } else { &mut s.main };
        for path in paths {
            if path.is_file() && !list.contains(&path) {
                list.push(path);
            }
        }
    });
    show(window, thread);
}

pub fn unstage(window: &AppWindow, thread: bool, index: usize) {
    STAGED.with_borrow_mut(|s| {
        let list = if thread { &mut s.thread } else { &mut s.main };
        if index < list.len() {
            list.remove(index);
        }
    });
    show(window, thread);
}

/// The files of the composer, which leave it.
pub fn take(window: &AppWindow, thread: bool) -> Vec<PathBuf> {
    let files = STAGED
        .with_borrow_mut(|s| std::mem::take(if thread { &mut s.thread } else { &mut s.main }));
    show(window, thread);
    files
}

/// Clears both composers' files (another room opened).
pub fn clear(window: &AppWindow) {
    STAGED.with_borrow_mut(|s| {
        s.main.clear();
        s.thread.clear();
    });
    show(window, false);
    show(window, true);
}

/// Files dropped on the window go to the composer under the pointer.
pub fn install_drop(window: &AppWindow) {
    let weak = window.as_weak();
    window.window().on_winit_window_event(move |_, event| {
        match event {
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                STAGED.with_borrow_mut(|s| s.cursor_x = position.x);
            }
            winit::event::WindowEvent::HoveredFile(_) => {
                if let Some(w) = weak.upgrade() {
                    w.set_drop_hover(true);
                }
            }
            winit::event::WindowEvent::HoveredFileCancelled => {
                if let Some(w) = weak.upgrade() {
                    w.set_drop_hover(false);
                }
            }
            winit::event::WindowEvent::DroppedFile(path) => {
                let first = STAGED.with_borrow_mut(|s| {
                    s.dropping.push(path.clone());
                    s.dropping.len() == 1
                });
                if first {
                    let weak = weak.clone();
                    slint::Timer::single_shot(Duration::from_millis(100), move || {
                        let Some(w) = weak.upgrade() else { return };
                        w.set_drop_hover(false);
                        if w.get_current_room_id().is_empty() {
                            STAGED.with_borrow_mut(|s| s.dropping.clear());
                            return;
                        }
                        let (paths, x) = STAGED
                            .with_borrow_mut(|s| (std::mem::take(&mut s.dropping), s.cursor_x));
                        // The thread panel is the last 380 px of the window
                        let scale = w.window().scale_factor() as f64;
                        let width = w.window().size().width as f64;
                        let on_thread = w.get_thread_open() && x > width - 380.0 * scale;
                        stage(&w, paths, on_thread);
                    });
                }
            }
            _ => {}
        }
        EventResult::Propagate
    });
}

/// Cmd/Ctrl+V: an image or files copied in another app join the composer's files (the
/// text of the clipboard is pasted by the field itself).
pub fn paste(window: &AppWindow, thread: bool) {
    let Ok(mut clipboard) = arboard::Clipboard::new() else {
        return;
    };
    if let Ok(files) = clipboard.get().file_list()
        && !files.is_empty()
    {
        stage(window, files, thread);
        return;
    }
    if let Ok(image) = clipboard.get_image() {
        match save_png(&image) {
            Ok(path) => stage(window, vec![path], thread),
            Err(e) => window.set_notice(format!("Pasted image not kept: {e}").into()),
        }
    }
}

/// A pasted image as a PNG in the temporary folder, named after the time.
fn save_png(image: &arboard::ImageData<'_>) -> Result<PathBuf, String> {
    let dir = std::env::temp_dir().join("twake-chat-pasted");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let name = chrono::Local::now()
        .format("Pasted image %Y-%m-%d %H.%M.%S.png")
        .to_string();
    let path = dir.join(name);
    let file = std::fs::File::create(&path).map_err(|e| e.to_string())?;
    let mut encoder = png::Encoder::new(
        std::io::BufWriter::new(file),
        image.width as u32,
        image.height as u32,
    );
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
    writer
        .write_image_data(&image.bytes)
        .map_err(|e| e.to_string())?;
    Ok(path)
}
