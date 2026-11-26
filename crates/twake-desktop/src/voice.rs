//! Voice messages (web `useVoiceMessage`, D54): the microphone recorded with cpal, mixed to
//! mono, encoded to Ogg Opus (the format Matrix clients play; no AAC encoder exists in
//! Rust) with its waveform, then sent as an MSC3245 voice message. One recording at a
//! time; it stops by itself after 30 minutes.

use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use rodio::cpal;
use rodio::cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use twake_core::Command;
use twake_domain::attachment::{
    MAX_VOICE_MS, MAX_WAVEFORM_VALUE, WAVEFORM_POINTS, compute_waveform,
};

use slint::ComponentHandle;

use crate::{AppWindow, send};

/// Opus works at 48 kHz; a frame is 20 ms.
const RATE: u32 = 48_000;
const FRAME: usize = 960;

/// `samples` at `from` Hz, at 48 kHz (linear interpolation: enough for a voice).
pub fn resample(samples: &[f32], from: u32) -> Vec<f32> {
    if from == RATE || samples.is_empty() || from == 0 {
        return samples.to_vec();
    }
    let ratio = f64::from(from) / f64::from(RATE);
    let count = (samples.len() as f64 / ratio).floor() as usize;
    (0..count)
        .map(|i| {
            let at = i as f64 * ratio;
            let before = at.floor() as usize;
            let after = (before + 1).min(samples.len() - 1);
            let part = (at - before as f64) as f32;
            samples[before] * (1.0 - part) + samples[after] * part
        })
        .collect()
}

/// Mono 48 kHz samples as an Ogg Opus file (RFC 7845).
pub fn encode_ogg_opus(samples: &[f32]) -> Result<Vec<u8>, String> {
    let mut encoder = opus::Encoder::new(RATE, opus::Channels::Mono, opus::Application::Voip)
        .map_err(|e| e.to_string())?;
    let pre_skip = encoder.get_lookahead().unwrap_or(312).max(0) as u16;
    let mut out = Vec::new();
    let mut writer = ogg::PacketWriter::new(&mut out);
    let serial = 0x7477_6b65; // any number: one stream in the file
    let mut head = Vec::with_capacity(19);
    head.extend_from_slice(b"OpusHead");
    head.push(1); // version
    head.push(1); // channels
    head.extend_from_slice(&pre_skip.to_le_bytes());
    head.extend_from_slice(&RATE.to_le_bytes());
    head.extend_from_slice(&0i16.to_le_bytes()); // output gain
    head.push(0); // mapping family: mono or stereo
    let io = |e: std::io::Error| e.to_string();
    writer
        .write_packet(head, serial, ogg::PacketWriteEndInfo::EndPage, 0)
        .map_err(io)?;
    let vendor = b"twake-chat";
    let mut tags = Vec::new();
    tags.extend_from_slice(b"OpusTags");
    tags.extend_from_slice(&(vendor.len() as u32).to_le_bytes());
    tags.extend_from_slice(vendor);
    tags.extend_from_slice(&0u32.to_le_bytes()); // no comments
    writer
        .write_packet(tags, serial, ogg::PacketWriteEndInfo::EndPage, 0)
        .map_err(io)?;
    let frames = samples.len().div_ceil(FRAME).max(1);
    let mut packet = vec![0u8; 4000];
    for index in 0..frames {
        let mut frame = [0f32; FRAME];
        let start = index * FRAME;
        let end = (start + FRAME).min(samples.len());
        if start < end {
            frame[..end - start].copy_from_slice(&samples[start..end]);
        }
        let size = encoder
            .encode_float(&frame, &mut packet)
            .map_err(|e| e.to_string())?;
        let last = index + 1 == frames;
        // The granule counts the samples played, pre-skip included; the last one where
        // the sound ends, so the padding of its frame is not played
        let granule = if last {
            u64::from(pre_skip) + samples.len() as u64
        } else {
            u64::from(pre_skip) + ((index + 1) * FRAME) as u64
        };
        let end_info = if last {
            ogg::PacketWriteEndInfo::EndStream
        } else {
            ogg::PacketWriteEndInfo::NormalPacket
        };
        writer
            .write_packet(packet[..size].to_vec(), serial, end_info, granule)
            .map_err(io)?;
    }
    drop(writer);
    Ok(out)
}

/// The recording on its way: the stream (kept on this thread) and what it heard.
struct Recording {
    _stream: cpal::Stream,
    samples: Arc<Mutex<Vec<f32>>>,
    rate: u32,
    started: Instant,
    thread: bool,
    reply_to: Option<String>,
    ticker: slint::Timer,
}

thread_local! {
    static RECORDING: RefCell<Option<Recording>> = const { RefCell::new(None) };
}

/// Mono samples of a frame of `channels` interleaved ones.
fn push_mono<T: Copy>(
    data: &[T],
    channels: usize,
    to_f32: impl Fn(T) -> f32,
    into: &Mutex<Vec<f32>>,
) {
    let Ok(mut samples) = into.lock() else { return };
    for frame in data.chunks(channels.max(1)) {
        let sum: f32 = frame.iter().map(|s| to_f32(*s)).sum();
        samples.push(sum / frame.len() as f32);
    }
}

fn open_microphone(samples: Arc<Mutex<Vec<f32>>>) -> Result<(cpal::Stream, u32), String> {
    let device = cpal::default_host()
        .default_input_device()
        .ok_or("No microphone found.")?;
    let config = device.default_input_config().map_err(|e| e.to_string())?;
    let rate = config.sample_rate().0;
    let channels = usize::from(config.channels());
    let format = config.sample_format();
    let config: cpal::StreamConfig = config.into();
    let failed = |e: cpal::StreamError| eprintln!("microphone: {e}");
    let stream = match format {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config,
            move |data: &[f32], _| push_mono(data, channels, |s| s, &samples),
            failed,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config,
            move |data: &[i16], _| push_mono(data, channels, |s| f32::from(s) / 32768.0, &samples),
            failed,
            None,
        ),
        cpal::SampleFormat::U16 => device.build_input_stream(
            &config,
            move |data: &[u16], _| {
                push_mono(
                    data,
                    channels,
                    |s| (f32::from(s) - 32768.0) / 32768.0,
                    &samples,
                )
            },
            failed,
            None,
        ),
        other => return Err(format!("microphone format {other} not supported")),
    }
    .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok((stream, rate))
}

fn clock(elapsed: Duration) -> String {
    let seconds = elapsed.as_secs();
    format!("{}:{:02}", seconds / 60, seconds % 60)
}

/// The microphone button: recording starts in the composer of the room or the thread.
pub fn start(window: &AppWindow, thread: bool, reply_to: Option<String>) {
    if RECORDING.with_borrow(|r| r.is_some()) {
        return;
    }
    let samples = Arc::new(Mutex::new(Vec::new()));
    let (stream, rate) = match open_microphone(samples.clone()) {
        Ok(opened) => opened,
        Err(e) => {
            window.set_notice(format!("Cannot record: {e}").into());
            return;
        }
    };
    let ticker = slint::Timer::default();
    let weak = window.as_weak();
    ticker.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(250),
        move || {
            let Some(w) = weak.upgrade() else { return };
            let elapsed = RECORDING.with_borrow(|r| r.as_ref().map(|r| r.started.elapsed()));
            if let Some(elapsed) = elapsed {
                w.set_recording_time(clock(elapsed).into());
                if elapsed.as_millis() as u64 >= MAX_VOICE_MS {
                    finish(&w, true);
                }
            }
        },
    );
    RECORDING.with_borrow_mut(|r| {
        *r = Some(Recording {
            _stream: stream,
            samples,
            rate,
            started: Instant::now(),
            thread,
            reply_to,
            ticker,
        });
    });
    window.set_recording_time("0:00".into());
    window.set_recording_in_thread(thread);
    window.set_recording(true);
}

/// Stops the recording: sent (`send`) or dropped.
pub fn finish(window: &AppWindow, send_it: bool) {
    let Some(recording) = RECORDING.with_borrow_mut(Option::take) else {
        return;
    };
    window.set_recording(false);
    recording.ticker.stop();
    let elapsed = recording.started.elapsed();
    let Recording {
        _stream,
        samples,
        rate,
        thread,
        reply_to,
        ..
    } = recording;
    drop(_stream);
    if !send_it {
        return;
    }
    let heard = samples.lock().map(|s| s.clone()).unwrap_or_default();
    if heard.is_empty() {
        window.set_notice("Nothing was recorded: check the microphone.".into());
        return;
    }
    let duration_ms = (elapsed.as_millis() as u64).min(MAX_VOICE_MS);
    send_samples(window, &heard, rate, duration_ms, thread, reply_to);
}

/// Encodes and sends samples as a voice message (also the dev hook's way in).
pub fn send_samples(
    window: &AppWindow,
    samples: &[f32],
    rate: u32,
    duration_ms: u64,
    thread: bool,
    reply_to: Option<String>,
) {
    let waveform: Vec<f32> = compute_waveform(samples, WAVEFORM_POINTS)
        .into_iter()
        .map(|v| v as f32 / MAX_WAVEFORM_VALUE as f32)
        .collect();
    let encoded = encode_ogg_opus(&resample(samples, rate));
    let written = encoded.and_then(|bytes| {
        let dir = std::env::temp_dir().join("twake-chat-voice");
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let name = chrono::Local::now()
            .format("Voice message %Y-%m-%d %H.%M.%S.ogg")
            .to_string();
        let path: PathBuf = dir.join(name);
        std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
        Ok(path)
    });
    match written {
        Ok(path) => send(Command::SendVoice {
            path,
            thread,
            reply_to,
            duration_ms,
            waveform,
        }),
        Err(e) => window.set_notice(format!("Voice message not sent: {e}").into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resamples_to_48k() {
        let one_second: Vec<f32> = vec![0.5; 44_100];
        let out = resample(&one_second, 44_100);
        assert!((out.len() as i64 - 48_000).abs() <= 1);
        assert!(out.iter().all(|s| (s - 0.5).abs() < 1e-6));
    }

    #[test]
    fn writes_an_ogg_opus_file() {
        let tone: Vec<f32> = (0..48_000)
            .map(|i| (i as f32 * 440.0 * std::f32::consts::TAU / 48_000.0).sin() * 0.3)
            .collect();
        let file = encode_ogg_opus(&tone).unwrap();
        assert_eq!(&file[..4], b"OggS");
        assert!(file.windows(8).any(|w| w == b"OpusHead"));
        assert!(file.windows(8).any(|w| w == b"OpusTags"));
        // One second of voice at the Opus default rate stays small
        assert!(file.len() < 40_000, "{} bytes", file.len());
    }
}
