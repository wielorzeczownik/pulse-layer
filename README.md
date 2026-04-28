<h1 align="center">PulseLayer</h1>

<p align="center">
  <a href="https://github.com/wielorzeczownik/pulse-layer/actions/workflows/release.yml"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/pulse-layer/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/pulse-layer/release.yml?branch=main&style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/pulse-layer/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950" alt="Build"/></picture></a> <a href="https://github.com/wielorzeczownik/pulse-layer/releases/latest"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/pulse-layer?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/pulse-layer?style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/v/release/wielorzeczownik/pulse-layer?style=flat-square&labelColor=2d333b&color=3fb950" alt="Latest Release"/></picture></a> <a href="https://github.com/wielorzeczownik/pulse-layer/blob/main/LICENSE"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/badge/License-MIT-2ea043?style=flat-square"/><img src="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b" alt="License: MIT"/></picture></a>
  <br/>
  <img src="https://img.shields.io/badge/Rust-B7410E?style=flat-square&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/Iced-4D9DE0?style=flat-square&logo=iced&logoColor=white" alt="Iced"/>
  <img src="https://img.shields.io/badge/TypeScript-3178C6?style=flat-square&logo=typescript&logoColor=white" alt="TypeScript"/>
</p>

<p align="center">🇬🇧 English | 🇵🇱 <a href="README.pl.md">Polski</a></p>

A real-time **heart rate OBS overlay** that reads live BPM from a Bluetooth smart ring and serves a browser widget directly to your stream. No subscriptions, no cloud – runs entirely on your machine.

Made for VTubers and streamers who want to show live heart rate on stream with nothing more than a cheap smart ring and OBS. Originally built for **[KitsuneTsuyu](https://www.twitch.tv/kitsunetsuyu)**.

## Features

- Live BPM with color-coded zones (calm → alarm)
- Two overlay styles: **heart** or **ECG**
- Custom hex colors per zone

## Compatibility

Tested on **Smartring COLMI R12**. Older COLMI models and other rings using the same BLE protocol should work too.

> [!NOTE]
> Only Qring App rings are supported right now. Adding other brands isn't planned, but if there's enough interest I'm open to it.

## Download & Install

Latest release: [GitHub Releases](https://github.com/wielorzeczownik/pulse-layer/releases/latest)

Download the latest release asset for your platform:

**Linux:**

- [pulse-layer-x86_64-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/pulse-layer/releases/latest/download/pulse-layer-x86_64-unknown-linux-gnu.tar.gz) – Linux (Intel/AMD 64-bit)

**macOS:**

- [pulse-layer-x86_64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/pulse-layer/releases/latest/download/pulse-layer-x86_64-apple-darwin.tar.gz) – macOS on Intel
- [pulse-layer-aarch64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/pulse-layer/releases/latest/download/pulse-layer-aarch64-apple-darwin.tar.gz) – macOS on Apple Silicon (M1/M2/M3/M4)

**Windows:**

- [pulse-layer-x86_64-pc-windows-msvc.zip](https://github.com/wielorzeczownik/pulse-layer/releases/latest/download/pulse-layer-x86_64-pc-windows-msvc.zip) – Windows 64-bit (x86_64)
- [pulse-layer-aarch64-pc-windows-msvc.zip](https://github.com/wielorzeczownik/pulse-layer/releases/latest/download/pulse-layer-aarch64-pc-windows-msvc.zip) – Windows ARM64
- [pulse-layer-i686-pc-windows-msvc.zip](https://github.com/wielorzeczownik/pulse-layer/releases/latest/download/pulse-layer-i686-pc-windows-msvc.zip) – Windows 32-bit (x86)

### macOS

Extract the archive and you'll get `PulseLayer.app`. On first launch macOS will block it because the app isn't signed with a paid Apple certificate. To get past that:

**Option A – right-click:**

1. Right-click `PulseLayer.app` → **Open**
2. Click **Open** in the dialog

**Option B – terminal (one-time):**

```bash
xattr -cr PulseLayer.app
open PulseLayer.app
```

### Windows

Extract the zip and run `pulse-layer.exe`. No installer needed.

### Linux

```bash
tar -xzf pulse-layer-*.tar.gz
./pulse-layer
```

## Connecting your ring

### First-time pairing

1. Make sure your ring is charged and nearby.
2. Launch PulseLayer and click **Scan**.
3. Your ring appears in the list – click **Connect**.

### Ring is already paired with your phone

The ring can only talk to one device at a time. If it's connected to the Qring app on your phone, PulseLayer won't see it.

Fix: **turn off Bluetooth on your phone** before scanning. PulseLayer will pick it up normally.

### Going back to your phone after using PulseLayer

The ring remembers the pairing. To re-pair with your phone you need to clear it from both sides:

1. In PulseLayer – click **Disconnect**.
2. On your computer – Bluetooth settings, find the ring, click **Forget / Remove device**.
3. On your phone – forget the ring too.
4. Re-pair from scratch via the Qring app.

> [!IMPORTANT]
> Skipping step 2 or 3 will cause the pairing to fail on the phone side. Both devices need to forget the ring first.

## OBS Setup

The overlay runs as a local browser page served by PulseLayer. No external hosting needed.

1. **Start PulseLayer** and connect to your ring – the server starts on port `9000`.
2. In OBS, click **+** in the Sources panel → **Browser**.
3. Set the URL to:

   ```text
   http://localhost:9000
   ```

4. Set **Width** to `400` and **Height** to `300` (you can resize/crop freely in OBS).
5. Check **Refresh browser when scene becomes active** if you want it to reconnect automatically.
6. Click **OK** and the widget shows up. Use **Edit Transform** to place it where you want.

> [!TIP]
> The widget anchors to the bottom-left corner of the browser frame. Crop the browser source tightly and you can place it anywhere on the canvas.

The overlay reconnects on its own if PulseLayer restarts, so you don't need to touch the browser source again.

## Overlay styles

Switch between styles in the **Settings** panel inside PulseLayer.

**Heart** – a beating heart icon with a large BPM number. Simple and readable.

**ECG** – an animated ECG line that draws once per heartbeat, with a BPM readout below. Better for a more technical look.

Both change color based on BPM zone. All colors are customizable.

| Zone   | Default BPM range | Default color |
| ------ | ----------------- | ------------- |
| Calm   | 0 – 64            | Green         |
| Normal | 65 – 80           | Blue          |
| High   | 81 – 100          | Yellow        |
| Fast   | 101 – 130         | Orange        |
| Alarm  | 131+              | Red           |

## Building from source

Requirements: [Rust](https://rustup.rs) stable, [Node.js](https://nodejs.org) 24+.

```bash
git clone https://github.com/wielorzeczownik/pulse-layer
cd pulse-layer
cargo build --release
```

The overlay frontend is compiled by Vite as part of `cargo build` and embedded into the binary – no separate build step.

**macOS quick launch (debug build):**

```bash
./run_macos.sh
```

Builds a `.app` bundle under `target/`, signs it ad-hoc, and opens it.

### Linux system dependencies

```bash
# Ubuntu / Debian
sudo apt-get install libdbus-1-dev pkg-config libxkbcommon-dev \
  libxcb-shape0-dev libxcb-xfixes0-dev libwayland-dev
```

## Disclaimer

Unofficial, unaffiliated with COLMI or any ring manufacturer. May break if the ring firmware changes the BLE protocol.
