//! M0 spike (ADR-002): 10 000 rich-text bubbles in a Slint ListView.
//! Prints time to first frame and resident memory, so it can be compared with
//! `e2e/perf-baseline.json` of the web app. Throwaway.

use std::rc::Rc;
use std::time::Instant;

slint::include_modules!();

const SAMPLES: &[&str] = &[
    "Hello **team**, the release is _today_.",
    "See the spec at [twake.app](https://twake.app) and `cargo build --release`.",
    "Three things:\n\n- the room list\n- the timeline\n- the composer",
    "A ~~cancelled~~ meeting, moved to <u>Thursday</u>.",
    "Plain text with an emoji 🎉 and a very long line that has to wrap inside the bubble because it is longer than the maximum width of a bubble in the timeline.",
];

fn main() -> Result<(), slint::PlatformError> {
    let started = Instant::now();
    let count: usize = std::env::args()
        .nth(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(10_000);

    let messages: Vec<SpikeMessage> = (0..count)
        .map(|i| SpikeMessage {
            sender: format!("User {}", i % 7).into(),
            body: slint::StyledText::from_markdown(SAMPLES[i % SAMPLES.len()])
                .unwrap_or_else(|_| slint::StyledText::from_plain_text(SAMPLES[i % SAMPLES.len()])),
            time: format!("{:02}:{:02}", (i / 60) % 24, i % 60).into(),
            own: i % 3 == 0,
        })
        .collect();

    let window = SpikeWindow::new()?;
    window.set_messages(Rc::new(slint::VecModel::from(messages)).into());
    window.on_link_clicked(|url| println!("link clicked: {url}"));

    let weak = window.as_weak();
    slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
        let ms = started.elapsed().as_millis();
        let rss = resident_mb();
        let line = format!("{count} messages · first frame {ms} ms · resident {rss:.1} MB");
        println!("{line}");
        if let Some(w) = weak.upgrade() {
            w.set_stats(line.into());
        }
    });
    window.run()
}

/// Resident memory of this process in MB (`ps`, so it works on macOS and Linux).
fn resident_mb() -> f64 {
    let pid = std::process::id().to_string();
    std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &pid])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse::<f64>().ok())
        .map_or(0.0, |kb| kb / 1024.0)
}
