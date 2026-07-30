# Contributing to PulseLayer

Thank you for considering a contribution. This document covers everything you need to get started.

## Overview

A macOS application that reads data from BLE smart rings and displays it in an on-screen overlay.

## Project structure

```text
.
├── src/                        Rust source code, with tests beside the code they cover
├── overlay/
│   ├── src/                    overlay frontend (Vite + TypeScript)
│   └── tests/                  vitest suites, mirroring overlay/src
├── macos/                      macOS app bundle metadata
└── scripts/
    ├── bump-version.sh         determines the next release version and bumps Cargo.toml
    ├── install-linux-deps.sh   apt packages the Linux build needs
    ├── package-macos.sh        assembles and signs PulseLayer.app for a release
    └── security-audit.sh       runs both audits, attempts a fix, reports what is left
```

## Development setup

```bash
git clone https://github.com/wielorzeczownik/pulse-layer.git
cd pulse-layer
cargo run
```

The `overlay/` frontend is built automatically by `build.rs` during `cargo build`/`cargo run`.

## Running checks locally

CI runs exactly these commands. Anything that passes here passes there.

### With tools installed

```bash
# Rust
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo audit                          # reported, never blocking

# Overlay
cd overlay
npm ci
npm run lint                         # eslint, warnings are errors
npm run lint:css                     # stylelint
npm run typecheck                    # tsc --noEmit
npm test                             # vitest run
npm run build
npm audit                            # reported, never blocking
cd ..

# Formatting (whole repo, honours .prettierignore)
npx prettier@3.9.6 --check .

# Shell
shfmt --diff scripts/ run_macos.sh
shellcheck scripts/*.sh run_macos.sh

# Workflows
actionlint

# Markdown (exclusions come from .markdownlint-cli2.yaml)
markdownlint-cli2 "**/*.md"
```

`npm run fix` inside `overlay/` applies every autofixable finding from eslint,
stylelint and prettier in one go.

### With Docker (no local installs required)

```bash
docker run --rm -v "$(pwd):/src" -w /src mvdan/shfmt --diff scripts/ run_macos.sh

docker run --rm -v "$(pwd):/mnt" -w /mnt koalaman/shellcheck:stable scripts/*.sh run_macos.sh

docker run --rm -v "$(pwd):/repo" -w /repo rhysd/actionlint:1.7.12

docker run --rm -v "$(pwd):/workdir" davidanson/markdownlint-cli2 "**/*.md"
```

## Commit style

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Commit messages drive automatic changelog generation and version bumping.

| Prefix      | When to use                                |
| ----------- | ------------------------------------------ |
| `feat:`     | New feature or behavior                    |
| `fix:`      | Bug fix                                    |
| `perf:`     | Performance improvement                    |
| `refactor:` | Code change without behavior change        |
| `test:`     | Tests only                                 |
| `docs:`     | Documentation only                         |
| `style:`    | Formatting, no logic change                |
| `build:`    | Build tooling and development dependencies |
| `ci:`       | Workflows and CI configuration             |
| `chore:`    | Maintenance that fits nothing above        |

Breaking changes must include `BREAKING CHANGE:` in the commit footer.

Keep commits focused on a single concern. If a change touches both logic and tests, a single commit is fine – if it touches unrelated areas, split it.

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
