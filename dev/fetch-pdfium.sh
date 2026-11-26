#!/usr/bin/env bash
# Downloads PDFium (ADR-017) into vendor/pdfium/<platform>, where debug builds find it.
# The build must match pdfium-render's `pdfium_8125` feature (crates/twake-core/Cargo.toml).
# Usage: dev/fetch-pdfium.sh [platform]   (mac-arm64, mac-x64, linux-x64, linux-arm64,
#                                          win-x64, win-arm64; default: this computer)
set -euo pipefail
cd "$(dirname "$0")/.."
build=8125
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
dir=vendor/pdfium/$platform
mkdir -p "$dir"
curl -fL "https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F$build/pdfium-$platform.tgz" \
  | tar -xz -C "$dir"
ls "$dir"/lib "$dir"/bin 2>/dev/null || true
