# Glossary

Domain terms of the web app and their names in Rust. Add to it as units bring new terms.

| Web (TS) | Meaning | Rust |
| --- | --- | --- |
| `Room` (`room.ts`) | A Matrix room as the screens see it | `twake_domain::RoomSummary` (room list subset) |
| `RoomKind` | direct, discussion or broadcast | `twake_domain::RoomKind` |
| `RoomPreview` (`roomPreview.ts`) | Last thing said in a room, one line | `twake_domain::RoomPreview` |
| `RoomPreviewKind` | text, image, file, poll, encrypted, deleted... | `twake_domain::PreviewKind` |
| `toPreviewText` | One-line, 200-unit preview of a body | `twake_domain::room_preview::to_preview_text` |
| `Message` (`message.ts`) | A message of the timeline | `twake_domain::Message` |
| `MessageNotice` | Line drawn instead of a bubble (deleted, unsupported) | `twake_domain::MessageNotice` |
| `UndecryptableReason` (`encryption.ts`) | Why a message cannot be read | `twake_domain::Undecryptable` |
| `sendStatus` | sending, sent, failed (local echo) | `twake_domain::SendStatus` |
| `formatDuration` (`format.ts`) | `m:ss` / `h:mm:ss` | `twake_domain::format::format_duration` |
| Recovery key | Secret that unlocks the key backup on a new device (from ToM on the web) | `Command::Recover` |
| Sliding Sync (MSC4186) | Fast sync of the room list | matrix-sdk-ui `SyncService` / `RoomListService` |
| ToM | Twake on Matrix server: recovery vault, address book | `twake-core` adapter (M5) |
