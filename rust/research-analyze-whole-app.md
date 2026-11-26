# Rust migration plan: twake-space-chat (TypeScript/JavaScript)

2 packages, 493 source files (103857 lines), 842 test files, 343 UI files, grouped into 155 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 17 | 11 | 0 | 127 |

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `src/adapters/analytics/` | 94 | 75 | 100 | 1 | napi-rs | - |
| 2 | `src/adapters/assistant/` | 103 | 75 | 100 | 1 | napi-rs | Async and promises (3); Exceptions (1); JSON round-trips (1) |
| 3 | `src/adapters/proxy/` | 194 | 75 | 90 | 1 | napi-rs | Async and promises (13); Insertion-ordered maps and object keys (1); Callback-shaped APIs (1) |
| 4 | `src/adapters/mail/` | 59 | 72 | 100 | 1 | napi-rs | Async and promises (2); Dynamic typing (1) |
| 5 | `src/adapters/` | 84 | 71 | 86 | 4 | napi-rs | Async and promises (4); Callback-shaped APIs (2); Exceptions (1) |
| 6 | `src/adapters/sentry/` | 242 | 64 | 76 | 3 | napi-rs | Regex features the `regex` crate lacks (1); Async and promises (9); Callback-shaped APIs (8) |
| 7 | `src/adapters/meet/` | 335 | 63 | 75 | 4 | napi-rs | Async and promises (11); Callback-shaped APIs (4); Reflection and prototypes (1) |
| 8 | `src/adapters/matrix/integration/` | 385 | 63 | 68 | 31 | napi-rs | String offsets and slicing (2); Async and promises (47); Callback-shaped APIs (4) |
| 9 | `src/adapters/drive/` | 376 | 58 | 66 | 8 | napi-rs | Async and promises (22); Callback-shaped APIs (8); Exceptions (11) |
| 10 | `scripts/` | 340 | 55 | 57 | 0 | napi-rs | String offsets and slicing (4); Async and promises (13); Insertion-ordered maps and object keys (7) |

### `src/adapters/analytics/`

94 lines in 1 files, wave 4, priority 75, readiness 100, bridge **napi-rs**.

- 1 test files import it directly.

### `src/adapters/assistant/`

103 lines in 2 files, wave 4, priority 75, readiness 100, bridge **napi-rs**.

- 1 test files import it directly.
- Async and promises x3: `src/adapters/assistant/TwakeScribe.ts:38`, `src/adapters/assistant/TwakeScribe.ts:43`, `src/adapters/assistant/TwakeScribe.ts:44`
- Exceptions x1: `src/adapters/assistant/TwakeScribe.ts:53`
- JSON round-trips x1: `src/adapters/assistant/TwakeScribe.ts:49`

### `src/adapters/proxy/`

194 lines in 1 files, wave 4, priority 75, readiness 90, bridge **napi-rs**.

- 1 test files import it directly.
- Async and promises x13: `src/adapters/proxy/TwakeProxyGifs.ts:29`, `src/adapters/proxy/TwakeProxyGifs.ts:30`, `src/adapters/proxy/TwakeProxyGifs.ts:39`
- Insertion-ordered maps and object keys x1: `src/adapters/proxy/TwakeProxyGifs.ts:111`
- Callback-shaped APIs x1: `src/adapters/proxy/TwakeProxyGifs.ts:24`
- Exceptions x2: `src/adapters/proxy/TwakeProxyGifs.ts:63`, `src/adapters/proxy/TwakeProxyGifs.ts:88`

### `src/adapters/mail/`

59 lines in 1 files, wave 4, priority 72, readiness 100, bridge **napi-rs**.

- 1 test files import it directly.
- Async and promises x2: `src/adapters/mail/TwakeMailComposer.ts:51`, `src/adapters/mail/TwakeMailComposer.ts:56`
- Dynamic typing x1: `src/adapters/mail/TwakeMailComposer.ts:24`

### `src/adapters/`

84 lines in 2 files, wave 3, priority 71, readiness 86, bridge **napi-rs**.

- 4 test files import it directly.
- Async and promises x4: `src/adapters/OpenIdFetch.ts:22`, `src/adapters/OpenIdFetch.ts:40`, `src/adapters/OpenIdFetch.ts:41`
- Callback-shaped APIs x2: `src/adapters/OpenIdFetch.ts:14`, `src/adapters/OpenIdFetch.ts:17`
- Exceptions x1: `src/adapters/OpenIdFetch.ts:73`
- JSON round-trips x1: `src/adapters/OpenIdFetch.ts:48`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. twake-chat (`.`, node, 616 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `public/embed/` | ready | 5 | 100 | napi-rs | - |
| 0 | `src/ds/CornerBadge/` | keep | 48 | 100 | keep | Callback-shaped APIs (1) |
| 0 | `src/ds/DecimalComparison/` | keep | 28 | 100 | keep | - |
| 0 | `src/ds/DetailPage/` | keep | 164 | 100 | keep | - |
| 0 | `src/ds/InfoSkeleton/` | keep | 31 | 100 | keep | - |
| 0 | `src/ds/PresenceBadge/` | keep | 51 | 100 | keep | - |
| 0 | `src/ds/CappedList/` | keep | 42 | 99 | keep | String offsets and slicing (1) |
| 0 | `src/ds/LinkPreviewCard/` | keep | 175 | 99 | keep | Exceptions (1) |
| 0 | `src/ds/QuestionButtons/` | keep | 75 | 99 | keep | Callback-shaped APIs (2) |
| 0 | `src/ds/AssistantMark/` | keep | 33 | 98 | keep | - |
| 0 | `src/ds/DialogTitleBar/` | keep | 34 | 98 | keep | - |
| 0 | `src/ds/Divider/` | keep | 39 | 98 | keep | - |
| 0 | `src/ds/EmojiComparison/` | keep | 37 | 98 | keep | - |
| 0 | `src/ds/FabDock/` | keep | 21 | 98 | keep | - |
| 0 | `src/ds/InertPreview/` | keep | 27 | 98 | keep | - |
| 0 | `src/ds/ListDialog/` | keep | 27 | 98 | keep | - |
| 0 | `src/ds/LiveAnnouncement/` | keep | 23 | 98 | keep | - |
| 0 | `src/ds/MainColumn/` | keep | 23 | 98 | keep | - |
| 0 | `src/ds/Notice/` | keep | 30 | 98 | keep | - |
| 0 | `src/ds/RoomLine/` | keep | 23 | 98 | keep | - |
| 0 | `src/ds/TwoColumns/` | keep | 21 | 98 | keep | - |
| 0 | `src/ds/CallFrame/` | keep | 79 | 97 | keep | - |
| 0 | `src/ds/CollapsingHeader/` | keep | 65 | 97 | keep | - |
| 0 | `src/ds/FormColumn/` | keep | 15 | 97 | keep | - |
| 0 | `src/ds/MessagesSkeleton/` | keep | 59 | 97 | keep | - |
| 0 | `src/ds/RequestCard/` | keep | 61 | 97 | keep | - |
| 0 | `src/ds/AvatarStack/` | keep | 22 | 96 | keep | - |
| 0 | `src/ds/Loading/` | keep | 29 | 96 | keep | - |
| 0 | `src/ds/PanelCard/` | keep | 243 | 96 | keep | Callback-shaped APIs (1) |
| 0 | `src/ds/TintedIcon/` | keep | 34 | 96 | keep | - |
| 0 | `src/ds/ListFooter/` | keep | 49 | 95 | keep | Callback-shaped APIs (1) |
| 0 | `src/ds/OptionCards/` | keep | 61 | 95 | keep | Callback-shaped APIs (1) |
| 0 | `src/ds/PromptButtons/` | keep | 53 | 95 | keep | Callback-shaped APIs (1) |
| 0 | `src/ds/QrCode/` | keep | 85 | 95 | keep | Async and promises (2) |
| 0 | `src/ds/ImagePreview/` | keep | 105 | 94 | keep | Callback-shaped APIs (1) |
| 0 | `src/ds/ResolvedMark/` | keep | 18 | 93 | keep | - |
| 0 | `src/ds/SearchField/` | keep | 52 | 93 | keep | - |
| 0 | `src/ds/AssistantAvatar/` | keep | 24 | 92 | keep | - |
| 0 | `src/ds/HeroPhoto/` | keep | 102 | 92 | keep | Callback-shaped APIs (2); JSON round-trips (1) |
| 0 | `src/ds/Density/` | keep | 31 | 90 | keep | - |
| 0 | `src/ds/IconAction/` | keep | 54 | 90 | keep | - |
| 0 | `src/ds/Stack/` | keep | 143 | 89 | keep | Callback-shaped APIs (2) |
| 0 | `src/ds/CameraScanner/` | keep | 113 | 88 | keep | Async and promises (5); Callback-shaped APIs (2) |
| 0 | `src/ds/Highlight/` | keep | 124 | 84 | keep | String offsets and slicing (8) |
| 0 | `src/ds/InlineMarkdown/` | keep | 112 | 84 | keep | String offsets and slicing (6); Regex features the `regex` crate lacks (1) |
| 0 | `src/adapters/oidc/` | caution | 195 | 76 | napi-rs | Callback-shaped APIs (21); Async and promises (14) |
| 0 | `public/` | ready | 699 | 65 | napi-rs | String offsets and slicing (3); Async and promises (38) |
| 0 | `src/ds/` | keep | 661 | 62 | keep | Callback-shaped APIs (3) |
| 0 | `scripts/` | ready | 340 | 57 | napi-rs | String offsets and slicing (4); Async and promises (13) |
| 0 | `synapse-module/` | caution | 806 | 46 | napi-rs | String offsets and slicing (8); Regexes built at runtime (1) |
| 0 | `src/domain/` | caution | 6124 | 0 | napi-rs | String offsets and slicing (54); Regexes built at runtime (3) |
| 1 | `src/ds/PanelTile/` | keep | 94 | 100 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/StatusLine/` | keep | 86 | 100 | keep | - |
| 1 | `src/ds/Arriving/` | keep | 36 | 99 | keep | - |
| 1 | `src/ds/SeenBy/` | keep | 64 | 99 | keep | String offsets and slicing (1) |
| 1 | `src/ds/ThreadCard/` | keep | 135 | 99 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/GroupLabel/` | keep | 36 | 98 | keep | - |
| 1 | `src/ds/HoverActionItem/` | keep | 56 | 97 | keep | - |
| 1 | `src/ds/ListRow/` | keep | 57 | 97 | keep | - |
| 1 | `src/ds/LiveStatus/` | keep | 14 | 97 | keep | - |
| 1 | `src/ds/SettingsSection/` | keep | 50 | 97 | keep | - |
| 1 | `src/ds/BannerStack/` | keep | 102 | 96 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/CallInvite/` | keep | 190 | 96 | keep | Callback-shaped APIs (3) |
| 1 | `src/ds/ConsentBar/` | keep | 81 | 96 | keep | - |
| 1 | `src/ds/PanelHero/` | keep | 91 | 96 | keep | - |
| 1 | `src/ds/PickRowButton/` | keep | 38 | 96 | keep | - |
| 1 | `src/ds/PillButton/` | keep | 27 | 96 | keep | - |
| 1 | `src/ds/ReadTicks/` | keep | 39 | 96 | keep | - |
| 1 | `src/ds/SplashLoader/` | keep | 91 | 96 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/ActivityItem/` | keep | 132 | 95 | keep | - |
| 1 | `src/ds/HoverRow/` | keep | 56 | 95 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/MenuAction/` | keep | 47 | 95 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/MiddleCut/` | keep | 33 | 95 | keep | - |
| 1 | `src/ds/PaneHeader/` | keep | 223 | 95 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/PickedStrip/` | keep | 118 | 95 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/PinnedBanner/` | keep | 72 | 95 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/Spotlight/` | keep | 201 | 95 | keep | Callback-shaped APIs (3) |
| 1 | `src/ds/AddRowContent/` | keep | 32 | 94 | keep | - |
| 1 | `src/ds/DropZone/` | keep | 95 | 94 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/MediaGrid/` | keep | 102 | 94 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/MenuButton/` | keep | 22 | 94 | keep | - |
| 1 | `src/ds/AlbumGrid/` | keep | 144 | 93 | keep | - |
| 1 | `src/ds/AppBar/` | keep | 131 | 93 | keep | Callback-shaped APIs (3) |
| 1 | `src/ds/IntentDialog/` | keep | 178 | 93 | keep | Callback-shaped APIs (2) |
| 1 | `src/ds/MenuRadio/` | keep | 32 | 92 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/SpaceCard/` | keep | 92 | 92 | keep | - |
| 1 | `src/ds/DialogHeader/` | keep | 73 | 91 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/SelectionBar/` | keep | 72 | 91 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/EmojiPicker/` | keep | 262 | 90 | keep | Callback-shaped APIs (7) |
| 1 | `src/ds/ChatShell/` | keep | 420 | 89 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/ColorDots/` | keep | 91 | 89 | keep | Callback-shaped APIs (2) |
| 1 | `src/ds/NameAvatar/` | keep | 91 | 89 | keep | String offsets and slicing (2); JSON round-trips (1) |
| 1 | `src/ds/PicturePicker/` | keep | 65 | 89 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/ToggleRow/` | keep | 69 | 89 | keep | Callback-shaped APIs (1) |
| 1 | `src/ds/LinkRow/` | keep | 130 | 88 | keep | String offsets and slicing (1); Callback-shaped APIs (1) |
| 1 | `src/ds/ReactionChips/` | keep | 266 | 88 | keep | String offsets and slicing (2); Callback-shaped APIs (2) |
| 1 | `src/ds/SearchMatchBar/` | keep | 97 | 88 | keep | Callback-shaped APIs (3) |
| 1 | `src/ds/AssistantQuestionCard/` | keep | 308 | 87 | keep | Callback-shaped APIs (6) |
| 1 | `src/ds/FilterChips/` | keep | 306 | 87 | keep | Callback-shaped APIs (7) |
| 1 | `src/ds/VoicePlayer/` | keep | 230 | 87 | keep | Callback-shaped APIs (4); Insertion-ordered maps and object keys (2) |
| 1 | `src/ds/PollCard/` | keep | 216 | 86 | keep | String offsets and slicing (1); Callback-shaped APIs (2) |
| 1 | `src/ds/ProfileCard/` | keep | 194 | 86 | keep | Callback-shaped APIs (2) |
| 1 | `src/ds/VideoPreview/` | keep | 229 | 86 | keep | Callback-shaped APIs (4); Insertion-ordered maps and object keys (1) |
| 1 | `src/ds/ImageViewer/` | keep | 430 | 84 | keep | Callback-shaped APIs (5) |
| 1 | `src/ds/OutgoingAttachment/` | keep | 249 | 84 | keep | Callback-shaped APIs (3) |
| 1 | `src/ds/AudioPlayer/` | keep | 155 | 83 | keep | Callback-shaped APIs (4); Insertion-ordered maps and object keys (2) |
| 1 | `src/ds/CallWindow/` | keep | 521 | 80 | keep | Callback-shaped APIs (12); Dynamic typing (1) |
| 1 | `src/ds/MessageToolbar/` | keep | 714 | 75 | keep | Callback-shaped APIs (19) |
| 1 | `src/application/analytics/` | ready | 949 | 71 | napi-rs | - |
| 1 | `src/adapters/livekit/background/` | ready | 418 | 69 | napi-rs | Async and promises (13); Insertion-ordered maps and object keys (1) |
| 1 | `src/ds/RichText/` | keep | 788 | 40 | keep | String offsets and slicing (7); Regex features the `regex` crate lacks (3) |
| 2 | `src/ds/SidebarHeader/` | keep | 127 | 100 | keep | - |
| 2 | `src/ds/CheckCircle/` | keep | 47 | 99 | keep | - |
| 2 | `src/ds/EmptyConversation/` | keep | 138 | 98 | keep | - |
| 2 | `src/ds/GifPicker/` | keep | 134 | 97 | keep | Callback-shaped APIs (3) |
| 2 | `src/ds/CallLayout/` | keep | 111 | 94 | keep | - |
| 2 | `src/ds/FileChip/` | keep | 58 | 94 | keep | String offsets and slicing (2); Callback-shaped APIs (1) |
| 2 | `src/ds/Timeline/` | keep | 269 | 92 | keep | Callback-shaped APIs (5) |
| 2 | `src/ds/SpaceTree/` | keep | 203 | 86 | keep | String offsets and slicing (1); Callback-shaped APIs (3) |
| 2 | `src/ds/CallStage/` | keep | 503 | 79 | keep | Callback-shaped APIs (6); Async and promises (2) |
| 2 | `src/ds/MessageBubble/` | keep | 744 | 76 | keep | Callback-shaped APIs (4) |
| 2 | `src/ds/RoomRow/` | keep | 851 | 71 | keep | Callback-shaped APIs (3) |
| 2 | `src/application/ports/` | caution | 1746 | 34 | napi-rs | Callback-shaped APIs (317) |
| 2 | `src/ds/Composer/` | keep | 4434 | 0 | keep | String offsets and slicing (58); Regex features the `regex` crate lacks (2) |
| 3 | `src/ds/CenteredState/` | keep | 78 | 100 | keep | - |
| 3 | `src/ds/EmptyState/` | keep | 51 | 100 | keep | - |
| 3 | `src/ds/FloatingWindow/` | keep | 191 | 100 | keep | - |
| 3 | `src/ds/SectionTitle/` | keep | 85 | 100 | keep | Callback-shaped APIs (1) |
| 3 | `src/ds/BottomNav/` | keep | 140 | 99 | keep | Callback-shaped APIs (1) |
| 3 | `src/ds/CallLine/` | keep | 147 | 97 | keep | - |
| 3 | `src/ds/SettingsList/` | keep | 147 | 94 | keep | Callback-shaped APIs (2) |
| 3 | `src/ds/InviteRow/` | keep | 80 | 93 | keep | Callback-shaped APIs (2) |
| 3 | `src/adapters/memory/` | caution | 159 | 90 | napi-rs | Async and promises (4); Callback-shaped APIs (4) |
| 3 | `src/adapters/` | ready | 84 | 86 | napi-rs | Async and promises (4); Callback-shaped APIs (2) |
| 3 | `src/adapters/indexeddb/` | caution | 288 | 79 | napi-rs | Async and promises (35); Callback-shaped APIs (6) |
| 3 | `src/adapters/sentry/` | ready | 242 | 76 | napi-rs | Regex features the `regex` crate lacks (1); Async and promises (9) |
| 3 | `src/adapters/drive/` | ready | 376 | 66 | napi-rs | Async and promises (22); Callback-shaped APIs (8) |
| 3 | `src/adapters/browser/` | ready | 539 | 54 | napi-rs | String offsets and slicing (2); Async and promises (26) |
| 3 | `src/adapters/livekit/` | ready | 595 | 51 | napi-rs | String offsets and slicing (5); Async and promises (54) |
| 3 | `src/application/ports/contracts/` | caution | 1337 | 37 | napi-rs | String offsets and slicing (2); Async and promises (508) |
| 3 | `src/application/useCases/` | caution | 6797 | 0 | napi-rs | String offsets and slicing (1); Async and promises (354) |
| 4 | `src/adapters/analytics/` | ready | 94 | 100 | napi-rs | - |
| 4 | `src/adapters/assistant/` | ready | 103 | 100 | napi-rs | Async and promises (3); Exceptions (1) |
| 4 | `src/adapters/calendar/` | caution | 57 | 100 | napi-rs | Async and promises (2) |
| 4 | `src/adapters/mail/` | ready | 59 | 100 | napi-rs | Async and promises (2); Dynamic typing (1) |
| 4 | `src/adapters/proxy/` | ready | 194 | 90 | napi-rs | Async and promises (13); Insertion-ordered maps and object keys (1) |
| 4 | `src/adapters/sideService/` | caution | 269 | 86 | napi-rs | Async and promises (26); Insertion-ordered maps and object keys (1) |
| 4 | `src/adapters/tom/` | ready | 609 | 62 | napi-rs | String offsets and slicing (2); Async and promises (45) |
| 4 | `src/adapters/matrix/` | caution | 12412 | 0 | napi-rs | String offsets and slicing (33); Regex features the `regex` crate lacks (5) |
| 5 | `src/adapters/meet/` | ready | 335 | 75 | napi-rs | Async and promises (11); Callback-shaped APIs (4) |
| 6 | `src/ui/create/` | keep | 902 | 52 | keep | Callback-shaped APIs (22); Async and promises (5) |
| 6 | `src/app/` | keep | 5305 | 0 | keep | String offsets and slicing (8); Async and promises (215) |
| 6 | `src/ui/` | keep | 37766 | 0 | keep | String offsets and slicing (56); Regexes built at runtime (1) |
| 6 | `src/ui/calls/` | keep | 2811 | 0 | keep | String offsets and slicing (1); Callback-shaped APIs (41) |
| 7 | `src/adapters/matrix/integration/` | ready | 385 | 68 | napi-rs | String offsets and slicing (2); Async and promises (47) |

## npm dependencies

| Package | Units | Rust | Fit | Note |
| --- | ---: | --- | --- | --- |
| `@linagora/twake-sdk` | 2 | ? | unknown |  |
| `cozy-interapp` | 2 | ? | unknown |  |
| `matrix-js-sdk` | 2 | ? | unknown |  |
| `@linagora/twake-feedback` | 1 | ? | unknown |  |
| `@linagora/twake-oidc` | 1 | ? | unknown |  |
| `@livekit/track-processors` | 1 | ? | unknown |  |
| `@matrix-org/matrix-sdk-crypto-wasm` | 1 | ? | unknown |  |
| `@mediapipe/tasks-vision` | 1 | ? | unknown |  |
| `@sentry/react` | 1 | ? | unknown |  |
| `emojibase-data` | 1 | ? | unknown |  |
| `livekit-client` | 1 | ? | unknown |  |
| `micromark` | 1 | ? | unknown |  |
| `openid-client` | 1 | ? | unknown |  |

## Risk guide

- **String offsets and slicing** (High): JS indexes strings in UTF-16 code units, Rust in UTF-8 bytes. Offsets that cross the boundary must be converted, or non-ASCII text gets wrong ranges silently.
- **Regex features the `regex` crate lacks** (High): Lookaround, backreferences and the `y` flag are not in the `regex` crate. Use `fancy-regex` for those patterns, or rewrite them, and test them on non-ASCII text: `\w`, `\b` and case-insensitive matching differ between engines even where a pattern compiles.
- **Regexes built at runtime** (High): Patterns come from data or config in JS syntax. Rust regex syntax differs, so user-visible behavior can change. Pin it with tests on real patterns.
- **Dynamic typing** (Medium): Values whose shape is checked at runtime need an explicit Rust enum or `serde_json::Value`.
- **Insertion-ordered maps and object keys** (Medium): JS Map, Set and object keys keep insertion order. `HashMap` does not: use `indexmap` wherever order reaches output. With serde_json's `preserve_order`, `Map::remove` swaps the last key into the hole: use `shift_remove` for JS `delete`. Sort with `js::sort` when a comparator can return NaN: the order then depends on V8's algorithm.
- **Exceptions** (Low): Thrown errors become `Result`. Errors crossing the bridge must map back to the same JS error shape.
- **Async and promises** (Medium): Promises become futures. Bridge with napi-rs `async fn` or `wasm-bindgen-futures`, or keep the async shell in JS and port the sync core.
- **Class inheritance** (Medium): Class hierarchies become traits plus composition; `instanceof` checks become enums.
- **Reflection and prototypes** (Medium): Proxy, Reflect, Symbol and prototype tricks need a redesign, not a translation.
- **Callback-shaped APIs** (Medium): Function-typed parameters and methods cross the bridge on every call. Keep boundaries coarse: pass data in, get data out.
- **Number semantics** (Low): JS numbers are f64; bitwise ops truncate to i32; parseInt is lenient. Pick Rust integer types deliberately.
- **JSON round-trips** (Low): JSON.stringify key order and number formatting must match `serde_json` output if the result is user-visible.

## Whole app

Shape: **cli**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| `synapse-module/close-public-rooms.mjs` | cli | `#!/usr/bin/env node` at synapse-module/close-public-rooms.mjs:1 |
| `synapse-module/invite-listener.mjs` | cli | `#!/usr/bin/env node` at synapse-module/invite-listener.mjs:1 |
| `synapse-module/migrate-direct.mjs` | cli | `#!/usr/bin/env node` at synapse-module/migrate-direct.mjs:1 |

### Every dependency

| Dependency | Declared | Used by | Rust | Fit | Note |
| --- | --- | --- | --- | --- | --- |
| `@linagora/twake-mui` | yes | 127 | ? | unknown |  |
| `@linagora/twake-icons` | yes | 37 | ? | unknown |  |
| `@linagora/twake-sdk` | yes | 4 | ? | unknown |  |
| `@tanstack/react-query` | yes | 4 | ? | unknown |  |
| `react-router` | yes | 3 | ? | unknown |  |
| `@linagora/twake-css` | yes | 2 | ? | unknown |  |
| `@linagora/twake-feedback` | yes | 2 | ? | unknown |  |
| `@mui/material` | yes | 2 | ? | unknown |  |
| `cozy-interapp` | yes | 2 | ? | unknown |  |
| `emojibase-data` | yes | 2 | ? | unknown |  |
| `matrix-js-sdk` | yes | 2 | ? | unknown |  |
| `twake-i18n` | yes | 2 | ? | unknown |  |
| `@emotion/cache` | yes | 1 | ? | unknown |  |
| `@emotion/react` | yes | 1 | ? | unknown |  |
| `@linagora/twake-bar` | yes | 1 | ? | unknown |  |
| `@linagora/twake-embed` | yes | 1 | ? | unknown |  |
| `@linagora/twake-oidc` | yes | 1 | ? | unknown |  |
| `@linagora/twake-react-new-version` | yes | 1 | ? | unknown |  |
| `@livekit/track-processors` | yes | 1 | ? | unknown |  |
| `@matrix-org/matrix-sdk-crypto-wasm` | yes | 1 | ? | unknown |  |
| `@mediapipe/tasks-vision` | yes | 1 | ? | unknown |  |
| `@sentry/react` | yes | 1 | ? | unknown |  |
| `dompurify` | yes | 1 | ? | unknown |  |
| `jsqr` | yes | 1 | ? | unknown |  |
| `livekit-client` | yes | 1 | ? | unknown |  |
| `lottie-web` | yes | 1 | ? | unknown |  |
| `micromark` | no | 1 | ? | unknown |  |
| `openid-client` | yes | 1 | ? | unknown |  |
| `qrcode-generator` | yes | 1 | ? | unknown |  |
| `react-dom` | yes | 1 | ? | unknown |  |
| `web-vitals` | yes | 1 | ? | unknown |  |
| `@emotion/styled` | yes | 0 | ? | unknown |  |
| `@mui/lab` | yes | 0 | ? | unknown |  |
| `cozy-intent` | yes | 0 | ? | unknown |  |
| `react` | yes | 0 | ? | unknown |  |

### Pieces, in port order

- wave 0: `public/` (699 lines)
- wave 0: `public/embed/` (5 lines)
- wave 0: `scripts/` (340 lines)
- wave 0: `src/adapters/oidc/` (195 lines)
- wave 0: `src/domain/` (6124 lines)
- wave 0: `synapse-module/` (806 lines)
- wave 1: `src/adapters/livekit/background/` (418 lines)
- wave 1: `src/application/analytics/` (949 lines)
- wave 2: `src/application/ports/` (1746 lines)
- wave 3: `src/adapters/` (84 lines)
- wave 3: `src/adapters/browser/` (539 lines)
- wave 3: `src/adapters/drive/` (376 lines)
- wave 3: `src/adapters/indexeddb/` (288 lines)
- wave 3: `src/adapters/livekit/` (595 lines)
- wave 3: `src/adapters/memory/` (159 lines)
- wave 3: `src/adapters/sentry/` (242 lines)
- wave 3: `src/application/ports/contracts/` (1337 lines)
- wave 3: `src/application/useCases/` (6797 lines)
- wave 4: `src/adapters/analytics/` (94 lines)
- wave 4: `src/adapters/assistant/` (103 lines)
- wave 4: `src/adapters/calendar/` (57 lines)
- wave 4: `src/adapters/mail/` (59 lines)
- wave 4: `src/adapters/matrix/` (12412 lines)
- wave 4: `src/adapters/proxy/` (194 lines)
- wave 4: `src/adapters/sideService/` (269 lines)
- wave 4: `src/adapters/tom/` (609 lines)
- wave 5: `src/adapters/meet/` (335 lines)
- wave 7: `src/adapters/matrix/integration/` (385 lines)

### Decisions for the user

**D1** (shape): The app is a cli, entered through synapse-module/close-public-rooms.mjs, synapse-module/invite-listener.mjs, synapse-module/migrate-direct.mjs. Port every entry point, in that shape?

> Yes: a clap front door (`app front-door`) takes every command over first

**D2** (keep src/app/): src/app/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D3** (keep src/ds/): src/ds/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D4** (keep src/ds/ActivityItem/): src/ds/ActivityItem/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D5** (keep src/ds/AddRowContent/): src/ds/AddRowContent/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D6** (keep src/ds/AlbumGrid/): src/ds/AlbumGrid/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D7** (keep src/ds/AppBar/): src/ds/AppBar/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D8** (keep src/ds/Arriving/): src/ds/Arriving/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D9** (keep src/ds/AssistantAvatar/): src/ds/AssistantAvatar/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D10** (keep src/ds/AssistantMark/): src/ds/AssistantMark/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D11** (keep src/ds/AssistantQuestionCard/): src/ds/AssistantQuestionCard/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D12** (keep src/ds/AudioPlayer/): src/ds/AudioPlayer/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D13** (keep src/ds/AvatarStack/): src/ds/AvatarStack/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D14** (keep src/ds/BannerStack/): src/ds/BannerStack/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D15** (keep src/ds/BottomNav/): src/ds/BottomNav/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D16** (keep src/ds/CallFrame/): src/ds/CallFrame/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D17** (keep src/ds/CallInvite/): src/ds/CallInvite/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D18** (keep src/ds/CallLayout/): src/ds/CallLayout/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D19** (keep src/ds/CallLine/): src/ds/CallLine/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D20** (keep src/ds/CallStage/): src/ds/CallStage/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D21** (keep src/ds/CallWindow/): src/ds/CallWindow/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D22** (keep src/ds/CameraScanner/): src/ds/CameraScanner/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D23** (keep src/ds/CappedList/): src/ds/CappedList/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D24** (keep src/ds/CenteredState/): src/ds/CenteredState/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D25** (keep src/ds/ChatShell/): src/ds/ChatShell/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D26** (keep src/ds/CheckCircle/): src/ds/CheckCircle/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D27** (keep src/ds/CollapsingHeader/): src/ds/CollapsingHeader/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D28** (keep src/ds/ColorDots/): src/ds/ColorDots/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D29** (keep src/ds/Composer/): src/ds/Composer/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D30** (keep src/ds/ConsentBar/): src/ds/ConsentBar/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D31** (keep src/ds/CornerBadge/): src/ds/CornerBadge/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D32** (keep src/ds/DecimalComparison/): src/ds/DecimalComparison/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D33** (keep src/ds/Density/): src/ds/Density/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D34** (keep src/ds/DetailPage/): src/ds/DetailPage/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D35** (keep src/ds/DialogHeader/): src/ds/DialogHeader/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D36** (keep src/ds/DialogTitleBar/): src/ds/DialogTitleBar/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D37** (keep src/ds/Divider/): src/ds/Divider/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D38** (keep src/ds/DropZone/): src/ds/DropZone/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D39** (keep src/ds/EmojiComparison/): src/ds/EmojiComparison/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D40** (keep src/ds/EmojiPicker/): src/ds/EmojiPicker/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D41** (keep src/ds/EmptyConversation/): src/ds/EmptyConversation/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D42** (keep src/ds/EmptyState/): src/ds/EmptyState/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D43** (keep src/ds/FabDock/): src/ds/FabDock/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D44** (keep src/ds/FileChip/): src/ds/FileChip/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D45** (keep src/ds/FilterChips/): src/ds/FilterChips/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D46** (keep src/ds/FloatingWindow/): src/ds/FloatingWindow/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D47** (keep src/ds/FormColumn/): src/ds/FormColumn/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D48** (keep src/ds/GifPicker/): src/ds/GifPicker/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D49** (keep src/ds/GroupLabel/): src/ds/GroupLabel/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D50** (keep src/ds/HeroPhoto/): src/ds/HeroPhoto/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D51** (keep src/ds/Highlight/): src/ds/Highlight/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D52** (keep src/ds/HoverActionItem/): src/ds/HoverActionItem/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D53** (keep src/ds/HoverRow/): src/ds/HoverRow/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D54** (keep src/ds/IconAction/): src/ds/IconAction/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D55** (keep src/ds/ImagePreview/): src/ds/ImagePreview/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D56** (keep src/ds/ImageViewer/): src/ds/ImageViewer/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D57** (keep src/ds/InertPreview/): src/ds/InertPreview/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D58** (keep src/ds/InfoSkeleton/): src/ds/InfoSkeleton/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D59** (keep src/ds/InlineMarkdown/): src/ds/InlineMarkdown/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D60** (keep src/ds/IntentDialog/): src/ds/IntentDialog/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D61** (keep src/ds/InviteRow/): src/ds/InviteRow/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D62** (keep src/ds/LinkPreviewCard/): src/ds/LinkPreviewCard/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D63** (keep src/ds/LinkRow/): src/ds/LinkRow/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D64** (keep src/ds/ListDialog/): src/ds/ListDialog/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D65** (keep src/ds/ListFooter/): src/ds/ListFooter/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D66** (keep src/ds/ListRow/): src/ds/ListRow/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D67** (keep src/ds/LiveAnnouncement/): src/ds/LiveAnnouncement/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D68** (keep src/ds/LiveStatus/): src/ds/LiveStatus/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D69** (keep src/ds/Loading/): src/ds/Loading/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D70** (keep src/ds/MainColumn/): src/ds/MainColumn/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D71** (keep src/ds/MediaGrid/): src/ds/MediaGrid/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D72** (keep src/ds/MenuAction/): src/ds/MenuAction/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D73** (keep src/ds/MenuButton/): src/ds/MenuButton/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D74** (keep src/ds/MenuRadio/): src/ds/MenuRadio/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D75** (keep src/ds/MessageBubble/): src/ds/MessageBubble/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D76** (keep src/ds/MessageToolbar/): src/ds/MessageToolbar/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D77** (keep src/ds/MessagesSkeleton/): src/ds/MessagesSkeleton/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D78** (keep src/ds/MiddleCut/): src/ds/MiddleCut/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D79** (keep src/ds/NameAvatar/): src/ds/NameAvatar/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D80** (keep src/ds/Notice/): src/ds/Notice/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D81** (keep src/ds/OptionCards/): src/ds/OptionCards/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D82** (keep src/ds/OutgoingAttachment/): src/ds/OutgoingAttachment/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D83** (keep src/ds/PaneHeader/): src/ds/PaneHeader/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D84** (keep src/ds/PanelCard/): src/ds/PanelCard/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D85** (keep src/ds/PanelHero/): src/ds/PanelHero/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D86** (keep src/ds/PanelTile/): src/ds/PanelTile/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D87** (keep src/ds/PickRowButton/): src/ds/PickRowButton/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D88** (keep src/ds/PickedStrip/): src/ds/PickedStrip/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D89** (keep src/ds/PicturePicker/): src/ds/PicturePicker/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D90** (keep src/ds/PillButton/): src/ds/PillButton/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D91** (keep src/ds/PinnedBanner/): src/ds/PinnedBanner/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D92** (keep src/ds/PollCard/): src/ds/PollCard/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D93** (keep src/ds/PresenceBadge/): src/ds/PresenceBadge/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D94** (keep src/ds/ProfileCard/): src/ds/ProfileCard/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D95** (keep src/ds/PromptButtons/): src/ds/PromptButtons/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D96** (keep src/ds/QrCode/): src/ds/QrCode/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D97** (keep src/ds/QuestionButtons/): src/ds/QuestionButtons/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D98** (keep src/ds/ReactionChips/): src/ds/ReactionChips/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D99** (keep src/ds/ReadTicks/): src/ds/ReadTicks/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D100** (keep src/ds/RequestCard/): src/ds/RequestCard/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D101** (keep src/ds/ResolvedMark/): src/ds/ResolvedMark/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D102** (keep src/ds/RichText/): src/ds/RichText/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D103** (keep src/ds/RoomLine/): src/ds/RoomLine/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D104** (keep src/ds/RoomRow/): src/ds/RoomRow/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D105** (keep src/ds/SearchField/): src/ds/SearchField/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D106** (keep src/ds/SearchMatchBar/): src/ds/SearchMatchBar/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D107** (keep src/ds/SectionTitle/): src/ds/SectionTitle/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D108** (keep src/ds/SeenBy/): src/ds/SeenBy/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D109** (keep src/ds/SelectionBar/): src/ds/SelectionBar/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D110** (keep src/ds/SettingsList/): src/ds/SettingsList/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D111** (keep src/ds/SettingsSection/): src/ds/SettingsSection/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D112** (keep src/ds/SidebarHeader/): src/ds/SidebarHeader/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D113** (keep src/ds/SpaceCard/): src/ds/SpaceCard/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D114** (keep src/ds/SpaceTree/): src/ds/SpaceTree/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D115** (keep src/ds/SplashLoader/): src/ds/SplashLoader/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D116** (keep src/ds/Spotlight/): src/ds/Spotlight/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D117** (keep src/ds/Stack/): src/ds/Stack/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D118** (keep src/ds/StatusLine/): src/ds/StatusLine/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D119** (keep src/ds/ThreadCard/): src/ds/ThreadCard/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D120** (keep src/ds/Timeline/): src/ds/Timeline/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D121** (keep src/ds/TintedIcon/): src/ds/TintedIcon/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D122** (keep src/ds/ToggleRow/): src/ds/ToggleRow/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D123** (keep src/ds/TwoColumns/): src/ds/TwoColumns/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D124** (keep src/ds/VideoPreview/): src/ds/VideoPreview/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D125** (keep src/ds/VoicePlayer/): src/ds/VoicePlayer/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D126** (keep src/ui/): src/ui/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D127** (keep src/ui/calls/): src/ui/calls/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D128** (keep src/ui/create/): src/ui/create/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D129** (dependency @linagora/twake-mui): Which crate replaces @linagora/twake-mui (used by 127 units)?

> Research it against primary sources into rust/research/@linagora-twake-mui.md before porting what uses it

**D130** (dependency @linagora/twake-icons): Which crate replaces @linagora/twake-icons (used by 37 units)?

> Research it against primary sources into rust/research/@linagora-twake-icons.md before porting what uses it

**D131** (dependency @linagora/twake-sdk): Which crate replaces @linagora/twake-sdk (used by 4 units)?

> Research it against primary sources into rust/research/@linagora-twake-sdk.md before porting what uses it

**D132** (dependency @tanstack/react-query): Which crate replaces @tanstack/react-query (used by 4 units)?

> Research it against primary sources into rust/research/@tanstack-react-query.md before porting what uses it

**D133** (dependency react-router): Which crate replaces react-router (used by 3 units)?

> Research it against primary sources into rust/research/react-router.md before porting what uses it

**D134** (dependency @linagora/twake-css): Which crate replaces @linagora/twake-css (used by 2 units)?

> Research it against primary sources into rust/research/@linagora-twake-css.md before porting what uses it

**D135** (dependency @linagora/twake-feedback): Which crate replaces @linagora/twake-feedback (used by 2 units)?

> Research it against primary sources into rust/research/@linagora-twake-feedback.md before porting what uses it

**D136** (dependency @mui/material): Which crate replaces @mui/material (used by 2 units)?

> Research it against primary sources into rust/research/@mui-material.md before porting what uses it

**D137** (dependency cozy-interapp): Which crate replaces cozy-interapp (used by 2 units)?

> Research it against primary sources into rust/research/cozy-interapp.md before porting what uses it

**D138** (dependency emojibase-data): Which crate replaces emojibase-data (used by 2 units)?

> Research it against primary sources into rust/research/emojibase-data.md before porting what uses it

**D139** (dependency matrix-js-sdk): Which crate replaces matrix-js-sdk (used by 2 units)?

> Research it against primary sources into rust/research/matrix-js-sdk.md before porting what uses it

**D140** (dependency twake-i18n): Which crate replaces twake-i18n (used by 2 units)?

> Research it against primary sources into rust/research/twake-i18n.md before porting what uses it

**D141** (dependency @emotion/cache): Which crate replaces @emotion/cache (used by 1 unit)?

> Research it against primary sources into rust/research/@emotion-cache.md before porting what uses it

**D142** (dependency @emotion/react): Which crate replaces @emotion/react (used by 1 unit)?

> Research it against primary sources into rust/research/@emotion-react.md before porting what uses it

**D143** (dependency @linagora/twake-bar): Which crate replaces @linagora/twake-bar (used by 1 unit)?

> Research it against primary sources into rust/research/@linagora-twake-bar.md before porting what uses it

**D144** (dependency @linagora/twake-embed): Which crate replaces @linagora/twake-embed (used by 1 unit)?

> Research it against primary sources into rust/research/@linagora-twake-embed.md before porting what uses it

**D145** (dependency @linagora/twake-oidc): Which crate replaces @linagora/twake-oidc (used by 1 unit)?

> Research it against primary sources into rust/research/@linagora-twake-oidc.md before porting what uses it

**D146** (dependency @linagora/twake-react-new-version): Which crate replaces @linagora/twake-react-new-version (used by 1 unit)?

> Research it against primary sources into rust/research/@linagora-twake-react-new-version.md before porting what uses it

**D147** (dependency @livekit/track-processors): Which crate replaces @livekit/track-processors (used by 1 unit)?

> Research it against primary sources into rust/research/@livekit-track-processors.md before porting what uses it

**D148** (dependency @matrix-org/matrix-sdk-crypto-wasm): Which crate replaces @matrix-org/matrix-sdk-crypto-wasm (used by 1 unit)?

> Research it against primary sources into rust/research/@matrix-org-matrix-sdk-crypto-wasm.md before porting what uses it

**D149** (dependency @mediapipe/tasks-vision): Which crate replaces @mediapipe/tasks-vision (used by 1 unit)?

> Research it against primary sources into rust/research/@mediapipe-tasks-vision.md before porting what uses it

**D150** (dependency @sentry/react): Which crate replaces @sentry/react (used by 1 unit)?

> Research it against primary sources into rust/research/@sentry-react.md before porting what uses it

**D151** (dependency dompurify): Which crate replaces dompurify (used by 1 unit)?

> Research it against primary sources into rust/research/dompurify.md before porting what uses it

**D152** (dependency jsqr): Which crate replaces jsqr (used by 1 unit)?

> Research it against primary sources into rust/research/jsqr.md before porting what uses it

**D153** (dependency livekit-client): Which crate replaces livekit-client (used by 1 unit)?

> Research it against primary sources into rust/research/livekit-client.md before porting what uses it

**D154** (dependency lottie-web): Which crate replaces lottie-web (used by 1 unit)?

> Research it against primary sources into rust/research/lottie-web.md before porting what uses it

**D155** (dependency micromark): Which crate replaces micromark (used by 1 unit)?

> Research it against primary sources into rust/research/micromark.md before porting what uses it

**D156** (dependency openid-client): Which crate replaces openid-client (used by 1 unit)?

> Research it against primary sources into rust/research/openid-client.md before porting what uses it

**D157** (dependency qrcode-generator): Which crate replaces qrcode-generator (used by 1 unit)?

> Research it against primary sources into rust/research/qrcode-generator.md before porting what uses it

**D158** (dependency react-dom): Which crate replaces react-dom (used by 1 unit)?

> Research it against primary sources into rust/research/react-dom.md before porting what uses it

**D159** (dependency web-vitals): Which crate replaces web-vitals (used by 1 unit)?

> Research it against primary sources into rust/research/web-vitals.md before porting what uses it

**D160** (dependency @emotion/styled): Which crate replaces @emotion/styled (declared, no unit imports it)?

> Research it against primary sources into rust/research/@emotion-styled.md before porting what uses it

**D161** (dependency @mui/lab): Which crate replaces @mui/lab (declared, no unit imports it)?

> Research it against primary sources into rust/research/@mui-lab.md before porting what uses it

**D162** (dependency cozy-intent): Which crate replaces cozy-intent (declared, no unit imports it)?

> Research it against primary sources into rust/research/cozy-intent.md before porting what uses it

**D163** (dependency react): Which crate replaces react (declared, no unit imports it)?

> Research it against primary sources into rust/research/react.md before porting what uses it

