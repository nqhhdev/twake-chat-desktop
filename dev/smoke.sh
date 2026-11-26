#!/usr/bin/env bash
# Tests, release build, then the app signed in as a test user, opening a room, saving a
# screenshot and quitting. Usage: dev/smoke.sh [user] [room] [snapshot-name]
set -euo pipefail
cd "$(dirname "$0")/.."
user=${1:-alice}; room=${2:-General}; name=${3:-smoke}
# FRESH=1: forget this test user's device and store first (a brand new sign-in).
if [ -n "${FRESH:-}" ]; then
  for base in "$HOME/Library/Application Support/app.Linagora.Twake-Chat" "${XDG_DATA_HOME:-$HOME/.local/share}/twakechat"; do
    rm -rf "$base/profiles/dev-$user"
  done
fi
cargo test --workspace --message-format short
cargo build --release -p twake-desktop
TWAKE_PROFILE="dev-$user" TWAKE_DEV_SESSION_FILE=1 TWAKE_DEV_USER="$user" TWAKE_DEV_PASSWORD="$user-dev-password" \
TWAKE_DEV_HOMESERVER="http://localhost:18008" TWAKE_DEV_ROOM="$room" TWAKE_DEV_STATS=1 \
TWAKE_DEV_SNAPSHOT="$PWD/dev/snapshots/$name.png" TWAKE_DEV_QUIT=1 \
  ./target/release/twake-chat
