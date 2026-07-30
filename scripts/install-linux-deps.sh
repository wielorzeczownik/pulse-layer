#!/usr/bin/env bash
# Installs the system libraries the Linux build needs: D-Bus for btleplug, and
# the xkbcommon / xcb / wayland headers for the iced window backend.
#
# It lives in one file because the list is needed by both the validate and the
# release workflow. A copy that drifts fails the build with a linker error that
# names a symbol, not a missing package.
set -euo pipefail

sudo apt-get update
sudo apt-get install --no-install-recommends -y \
  libdbus-1-dev \
  libwayland-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev \
  libxkbcommon-dev \
  pkg-config
