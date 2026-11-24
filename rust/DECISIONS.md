# Decisions (ADRs)

Source of truth for the native Rust rewrite of Twake Chat. Numbered in order, never
renumbered; a changed decision gets a new ADR that supersedes the old one.

The reference is `reference/twake-space-chat` (see `reference/SOURCE.txt` for the commit).
Its own `DECISION.md` (D1..D7xx) stays the product spec: what the screen does and why.
These ADRs only cover how the desktop app is built.

## ADR-001: Full native Rust rewrite, not a web shell
Status: accepted (2026-11-19)
Context: the goal is speed and memory. A Tauri/Electron shell would keep the React UI,
matrix-js-sdk and the WASM crypto in a webview, so most of the cost stays.
Decision: a native desktop app for macOS, Windows and Linux, written in Rust end to end,
no webview, no JavaScript.
Consequences: every screen is rewritten. The React app is the reference, not a dependency.

## ADR-002: UI toolkit is Slint
Status: accepted (2026-11-19)
Context: candidates were Slint, iced and GPUI. The web app is AA-accessible and has a design
system (`src/ds`) of declarative components.
Decision: Slint 1.x (GPLv3 option, compatible with the AGPL-3.0 of the app). Declarative
`.slint` components mirror `src/ds`; AccessKit gives screen-reader support.
Consequences: rich text in bubbles (bold, links, mentions, code) is the main risk. M0 proves
it before anything else is built on Slint. If it fails, a new ADR picks another toolkit.

## ADR-003: Matrix engine is matrix-rust-sdk
Status: accepted (2026-11-19)
Context: matrix-js-sdk + matrix-sdk-crypto-wasm is what the web app uses. The Rust SDK is the
same crypto, native, and ships Sliding Sync, a room list and a timeline (Element X uses it).
Decision: `matrix-sdk` + `matrix-sdk-ui` 0.19 (`SyncService`, `RoomListService`, `Timeline`),
with the encrypted SQLite store. The store passphrase lives in the OS keychain.
Consequences: the web adapters in `src/adapters/matrix` are replaced, not ported. The
application ports of `src/application/ports` are the contract the new adapters satisfy.

## ADR-004: Sign-in through the system browser; session survives restarts
Status: accepted (2026-11-19)
Context: the web app keeps the session in `sessionStorage` (lost when the tab closes) and signs
in with Synapse SSO / the Twake identity provider.
Decision: SSO / OIDC in the system browser, redirect to a loopback HTTP listener on 127.0.0.1.
The Matrix session (access token, device id) is stored in the OS keychain (`keyring` crate)
and restored at start. Password login stays available for the local test server.
Consequences: a deliberate behaviour change from the web app, normal for a desktop app.

## ADR-005: Milestones
Status: accepted (2026-11-19)
Decision:
- M0: Slint rich-text timeline spike; speed baseline from `e2e/perf-baseline.json`.
- M1: sign-in, room list, timeline, send, E2EE + recovery key from ToM.
- M2: threads, reactions, edits, files and media, notifications.
- M3: room info and admin, contacts, local search, polls.
- M4: calls over LiveKit.
- M5: assistant, bots and Scribe; Drive, Meet, mail, calendar; analytics.
- M6: installers for macOS, Windows, Linux.
Consequences: each milestone ends with an app that builds and runs.

## ADR-006: Web-only features dropped or replaced
Status: accepted (2026-11-19)
Decision: no Twake Space embed (`/embed/...`), no `@linagora/twake-bar`; the app always draws
its own frame (D29). Web push is replaced by OS notifications.

## ADR-007: Layout of the code
Status: accepted (2026-11-19)
Decision: a Cargo workspace in this folder.
- `crates/twake-domain`: port of `src/domain` (pure logic, no I/O). Ported with rust-anything
  where its recording applies, checked against the TS original.
- `crates/twake-core`: application layer (use cases, ports) and adapters (matrix-sdk, ToM,
  LiveKit, side-service, ...). No UI types.
- `crates/twake-desktop`: the Slint app (binary `twake-chat`).
Port documents (`MAP.md`, `DECISIONS.md`, `GLOSSARY.md`, `HANDOFF.md`) live in `rust/`.
Git is local; nothing is committed or pushed without the user asking.

## ADR-008: Test against a local Synapse in Docker
Status: accepted (2026-11-19)
Decision: use the reference's `e2e/docker` stack (Synapse, ToM, LiveKit, ...) and its seed
scripts (`e2e/scripts/seed*.mjs`) for manual and integration tests. No real accounts.

## ADR-009: Checkpoints
Status: accepted (2026-11-19)
Decision: report at the end of each milestone and keep going; stop only for decisions that
belong to the user.

## ADR-010: Speed and memory are measured
Status: accepted (2026-11-19)
Context: speed and memory are the reason for the rewrite (ADR-001).
Decision: the web numbers in `e2e/perf-baseline.json` (2026-10-31) are the baseline: room list
of 300 rooms shown in 1433 ms, JS heap 133 MB at rest, room open 238 ms, encrypted first read
1487 ms. The Rust app measures the same scenarios on the same seeded server and records them in
`rust/perf.md`. Memory is compared as whole-process resident memory, since the browser's JS
heap leaves out the rest of the tab.

## ADR-011: How rust-anything checks the domain port
Status: accepted (2026-11-19)
Context: `rust-anything analyze --whole-app` (saved in `rust/research-analyze-whole-app.md`) sees
the web app as a CLI (the `synapse-module` scripts) and marks 127 of 155 units "keep" (UI). Its
front door / lift flow is for CLIs and servers, not for a browser UI. What it does well here is
record `src/domain` (pure logic): 12 473 cases over 222 functions, in
`rust/units/domain-rs/cases.jsonl`.
Decision: the typed Rust domain lives in `crates/twake-domain`. The generated crate
`rust/units/domain-rs` (napi bridge, outside the workspace) is only the test harness: its
`src/port.rs` converts the recorded JSON to the typed functions of `twake-domain` and back, so
`rust-anything diff rust/units/domain-rs` replays the TS recordings against the real Rust code.
A function counts as ported when all its recorded cases match. The rest of the app (adapters,
UI) is written by hand against the web app's `DECISION.md` and e2e scenarios.
Consequences: the desktop app never links napi. Functions the bridge keeps in TS (generic,
callback-taking: `shownTimelineOf`, `matchEmojis`, ...) are ported by hand with Rust tests.
Recording uses `tsx` from `rust/tools/node` (the web app's own `node_modules` are not needed).

## ADR-012: Local Synapse on port 18008
Status: accepted (2026-11-19)
Context: port 8008 was taken on the developer machine (likely the web app's own e2e stack).
Decision: `dev/up.sh` runs Synapse v1.162.0 (same pin as the web e2e) on 127.0.0.1:18008 with
SQLite and the test users alice, bob, carol (password `<user>-dev-password`, local test values).

## ADR-013: Native conventions of each system over the web's look
Status: accepted (2026-11-19). Refines ADR-005 ("the look stays that of the web app" in the
first plan).
Context: the user expects every feature of the web app, but not a page-by-page copy of its
look; the app should feel right on macOS, Windows and Linux.
Decision: features and behaviour follow the web app (its `DECISION.md`); look and system
integration follow each platform: Slint's `cupertino` style on macOS, `fluent` on Windows,
`cosmic` on Linux; the global menu bar and Cmd shortcuts on macOS; native notifications,
unread badge (Dock, taskbar, launcher), tray, secret store, deep links and autostart per system.
Twake's colours stay as the accent. Details in `rust/PLAN.md`.
Consequences: per-OS code lives behind small modules in `twake-desktop` (`platform/`), and CI
builds and tests on the three systems for every PR. The milestones after M2 are replanned in
`rust/PLAN.md` (M3 rooms and people ... M8 packaging), superseding the list of ADR-005.

## ADR-014: Commits, pull requests and merge
Status: accepted (2026-11-19)
Decision: the code lives in the user's private repository `nqhhdev/twake-chat-desktop`. Small
commits by concern, authored with the user's git identity, no assistant attribution. Every
change goes through a PR; CI (fmt, clippy, tests and release build on macOS, Windows and Linux,
replay of the domain recordings) must be green; the PR then merges by itself (merge commit, so
the commits stay as written).

## ADR-015: Local search keeps the web's index, not matrix-sdk's tantivy index

**Context.** PLAN.md planned matrix-sdk's `experimental-search` (tantivy) for the local
search. Its query parser joins words with OR, folds case but not accents, and refuses a prefix
on a single word ("a phrase-prefix query requires at least two terms"). The web matches every
word, ignores accents, and reads the last word as a prefix until the query ends with a space
(`matchesFilter`, D392).

**Decision.** Keep the web's design: an index of `IndexedMessage` per room, fed by every
synced message and by the open timeline, stored as a custom value of the encrypted SQLite
store, matched with the ported `matchesFilter`. Unencrypted rooms also ask the server's
`/search`, whose results join the index (web `MatrixServerSearch`).

**Consequences.** Same results as the web; no tantivy in the build. A linear scan per room:
fine for tens of thousands of messages; revisit if a room's index grows past that.

## ADR-016: Audio with rodio, symphonia and libopus; videos in the system player

**Context.** M4 plays audio files and voice messages inline and records voice messages. The
web records AAC in MP4 where the browser can (Safari, the phone app) and Opus in Ogg or WebM
otherwise, so the desktop must read all three. Pure Rust has no AAC encoder and symphonia has
no Opus decoder. Inline video would need GStreamer or FFmpeg (50 to 100 MB per OS).

**Decision.**
- Playback: rodio on cpal, one sound at a time on a thread owning the output device; symphonia
  reads MP4, Ogg, WebM/Matroska, MP3, FLAC, WAV and their codecs, libopus (`opus` crate,
  built from source with CMake) decodes Opus. A file is decoded as it plays.
- Recording: cpal input, mixed to mono, resampled to 48 kHz, encoded to Ogg Opus (RFC 7845)
  with its waveform, sent as an MSC3245 voice message through matrix-sdk.
- Video: opens in the system player, as before (the user's call, 2026-11-20); no GStreamer.

**Consequences.** Every client reads what we send (Ogg Opus is the Matrix voice norm). CMake is
a build dependency on every OS; `.cargo/config.toml` sets `CMAKE_POLICY_VERSION_MINIMUM=3.5`
for CMake 4 and the libopus bundled by `audiopus_sys`. Linux needs ALSA headers to build.
The macOS bundle (M8) needs `NSMicrophoneUsageDescription`.

## ADR-017: PDFs drawn by PDFium, loaded at run time

**Context.** The web shows a PDF in the browser's own viewer (`RoomPdfViewer`). A desktop app
has none. Pure-Rust renderers (hayro) are young and miss fonts and features; PDFium is the
engine of Chrome, so pages look as they do on the web. The user chose it (2026-11-21).

**Decision.**
- `pdfium-render` 0.9.5 (feature `pdfium_8125`) binds PDFium chromium/8125 from
  github.com/bblanchon/pdfium-binaries, loaded at run time: `TWAKE_PDFIUM_DIR`, then next to
  the app (`Contents/Frameworks` on macOS), then `vendor/pdfium/<platform>` in debug builds,
  then the system library. `dev/fetch-pdfium.sh` downloads it; `vendor/pdfium` stays out of
  git.
- PDFium is bound once per process and is not thread-safe: one worker thread in twake-core
  owns it and the one PDF open, and draws pages on request.
- The viewer draws only the pages on screen or near it, at the width shown in pixels, and
  drops pages far from it.
- Without the library, « Open » hands a PDF to the system's viewer, as before (the web's
  `isViewablePdf` with `canShowPdf` false).

**Consequences.** About 7 MB per OS in the bundle (M8 copies it and its licences, BSD and
Apache). Updating PDFium means a `pdfium-render` that supports the new build. Password-protected
PDFs are not shown (Open in app).

## ADR-018: Calls with the LiveKit Rust SDK, signalling in twake-core

**Context.** The web calls through MatrixRTC (MSC4143) with a LiveKit focus and its own UI
(DECISION.md D4, D8): matrix-js-sdk 43 for the membership and the media keys,
`livekit-client` for the media. A desktop app needs a native WebRTC stack. The user agreed to
download the LiveKit SDK's prebuilt libwebrtc and the local LiveKit images (2026-11-22).

**Decision.**
- Media in a crate of its own, `crates/twake-call`: `livekit` 0.12 over the prebuilt
  libwebrtc `webrtc-a545de4` (264 MB download, 1.1 GB unpacked), fetched by
  `dev/fetch-webrtc.sh` into `vendor/webrtc` (out of git) and found through
  `LK_CUSTOM_WEBRTC` in `.cargo/config.toml`. Microphone and speakers through libwebrtc's
  audio device module (`PlatformAudio`), the camera through `nokhwa` (no MJPEG decoder: the
  camera's NV12 or YUYV goes straight to I420). The desktop binary links with `-ObjC` on
  macOS, or libwebrtc's Objective-C categories are dropped and the first call crashes.
- Signalling in `twake-core/src/calls.rs`, as the web's `MatrixCalls`: the focus from the
  well-known (`org.matrix.msc4143.rtc_foci`, the oldest member's when a call goes on), a token
  from its `/sfu/get` with an OpenID token, the legacy membership state event
  `org.matrix.msc3401.call.member` (state key `_<user>_<device>_m.call`, 4 h expiry), a
  delayed leave (MSC4140, 8 s, renewed every 5 s on its own task) so a dead app leaves the call.
- In an encrypted room, one 16-byte key per device sent to each device in the call as an
  encrypted to-device `io.element.call.encryption_keys`, rotated when someone leaves or joins
  after 10 s, used 1 s after it is sent (`RTCEncryptionManager`); LiveKit keys per
  participant, no ratchet, HKDF, as the web's `MatrixKeyProvider`.
- Rooms the app creates let every member join a call (the membership at power level 0).

**Consequences.** twake-core links libwebrtc: its tests take about 14 s more to link, and every
OS needs its libwebrtc build (M8). `nokhwa` 0.10 pulls `block` 0.1.6, which a future Rust will
reject. The camera needs an app bundle that declares its use (`NSCameraUsageDescription`): an
unbundled debug binary never gets an answer from macOS. Key ring 255 (libwebrtc's maximum)
where the web has 256: after 255 rotations index 255 shares a slot with 0.
