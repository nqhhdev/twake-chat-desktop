# Twake Chat desktop (native Rust)

A native rewrite of [Twake Chat](https://github.com/linagora/twake-space-chat) for macOS, Windows
and Linux: Rust end to end, [Slint](https://slint.dev) for the UI, matrix-rust-sdk for Matrix.
No webview, no JavaScript. The goal is the same product as the web app, faster and lighter.

Work in progress: see [rust/MAP.md](rust/MAP.md) for the milestones and
[rust/DECISIONS.md](rust/DECISIONS.md) for why it is built this way.

## Build and run

Needs Rust 1.97 or newer (`rustup update stable`).

```sh
cargo run --release -p twake-desktop
```

Linux also needs the Secret Service (GNOME Keyring or KWallet) for the session, and the
development packages of `libdbus-1`, `libxkbcommon`, `fontconfig` and `libasound2` (ALSA).

## Local test server

```sh
dev/up.sh     # Synapse on http://localhost:18008, users alice, bob, carol
dev/seed.sh   # rooms and messages for them
```

Passwords are `<user>-dev-password` (local test values only). Sign in with the homeserver
`http://localhost:18008`.

## Checks

```sh
cargo test --workspace
dev/smoke.sh alice General   # tests, release build, signs in, opens a room, saves a screenshot
```

The domain logic is checked against recordings of the web app's TypeScript (ADR-011):

```sh
rust-anything build rust/units/domain-rs && rust-anything diff rust/units/domain-rs
```

Recording again needs `npm install --prefix rust/tools/node` and runs from
`reference/twake-space-chat` (a copy of the web app at the commit in `reference/SOURCE.txt`).

## Licence

AGPL-3.0, as the web app.
