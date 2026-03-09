# Contributing to PulseLayer

Thank you for considering a contribution. This document describes how to get started.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain, 1.88+)
- [Node.js](https://nodejs.org/) 24+ (for the overlay frontend)
- Bluetooth adapter (for testing BLE functionality)

## Development setup

```bash
git clone https://github.com/wielorzeczownik/pulse-layer.git
cd pulse-layer
cargo run
```

The overlay frontend is built automatically by `build.rs` during `cargo build`/`cargo run`.

## Before submitting a PR

Make sure these pass locally:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --all-targets --locked
```

## Commit style

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Commit messages drive automatic changelog generation and version bumping.

Common prefixes:

| Prefix | When to use |
|--------|-------------|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `chore:` | Maintenance, dependency updates |
| `refactor:` | Code change without behavior change |
| `docs:` | Documentation only |
| `style:` | Formatting, no logic change |
| `ci:` | CI/CD changes |

Breaking changes must include `BREAKING CHANGE:` in the commit footer.

## Pull requests

- Keep PRs focused on a single concern.
- Reference any related issue in the PR description.
- The CI `cargo-check` workflow must pass.

## Reporting bugs

Open an [issue](https://github.com/wielorzeczownik/pulse-layer/issues) and include:
- What you did
- What you expected
- What actually happened
- Relevant logs or error messages
- Your platform and Bluetooth adapter model
- Smart ring model (if relevant)

> For security issues, please read [SECURITY.md](SECURITY.md) before opening a public issue.

## License

By contributing you agree that your changes will be licensed under the [MIT License](LICENSE).
