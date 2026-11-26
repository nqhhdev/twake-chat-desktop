//! The icons of the OS, drawn once at start-up and handed to the window (`Icons` in
//! parts.slint). macOS: SF Symbols, taken from AppKit as a black picture whose alpha
//! the UI colours with the theme. Elsewhere none yet: the UI shows its emoji.

use slint::ComponentHandle;

use crate::{AppWindow, Icons};

/// Each property of `Icons` and the SF Symbol drawn for it.
macro_rules! icons {
    ($($setter:ident = $symbol:literal),* $(,)?) => {
        pub fn load(window: &AppWindow) {
            let icons = window.global::<Icons>();
            $(
                if let Some(image) = symbol($symbol) {
                    icons.$setter(image);
                }
            )*
        }
    };
}

icons! {
    set_bell = "bell.fill",
    set_chats = "bubble.left.and.bubble.right.fill",
    set_lock = "lock.fill",
    set_pin = "pin.fill",
    set_phone = "phone.fill",
    set_video = "video.fill",
    set_mic = "mic.fill",
    set_send = "paperplane.fill",
    set_plus = "plus",
    set_mic_off = "mic.slash.fill",
    set_screen = "rectangle.inset.filled.and.person.filled",
    set_attach = "paperclip",
    set_poll = "chart.bar.fill",
    set_emoji = "face.smiling",
    set_close = "xmark",
    set_reply = "arrowshape.turn.up.left",
    set_thread = "bubble.left.and.bubble.right",
    set_bookmark = "bookmark",
    set_edit = "pencil",
    set_delete = "trash",
    set_play = "play.fill",
    set_pause = "pause.fill",
    set_file = "doc.fill",
    set_film = "film",
    set_music = "music.note",
    set_clock = "clock",
    set_check = "checkmark",
    set_link = "link",
    set_quote = "text.quote",
    set_bullets = "list.bullet",
}

/// The size a symbol is drawn at (points): twice the largest it is shown at, so that it
/// stays sharp on a Retina screen.
#[cfg(target_os = "macos")]
const POINT_SIZE: f64 = 44.0;

/// An SF Symbol, None when this macOS has no symbol of that name.
#[cfg(target_os = "macos")]
fn symbol(name: &str) -> Option<slint::Image> {
    use objc2_app_kit::{NSImage, NSImageSymbolConfiguration};
    use objc2_foundation::NSString;

    let image = NSImage::imageWithSystemSymbolName_accessibilityDescription(
        &NSString::from_str(name),
        None,
    )?;
    // Weight 0: regular (NSFontWeightRegular)
    let config = NSImageSymbolConfiguration::configurationWithPointSize_weight(POINT_SIZE, 0.0);
    let tiff = image
        .imageWithSymbolConfiguration(&config)?
        .TIFFRepresentation()?
        .to_vec();
    alpha_of(&tiff)
}

#[cfg(not(target_os = "macos"))]
fn symbol(_name: &str) -> Option<slint::Image> {
    None
}

/// The alpha of a TIFF picture (RGBA or grey and alpha, 8 bits), on black.
#[cfg(target_os = "macos")]
fn alpha_of(tiff: &[u8]) -> Option<slint::Image> {
    use slint::{Rgba8Pixel, SharedPixelBuffer};
    use tiff::ColorType;
    use tiff::decoder::{Decoder, DecodingResult};

    let mut decoder = Decoder::new(std::io::Cursor::new(tiff)).ok()?;
    let (width, height) = decoder.dimensions().ok()?;
    let channels = match decoder.colortype().ok()? {
        ColorType::RGBA(8) => 4,
        ColorType::GrayA(8) => 2,
        _ => return None,
    };
    let DecodingResult::U8(data) = decoder.read_image().ok()? else {
        return None;
    };
    let mut pixels = SharedPixelBuffer::<Rgba8Pixel>::new(width, height);
    for (pixel, read) in pixels
        .make_mut_slice()
        .iter_mut()
        .zip(data.chunks_exact(channels))
    {
        *pixel = Rgba8Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: read[channels - 1],
        };
    }
    Some(slint::Image::from_rgba8(pixels))
}
