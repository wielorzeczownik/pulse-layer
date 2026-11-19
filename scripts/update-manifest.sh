#!/usr/bin/env bash
set -euo pipefail

dist=$1
base="https://github.com/$2/releases/download/$3"

entry() {
  jq -n --arg url "$base/$1" --arg sig "$(cat "$dist/$1.sig")" --arg format "$2" \
    '{url: $url, signature: $sig, format: $format}'
}

# The universal macOS build serves both architectures
mac=$(entry pulse-layer-universal-apple-darwin.tar.gz app)

jq -n \
  --arg version "$3" \
  --arg date "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson mac "$mac" \
  --argjson win64 "$(entry pulse-layer-x86_64-pc-windows-msvc-setup.exe nsis)" \
  --argjson win32 "$(entry pulse-layer-i686-pc-windows-msvc-setup.exe nsis)" \
  --argjson winarm "$(entry pulse-layer-aarch64-pc-windows-msvc-setup.exe nsis)" \
  --argjson linux "$(entry pulse-layer-x86_64-unknown-linux-gnu.AppImage appimage)" \
  '{version: $version, pub_date: $date, platforms: {
    "macos-x86_64": $mac, "macos-aarch64": $mac,
    "windows-x86_64": $win64, "windows-i686": $win32, "windows-aarch64": $winarm,
    "linux-x86_64": $linux
  }}'
