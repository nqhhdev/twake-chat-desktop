# M0 spike: rich-text timeline in Slint (throwaway)

Proved ADR-002: 10 000 `StyledText` bubbles in a `ListView`, first frame 211 ms, 43.9 MB
resident (release, macOS arm64, 2026-11-19). Kept for reference only; not built.
To run it again, put the two files back as `crates/twake-desktop/examples/` and
`crates/twake-desktop/ui/`, and compile `ui/spike_timeline.slint` in `build.rs`.
