//! The camera, read on a thread of its own: its pictures go to the call (I420) and,
//! smaller and as in a mirror (web `shouldMirror`), to the window as the user's tile.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::Duration;

use livekit::webrtc::native::yuv_helper;
use livekit::webrtc::prelude::VideoBuffer;
use livekit::webrtc::video_frame::{I420Buffer, VideoFrame, VideoRotation};
use livekit::webrtc::video_source::VideoResolution;
use livekit::webrtc::video_source::native::NativeVideoSource;
use nokhwa::Camera;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType};

use crate::{Frame, to_rgba};

/// The width of the user's own tile picture (pixels).
const PREVIEW_WIDTH: u32 = 480;

pub type OnPreview = Arc<dyn Fn(Frame) + Send + Sync>;

/// The camera while it is open; dropping it closes it.
pub struct Capture {
    running: Arc<AtomicBool>,
}

impl Drop for Capture {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

impl Capture {
    /// Opens the first camera (asking the system's permission first) and starts reading.
    pub fn start(on_preview: OnPreview) -> Result<(Self, NativeVideoSource), String> {
        allowed()?;
        let running = Arc::new(AtomicBool::new(true));
        let (opened, told) = mpsc::channel();
        let still = running.clone();
        // The camera is opened on its thread: on macOS it stays with the thread that made it
        std::thread::Builder::new()
            .name("camera".into())
            .spawn(move || {
                let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Closest(
                    CameraFormat::new_from(1280, 720, FrameFormat::NV12, 30),
                ));
                let mut camera = match Camera::new(CameraIndex::Index(0), format)
                    .and_then(|mut c| c.open_stream().map(|()| c))
                {
                    Ok(camera) => camera,
                    Err(e) => {
                        let _ = opened.send(Err(format!("media_failed: {e}")));
                        return;
                    }
                };
                let resolution = camera.camera_format().resolution();
                let source = NativeVideoSource::new(
                    VideoResolution {
                        width: resolution.width(),
                        height: resolution.height(),
                    },
                    false,
                );
                let _ = opened.send(Ok(source.clone()));
                read(&mut camera, &source, &still, &on_preview);
                let _ = camera.stop_stream();
            })
            .map_err(|e| e.to_string())?;
        let source = told
            .recv_timeout(Duration::from_secs(10))
            .map_err(|_| "media_failed: the camera did not open".to_string())??;
        Ok((Self { running }, source))
    }
}

/// The system's permission to use the camera (asked once; macOS remembers it).
fn allowed() -> Result<(), String> {
    if nokhwa::nokhwa_check() {
        return Ok(());
    }
    let (sent, answer) = mpsc::channel();
    nokhwa::nokhwa_initialize(move |granted| {
        let _ = sent.send(granted);
    });
    match answer.recv_timeout(Duration::from_secs(60)) {
        Ok(true) => Ok(()),
        _ => Err("media_failed: the camera is not allowed".into()),
    }
}

fn read(
    camera: &mut Camera,
    source: &NativeVideoSource,
    running: &AtomicBool,
    on_preview: &OnPreview,
) {
    let mut count = 0u32;
    let mut warned = false;
    while running.load(Ordering::Relaxed) {
        let buffer = match camera.frame() {
            Ok(buffer) => buffer,
            Err(e) => {
                tracing::warn!("camera: {e}");
                std::thread::sleep(Duration::from_millis(30));
                continue;
            }
        };
        let resolution = buffer.resolution();
        let Some(i420) = to_i420(
            buffer.buffer(),
            buffer.source_frame_format(),
            resolution.width(),
            resolution.height(),
        ) else {
            if !warned {
                tracing::warn!("camera format {:?} not read", buffer.source_frame_format());
                warned = true;
            }
            continue;
        };
        // Every other picture is enough for the user's own tile
        count = count.wrapping_add(1);
        let mut i420 = i420;
        if count.is_multiple_of(2) {
            on_preview(preview(&mut i420));
        }
        source.capture_frame(&VideoFrame::new(VideoRotation::VideoRotation0, i420));
    }
}

fn to_i420(data: &[u8], format: FrameFormat, width: u32, height: u32) -> Option<I420Buffer> {
    let (w, h) = (width as usize, height as usize);
    if w == 0 || h == 0 {
        return None;
    }
    let mut i420 = I420Buffer::new(width, height);
    let (stride_y, stride_u, stride_v) = i420.strides();
    let (y, u, v) = i420.data_mut();
    match format {
        FrameFormat::NV12 if data.len() >= w * h + w.div_ceil(2) * 2 * h.div_ceil(2) => {
            let (src_y, src_uv) = data.split_at(w * h);
            yuv_helper::nv12_to_i420(
                src_y,
                width,
                src_uv,
                width.div_ceil(2) * 2,
                y,
                stride_y,
                u,
                stride_u,
                v,
                stride_v,
                width as i32,
                height as i32,
            );
        }
        FrameFormat::YUYV => {
            let rgba = nokhwa::utils::yuyv422_to_rgb(data, true).ok()?;
            if rgba.len() < w * h * 4 {
                return None;
            }
            yuv_helper::abgr_to_i420(
                &rgba,
                width * 4,
                y,
                stride_y,
                u,
                stride_u,
                v,
                stride_v,
                width as i32,
                height as i32,
            );
        }
        _ => return None,
    }
    Some(i420)
}

/// The user's own picture, smaller and flipped as in a mirror.
fn preview(i420: &mut I420Buffer) -> Frame {
    let (width, height) = (i420.width(), i420.height());
    let small = width.min(PREVIEW_WIDTH);
    let small = i420.scale(small as i32, (height * small / width) as i32);
    let mut frame = to_rgba(&small);
    let row = frame.width as usize * 4;
    if let Some(rgba) = Arc::get_mut(&mut frame.rgba) {
        for line in rgba.chunks_exact_mut(row) {
            let pixels = line.len() / 4;
            for x in 0..pixels / 2 {
                for c in 0..4 {
                    line.swap(x * 4 + c, (pixels - 1 - x) * 4 + c);
                }
            }
        }
    }
    frame
}
