//! The window a call floats in (web `domain/callWindow.ts`, D571): where it stands, how
//! large it is, and when the call shows in it rather than in its room.

use crate::js::{math_max, math_min};

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WindowRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// The room the window lives in: the app's window, or the screen for a window of its own.
#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize)]
pub struct WindowViewport {
    pub width: f64,
    pub height: f64,
}

/// The smallest window; resized only by its edges (D645).
const LEAST_WIDTH: f64 = 320.0;
const LEAST_HEIGHT: f64 = 260.0;
const START_WIDTH: f64 = 420.0;
const START_HEIGHT: f64 = 340.0;
/// At the start only: off the edge, above the send button.
const START_RIGHT: f64 = 16.0;
const START_BOTTOM: f64 = 88.0;

fn within(value: f64, least: f64, most: f64) -> f64 {
    math_max(&[least, math_min(&[value, most])])
}

/// The window whole on the screen, whatever it was given (web `fitWindow`).
pub fn fit_window(rect: WindowRect, screen: WindowViewport) -> WindowRect {
    let width = math_min(&[math_max(&[rect.width, LEAST_WIDTH]), screen.width]);
    let height = math_min(&[math_max(&[rect.height, LEAST_HEIGHT]), screen.height]);
    WindowRect {
        x: within(rect.x, 0.0, screen.width - width),
        y: within(rect.y, 0.0, screen.height - height),
        width,
        height,
    }
}

/// Where the window opens the first time: the bottom right corner (web `defaultCallWindow`).
pub fn default_call_window(screen: WindowViewport) -> WindowRect {
    fit_window(
        WindowRect {
            x: screen.width - START_WIDTH - START_RIGHT,
            y: screen.height - START_HEIGHT - START_BOTTOM,
            width: START_WIDTH,
            height: START_HEIGHT,
        },
        screen,
    )
}

/// The window dragged by its bar (web `moveWindow`).
pub fn move_window(rect: WindowRect, dx: f64, dy: f64, screen: WindowViewport) -> WindowRect {
    fit_window(
        WindowRect {
            x: rect.x + dx,
            y: rect.y + dy,
            ..rect
        },
        screen,
    )
}

/// The window in a corner: `top-left`, `top-right`, `bottom-left` or `bottom-right` (web
/// `placeInCorner`).
pub fn place_in_corner(rect: WindowRect, corner: &str, screen: WindowViewport) -> WindowRect {
    fit_window(
        WindowRect {
            x: if corner.ends_with("left") {
                0.0
            } else {
                screen.width
            },
            y: if corner.starts_with("top") {
                0.0
            } else {
                screen.height
            },
            ..rect
        },
        screen,
    )
}

/// One axis of a resize: `is_start` pulls its left or top side.
fn resize_axis(
    start: f64,
    length: f64,
    delta: f64,
    (is_start, is_end): (bool, bool),
    least: f64,
    screen: f64,
) -> (f64, f64) {
    if is_start {
        let end = start + length;
        let next = within(start + delta, 0.0, end - least);
        return (next, end - next);
    }
    if is_end {
        return (start, within(length + delta, least, screen - start));
    }
    (start, length)
}

/// The window resized by `edge`, the side or corner pulled: `n`, `s`, `e`, `w`, `nw`, `ne`,
/// `sw` or `se` (web `resizeWindow`).
pub fn resize_window(
    rect: WindowRect,
    edge: &str,
    dx: f64,
    dy: f64,
    screen: WindowViewport,
) -> WindowRect {
    let (x, width) = resize_axis(
        rect.x,
        rect.width,
        dx,
        (edge.contains('w'), edge.contains('e')),
        LEAST_WIDTH,
        screen.width,
    );
    let (y, height) = resize_axis(
        rect.y,
        rect.height,
        dy,
        (edge.contains('n'), edge.contains('s')),
        LEAST_HEIGHT,
        screen.height,
    );
    WindowRect {
        x,
        y,
        width,
        height,
    }
}

/// The window as it was kept, once read: anything else is forgotten (web `parseCallWindow`).
pub fn parse_call_window(stored: Option<&str>) -> Option<WindowRect> {
    let stored = stored.filter(|s| !s.is_empty())?;
    let value: serde_json::Value = serde_json::from_str(stored).ok()?;
    let number = |key: &str| value.get(key)?.as_f64().filter(|n| n.is_finite());
    Some(WindowRect {
        x: number("x")?,
        y: number("y")?,
        width: number("width")?,
        height: number("height")?,
    })
}

/// Where the call shows (web `CallPlacement`).
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallPlacement {
    /// The room of the call that floats; the room shown stays free.
    pub detached_room_id: Option<String>,
    /// The room on screen, as last seen.
    pub shown_room_id: Option<String>,
}

/// The placement once the screen shows `shown_room_id`. Leaving the room of the call
/// detaches it and coming back does not put it back; arriving in it, as an answered call
/// does, keeps it large (web `placeCall`).
pub fn place_call(
    state: &CallPlacement,
    shown_room_id: Option<&str>,
    call_room_id: Option<&str>,
) -> CallPlacement {
    let has_left_it = call_room_id.is_some()
        && state.shown_room_id.as_deref() == call_room_id
        && shown_room_id != call_room_id;
    let stays = call_room_id.is_some() && state.detached_room_id.as_deref() == call_room_id;
    CallPlacement {
        shown_room_id: shown_room_id.map(str::to_string),
        detached_room_id: if has_left_it || stays {
            call_room_id.map(str::to_string)
        } else {
            None
        },
    }
}

/// The call shows in its window, not in its room (web `isCallFloating`).
pub fn is_call_floating(state: &CallPlacement, call_room_id: Option<&str>) -> bool {
    call_room_id.is_some()
        && (state.detached_room_id.as_deref() == call_room_id
            || state.shown_room_id.as_deref() != call_room_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN: WindowViewport = WindowViewport {
        width: 1600.0,
        height: 900.0,
    };

    fn rect(x: f64, y: f64, width: f64, height: f64) -> WindowRect {
        WindowRect {
            x,
            y,
            width,
            height,
        }
    }

    #[test]
    fn opens_at_the_bottom_right_above_the_composer() {
        assert_eq!(
            default_call_window(SCREEN),
            rect(1164.0, 472.0, 420.0, 340.0)
        );
    }

    #[test]
    fn stays_whole_on_the_screen() {
        assert_eq!(
            move_window(rect(1164.0, 472.0, 420.0, 340.0), 500.0, -900.0, SCREEN),
            rect(1180.0, 0.0, 420.0, 340.0)
        );
        assert_eq!(
            place_in_corner(rect(10.0, 10.0, 420.0, 340.0), "bottom-left", SCREEN),
            rect(0.0, 560.0, 420.0, 340.0)
        );
    }

    #[test]
    fn resizes_by_its_edges_never_below_the_least() {
        // The left side pulled right: the right side stays
        assert_eq!(
            resize_window(rect(100.0, 100.0, 420.0, 340.0), "w", 300.0, 0.0, SCREEN),
            rect(200.0, 100.0, 320.0, 340.0)
        );
        assert_eq!(
            resize_window(rect(100.0, 100.0, 420.0, 340.0), "se", 50.0, 20.0, SCREEN),
            rect(100.0, 100.0, 470.0, 360.0)
        );
    }

    #[test]
    fn reads_only_a_whole_kept_window() {
        assert_eq!(
            parse_call_window(Some(r#"{"x":1,"y":2,"width":400,"height":300}"#)),
            Some(rect(1.0, 2.0, 400.0, 300.0))
        );
        assert_eq!(
            parse_call_window(Some(r#"{"x":1,"y":2,"width":"4"}"#)),
            None
        );
        assert_eq!(parse_call_window(Some("not json")), None);
        assert_eq!(parse_call_window(None), None);
    }

    #[test]
    fn floats_once_its_room_is_left_until_expanded() {
        let mut state = CallPlacement::default();
        state = place_call(&state, Some("!call"), Some("!call"));
        assert!(!is_call_floating(&state, Some("!call")));
        state = place_call(&state, Some("!other"), Some("!call"));
        assert!(is_call_floating(&state, Some("!call")));
        // Coming back does not put it back
        state = place_call(&state, Some("!call"), Some("!call"));
        assert!(is_call_floating(&state, Some("!call")));
        assert!(!is_call_floating(&state, None));
    }
}
