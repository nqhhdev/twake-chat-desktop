//! The audio player of the timeline (web `VoicePlayer`, `AudioPlayer`): one sound at a
//! time, played on a thread that owns the output device (rodio on cpal). symphonia reads
//! the containers (MP4, Ogg, WebM, MP3, FLAC, WAV) and their codecs; Opus, which it does
//! not decode, goes through libopus. A file is decoded as it plays, never whole.

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use rodio::Source;
use rodio::source::SeekError;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_OPUS, Decoder, DecoderOptions};
use symphonia::core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::Time;
use twake_core::Command;

use slint::ComponentHandle;

use crate::{AppWindow, AudioState, send};

/// The largest Opus frame: 120 ms at 48 kHz.
const OPUS_MAX_FRAME: usize = 5760;

enum Codec {
    Native(Box<dyn Decoder>),
    Opus(opus::Decoder),
}

/// A sound decoded as rodio asks for its samples (interleaved f32).
struct Track {
    format: Box<dyn FormatReader>,
    codec: Codec,
    track_id: u32,
    channels: u16,
    rate: u32,
    duration: Option<Duration>,
    buffer: Vec<f32>,
    at: usize,
}

impl Track {
    fn open(bytes: Arc<Vec<u8>>, mime: &str) -> Result<Self, String> {
        let source =
            MediaSourceStream::new(Box::new(Cursor::new(ArcBytes(bytes))), Default::default());
        let mut hint = Hint::new();
        hint.mime_type(mime);
        let probed = symphonia::default::get_probe()
            .format(
                &hint,
                source,
                &FormatOptions {
                    enable_gapless: true,
                    ..Default::default()
                },
                &MetadataOptions::default(),
            )
            .map_err(|e| e.to_string())?;
        let format = probed.format;
        let track = format.default_track().ok_or("no audio track")?.clone();
        let params = &track.codec_params;
        let channels = params.channels.map_or(1, |c| c.count()) as u16;
        let codec = if params.codec == CODEC_TYPE_OPUS {
            let layout = if channels > 1 {
                opus::Channels::Stereo
            } else {
                opus::Channels::Mono
            };
            Codec::Opus(opus::Decoder::new(48_000, layout).map_err(|e| e.to_string())?)
        } else {
            Codec::Native(
                symphonia::default::get_codecs()
                    .make(params, &DecoderOptions::default())
                    .map_err(|e| e.to_string())?,
            )
        };
        let rate = match codec {
            Codec::Opus(_) => 48_000,
            Codec::Native(_) => params.sample_rate.unwrap_or(48_000),
        };
        let duration = params.time_base.zip(params.n_frames).map(|(base, frames)| {
            let time = base.calc_time(frames);
            Duration::from_secs_f64(time.seconds as f64 + time.frac)
        });
        Ok(Self {
            format,
            codec,
            track_id: track.id,
            channels: channels.clamp(1, 2),
            rate,
            duration,
            buffer: Vec::new(),
            at: 0,
        })
    }

    /// Decodes the next packet of the track into the buffer; false at its end.
    fn refill(&mut self) -> bool {
        loop {
            let Ok(packet) = self.format.next_packet() else {
                return false;
            };
            if packet.track_id() != self.track_id {
                continue;
            }
            self.at = 0;
            self.buffer.clear();
            match &mut self.codec {
                Codec::Opus(decoder) => {
                    let channels = usize::from(self.channels);
                    let mut out = vec![0f32; OPUS_MAX_FRAME * channels];
                    match decoder.decode_float(&packet.data, &mut out, false) {
                        Ok(frames) => out.truncate(frames * channels),
                        Err(_) => continue,
                    }
                    self.buffer = out;
                }
                Codec::Native(decoder) => {
                    let Ok(decoded) = decoder.decode(&packet) else {
                        continue;
                    };
                    let spec = *decoded.spec();
                    let mut samples = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
                    samples.copy_interleaved_ref(decoded);
                    let source_channels = spec.channels.count().max(1);
                    let wanted = usize::from(self.channels);
                    // More than two channels: the first two
                    self.buffer = samples
                        .samples()
                        .chunks(source_channels)
                        .flat_map(|frame| frame.iter().copied().take(wanted))
                        .collect();
                }
            }
            if !self.buffer.is_empty() {
                return true;
            }
        }
    }
}

impl Iterator for Track {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if self.at >= self.buffer.len() && !self.refill() {
            return None;
        }
        let sample = self.buffer.get(self.at).copied();
        self.at += 1;
        sample
    }
}

impl Source for Track {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        self.channels
    }
    fn sample_rate(&self) -> u32 {
        self.rate
    }
    fn total_duration(&self) -> Option<Duration> {
        self.duration
    }
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.format
            .seek(
                SeekMode::Coarse,
                SeekTo::Time {
                    time: Time::from(pos.as_secs_f64()),
                    track_id: Some(self.track_id),
                },
            )
            .map_err(|e| SeekError::Other(Box::new(e)))?;
        match &mut self.codec {
            Codec::Native(decoder) => decoder.reset(),
            Codec::Opus(decoder) => {
                let _ = decoder.reset_state();
            }
        }
        self.buffer.clear();
        self.at = 0;
        Ok(())
    }
}

/// Shared bytes read by symphonia without a copy.
struct ArcBytes(Arc<Vec<u8>>);

impl AsRef<[u8]> for ArcBytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

enum Order {
    Play {
        key: String,
        bytes: Arc<Vec<u8>>,
        mime: String,
    },
    Pause,
    Resume,
    Seek(Duration),
    Stop,
}

/// What the player thread reports to the window.
#[derive(Clone, Default)]
struct Progress {
    key: String,
    playing: bool,
    position_ms: i64,
    duration_ms: i64,
    failed: bool,
}

/// The player thread: owns the output stream (not `Send` on every OS), reports the
/// position ten times a second while a sound plays.
fn run(orders: Receiver<Order>, report: impl Fn(Progress) + Send + 'static) {
    let Ok((_stream, handle)) = rodio::OutputStream::try_default() else {
        while let Ok(order) = orders.recv() {
            if let Order::Play { key, .. } = order {
                report(Progress {
                    key,
                    failed: true,
                    ..Progress::default()
                });
            }
        }
        return;
    };
    let mut sink: Option<rodio::Sink> = None;
    let mut now = Progress::default();
    loop {
        let order = if now.playing {
            match orders.recv_timeout(Duration::from_millis(100)) {
                Ok(order) => Some(order),
                Err(mpsc::RecvTimeoutError::Timeout) => None,
                Err(mpsc::RecvTimeoutError::Disconnected) => return,
            }
        } else {
            match orders.recv() {
                Ok(order) => Some(order),
                Err(_) => return,
            }
        };
        match order {
            Some(Order::Play { key, bytes, mime }) => {
                sink = None;
                match Track::open(bytes, &mime).and_then(|track| {
                    let duration = track.total_duration();
                    let new = rodio::Sink::try_new(&handle).map_err(|e| e.to_string())?;
                    new.append(track);
                    Ok((new, duration))
                }) {
                    Ok((new, duration)) => {
                        sink = Some(new);
                        now = Progress {
                            key,
                            playing: true,
                            position_ms: 0,
                            duration_ms: duration.map_or(0, |d| d.as_millis() as i64),
                            failed: false,
                        };
                    }
                    Err(e) => {
                        tracing_note(&e);
                        now = Progress {
                            key,
                            failed: true,
                            ..Progress::default()
                        };
                    }
                }
            }
            Some(Order::Pause) => {
                if let Some(s) = &sink {
                    s.pause();
                }
                now.playing = false;
            }
            Some(Order::Resume) => {
                if let Some(s) = &sink {
                    s.play();
                    now.playing = !s.empty();
                }
            }
            Some(Order::Seek(to)) => {
                if let Some(s) = &sink {
                    let _ = s.try_seek(to);
                    now.position_ms = to.as_millis() as i64;
                }
            }
            Some(Order::Stop) => {
                sink = None;
                now.playing = false;
                now.position_ms = 0;
            }
            None => {}
        }
        if let Some(s) = &sink {
            if s.empty() {
                // The end: back to the start, stopped
                sink = None;
                now.playing = false;
                now.position_ms = 0;
            } else {
                now.position_ms = s.get_pos().as_millis() as i64;
            }
        }
        report(now.clone());
    }
}

fn tracing_note(error: &str) {
    if std::env::var_os("TWAKE_DEV_STATS").is_some() {
        eprintln!("[dev] audio: {error}");
    }
}

#[derive(Default)]
struct Player {
    orders: Option<Sender<Order>>,
    /// Files fetched for playing, by key (a few voice messages, kept while the app runs).
    files: HashMap<String, Arc<Vec<u8>>>,
    /// The sound asked for while its file downloads.
    waiting: Option<(String, String)>,
}

thread_local! {
    static PLAYER: RefCell<Player> = RefCell::new(Player::default());
}

/// Starts the player thread; its reports set the window's audio properties.
pub fn install(window: &AppWindow) {
    let (orders, receive) = mpsc::channel();
    let weak = window.as_weak();
    std::thread::Builder::new()
        .name("audio".into())
        .spawn(move || {
            run(receive, move |p| {
                let weak = weak.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(w) = weak.upgrade() {
                        w.global::<AudioState>().set_key(p.key.as_str().into());
                        w.global::<AudioState>().set_playing(p.playing);
                        w.global::<AudioState>().set_position(p.position_ms as i32);
                        if p.duration_ms > 0 {
                            w.global::<AudioState>().set_duration(p.duration_ms as i32);
                        }
                        if p.failed {
                            w.set_notice("This audio cannot be played here: use Open.".into());
                        }
                    }
                });
            });
        })
        .ok();
    PLAYER.with_borrow_mut(|p| p.orders = Some(orders));
}

fn order(order: Order) {
    PLAYER.with_borrow(|p| {
        if let Some(orders) = &p.orders {
            let _ = orders.send(order);
        }
    });
}

/// Play or pause the sound of `key` (its file fetched first). `duration_ms` is what the
/// message says, shown until the file tells.
pub fn toggle(window: &AppWindow, key: &str, mime: &str, duration_ms: i32) {
    if window.global::<AudioState>().get_key() == key {
        if window.global::<AudioState>().get_playing() {
            order(Order::Pause);
            window.global::<AudioState>().set_playing(false);
        } else if window.global::<AudioState>().get_position() > 0 {
            order(Order::Resume);
            window.global::<AudioState>().set_playing(true);
        } else {
            start(window, key, mime, duration_ms);
        }
        return;
    }
    start(window, key, mime, duration_ms);
}

fn start(window: &AppWindow, key: &str, mime: &str, duration_ms: i32) {
    window.global::<AudioState>().set_key(key.into());
    window.global::<AudioState>().set_position(0);
    window.global::<AudioState>().set_duration(duration_ms);
    let file = PLAYER.with_borrow(|p| p.files.get(key).cloned());
    match file {
        Some(bytes) => {
            window.global::<AudioState>().set_playing(true);
            order(Order::Play {
                key: key.to_string(),
                bytes,
                mime: mime.to_string(),
            });
        }
        None => {
            window.global::<AudioState>().set_playing(false);
            order(Order::Stop);
            PLAYER.with_borrow_mut(|p| p.waiting = Some((key.to_string(), mime.to_string())));
            send(Command::FetchFile {
                key: key.to_string(),
            });
        }
    }
}

/// A file arrived: it plays if it is the one waited for.
pub fn file_arrived(window: &AppWindow, key: &str, bytes: Option<Arc<Vec<u8>>>) {
    let Some(bytes) = bytes else {
        let waited = PLAYER.with_borrow(|p| p.waiting.as_ref().is_some_and(|(k, _)| k == key));
        if waited {
            window.set_notice("The audio could not be downloaded.".into());
        }
        return;
    };
    let waited = PLAYER.with_borrow_mut(|p| {
        // A few files kept: the one played again does not download again
        if p.files.len() >= 8 {
            p.files.clear();
        }
        p.files.insert(key.to_string(), bytes.clone());
        match &p.waiting {
            Some((k, mime)) if k == key => {
                let mime = mime.clone();
                p.waiting = None;
                Some(mime)
            }
            _ => None,
        }
    });
    if let Some(mime) = waited
        && window.global::<AudioState>().get_key() == key
    {
        window.global::<AudioState>().set_playing(true);
        order(Order::Play {
            key: key.to_string(),
            bytes,
            mime,
        });
    }
}

/// A click on the progress bar: `fraction` of the sound.
pub fn seek(window: &AppWindow, key: &str, fraction: f32) {
    if window.global::<AudioState>().get_key() != key {
        return;
    }
    let to = (window.global::<AudioState>().get_duration().max(0) as f32 * fraction.clamp(0.0, 1.0))
        as u64;
    window.global::<AudioState>().set_position(to as i32);
    order(Order::Seek(Duration::from_millis(to)));
}

/// Another room: the sound stops.
pub fn stop(window: &AppWindow) {
    order(Order::Stop);
    window.global::<AudioState>().set_playing(false);
    window.global::<AudioState>().set_key("".into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays_back_a_recorded_voice_message() {
        let tone: Vec<f32> = (0..48_000)
            .map(|i| (i as f32 * 440.0 * std::f32::consts::TAU / 48_000.0).sin() * 0.3)
            .collect();
        let file = crate::voice::encode_ogg_opus(&tone).unwrap();
        let track = Track::open(Arc::new(file), "audio/ogg").unwrap();
        assert_eq!((track.channels(), track.sample_rate()), (1, 48_000));
        let samples: Vec<f32> = track.collect();
        // Every frame comes back (the pre-skip of the encoder is not cut here)
        assert!(
            (47_000..=50_000).contains(&samples.len()),
            "{}",
            samples.len()
        );
        let loudest = samples.iter().fold(0f32, |m, s| m.max(s.abs()));
        assert!(loudest > 0.2, "{loudest}");
    }

    /// The formats other clients send: AAC in MP4 (Safari, the phone app), Opus in WebM
    /// (Chrome), MP3 (dev/fixtures/audio, 2 s tones made with ffmpeg).
    #[test]
    fn reads_the_formats_of_other_clients() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dev/fixtures/audio");
        for (file, mime) in [
            ("tone.m4a", "audio/mp4"),
            ("tone.webm", "audio/webm"),
            ("tone.mp3", "audio/mpeg"),
        ] {
            let bytes = std::fs::read(dir.join(file)).unwrap();
            let track =
                Track::open(Arc::new(bytes), mime).unwrap_or_else(|e| panic!("{file}: {e}"));
            let (channels, rate) = (track.channels(), track.sample_rate());
            let samples = track.count() / usize::from(channels);
            let seconds = samples as f64 / f64::from(rate);
            assert!((1.8..=2.3).contains(&seconds), "{file}: {seconds} s");
        }
    }
}
