# Map of the Rust rewrite

Goal: Twake Chat as a native Rust desktop app for macOS, Windows and Linux, faster and lighter
than the web app, same product (ADR-001, ADR-010). Reference: `reference/twake-space-chat`
(commit in `reference/SOURCE.txt`), product spec in its `DECISION.md`.

Read first: [DECISIONS.md](DECISIONS.md) (ADRs), then [HANDOFF.md](HANDOFF.md) if present,
[GLOSSARY.md](GLOSSARY.md), [perf.md](perf.md).

## Layout

| Path | What |
| --- | --- |
| `crates/twake-domain` | Pure domain logic, port of `src/domain` (checked by ADR-011) |
| `crates/twake-core` | Engine: matrix-sdk session, sync, room list, timeline; later ToM, LiveKit, side-service |
| `crates/twake-desktop` | Slint app, binary `twake-chat` (`ui/*.slint`) |
| `rust/units/domain-rs` | rust-anything harness: recorded cases of `src/domain` |
| `dev/` | Local Synapse (`up.sh`), `logged.sh` build helper |

## Milestones (ADR-005)

| Milestone | State | Scope |
| --- | --- | --- |
| M0 | done | Slint rich-text timeline spike (211 ms / 43.9 MB for 10 000 bubbles); web baseline in perf.md |
| M1 | done (2026-11-19) | Sign-in (password, SSO), session in keychain, Sliding Sync room list, timeline, send, recovery key |
| M2 | done (2026-11-19) | Threads, reactions, edits, replies, files and media, OS notifications |
| M3 | done (2026-11-19), folders and contacts moved | Rooms and people: room info and admin, members and roles, invitations, create rooms and DMs, typing, spaces, filter chips, pins, polls |
| M4 | in progress (first part 2026-11-20) | Done: composer suggestions, mentions, commands, drafts, emoji, menu bar, quick switcher, search, activity, staged files, drop and paste, image viewer, reasons. Next: voice, players, PDF, link previews, GIFs, formats bar, jump to date |
| M5 | open | Calls (MatrixRTC + LiveKit) |
| M6 | open | Twake platform: SSO, ToM, verification, assistant and Scribe, Drive, Meet, mail, analytics, multi-account |
| M7 | open | Settings, offline, localisation, accessibility, per-OS integration (ADR-013) |
| M8 | open | Packaging, signing, updates |

Details per feature and per system: [PLAN.md](PLAN.md).

## Domain functions (ADR-011)

222 recorded functions. Ported and matching all recorded cases:

| Function | Rust | Cases |
| --- | --- | --- |
| `formatDuration` | `twake_domain::format::format_duration` | 25/25 |
| `toPreviewText` | `twake_domain::room_preview::to_preview_text` | 72/72 |
| `fitPreview` | `twake_domain::format::fit_preview` | 69/69 |
| `computeAttachmentKind` | `twake_domain::attachment::compute_attachment_kind` | 57/57 |
| `inferMimeType` | `twake_domain::attachment::infer_mime_type` | 74/74 |
| `isGif` | `twake_domain::attachment::is_gif` | 50/50 |
| `levelOfRole`, `computeRole` | `twake_domain::room_management` | 69/69, 25/25 |
| `isRoomOwner`, `canManageMember`, `canChangeRole`, `assignableRoles`, `canBanMember` | `twake_domain::room_management` | all typed cases (the rest skipped: not of the typed shape) |
| `pressAnswer` | `twake_domain::poll::press_answer` | 63/63 |

Check with `rust-anything build rust/units/domain-rs && node rust/tools/check-ported.mjs`
(list in `rust/units/ported.txt`).

## Open questions

- ToM recovery-key API for desktop: the web gets the key from ToM after SSO (M1 uses the key typed
  by the user; ToM comes with M5 or earlier if needed).
- Linux keychain needs the Secret Service (GNOME Keyring / KWallet); a fallback for headless
  Linux is undecided.
