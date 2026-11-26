//! A shared screen (web `setScreenShareEnabled`): the system's picker chooses what to
//! share (ScreenCaptureKit's on macOS), read on a thread of its own; its pictures go to
//! the call (I420) and, smaller, to the window as the user's screen tile.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::Duration;

use livekit::webrtc::desktop_capturer::{
    CaptureError, DesktopCaptureSourceType, DesktopCapturer, DesktopCapturerOptions,
};
use livekit::webrtc::native::yuv_helper;
use livekit::webrtc::prelude::VideoBuffer;
use livekit::webrtc::video_frame::{I420Buffer, VideoFrame, VideoRotation};
use livekit::webrtc::video_source::VideoResolution;
use livekit::webrtc::video_source::native::NativeVideoSource;

use crate::camera::OnPreview;
use crate::to_rgba;

/// Pictures a second sent of a screen (LiveKit's screen share preset).
const FPS: u64 = 15;
/// The width of the user's own screen tile picture (pixels).
const PREVIEW_WIDTH: u32 = 640;
/// How long the user may take to choose in the system's picker.
const PICK_TIMEOUT: Duration = Duration::from_secs(300);

/// A screen chosen and being read; dropping it stops the sharing.
pub struct Screen {
    running: Arc<AtomicBool>,
    pub(crate) source: NativeVideoSource,
}

impl Drop for Screen {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

impl Screen {
    /// Shows the system's picker and reads what the user chose. None when they chose
    /// nothing or the system refused (web: a cancelled share says nothing). Blocks until
    /// they choose. `on_end` is told when the system stops the sharing (the user stopped
    /// it from the menu bar).
    pub fn pick(on_preview: OnPreview, on_end: Arc<dyn Fn() + Send + Sync>) -> Option<Self> {
        let running = Arc::new(AtomicBool::new(true));
        let source = NativeVideoSource::new(
            VideoResolution {
                width: 1920,
                height: 1080,
            },
            true,
        );
        let (picked, told) = mpsc::channel::<bool>();
        let still = running.clone();
        let sent = source.clone();
        std::thread::Builder::new()
            .name("screen".into())
            .spawn(move || read(&sent, &still, picked, &on_preview, &*on_end))
            .ok()?;
        match told.recv_timeout(PICK_TIMEOUT) {
            Ok(true) => Some(Self { running, source }),
            _ => {
                running.store(false, Ordering::Relaxed);
                None
            }
        }
    }
}

fn read(
    source: &NativeVideoSource,
    running: &Arc<AtomicBool>,
    picked: mpsc::Sender<bool>,
    on_preview: &OnPreview,
    on_end: &(dyn Fn() + Send + Sync),
) {
    // The picker lets the user choose a screen or a window where the system has one
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let kind = DesktopCaptureSourceType::Generic;
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let kind = DesktopCaptureSourceType::Screen;
    let Some(mut capturer) = DesktopCapturer::new(DesktopCapturerOptions::new(kind)) else {
        let _ = picked.send(false);
        return;
    };
    // Without a picker, the first screen
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let chosen = None;
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let chosen = capturer.get_source_list().into_iter().next();
    let sent = source.clone();
    let preview = on_preview.clone();
    let stopped = running.clone();
    let by_system = Arc::new(AtomicBool::new(false));
    let ended = by_system.clone();
    let mut told = false;
    let mut count = 0u32;
    capturer.start_capture(chosen, move |result| match result {
        Ok(frame) => {
            if !told {
                told = true;
                let _ = picked.send(true);
            }
            let Some(mut i420) =
                to_i420(frame.data(), frame.stride(), frame.width(), frame.height())
            else {
                return;
            };
            // Four pictures a second are enough for the user's own tile
            count = count.wrapping_add(1);
            if count.is_multiple_of(4) {
                preview(small(&mut i420));
            }
            sent.capture_frame(&VideoFrame::new(VideoRotation::VideoRotation0, i420));
        }
        // Cancelled in the picker, refused, or stopped by the system
        Err(CaptureError::Permanent) => {
            if told {
                ended.store(true, Ordering::Relaxed);
            } else {
                told = true;
                let _ = picked.send(false);
            }
            stopped.store(false, Ordering::Relaxed);
        }
        Err(CaptureError::Temporary) => {}
    });
    while running.load(Ordering::Relaxed) {
        capturer.capture_frame();
        std::thread::sleep(Duration::from_millis(1000 / FPS));
    }
    // Stopped by the system, not by whoever dropped the screen: they are told
    if by_system.load(Ordering::Relaxed) {
        on_end();
    }
}

/// A picture of the screen (BGRA in memory, libyuv's ARGB) as I420.
fn to_i420(data: &[u8], stride: u32, width: i32, height: i32) -> Option<I420Buffer> {
    if width <= 0 || height <= 0 || data.len() < stride as usize * height as usize {
        return None;
    }
    // I420 takes even sizes
    let (w, h) = ((width & !1) as u32, (height & !1) as u32);
    if w == 0 || h == 0 {
        return None;
    }
    let mut i420 = I420Buffer::new(w, h);
    let (stride_y, stride_u, stride_v) = i420.strides();
    let (y, u, v) = i420.data_mut();
    yuv_helper::argb_to_i420(
        data, stride, y, stride_y, u, stride_u, v, stride_v, w as i32, h as i32,
    );
    Some(i420)
}

fn small(i420: &mut I420Buffer) -> crate::Frame {
    let (width, height) = (i420.width(), i420.height());
    let w = width.min(PREVIEW_WIDTH);
    to_rgba(&i420.scale(w as i32, (height * w / width) as i32))
}
