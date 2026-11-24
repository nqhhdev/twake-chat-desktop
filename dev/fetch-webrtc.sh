#!/usr/bin/env bash
# Downloads the prebuilt libwebrtc of the LiveKit Rust SDK (calls, M5) into
# vendor/webrtc/<platform>-release, where .cargo/config.toml points LK_CUSTOM_WEBRTC.
# The tag must match the `webrtc-sys` crate in Cargo.lock (its WEBRTC_TAG).
# Usage: dev/fetch-webrtc.sh [platform]   (mac-arm64, mac-x64, linux-x64, linux-arm64,
#                                          win-x64, win-arm64; default: this computer)
set -euo pipefail
cd "$(dirname "$0")/.."
tag=webrtc-a545de4
platform=${1:-}
if [ -z "$platform" ]; then
  case "$(uname -s)-$(uname -m)" in
    Darwin-arm64) platform=mac-arm64 ;;
    Darwin-x86_64) platform=mac-x64 ;;
    Linux-x86_64) platform=linux-x64 ;;
    Linux-aarch64) platform=linux-arm64 ;;
    *) echo "unknown platform: pass one" >&2; exit 1 ;;
  esac
fi
mkdir -p vendor/webrtc
zip=vendor/webrtc/$platform.zip
curl -fL -o "$zip" \
  "https://github.com/livekit/rust-sdks/releases/download/$tag/webrtc-$platform-release.zip"
unzip -q -o "$zip" -d vendor/webrtc
rm "$zip"
ls vendor/webrtc
