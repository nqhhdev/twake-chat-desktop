//! The media of calls (M5, ADR-018): a LiveKit room joined with the token of the
//! MatrixRTC focus, the microphone and speakers of the system (libwebrtc's audio device
//! module), the camera, and the video of the others as RGBA frames for the grid.
//!
//! An encrypted call takes one key per participant, as MatrixRTC shares them: the web's
//! `MatrixKeyProvider` (`LiveKitCallMedia.ts`), no ratchet, keys derived with HKDF.

mod camera;
mod screen;

pub use screen::Screen;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use futures_util::StreamExt;
use livekit::e2ee::key_provider::{KeyDerivationAlgorithm, KeyProvider, KeyProviderOptions};
use livekit::e2ee::{E2eeOptions, EncryptionType};
use livekit::options::TrackPublishOptions;
use livekit::prelude::*;
use livekit::webrtc::native::yuv_helper;
use livekit::webrtc::prelude::VideoBuffer;
use livekit::webrtc::video_stream::native::NativeVideoStream;
use tokio::task::JoinHandle;

/// The widest frame handed to the window: a tile is never wider (pixels).
const MAX_FRAME_WIDTH: u32 = 960;
/// A shared screen keeps more of its detail: its text is read.
const MAX_SCREEN_WIDTH: u32 = 1920;
/// The tile of a shared screen: its sharer's identity and this (web `SCREEN_SUFFIX`).
pub const SCREEN_SUFFIX: &str = "#screen";

/// A picture of a video, RGBA.
#[derive(Clone, Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub rgba: Arc<Vec<u8>>,
}

/// A person of the call grid (web `CallTile`), by the identity LiveKit knows them by
/// (`<user>:<device>`, from the token service).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub identity: String,
    pub is_local: bool,
    /// The screen someone shares, after their own tile (identity + [`SCREEN_SUFFIX`]).
    pub is_screen: bool,
    pub has_video: bool,
    pub is_muted: bool,
    pub is_speaking: bool,
}

#[derive(Debug)]
pub enum MediaEvent {
    /// Who is in the call now, the user first.
    Tiles(Vec<Tile>),
    /// A new picture of the video of `identity`.
    Frame { identity: String, frame: Frame },
    /// The call server dropped the call, with why.
    Ended(String),
}

pub type OnEvent = Arc<dyn Fn(MediaEvent) + Send + Sync>;

/// What the window shows of who is speaking and who has video, kept by the room's task.
#[derive(Default)]
struct Shared {
    speaking: HashSet<String>,
    /// Remote video tasks, by track.
    videos: HashMap<TrackSid, JoinHandle<()>>,
    local_video: bool,
    local_muted: bool,
    local_screen: bool,
}

/// A call joined. Dropping it leaves.
pub struct Media {
    room: Arc<Room>,
    keys: Option<KeyProvider>,
    _audio: Option<PlatformAudio>,
    microphone: Option<(LocalAudioTrack, TrackSid)>,
    camera: Option<(camera::Capture, TrackSid)>,
    screen: Option<(Screen, TrackSid)>,
    shared: Arc<Mutex<Shared>>,
    on_event: OnEvent,
    events: JoinHandle<()>,
}

impl Media {
    /// Joins the LiveKit room of `url` with `token`; `encrypted`: the media is encrypted
    /// end to end, with the keys given by [`Media::set_key`].
    pub async fn connect(
        url: &str,
        token: &str,
        encrypted: bool,
        microphone: bool,
        on_event: OnEvent,
    ) -> Result<Self, String> {
        let keys = encrypted.then(|| {
            KeyProvider::new(KeyProviderOptions {
                ratchet_window_size: 0,
                key_ring_size: livekit::webrtc::native::frame_cryptor::MAX_KEY_RING_SIZE,
                key_derivation_algorithm: KeyDerivationAlgorithm::HKDF,
                ..KeyProviderOptions::default()
            })
        });
        let mut options = RoomOptions::default();
        options.auto_subscribe = true;
        options.adaptive_stream = true;
        options.dynacast = true;
        options.encryption = keys.clone().map(|key_provider| E2eeOptions {
            encryption_type: EncryptionType::Gcm,
            key_provider,
        });
        let (room, events) = Room::connect(url, token, options)
            .await
            .map_err(|e| format!("connection_failed: {e}"))?;
        let room = Arc::new(room);
        if encrypted {
            room.e2ee_manager().set_enabled(true);
        }
        let shared = Arc::new(Mutex::new(Shared::default()));
        let events = tokio::spawn(watch(
            room.clone(),
            events,
            shared.clone(),
            on_event.clone(),
        ));
        let mut media = Self {
            room,
            keys,
            _audio: None,
            microphone: None,
            camera: None,
            screen: None,
            shared,
            on_event,
            events,
        };
        // The microphone opens with the call; without it the user still hears
        match PlatformAudio::new() {
            Ok(audio) => {
                let track = LocalAudioTrack::create_audio_track("microphone", audio.rtc_source());
                let options = TrackPublishOptions {
                    source: TrackSource::Microphone,
                    ..TrackPublishOptions::default()
                };
                match media
                    .room
                    .local_participant()
                    .publish_track(LocalTrack::Audio(track.clone()), options)
                    .await
                {
                    Ok(publication) => media.microphone = Some((track, publication.sid())),
                    Err(e) => tracing::warn!("microphone not published: {e}"),
                }
                media._audio = Some(audio);
            }
            Err(e) => tracing::warn!("no audio devices: {e}"),
        }
        media.set_microphone(microphone);
        Ok(media)
    }

    /// A media key of a participant (`<user>:<device>`), as MatrixRTC shares it.
    pub fn set_key(&self, identity: &str, index: u8, key: Vec<u8>) {
        if let Some(keys) = &self.keys {
            keys.set_key(
                &ParticipantIdentity(identity.to_string()),
                i32::from(index),
                key,
            );
        }
    }

    pub fn set_microphone(&mut self, on: bool) {
        if let Some((track, _)) = &self.microphone {
            if on {
                track.unmute();
            } else {
                track.mute();
            }
        }
        if let Ok(mut s) = self.shared.lock() {
            s.local_muted = !on || self.microphone.is_none();
        }
        self.report();
    }

    /// Opens or closes the camera; an error when no camera opens.
    pub async fn set_camera(&mut self, on: bool) -> Result<(), String> {
        if !on {
            if let Some((capture, sid)) = self.camera.take() {
                drop(capture);
                let _ = self.room.local_participant().unpublish_track(&sid).await;
            }
        } else if self.camera.is_none() {
            let identity = self.room.local_participant().identity().0;
            let on_event = self.on_event.clone();
            // Asking the system's permission waits for the user: off the async threads
            let on_preview: camera::OnPreview = Arc::new(move |frame| {
                on_event(MediaEvent::Frame {
                    identity: identity.clone(),
                    frame,
                });
            });
            let (capture, source) =
                tokio::task::spawn_blocking(move || camera::Capture::start(on_preview))
                    .await
                    .map_err(|e| e.to_string())??;
            let track = LocalVideoTrack::create_video_track(
                "camera",
                livekit::webrtc::video_source::RtcVideoSource::Native(source),
            );
            let options = TrackPublishOptions {
                source: TrackSource::Camera,
                ..TrackPublishOptions::default()
            };
            let publication = self
                .room
                .local_participant()
                .publish_track(LocalTrack::Video(track), options)
                .await
                .map_err(|e| e.to_string())?;
            self.camera = Some((capture, publication.sid()));
        }
        if let Ok(mut s) = self.shared.lock() {
            s.local_video = self.camera.is_some();
        }
        self.report();
        Ok(())
    }

    /// What shows the system's picker and reads the screen chosen ([`Screen::pick`]):
    /// run it off the async threads, it waits for the user. `on_end`: the system stopped
    /// the sharing.
    pub fn screen_picker(
        &self,
        on_end: Arc<dyn Fn() + Send + Sync>,
    ) -> impl FnOnce() -> Option<Screen> + Send + 'static {
        let identity = self.room.local_participant().identity().0 + SCREEN_SUFFIX;
        let on_event = self.on_event.clone();
        let on_preview: camera::OnPreview = Arc::new(move |frame| {
            on_event(MediaEvent::Frame {
                identity: identity.clone(),
                frame,
            });
        });
        move || Screen::pick(on_preview, on_end)
    }

    /// Shares the screen chosen, in place of any shared before.
    pub async fn share_screen(&mut self, screen: Screen) -> Result<(), String> {
        self.stop_screen().await;
        let track = LocalVideoTrack::create_video_track(
            "screen",
            livekit::webrtc::video_source::RtcVideoSource::Native(screen.source.clone()),
        );
        let options = TrackPublishOptions {
            source: TrackSource::Screenshare,
            ..TrackPublishOptions::default()
        };
        let publication = self
            .room
            .local_participant()
            .publish_track(LocalTrack::Video(track), options)
            .await
            .map_err(|e| e.to_string())?;
        self.screen = Some((screen, publication.sid()));
        self.screen_changed();
        Ok(())
    }

    pub async fn stop_screen(&mut self) {
        if let Some((screen, sid)) = self.screen.take() {
            drop(screen);
            let _ = self.room.local_participant().unpublish_track(&sid).await;
        }
        self.screen_changed();
    }

    fn screen_changed(&self) {
        if let Ok(mut s) = self.shared.lock() {
            s.local_screen = self.screen.is_some();
        }
        self.report();
    }

    fn report(&self) {
        (self.on_event)(MediaEvent::Tiles(tiles(&self.room, &self.shared)));
    }

    pub async fn leave(mut self) {
        self.camera = None;
        self.screen = None;
        self.events.abort();
        let _ = self.room.close().await;
    }
}

impl Drop for Media {
    fn drop(&mut self) {
        self.events.abort();
        if let Ok(mut s) = self.shared.lock() {
            s.videos.drain().for_each(|(_, task)| task.abort());
        }
    }
}

/// The people of the call: the user, then the others by identity.
fn tiles(room: &Room, shared: &Mutex<Shared>) -> Vec<Tile> {
    let Ok(s) = shared.lock() else {
        return Vec::new();
    };
    let local = room.local_participant().identity().0;
    let own = Tile {
        is_speaking: s.speaking.contains(&local),
        identity: local,
        is_local: true,
        is_screen: false,
        has_video: s.local_video,
        is_muted: s.local_muted,
    };
    let mut tiles = with_screen(own, s.local_screen);
    let mut others: Vec<Vec<Tile>> = room
        .remote_participants()
        .values()
        .map(|p| {
            let publications = p.track_publications();
            let of = |source| {
                publications
                    .values()
                    .find(|t| t.source() == source)
                    .cloned()
            };
            let identity = p.identity().0;
            let person = Tile {
                is_speaking: s.speaking.contains(&identity),
                has_video: of(TrackSource::Camera).is_some_and(|t| !t.is_muted()),
                is_muted: of(TrackSource::Microphone).is_none_or(|t| t.is_muted()),
                is_local: false,
                is_screen: false,
                identity,
            };
            let sharing = of(TrackSource::Screenshare).is_some_and(|t| t.track().is_some());
            with_screen(person, sharing)
        })
        .collect();
    others.sort_by(|a, b| a[0].identity.cmp(&b[0].identity));
    tiles.extend(others.into_iter().flatten());
    tiles
}

/// A person's tile, and after it the screen they share (web `tilesOf`).
fn with_screen(person: Tile, sharing: bool) -> Vec<Tile> {
    let screen = sharing.then(|| Tile {
        identity: person.identity.clone() + SCREEN_SUFFIX,
        is_screen: true,
        has_video: true,
        ..person.clone()
    });
    std::iter::once(person).chain(screen).collect()
}

async fn watch(
    room: Arc<Room>,
    mut events: tokio::sync::mpsc::UnboundedReceiver<RoomEvent>,
    shared: Arc<Mutex<Shared>>,
    on_event: OnEvent,
) {
    on_event(MediaEvent::Tiles(tiles(&room, &shared)));
    while let Some(event) = events.recv().await {
        match event {
            RoomEvent::TrackSubscribed {
                track: RemoteTrack::Video(track),
                participant,
                publication,
            } => {
                let is_screen = publication.source() == TrackSource::Screenshare;
                let mut identity = participant.identity().0;
                if is_screen {
                    identity.push_str(SCREEN_SUFFIX);
                }
                let widest = if is_screen {
                    MAX_SCREEN_WIDTH
                } else {
                    MAX_FRAME_WIDTH
                };
                let stream = NativeVideoStream::new(track.rtc_track());
                let task = tokio::spawn(show_video(stream, identity, widest, on_event.clone()));
                if let Ok(mut s) = shared.lock()
                    && let Some(old) = s.videos.insert(publication.sid(), task)
                {
                    old.abort();
                }
            }
            RoomEvent::TrackUnsubscribed { publication, .. } => {
                if let Ok(mut s) = shared.lock()
                    && let Some(task) = s.videos.remove(&publication.sid())
                {
                    task.abort();
                }
            }
            RoomEvent::ActiveSpeakersChanged { speakers } => {
                if let Ok(mut s) = shared.lock() {
                    s.speaking = speakers.iter().map(|p| p.identity().0).collect();
                }
            }
            RoomEvent::Disconnected { reason } => {
                on_event(MediaEvent::Ended(format!("{reason:?}")));
                return;
            }
            RoomEvent::ParticipantConnected(_)
            | RoomEvent::ParticipantDisconnected(_)
            | RoomEvent::TrackPublished { .. }
            | RoomEvent::TrackUnpublished { .. }
            | RoomEvent::TrackMuted { .. }
            | RoomEvent::TrackUnmuted { .. } => {}
            _ => continue,
        }
        on_event(MediaEvent::Tiles(tiles(&room, &shared)));
    }
}

/// Hands each picture of a remote video to the window, at most a tile wide.
async fn show_video(
    mut stream: NativeVideoStream,
    identity: String,
    widest: u32,
    on_event: OnEvent,
) {
    while let Some(frame) = stream.next().await {
        let mut i420 = frame.buffer.to_i420();
        let (width, height) = (i420.width(), i420.height());
        if width == 0 || height == 0 {
            continue;
        }
        if width > widest {
            let scaled_height = height * widest / width;
            i420 = i420.scale(widest as i32, scaled_height as i32);
        }
        on_event(MediaEvent::Frame {
            identity: identity.clone(),
            frame: to_rgba(&i420),
        });
    }
}

fn to_rgba(i420: &livekit::webrtc::video_frame::I420Buffer) -> Frame {
    let (width, height) = (i420.width(), i420.height());
    let (stride_y, stride_u, stride_v) = i420.strides();
    let (y, u, v) = i420.data();
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    // libyuv's ABGR is R, G, B, A in memory
    yuv_helper::i420_to_abgr(
        y,
        stride_y,
        u,
        stride_u,
        v,
        stride_v,
        &mut rgba,
        width * 4,
        width as i32,
        height as i32,
    );
    Frame {
        width,
        height,
        rgba: Arc::new(rgba),
    }
}
