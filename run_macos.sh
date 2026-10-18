#!/usr/bin/env bash
set -euo pipefail

cargo build
cargo packager --formats app
open ./target/debug/PulseLayer.app
