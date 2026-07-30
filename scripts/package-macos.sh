#!/usr/bin/env bash
# Assembles PulseLayer.app around an already-built binary and archives it.
#
# Expects TARGET (the Rust target triple) and VERSION (without the leading "v").
set -euo pipefail

: "${TARGET:?TARGET is required}"
: "${VERSION:?VERSION is required}"

app="PulseLayer.app"

mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp "target/${TARGET}/release/pulse-layer" "$app/Contents/MacOS/pulse-layer"
sed "s/__VERSION__/${VERSION}/" macos/Info.plist >"$app/Contents/Info.plist"
chmod +x "$app/Contents/MacOS/pulse-layer"

# Ad-hoc signature: macOS refuses to show the Bluetooth permission prompt for an
# unsigned bundle, so the app would silently never find a ring.
codesign --force --deep --sign - "$app"

mkdir -p dist
tar -czf "dist/pulse-layer-${TARGET}.tar.gz" "$app"
