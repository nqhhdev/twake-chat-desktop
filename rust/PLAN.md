# Plan to full features

Goal (ADR-001, ADR-013): every feature of the web app, in a native app that follows the
conventions of each system. It does not have to look like the web page by page.

Built so far: M0 to M3, M4 first part and part of the second (see [HANDOFF.md](HANDOFF.md)). Each milestone below ends with an app
that builds on the three systems in CI, a smoke run on the local Synapse, and an updated
HANDOFF. One PR per milestone part, small commits.

Inventory source: `reference/twake-space-chat/src/ui` (screens) and `src/application/useCases`.

## M3: Rooms and people

| Feature | Web reference | Notes |
| --- | --- | --- |
| Room info pane: picture, name, description, members count, tiles | `InfoPanel`, `RoomEditPanel` | 360 px pane beside the conversation; floats when narrow |
| Members, roles, permissions, invite, kick, ban | `RoomMembers`, `RoomPermissions`, `InviteDialog`, `MemberRole` | power levels via matrix-sdk |
| Media / Links / Files tabs | `SharedContent` | Done: tabs of the info pane from the loaded messages (`computeSharedContent` recorded). Left: hide GIFs when the room has them off (M6), browse the room's pictures in the viewer |
| Edit room: rename, topic, picture, encryption switch | `RoomEditPanel`, `RoomSettings` | |
| Room access, public link, public rooms directory | `RoomAccessSettings`, `PublicRoomsDialog` | Done: « Who can join » (invite, on request, public) and « In directory » in the Edit view, the access line under the member count, the public rooms directory (« Public groups »: join, or ask to join). Left: public link and page (D388) |
| Leave, delete chat (hide history for oneself), block user, report | `RoomHeaderMenu`, `BlockAction`, `ReasonDialog` | Done: leave, delete chat for oneself (`app.twake.chat.deleted_chats`; `deletedChats.ts` recorded), Undo, report the room or a message with a reason, block the person of a direct message (`m.ignored_user_list`). Left: unblock (with the M7 settings) |
| Create room, direct message, broadcast, page | `CreateRoomScreen`, `CreatePage` | |
| Invites list, accept / decline | `InvitesSection` | |
| Spaces: channels, General room, move to space | `SpaceChannels`, `SpaceLanding`, `MoveToSpaceDialog` | |
| Folders of rooms, archive | `FolderPage`, `FolderSettings`, `ArchiveSettings` | account data; moved to M7 (settings) |
| Contacts screen, person card, profile, edit own profile | `ContactsScreen`, `PersonCard`, `ProfilePanel`, `EditOwnProfileDialog` | moved to M6 with the ToM address book |
| Pinned messages | `PinnedBar`, `PinnedMessages` | |
| Polls: create, vote, end | `PollDialog`, `PollMessage` | |
| Message details: versions, forward, copy link, select several | `MessageVersionsDialog`, `ForwardRoomRow`, `MessageSelectionBar` | Done: the message menu as the web (quick reactions, reply, « ⋯ »), copy message, copy link (matrix.to, ADR-020), download file, forward to several conversations, edit history, select several (forward, download, copy, pin, delete; `isSelectable` recorded). Left: delete others' messages as a moderator |
| Typing indicators, read receipts per person, presence | `useStatus`, `StatusDialog` | Done: typing; ticks on one's own messages (sent ✓, read ✓✓) and « N Seen » / « Seen by » per message, from each member's read receipt. Left: the « avatars » and « names » styles under messages, and the status (presence, its presets and « Do not disturb »): both come with the M7 settings |

## M4: Search, activity, composer

| Feature | Web reference | Notes |
| --- | --- | --- |
| Local search sealed on the device | `RoomSearch`, `search` use case | done: the web's index in the encrypted store (ADR-015) |
| Server search, jump to date | `RoomSearchBand`, `JumpToDate` | |
| Quick switcher (Cmd/Ctrl+K) | `QuickSwitcher` | |
| Activity screen: mentions, threads, reactions, missed calls | `ActivityScreen` | domain `activity.ts`, `activityFeed.ts` (recorded) |
| Unread threads, mark as read | `fetchUnreadThreads`, `markRoomAsRead` | |
| Composer: formats bar and classic style, mentions with pills, emoji picker and `:shortcodes:`, drafts per room, `/me`, `/shrug`, bot commands (MSC4332) | `ConversationComposer`, `EmojiPickerPanel`, `useCommandSuggestions` | domain `botCommand.ts`, `mention.ts`, `emojiShortcode.ts` (recorded) |
| Voice messages: record, waveform, play | `useVoiceMessage`, `AudioPlayer` | ADR-016 |
| Inline audio player, image viewer, PDF viewer, link previews, animated GIFs (video: the system player, ADR-016) | `RoomImageViewer`, `RoomPdfViewer`, `LinkPreview`, `useAutoplay` | done: PDFium (ADR-017); domain `isViewablePdf` (recorded) |
| Drag and drop and paste of files, staged files with captions | `useStagedFiles`, `StagedViewer` | |
| Delete confirmation, reasons | `ConfirmDialog`, `ReasonDialog` | |

## M5: Calls

| Feature | Web reference | Notes |
| --- | --- | --- |
| MatrixRTC calls over LiveKit, own UI: grid, speaker, screen share | `ui/calls`, `calls` use cases | `livekit` Rust SDK; camera/mic via the OS (ADR-018). Done: join/start, membership with delayed leave, E2EE keys, mic, camera, grid, screen share (system picker, « Screen of … » tile; sending not yet seen working), a shared screen on the stage with the people in a row under it (web `CallStage`: the web has no spotlight of who speaks, only the speaking ring); device menus (microphone, speaker, camera; `currentDeviceId` recorded); `shouldMirror`, `countCallPeople` recorded |
| Incoming call, ringing, call line in the timeline, call thread | `CallInvite`, `CallLine`, `CallThreadBand` | Done: ring (MSC4075) and decline (MSC4310) in the clear, banner with Answer and Decline, « Call declined », call line (`app.twake.chat.call`); call thread (« Meeting started at … » root, « Show the channel » / « Back to the meeting thread »); `toCallLine`, `ringAsks`, `isRinging`, `findCallThread`, `splitsCallChat` recorded. Left: OS notification and sound of a ring, the line as a bubble |
| Raised hands, captions, recording through Meet | `MeetingDialog`, `ScheduleMeeting` | side-service + Meet API |
| Background blur | `@livekit/track-processors` | later in M5: needs a segmentation model on the CPU/GPU |
| Picture in picture / always-on-top call window | `CallWindow` | Done: the window over the app (`FloatingCall`: leave the room or detach, drag, resize, corners, kept place; `callWindow.ts` recorded, 8 functions); pop out into a second native window, always on top (ADR-019) |

## M6: Twake platform

| Feature | Web reference | Notes |
| --- | --- | --- |
| SSO with the Twake identity provider, `.well-known` discovery | `twakeSso`, `clientConfig` | test with Dex in `dev/` |
| GIF picker (trending, search) through twake-common-proxy; a room's « GIFs off » | `useGifPicker`, `TwakeProxyGifs`, `RoomGifsSection` | moved from M4: needs the proxy |
| ToM: recovery key vault (unlock after sign-in), address book | `adapters/tom`, `RecoveryVault` | replaces typing the key |
| Device verification (emoji, QR), new devices notice, security settings | `DeviceVerificationDialog`, `SecuritySettings` | |
| Assistant (personal bot), Scribe, suggestions, questions, consent | `Assistant*`, `useScribe` | side-service |
| Auto-translate of messages | `MessageTranslate`, `AutoTranslateSetting` | |
| Drive picker, mail compose, calendar meeting | `useDrivePicker`, `MailCompose`, `ScheduleMeeting` | |
| Analytics with consent in account data | `AnalyticsConsentBar` | through the side-service only |
| Multiple accounts | `AccountsSettings`, `AddAccountDialog` | one engine per account |

## M7: Settings and system integration

| Feature | Notes |
| --- | --- |
| Settings: appearance (theme, accent, text size, composer style, bubble alignment), notifications (per room levels, do not disturb, keywords), devices, archive, contacts visibility | `SettingsScreen` and its parts |
| Offline banner, retry of failed sends, outgoing queue | `OfflineBanner`, `OutgoingList` |
| Localisation: the web's languages (`src/locales`) | Fluent or gettext; same keys |
| Accessibility: screen readers (VoiceOver, Narrator, Orca), keyboard navigation everywhere, high contrast | AccessKit through Slint |

Per system (ADR-013):

| | macOS | Windows | Linux |
| --- | --- | --- | --- |
| Widgets style | Slint `cupertino` | Slint `fluent` | Slint `cosmic` (GNOME/KDE neutral) |
| Menus | Global menu bar (App, Edit, View, Window, Help) | In-window menu / title bar | In-window menu |
| Shortcuts | Cmd | Ctrl | Ctrl |
| Notifications | User Notifications (signed bundle) | Toasts with an app id | freedesktop notifications |
| Unread count | Dock badge | Taskbar overlay icon | Launcher count (Unity API) where supported |
| Tray | Menu bar extra (optional) | System tray | StatusNotifierItem |
| Secrets | Keychain | Credential Manager | Secret Service |
| Deep links | `matrix:` and `https://matrix.to` handler | same, registry | same, `.desktop` MimeType |
| Start at login | Login item | Run key | XDG autostart |

## M8: Packaging and release

| | macOS | Windows | Linux |
| --- | --- | --- | --- |
| Package | `.app` in a `.dmg`, universal (arm64 + x86_64) | MSI (WiX) or MSIX | AppImage, `.deb`, Flatpak |
| Signing | Developer ID + notarisation | Authenticode | GPG for the repository |
| Updates | in-app check against GitHub releases | same | through the package manager / AppImage update |

CI builds the three on every PR (`.github/workflows/ci.yml`); release jobs come with M8 and need
the signing secrets from the user.

## Domain ports alongside (ADR-011)

Each milestone ports the `src/domain` functions it uses and adds them to
`rust/units/ported.txt`; CI replays their recorded cases. 71 of 222 done.
