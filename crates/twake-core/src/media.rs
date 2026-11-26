//! Media: avatars, inline images, attachments. Downloads go through matrix-sdk's media
//! cache; images are decoded and resized here, off the UI thread, so the UI only uploads
//! ready RGBA pixels.

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use image::{DynamicImage, imageops::FilterType};
use matrix_sdk::{
    Client,
    media::{MediaFormat, MediaRequestParameters, MediaThumbnailSettings},
    ruma::{
        OwnedMxcUri, UInt,
        api::client::media::get_content_thumbnail::v3::Method,
        events::room::{MediaSource, message::MessageType},
    },
};
use twake_domain::attachment::{Attachment, compute_attachment_kind, infer_mime_type};

use crate::engine::{Emit, Event};

/// A decoded image, ready for the UI.
#[derive(Clone, Debug)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    /// RGBA8, `width * height * 4` bytes.
    pub rgba: Arc<Vec<u8>>,
}

/// The key the UI asks for a media by: the JSON of its `MediaSource`.
pub fn source_key(source: &MediaSource) -> String {
    serde_json::to_string(source).unwrap_or_default()
}

pub fn mxc_key(mxc: &str) -> String {
    source_key(&MediaSource::Plain(OwnedMxcUri::from(mxc)))
}

/// The attachment of a message, if it carries a file.
pub fn attachment_of(msgtype: &MessageType) -> Option<Attachment> {
    // A voice message (MSC3245) and its waveform (MSC1767 audio)
    let voice = match msgtype {
        MessageType::Audio(c) if c.voice.is_some() => Some(twake_domain::attachment::Voice {
            waveform: c.audio.as_ref().map_or(Vec::new(), |a| {
                a.waveform
                    .iter()
                    .map(|v| u32::try_from(u64::from(v.get())).unwrap_or(0))
                    .collect()
            }),
        }),
        _ => None,
    };
    let audio_duration = match msgtype {
        MessageType::Audio(c) => c.audio.as_ref().map(|a| a.duration),
        _ => None,
    };
    let (name, source, mime, size, dims, duration, thumb) = match msgtype {
        MessageType::Image(c) => {
            let info = c.info.as_deref();
            (
                c.filename().to_string(),
                &c.source,
                info.and_then(|i| i.mimetype.clone()),
                info.and_then(|i| i.size),
                info.and_then(|i| Some((i.width?, i.height?))),
                None,
                info.and_then(|i| i.thumbnail_source.as_ref()),
            )
        }
        MessageType::Video(c) => {
            let info = c.info.as_deref();
            (
                c.filename().to_string(),
                &c.source,
                info.and_then(|i| i.mimetype.clone()),
                info.and_then(|i| i.size),
                info.and_then(|i| Some((i.width?, i.height?))),
                info.and_then(|i| i.duration),
                info.and_then(|i| i.thumbnail_source.as_ref()),
            )
        }
        MessageType::Audio(c) => {
            let info = c.info.as_deref();
            (
                c.filename().to_string(),
                &c.source,
                info.and_then(|i| i.mimetype.clone()),
                info.and_then(|i| i.size),
                None,
                info.and_then(|i| i.duration),
                None,
            )
        }
        MessageType::File(c) => {
            let info = c.info.as_deref();
            (
                c.filename().to_string(),
                &c.source,
                info.and_then(|i| i.mimetype.clone()),
                info.and_then(|i| i.size),
                None,
                None,
                info.and_then(|i| i.thumbnail_source.as_ref()),
            )
        }
        _ => return None,
    };
    let mime = infer_mime_type(&name, mime.as_deref().unwrap_or(""));
    Some(Attachment {
        kind: compute_attachment_kind(&mime),
        name,
        mime_type: mime,
        size: size.map(u64::from),
        is_encrypted: matches!(source, MediaSource::Encrypted(_)),
        dimensions: dims.map(|(w, h): (UInt, UInt)| {
            (
                u32::try_from(u64::from(w)).unwrap_or(0),
                u32::try_from(u64::from(h)).unwrap_or(0),
            )
        }),
        duration: duration
            .or(audio_duration)
            .map(|d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX)),
        source: source_key(source),
        thumbnail_source: thumb.map(source_key),
        voice,
    })
}

/// Images being fetched, so a key is asked for once at a time.
#[derive(Clone, Default)]
pub struct InFlight(Arc<Mutex<HashSet<String>>>);

impl InFlight {
    fn start(&self, key: &str) -> bool {
        self.0
            .lock()
            .map(|mut set| set.insert(key.to_string()))
            .unwrap_or(false)
    }
    fn done(&self, key: &str) {
        if let Ok(mut set) = self.0.lock() {
            set.remove(key);
        }
    }
}

/// Fetches and decodes `key` at `width` x `height` (cropped to fill when `crop`), then
/// emits [`Event::Image`] (or [`Event::ImageFailed`]).
pub fn fetch_image(
    client: Client,
    in_flight: InFlight,
    key: String,
    width: u32,
    height: u32,
    crop: bool,
    emit: Emit,
) {
    let id = format!("{key}@{width}x{height}");
    if !in_flight.start(&id) {
        return;
    }
    tokio::spawn(async move {
        let result = load(&client, &key, width, height, crop).await;
        in_flight.done(&id);
        match result {
            Ok(picture) => emit(Event::Image {
                key,
                width,
                height,
                picture,
            }),
            Err(e) => {
                tracing::debug!("image {key}: {e}");
                emit(Event::ImageFailed { key, width, height });
            }
        }
    });
}

async fn load(
    client: &Client,
    key: &str,
    width: u32,
    height: u32,
    crop: bool,
) -> Result<Picture, String> {
    let source: MediaSource = serde_json::from_str(key).map_err(|e| e.to_string())?;
    // The server makes thumbnails of plain media; encrypted media come whole and are
    // shrunk here.
    let format = match &source {
        MediaSource::Plain(_) => MediaFormat::Thumbnail(MediaThumbnailSettings::with_method(
            if crop { Method::Crop } else { Method::Scale },
            UInt::from(width),
            UInt::from(height),
        )),
        MediaSource::Encrypted(_) => MediaFormat::File,
    };
    let bytes = client
        .media()
        .get_media_content(&MediaRequestParameters { source, format }, true)
        .await
        .map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || decode(&bytes, width, height, crop))
        .await
        .map_err(|e| e.to_string())?
}

fn decode(bytes: &[u8], width: u32, height: u32, crop: bool) -> Result<Picture, String> {
    let image = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    let image: DynamicImage = if crop {
        image.resize_to_fill(width, height, FilterType::Triangle)
    } else if image.width() > width || image.height() > height {
        image.resize(width, height, FilterType::Triangle)
    } else {
        image
    };
    let rgba = image.to_rgba8();
    Ok(Picture {
        width: rgba.width(),
        height: rgba.height(),
        rgba: Arc::new(rgba.into_raw()),
    })
}

/// The bytes of a file (decrypted when it is encrypted), kept by the media cache.
pub async fn file_bytes(client: &Client, key: &str) -> Result<Vec<u8>, String> {
    let source: MediaSource = serde_json::from_str(key).map_err(|e| e.to_string())?;
    client
        .media()
        .get_media_content(
            &MediaRequestParameters {
                source,
                format: MediaFormat::File,
            },
            true,
        )
        .await
        .map_err(|e| e.to_string())
}

/// The frames of an animated image kept at most (a 10-second GIF at 25 frames a second
/// is 250), and the pixels of all of them: past either, the still thumbnail stays.
const MAX_FRAMES: usize = 600;
const MAX_FRAME_BYTES: usize = 96 * 1024 * 1024;

/// The frames of the GIF of `key` and how long each shows, in ms (empty when it does
/// not move). The whole file is read, as the web's `<img>` does.
pub async fn load_animation(
    client: &Client,
    key: &str,
    width: u32,
    height: u32,
) -> Result<Vec<(Picture, u32)>, String> {
    let bytes = file_bytes(client, key).await?;
    tokio::task::spawn_blocking(move || decode_animation(&bytes, width, height))
        .await
        .map_err(|e| e.to_string())?
}

pub(crate) fn decode_animation(
    bytes: &[u8],
    width: u32,
    height: u32,
) -> Result<Vec<(Picture, u32)>, String> {
    use image::AnimationDecoder;
    let decoder = image::codecs::gif::GifDecoder::new(std::io::Cursor::new(bytes))
        .map_err(|e| e.to_string())?;
    let mut frames = Vec::new();
    let mut kept = 0usize;
    for frame in decoder.into_frames() {
        let frame = frame.map_err(|e| e.to_string())?;
        if frames.len() == MAX_FRAMES {
            return Ok(Vec::new());
        }
        let (numer, denom) = frame.delay().numer_denom_ms();
        let ms = numer.checked_div(denom).unwrap_or(0);
        // Browsers show a frame of 10 ms or less for 100 ms
        let ms = if ms <= 10 { 100 } else { ms };
        let image = DynamicImage::ImageRgba8(frame.into_buffer());
        let image = if image.width() > width || image.height() > height {
            image.resize(width, height, FilterType::Triangle)
        } else {
            image
        };
        let rgba = image.to_rgba8();
        kept += rgba.as_raw().len();
        if kept > MAX_FRAME_BYTES {
            return Ok(Vec::new());
        }
        frames.push((
            Picture {
                width: rgba.width(),
                height: rgba.height(),
                rgba: Arc::new(rgba.into_raw()),
            },
            ms,
        ));
    }
    if frames.len() < 2 {
        frames.clear();
    }
    Ok(frames)
}

/// Downloads a file to the Downloads folder, under a name not taken yet.
pub async fn save_file(client: &Client, key: &str, name: &str) -> Result<PathBuf, String> {
    let source: MediaSource = serde_json::from_str(key).map_err(|e| e.to_string())?;
    let bytes = client
        .media()
        .get_media_content(
            &MediaRequestParameters {
                source,
                format: MediaFormat::File,
            },
            true,
        )
        .await
        .map_err(|e| e.to_string())?;
    let folder = directories::UserDirs::new()
        .and_then(|d| d.download_dir().map(PathBuf::from))
        .unwrap_or_else(std::env::temp_dir);
    let path = free_path(&folder, &safe_name(name));
    tokio::fs::write(&path, bytes)
        .await
        .map_err(|e| e.to_string())?;
    Ok(path)
}

/// A file name without folders or characters the file systems refuse.
fn safe_name(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| {
            if matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') || c.is_control() {
                '_'
            } else {
                c
            }
        })
        .collect();
    let trimmed = cleaned.trim().trim_start_matches('.');
    if trimmed.is_empty() {
        "attachment".into()
    } else {
        trimmed.into()
    }
}

fn free_path(folder: &std::path::Path, name: &str) -> PathBuf {
    let candidate = folder.join(name);
    if !candidate.exists() {
        return candidate;
    }
    let (stem, ext) = match name.rsplit_once('.') {
        Some((s, e)) if !s.is_empty() => (s.to_string(), format!(".{e}")),
        _ => (name.to_string(), String::new()),
    };
    (1..)
        .map(|n| folder.join(format!("{stem} ({n}){ext}")))
        .find(|p| !p.exists())
        .unwrap_or(candidate)
}

/// Sends the file at `path` in `timeline`: its type from its extension, and for an image
/// its size, so that every client sizes it before downloading it.
pub async fn send_file(
    timeline: &matrix_sdk_ui::timeline::Timeline,
    path: PathBuf,
    reply_to: Option<matrix_sdk::ruma::OwnedEventId>,
    caption: Option<String>,
) -> Result<(), String> {
    use matrix_sdk::attachment::{AttachmentInfo, BaseImageInfo};
    use matrix_sdk_ui::timeline::AttachmentConfig;
    let mime: mime::Mime = mime_guess::from_path(&path).first_or_octet_stream();
    let mut config = AttachmentConfig::default();
    if mime.type_() == mime::IMAGE
        && let Ok((width, height)) = image::image_dimensions(&path)
    {
        config.info = Some(AttachmentInfo::Image(BaseImageInfo {
            width: Some(UInt::from(width)),
            height: Some(UInt::from(height)),
            ..BaseImageInfo::default()
        }));
    }
    config.in_reply_to = reply_to;
    config.caption = caption
        .filter(|c| !twake_domain::js::trim(c).is_empty())
        .map(matrix_sdk::ruma::events::room::message::TextMessageEventContent::markdown);
    timeline
        .send_attachment(
            matrix_sdk_ui::timeline::AttachmentSource::File(path),
            mime,
            config,
        )
        .await
        .map_err(|e| e.to_string())
}

/// Sends a recorded voice message (MSC3245): Ogg Opus, its length and waveform (values
/// from 0 to 1; matrix-sdk writes them as MSC1767 amplitudes).
pub async fn send_voice(
    timeline: &matrix_sdk_ui::timeline::Timeline,
    path: PathBuf,
    duration_ms: u64,
    waveform: Vec<f32>,
    reply_to: Option<matrix_sdk::ruma::OwnedEventId>,
) -> Result<(), String> {
    use matrix_sdk::attachment::{AttachmentInfo, BaseAudioInfo};
    use matrix_sdk_ui::timeline::AttachmentConfig;
    let size = tokio::fs::metadata(&path).await.ok().map(|m| m.len());
    let config = AttachmentConfig {
        info: Some(AttachmentInfo::Voice(BaseAudioInfo {
            duration: Some(std::time::Duration::from_millis(duration_ms)),
            size: size.and_then(|s| UInt::try_from(s).ok()),
            waveform: Some(waveform),
        })),
        in_reply_to: reply_to,
        ..AttachmentConfig::default()
    };
    let mime: mime::Mime = "audio/ogg".parse().map_err(|_| "audio/ogg".to_string())?;
    timeline
        .send_attachment(
            matrix_sdk_ui::timeline::AttachmentSource::File(path),
            mime,
            config,
        )
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_safe() {
        assert_eq!(safe_name("../../etc/passwd"), "_.._etc_passwd");
        assert_eq!(safe_name("  "), "attachment");
        assert_eq!(safe_name("report.pdf"), "report.pdf");
    }

    fn gif(frames: &[(u8, u32)]) -> Vec<u8> {
        let mut out = Vec::new();
        {
            let mut encoder = image::codecs::gif::GifEncoder::new(&mut out);
            for (shade, ms) in frames {
                let buffer = image::RgbaImage::from_pixel(40, 20, image::Rgba([*shade, 0, 0, 255]));
                let delay = image::Delay::from_numer_denom_ms(*ms, 1);
                encoder
                    .encode_frame(image::Frame::from_parts(buffer, 0, 0, delay))
                    .unwrap();
            }
        }
        out
    }

    #[test]
    fn decodes_the_frames_of_a_gif() {
        let frames = decode_animation(&gif(&[(10, 50), (200, 0)]), 20, 20).unwrap();
        assert_eq!(frames.len(), 2);
        assert_eq!((frames[0].0.width, frames[0].0.height), (20, 10));
        assert_eq!(frames[0].1, 50);
        // A delay of 0 shows for 100 ms, as in browsers
        assert_eq!(frames[1].1, 100);
        assert_eq!(frames[1].0.rgba[0], 200);
        // One frame: nothing moves
        assert!(
            decode_animation(&gif(&[(10, 50)]), 20, 20)
                .unwrap()
                .is_empty()
        );
    }
}
