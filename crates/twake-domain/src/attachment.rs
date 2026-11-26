//! Port of `src/domain/attachment.ts` (the parts the timeline uses).

use serde::{Deserialize, Serialize};

/// `AttachmentKind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttachmentKind {
    Image,
    Video,
    Audio,
    File,
}

impl AttachmentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            AttachmentKind::Image => "image",
            AttachmentKind::Video => "video",
            AttachmentKind::Audio => "audio",
            AttachmentKind::File => "file",
        }
    }
}

/// `Attachment`: the file a message carries. `source` is the Matrix media source
/// (plain `mxc://` or encrypted file), as JSON, opaque to the domain.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub kind: AttachmentKind,
    pub name: String,
    pub mime_type: String,
    pub size: Option<u64>,
    pub is_encrypted: bool,
    pub dimensions: Option<(u32, u32)>,
    /// Milliseconds.
    pub duration: Option<u64>,
    pub source: String,
    /// Source of the sender's thumbnail, when there is one.
    pub thumbnail_source: Option<String>,
    /// A voice message (MSC3245), with its waveform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
}

/// The waveform of a voice message: integers from 0 to [`MAX_WAVEFORM_VALUE`], empty
/// when the sender gave none.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Voice {
    pub waveform: Vec<u32>,
}

/// Larger images and audio files show as a file to download, not inline.
pub const MAX_PREVIEW_BYTES: u64 = 20 * 1024 * 1024;

/// `isPlayableAudio`: an audio file (not a voice message) the timeline plays.
pub fn is_playable_audio(attachment: &Attachment) -> bool {
    attachment.kind == AttachmentKind::Audio
        && attachment.voice.is_none()
        && attachment.size.is_none_or(|s| s <= MAX_PREVIEW_BYTES)
}

/// A voice message stops by itself after 30 minutes (web D54).
pub const MAX_VOICE_MS: u64 = 30 * 60 * 1000;
pub const WAVEFORM_POINTS: usize = 100;
pub const MAX_WAVEFORM_VALUE: u32 = 1024;

/// `computeWaveform`: the loudest sample of each of `points` equal slices, scaled so that
/// the loudest slice reaches [`MAX_WAVEFORM_VALUE`] (MSC1767 audio). Also fits the
/// waveform of another client to fewer bars.
pub fn compute_waveform(samples: &[f32], points: usize) -> Vec<u32> {
    if samples.is_empty() || points == 0 {
        return Vec::new();
    }
    let size = samples.len().div_ceil(points);
    let peaks: Vec<f32> = samples
        .chunks(size)
        .map(|slice| slice.iter().fold(0.0f32, |peak, s| peak.max(s.abs())))
        .collect();
    let loudest = peaks.iter().copied().fold(0.0f32, f32::max);
    peaks
        .iter()
        .map(|peak| {
            if loudest > 0.0 {
                // JavaScript divides the samples as doubles; Math.round of a value >= 0
                let ratio = f64::from(*peak) / f64::from(loudest);
                (ratio * f64::from(MAX_WAVEFORM_VALUE) + 0.5).floor() as u32
            } else {
                0
            }
        })
        .collect()
}

/// `isGif`.
pub fn is_gif(mime_type: &str) -> bool {
    mime_type == "image/gif"
}

/// `baseType`: a media type without its parameters, lowercased (`Application/PDF; x=y`
/// is `application/pdf`).
pub fn base_type(mime_type: &str) -> String {
    crate::js::trim(mime_type.split(';').next().unwrap_or("")).to_lowercase()
}

/// `isViewablePdf`: a PDF the app shows in its viewer instead of saving it.
/// `can_show_pdf`: the PDF engine is there (the web's `navigator.pdfViewerEnabled`).
pub fn is_viewable_pdf(attachment: &Attachment, can_show_pdf: bool) -> bool {
    can_show_pdf
        && attachment.kind == AttachmentKind::File
        && (base_type(&attachment.mime_type) == "application/pdf"
            || attachment.name.to_lowercase().ends_with(".pdf"))
}

/// `computeAttachmentKind`.
pub fn compute_attachment_kind(mime_type: &str) -> AttachmentKind {
    if mime_type.starts_with("image/") {
        AttachmentKind::Image
    } else if mime_type.starts_with("video/") {
        AttachmentKind::Video
    } else if mime_type.starts_with("audio/") {
        AttachmentKind::Audio
    } else {
        AttachmentKind::File
    }
}

const UNKNOWN_TYPE: &str = "application/octet-stream";

fn video_type(extension: &str) -> Option<&'static str> {
    Some(match extension {
        "mov" => "video/quicktime",
        "mp4" | "m4v" => "video/mp4",
        "webm" => "video/webm",
        "ogv" => "video/ogg",
        "mkv" => "video/x-matroska",
        "avi" => "video/x-msvideo",
        _ => return None,
    })
}

/// Images a browser shows without running anything: never an SVG.
fn image_type(extension: &str) -> Option<&'static str> {
    Some(match extension {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "avif" => "image/avif",
        "bmp" => "image/bmp",
        _ => return None,
    })
}

/// `inferMimeType`: the type of a file whose sender gave none (or a Flutter
/// `file/video`, `file/image`), from its extension; otherwise the sender's.
pub fn infer_mime_type(name: &str, mime_type: &str) -> String {
    let by_extension: fn(&str) -> Option<&'static str> = match mime_type {
        "file/image" => image_type,
        "" | UNKNOWN_TYPE | "file/video" => video_type,
        _ => return mime_type.to_string(),
    };
    let extension = name.rsplit('.').next().unwrap_or("").to_lowercase();
    by_extension(&extension).unwrap_or(UNKNOWN_TYPE).to_string()
}

/// Human file size: `512 B`, `1.2 KB`, `3.4 MB`.
pub fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["KB", "MB", "GB", "TB"];
    if bytes < 1024 {
        return format!("{bytes} B");
    }
    let mut value = bytes as f64 / 1024.0;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    format!("{value:.1} {}", UNITS[unit])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infers_types_like_the_web() {
        assert_eq!(infer_mime_type("clip.MOV", ""), "video/quicktime");
        assert_eq!(infer_mime_type("photo.jpg", "file/image"), "image/jpeg");
        assert_eq!(infer_mime_type("a.svg", "file/image"), UNKNOWN_TYPE);
        assert_eq!(infer_mime_type("x.png", "image/png"), "image/png");
        assert_eq!(compute_attachment_kind("audio/ogg"), AttachmentKind::Audio);
        assert_eq!(format_size(1536), "1.5 KB");
    }
}

#[cfg(test)]
mod waveform_tests {
    use super::*;

    #[test]
    fn peaks_scaled_to_the_loudest() {
        assert_eq!(compute_waveform(&[], 100), Vec::<u32>::new());
        assert_eq!(
            compute_waveform(&[0.1, -0.5, 0.25, 0.0], 2),
            vec![1024, 512]
        );
        assert_eq!(compute_waveform(&[0.0; 5], 2), vec![0, 0]);
        // Fewer slices than asked when they do not divide evenly, as the web
        assert_eq!(compute_waveform(&[1.0; 5], 4).len(), 3);
    }
}
