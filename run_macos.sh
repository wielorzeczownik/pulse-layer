#!/usr/bin/env bash
set -euo pipefail

APP="./target/PulseLayer.app"
BIN="./target/debug/pulse-layer"

cargo build

VERSION=$(awk -F'"' '/^version[[:space:]]*=[[:space:]]*"/ { print $2; exit }' Cargo.toml)

mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp "$BIN" "$APP/Contents/MacOS/pulse-layer"
sed "s/__VERSION__/$VERSION/" ./macos/Info.plist > "$APP/Contents/Info.plist"
chmod +x "$APP/Contents/MacOS/pulse-layer"

# Ad-hoc sign — macOS requires this for BLE permission prompts to appear
codesign --force --deep --sign - "$APP"

open "$APP"
