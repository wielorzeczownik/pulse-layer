# Contributing to PulseLayer

Thank you for considering a contribution. This document covers everything you need to get started.

## Overview

A macOS application that reads data from BLE smart rings and displays it in an on-screen overlay.

## Project structure

```text
.
├── src/                     Rust source code
├── overlay/                 on-screen overlay frontend (Vite + TypeScript)
├── macos/                   macOS app bundle metadata
└── scripts/
    └── bump-version.sh      determines the next release version and bumps Cargo.toml
```

## Development setup

```bash
git clone https://github.com/wielorzeczownik/pulse-layer.git
cd pulse-layer
cargo run
```

The `overlay/` frontend is built automatically by `build.rs` during `cargo build`/`cargo run`.

## Running checks locally

### With tools installed locally

```bash
# Rust
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --all-targets --locked
cargo audit

# Overlay
cd overlay
npm run format:check
npm run lint
npm run lint:css
npm run typecheck
npm run build
npm audit
cd ..

# Shell
shfmt --diff scripts/ run_macos.sh

# Markdown
markdownlint-cli2 "**/*.md"
```

### With Docker (no local installs required)

```bash
docker run --rm -v "$(pwd):/src" -w /src mvdan/shfmt --diff scripts/ run_macos.sh

docker run --rm -v "$(pwd):/workdir" davidanson/markdownlint-cli2 "**/*.md"
```

## Commit style

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Commit messages drive automatic changelog generation and version bumping.

| Prefix      | When to use                         |
| ----------- | ----------------------------------- |
| `feat:`     | New feature or behavior             |
| `fix:`      | Bug fix                             |
| `chore:`    | Maintenance, dependency updates     |
| `refactor:` | Code change without behavior change |
| `docs:`     | Documentation only                  |
| `style:`    | Formatting, no logic change         |
| `ci:`       | CI/CD changes                       |

Breaking changes must include `BREAKING CHANGE:` in the commit footer.

Keep commits focused on a single concern. If a change touches both logic and tests, a single commit is fine — if it touches unrelated areas, split it.

## Pull requests

- Keep PRs focused on a single concern.
- Reference any related issue in the PR description.
- All CI checks must pass before merging.

## Reporting bugs

Open an [issue](https://github.com/wielorzeczownik/pulse-layer/issues) and include:

- What you did
- What you expected
- What actually happened
- Your environment (OS, Bluetooth adapter model, smart ring model)

> For security issues, read [SECURITY.md](SECURITY.md) before opening a public issue.

## License

By contributing you agree that your changes will be licensed under the [MIT License](LICENSE).
