# Handoff

State on 2026-11-19, end of M3. Read [MAP.md](MAP.md) and [DECISIONS.md](DECISIONS.md) first.
Screenshots of each check are in `dev/snapshots/` (git-ignored, local only).

## What works (verified on the local Synapse)

M1:
- Password sign-in; session restored from the keychain (`TWAKE_DEV_SESSION_FILE` in dev).
- Sliding Sync room list: names, previews, times, unread and mention badges, room pictures,
  search by name. 300 rooms listed in 233 ms after a restart (rust/perf.md).
- Timeline: day dividers, membership lines, grouped bubbles, rich text (Matrix HTML -> Slint
  StyledText), links, back-pagination, stays at the newest message, read receipts.
- E2EE: undecryptable notice, recovery key banner on a new device, keys from the backup.

M2:
- Avatars (people, rooms), inline images sized as the web's `fitPreview`, file rows with
  Open (system app) and Save (Downloads). Encrypted media decrypted in the engine.
- Reactions as chips (own highlighted, click toggles), the web's quick reactions on hover.
- Replies (quote above the reply), edits ("edited"), deletion ("Message deleted").
- Threads: thread replies stay out of the main timeline; "N replies" + latest under the root;
  a thread panel beside the conversation with its own composer.
- Composer modes: reply / edit with a hint and Cancel (Esc); "+" sends files (system picker).
- OS notifications for messages that notify (push rules or mentions), except in the open room.

M3:
- Room info pane (click the room header): picture, name, description, member count, edit
  (name, description), encryption switch (asks first), invite by Matrix id, members with roles
  (Owner, Admin, Moderator), per-member actions on hover limited by the web's rules
  (`assignableRoles`, `canManageMember`, `canBanMember`), leave (asks first).
- Invitations in the room list with Join / Decline; « + Room » (name, description, invitees,
  encrypted or not) and « + Direct » (reuses an existing DM; new DMs in the clear, as the web).
- Typing indicators (« bob is typing… ») and one's own typing sent.
- Polls: create (📊 in the composer), vote with the web's `pressAnswer` rule, results bars,
  end (asks first). Pins: pin / unpin on hover (moderators), 📌 count in the header, the pinned
  messages in the side panel.
- Spaces: a rail of the top-level spaces filters the room list; chips All / Unread / People /
  Rooms.
- Deleting a message now asks first.
- Media, Links and Files (2026-11-25), as the web's `SharedContent`: tabs of the room info
  after Members (a direct message has the three only), from the messages of the open room
  (`computeSharedContent`), so they grow as its history is loaded. Media: a grid of three,
  a picture opens in the viewer, a video in the system's player (no first frame drawn);
  Links: the cached preview's title when there is one, a click opens the link; Files: name,
  size and day (dd/mm/yy), a click saves it. Not yet: the GIFs of a room that has them off
  stay shown (that switch comes with M6), the viewer shows one picture, not the room's.
- Delete chat (2026-11-25), as the web's `deleteChat` (D186): « Delete chat » at the foot of
  the room info asks first (the web's words), then writes `app.twake.chat.deleted_chats`
  (shared with the web) with the later of now and the room's last activity, and marks the
  room read; the notice says « Chat deleted » with Undo until another room opens. The room
  leaves the list until something newer (`isChatHidden`); its timeline keeps what came
  after (`withoutDeleted`, events too, empty days dropped) under « You deleted the earlier
  messages on … »; search skips what came before (`isAfterDeletion`). Not like the web
  yet: that line is pinned above the timeline, the web shows it at the head of the
  history; the room list has no menu of its own to delete from. Not tried in a run.
- Report and block (2026-11-25), as the web's header and message menus: « Report room »
  (« Report » in a direct message) at the foot of the room info, « ⚑ Report » on someone
  else's message; both ask with the web's words and an optional reason, then say « Room
  reported » / « Message reported ». « Block user » in a direct message asks, then ignores
  the person (`m.ignored_user_list`: the server stops sending their events); nothing said
  after, as the web. Unblocking comes with the settings (M7). Not tried in a run.

M4, first part (2026-11-20):

- Composer: suggestions under the word before the caret, Up/Down/Tab/Enter/Esc: `:shortcode`
  emoji, `@person` (and `@room` when allowed), `#room`, `/command` (the bots' MSC4332 commands,
  `/me`, `/shrug`). Mentions go out as matrix.to links with `m.mentions`, as the web sends them
  (checked on the server). `/me` sends `m.emote`, drawn « * alice waves ». `/discardsession`
  drops the room's outgoing encryption session. An emoji picker (🙂). Drafts per room and per
  thread, in memory only (web D16).
- Menu bar (macOS global menu, in the window elsewhere): Go to Conversation (Cmd/Ctrl+K, the
  quick switcher), Search in Conversation (Cmd/Ctrl+F), Activity (Cmd/Ctrl+Shift+A), New Room
  (Cmd/Ctrl+N), New Direct Message, Room Info (Cmd/Ctrl+I), Sign Out, About, Quit.
- Search in a conversation: the web's own index (not tantivy, see DECISIONS ADR-015), fed by
  every synced message and by the open timeline, sealed in the encrypted store; the web's
  matching (all words, accents and case ignored, last word a prefix unless closed by a space);
  the server's search too in rooms that are not encrypted; « Files » filter; « Search older
  messages » reads more history. A result opens the messages around it in the side panel, the
  found one outlined.
- Activity screen (🔔 at the top of the rail): mentions, reactions on one's messages, followed
  threads with their unread replies, unread rooms; chips All / Unread / Mentions / Replies, a
  search, grouped by day; a line opens its room, thread or message.
- Files: staged above the composer (picked with +, dropped on the window, or pasted with
  Cmd/Ctrl+V: copied files or an image), sent with the composer text as their caption.
- Image viewer over the window (click an image): Save, Open in app, Esc.
- Delete, remove and ban take an optional reason.
- M4 second part so far:
  - Go to a date (Conversation menu, Cmd/Ctrl+J, Slint's date picker) and « Jump to the
    Beginning »: the server's `/timestamp_to_event` names the first message of the day (the
    web pages back up to 600 messages instead); the side panel shows the messages around it,
    titled « From 20 November 2026 », the message outlined.
  - Formats bar (« Aa » under the field): bold, italic, underline, strike, link (a dialog),
    numbered and bulleted lists, quote, code, code block, as Markdown around the selection
    (`twake_domain::text_format`, the web's `applyFormat`, its spec cases as Rust tests).
    Shortcuts of Slack: Cmd/Ctrl+B, I, U, Shift+X, Shift+C, Shift+U (link); a web address
    pasted over a selection links it. Room Info moved to Cmd/Ctrl+Shift+I for italic.
  - Link previews: the first web link of a message, as the homeserver previews it
    (`preview_url`), a banner on top or a square beside the text; never asked in an
    encrypted room (web D211). Links in text follow the web's rule now (`www.` and bare
    domains with a known ending link too; `twake_domain::web_links`, replayed 69/69).
    `dev/preview-page.sh` serves a test page; the dev Synapse has previews on for it.
  - Audio (ADR-016): audio files and voice messages play inline (play/pause, waveform or
    progress line, click to seek, time); AAC/M4A, Opus in Ogg or WebM, MP3, FLAC, WAV.
    Voice messages: 🎤 in the composer records (« Recording 0:12 », Cancel / Esc, Send /
    Enter), sent as Ogg Opus with its waveform (MSC3245/MSC1767). Videos still open in the
    system player.
  - Threads: an open thread sends its own read receipt as its replies show (MSC3771).
  - Saved messages: 🔖 on a message's action bar saves it in the account data
    (`app.twake.chat.saved_messages`, shared with the web); the Activity has a « Saved » chip.
  - Animated GIFs play in the timeline, the thread pane and the image viewer (the whole file
    is read, as the web's `<img>`; frames decoded at the size shown). They stay still when
    the account data `app.twake.chat.autoplay` says `{"enabled": false}` (set on the web).
  - PDFs: « Open » on a PDF shows it in the app (ADR-017): its pages one under the other,
    drawn by PDFium as they scroll into view, with Save, Open in app and Close. Run
    `dev/fetch-pdfium.sh` once (downloads PDFium into `vendor/pdfium`); without it a PDF
    opens in the system's viewer as before.

M5 (first part, calls, ADR-018):
- Run `dev/fetch-webrtc.sh` once (libwebrtc for the build) and `dev/up.sh` (now with LiveKit
  and the token service), then `dev/seed-m5.py` (calls open to every member of Design and
  Secret project).
- The room header offers « 📞 Call » and « 🎥 Video », or « Join the call (n people) » when a
  call goes on. In a call the stage takes the width (grid of 16:9 tiles, the web's
  `tileWidth`), the conversation a 380 px column. Leaving the room floats the call (see
  « Call window » below).
- Ringing: whoever starts a call sends a ring (MSC4075, in the clear even in an encrypted
  room, as the web); the others get a banner « alice is calling you in Design » with Decline
  and Answer (web rules `ringAsks`, `isRinging`). A decline tells the caller (« Call
  declined » on the stage). When the caller leaves, they write the call line
  (`app.twake.chat.call`), shown as « 📞 Outgoing Call · 9 sec » or « Incoming Call · Missed ».
  Checked with two apps: decline, then answer.
- Call thread (D105): whoever starts a call in a channel posts « Meeting started at 07:49 »,
  marked `app.twake.chat.call_thread`; during the call the side column shows that thread,
  titled « Meeting thread », with « Show the channel » and back (web `findCallThread`,
  `splitsCallChat`). Checked with two apps in a clear and an encrypted room.
- Screen share: « 🖥️ » in the call controls opens the system's picker (ScreenCaptureKit on
  macOS, libwebrtc's desktop capturer); the screen goes out at 15 pictures a second and shows
  as « Screen of alice », the button pressed while sharing. Stopping from the menu bar ends
  it. Not seen working: nobody answered the picker in the unattended run.
- Speaker view (2026-11-25), as the web's `CallStage`: a shared screen takes the stage, the
  width of the grid, and the people sit in a centred row of 16:9 tiles at most 200 px wide
  under it; the first screen if two are shared. The web has no spotlight of who speaks (the
  speaking ring only), so neither does the app. The ordering is tested
  (`call::tests::puts_the_first_screen_on_the_stage`); the layout was not looked at, as no
  screen was shared in a run.
- Device menus (2026-11-25), as the web's `CallDeviceMenu`: « ▴ » beside the microphone lists
  the microphones and speakers, beside the camera the cameras, read at each opening; the one
  in use checked (`currentDeviceId`: the one picked, else the system's default, the first);
  a click switches it during the call (LiveKit's `PlatformAudio`; the camera reopens on the
  new one). A system that gives no speakers shows no Speaker section (web). Not tried with
  two real devices in a run; a camera is known by its index, so unplugging one may move the
  pick to the next. The web's background choices under the camera come with the blur.
- Call window (2026-11-25), as the web's `FloatingCall` and `CallWindow` (D571): leaving the
  room of the call, or « ⧉ Detach the call » in its controls, floats the call in a window
  over the app; coming back to the room does not put it back (`placeCall`,
  `isCallFloating`). Dragged by its bar, resized by its edges and corners, never smaller
  than 320 × 260 nor out of the app (`fitWindow`, `moveWindow`, `resizeWindow`); its menu
  « Move and resize » offers Full screen (back in the room) and the four corners
  (`placeInCorner`). Where it was left is kept in `call-window.json` of the data folder
  (web `localStorage`), read with `parseCallWindow`; the first time it opens above the
  composer on the right (`defaultCallWindow`). Below 900 px wide the app shows a band with
  Expand and Leave instead (web: a phone). The window was not dragged in a run (no
  automated pointer moves), only built and its rules replayed.
- Pop-out (2026-11-25, ADR-019): « Pop out of the app » in the window's menu opens the call
  in a window of its own that stays above the others, at the window's size; the app shows
  the band meanwhile, and closing the pop-out (or Expand) brings the call back. No
  automatic pop-out. Not opened in a run.
- Two apps (alice, bob) joined the same call in a clear room and in an encrypted one; in the
  encrypted one each device sent its key to the other and received the other's. Leaving or
  quitting empties the membership (delayed leave).

Seeds: `dev/seed.sh`, `dev/seed-media.py`, `dev/seed-m2.py`, `dev/seed-large.sh`, `dev/seed-m3.py` (`--typing` for a typing notice),
`dev/seed-m4.py [delay]` (a mention, a reaction and a thread reply, while the app runs),
`cargo run -p twake-core --example e2ee_seed`. Unattended runs: `dev/smoke.sh` with the
`TWAKE_DEV_*` hooks listed in `crates/twake-desktop/src/dev.rs`.

- Look of twake-mui: `ui/theme.slint` holds the tokens of twake-mui 19.10.2 on twake-css 1.2.1
  (palette, radius, type sizes), own bubbles mixed as the web's, the conversation on the web's
  wallpaper (`ui/images/chat-bg-*.jpg`). Icons of the OS: `src/icons.rs` draws SF Symbols on
  macOS into the `Icons` global; `Icon` (parts.slint) shows its emoji where the OS has none
  (Windows and Linux for now). Snapshots: `dev/snapshots/ds-light.png`, `ds-icons.png`.
- Composer as the web's default "new" one (ds/Composer, ComposerBar): a card (paper, 1px
  divider edge, 14px radius) on the wallpaper, the text on top and a row of 32px tools under
  it (+, poll, Aa, emoji); at its end the microphone while there is nothing to send, a round
  primary Send once there is (the separate mic button and the Send text button are gone).
  « Aa » unfolds the formats in the row, right of it, as the web; a narrow row scrolls
  sideways (Flickable), the microphone and Send stay in view.
  Buttons as twake-mui: pills, primary-dark on hover. Snapshot
  `dev/snapshots/ds-composer-new.png`, formats open `ds-composer-formats.png`.

## Not verified

- The hover action bar (react, reply, edit, delete, thread) builds and the actions are wired,
  but no automated run moves the mouse: try them by hand. Edits and deletion were only seeded
  through the API, not sent from the app.
- SSO sign-in (no identity provider in `dev/`), Windows and Linux builds, dark theme, screen
  readers: same as at the end of M1.
- OS notifications: the event fires (logged); the system banner was not looked at, and clicking
  it does not open the room yet.

## Known gaps (next to fix)

- Calls: not heard or seen. Mic sound and the decryption of the media were not checked (no
  one spoke), and the camera never opened: macOS gives no answer to an unbundled debug
  binary (needs `NSCameraUsageDescription`, M8). No raised hands, captions, background blur (rest of M5). The
  membership is not renewed after 4 h; a membership left by a crash is not cleared at start
  (the web's `#leaveVanishedCalls`); the old « encrypted » flag of a call is ignored (the
  room decides). The composer is cramped in the 380 px column.
- Call thread and screen share: no voice rooms in this app yet and « may post » is not known,
  so every channel splits its call chat (the root is only sent by someone who may post).
  The thread takes the side column and the stage's header stays above the stage only. Screen
  share asks for the Screen Recording permission, which an unbundled binary may not get
  (as the camera, M8); the picker waits up to 5 minutes and a cancel says nothing (web).
- Rings: no OS notification or sound; the banner shows only while the app is open. A call
  line is a centred line, not the web's bubble on the caller's side. A room whose latest
  event is a ring or a call line shows no preview in the list. Events sent in the clear do
  not move the sender's read marker (« New » shows above their own call line). Rings sent
  before the app started are not heard (the web scans the recent timeline). The ring has no
  `slot_id` (sticky events, MSC4354): not checked with Element. No log output: the app has no
  tracing subscriber, so twake-core's `tracing` lines go nowhere.

- A direct message without a room picture still shows initials in some cases (hero avatar not
  cached for the room).
- A room counts thread replies as unread while its thread is open; mark the thread read too.
- Not exercised unattended: member role changes, remove, ban, rename, invite from the pane,
  creating rooms and DMs from the dialog, voting by click (the rules are checked; the clicks
  need a hand run).
- Folders and the archive moved to M7, contacts to M6 (rust/PLAN.md).
- Video and audio play in the system app (Open), not inline.
- Activity: a thread's unread count is the replies since one's own last reply (not read
  receipts per thread); mentions are gathered from the sync, so a fresh device starts empty.
- Drop on the thread panel is told by the pointer's last position (winit gives no drop point).
- The viewer stays on « Loading… » if the image fails to download.
- The room list preview shows a formatted message's Markdown (« Ship **it** _today_ »); the
  web's rule for it is not checked yet.
- A link preview skips `matrix.to` links only; the web also skips any link `parseLink`
  reads as a Matrix one.
- Audio: playback and recording were not heard or spoken into by an unattended run (it would
  use the speakers and ask for the microphone); decoding and encoding are tested, the sent
  voice message was read back from the server. The Opus pre-skip is not cut on playback
  (the first 6.5 ms).
- GIFs: the « Autoplay animated images » switch itself comes with the M7 settings (the web's
  choice is already honoured); the system's « Reduce motion » is not read yet (the web's
  default when the user never chose). A GIF over 600 frames or 96 MB of decoded frames stays
  still. A GIF's room-list preview says « Photo ».
- PDF viewer: scrolling through a long PDF (pages drawn on the way, far ones dropped) was
  not exercised unattended, only the first page shown; no zoom, no text selection or search
  in the PDF, password-protected PDFs not shown. PDFium is fetched for macOS arm64 only so far
  (`dev/fetch-pdfium.sh` takes the platform). Resident memory with a PDF open was 311 MB
  (three A4 pages drawn about 1700 px wide, 17 MB each).
- A saved message whose text this device never read shows « Saved in <room> » without text.
- The « classic » composer style (bar always shown, a setting synced in account data) is
  M7 settings; the bar is toggled per session for now.
- Recovery key from ToM (the web gets it after SSO) not done; the user types it.
- Memory: about 180 to 195 MB resident with a few rooms open, 159 MB with 300 rooms idle; the
  fair comparison with the web's whole browser footprint is still to do (rust/perf.md).

## How to run what exists

```sh
dev/up.sh && dev/seed.sh && python3 dev/seed-media.py && python3 dev/seed-m2.py
cargo run --release -p twake-desktop     # http://localhost:18008, alice / alice-dev-password
dev/smoke.sh alice General               # unattended: tests, build, sign in, screenshot
```

Builds run in the user's terminal, not in the agent sandbox (it cannot write to `~/.cargo`,
unpack some crates, or reach Docker).

## Next: rest of M5

M4 is done apart from the gaps above (the GIF picker moved to M6: it goes through
twake-common-proxy). M5 has started: joining calls with audio, camera and E2EE keys works
on the local stack, and so do ringing, declining, the call line and the call thread. Screen
share is built but not yet seen working (it needs someone to answer the system's picker).
Next in M5: raised hands and captions through Meet, then background blur. See
rust/PLAN.md.
