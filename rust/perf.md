# Speed and memory (ADR-010)

## Web baseline

From `reference/twake-space-chat/e2e/perf-baseline.json` (2026-10-31, MacBook Pro, local e2e
stack, median of 3). Times in ms, heap in MB of JS heap only (not the whole tab).

| Scenario | Web |
| --- | ---: |
| Room list, 300 rooms, shown | 1433 ms |
| Room list, 300 rooms, all rooms | 2294 ms |
| JS heap at rest, 300 rooms | 132.6 MB |
| Room open (normal) | 237.7 ms |
| Room open (empty window) | 171.4 ms |
| Encrypted room, first read | 1487 ms |
| Encrypted heap | 168.8 MB |
| History page (max) | 1428 ms |
| Search over 5000 messages | 1128 ms |
| Thread reply shown (clear / encrypted) | 1214 / 907 ms |
| JS loaded | 3192.7 ms |

## Rust app

| Date | Build | Scenario | Result | Notes |
| --- | --- | --- | ---: | --- |
| 2026-11-19 | M0 spike, release, macOS arm64 | 10 000 rich-text bubbles, first frame | 211 ms | `examples/spike_timeline`, no network |
| 2026-11-19 | M0 spike, release, macOS arm64 | same, resident memory (whole process) | 43.9 MB | `ps -o rss` |
| 2026-11-19 | M1, release, macOS arm64 | 300 rooms, fresh sign-in: first row (from process start, password sign-in 753 ms included) | 930 ms | web `room_list_300`: 1433 ms, sign-in not included |
| 2026-11-19 | M1, release, macOS arm64 | 300 rooms, fresh sign-in: all 300 listed | 2410 ms | web: 1433 + 2294 ms of scrolling (`room_list_300_all_rooms`) |
| 2026-11-19 | M1, release, macOS arm64 | 300 rooms, restart with the local store: all 300 listed | 233 ms | the web keeps no room list between loads |
| 2026-11-19 | M1, release, macOS arm64 | resident memory, 300 rooms, restart, one room open | 158.8 MB | whole process; web `heap_rest_300` is 132.6 MB of JS heap alone |
| 2026-11-19 | M1, release, macOS arm64 | resident memory, 300 rooms, fresh sign-in | 199.0 MB | includes the initial sync |
| 2026-11-19 | M1, release, macOS arm64 | encrypted room on a new device: messages readable after the recovery key | < 12 s, not timed finer | web `encrypted_first_read`: 1487 ms; needs a precise measure |

The M0 numbers only show that Slint's virtualised list and `StyledText` hold 10 000 bubbles
cheaply (no server, no sync). From M1 on, the scenarios follow the web's `e2e/tests/perf-large`.

Measured with `dev/seed-large.sh` (user `perf`, 300 rooms, one message each) and
`TWAKE_DEV_EXPECT_ROOMS=300 dev/smoke.sh perf "Channel 1"` (add `FRESH=1` for a new device).
Same machine class as the web baseline (MacBook Pro, Apple silicon) but not the same machine.

### Not comparable yet

The web's memory numbers are the JS heap of the tab only. A fair comparison needs the whole
browser footprint for the same account (renderer + GPU + browser processes), measured on this
machine; not done yet. The Rust number above is the whole process.
