//! Times and durations as the screens show them. `format_duration` ports
//! `formatDuration` of `src/domain/format.ts`; the others mirror the web's `Intl` use.

use chrono::{DateTime, Datelike, Duration, Local, TimeZone};

use crate::js;

/// `formatDuration`: `m:ss`, or `h:mm:ss` from one hour. Same arithmetic as the
/// original, NaN and Infinity included (`NaN:NaN`).
pub fn format_duration(milliseconds: f64) -> String {
    let rounded = js::math_round(milliseconds / 1000.0);
    // `Math.max(0, NaN)` is NaN.
    let total = if rounded.is_nan() {
        f64::NAN
    } else {
        rounded.max(0.0)
    };
    let seconds = pad2(&js::number_to_string(total % 60.0));
    let minutes = (total / 60.0).floor() % 60.0;
    let hours = (total / 3600.0).floor();
    if hours > 0.0 {
        format!(
            "{}:{}:{seconds}",
            js::number_to_string(hours),
            pad2(&js::number_to_string(minutes))
        )
    } else {
        format!("{}:{seconds}", js::number_to_string(minutes))
    }
}

/// `padStart(2, '0')`.
fn pad2(s: &str) -> String {
    if s.chars().count() < 2 {
        format!("0{s}")
    } else {
        s.to_string()
    }
}

/// `fitPreview`: the size an image takes in the timeline. Its own, shrunk to fit a
/// `max_side` box, never enlarged; a box of ratio `empty_ratio` (width over height, 1 by
/// default in the original) without dimensions; a very thin image keeps a quarter of the
/// box on its short side. `dimensions` is `None` where the original's is falsy or not
/// positive. NaN propagates as in JavaScript.
pub fn fit_preview(dimensions: Option<(f64, f64)>, max_side: f64, empty_ratio: f64) -> (f64, f64) {
    let Some((width, height)) = dimensions.filter(|(w, h)| !(*w <= 0.0 || *h <= 0.0)) else {
        return (max_side, js::math_round(max_side / empty_ratio));
    };
    let scale = js::math_min(&[1.0, max_side / width, max_side / height]);
    let min_side = js::math_round(max_side / 4.0);
    let fit = |side: f64| {
        js::math_max(&[
            js::math_round(side * scale),
            js::math_min(&[side, min_side]),
        ])
    };
    (fit(width), fit(height))
}

fn local(epoch_ms: i64) -> Option<DateTime<Local>> {
    Local.timestamp_millis_opt(epoch_ms).single()
}

/// Time of a message: `14:05`.
pub fn message_time(epoch_ms: i64) -> String {
    local(epoch_ms)
        .map(|t| t.format("%H:%M").to_string())
        .unwrap_or_default()
}

/// Time column of the room list: the time today, the weekday within a week,
/// else the date.
pub fn room_list_time(epoch_ms: i64, now_ms: i64) -> String {
    let (Some(t), Some(now)) = (local(epoch_ms), local(now_ms)) else {
        return String::new();
    };
    if t.date_naive() == now.date_naive() {
        t.format("%H:%M").to_string()
    } else if now.date_naive() - t.date_naive() < Duration::days(7) && t <= now {
        t.format("%a").to_string()
    } else if t.year() == now.year() {
        t.format("%d/%m").to_string()
    } else {
        t.format("%d/%m/%Y").to_string()
    }
}

/// Label of a day divider: `Today`, `Yesterday`, or the date.
pub fn day_label(epoch_ms: i64, now_ms: i64) -> String {
    let (Some(t), Some(now)) = (local(epoch_ms), local(now_ms)) else {
        return String::new();
    };
    let days = (now.date_naive() - t.date_naive()).num_days();
    match days {
        0 => "Today".into(),
        1 => "Yesterday".into(),
        _ if t.year() == now.year() => t.format("%A %-d %B").to_string(),
        _ => t.format("%A %-d %B %Y").to_string(),
    }
}

/// `describeEmote`: an action of the sender as it is drawn, « * alice waves ».
pub fn describe_emote(sender_name: &str, text: &str) -> String {
    format!("* {sender_name} {text}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations() {
        assert_eq!(format_duration(0.0), "0:00");
        assert_eq!(format_duration(65_000.0), "1:05");
        assert_eq!(format_duration(3_661_000.0), "1:01:01");
        assert_eq!(format_duration(1_500.0), "0:02"); // Math.round, ties up
        assert_eq!(format_duration(-5_000.0), "0:00");
        assert_eq!(format_duration(f64::NAN), "NaN:NaN");
        assert_eq!(format_duration(f64::INFINITY), "Infinity:NaN:NaN");
    }
}
