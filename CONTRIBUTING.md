# Contributing

Every contribution helps, including a typo fix, a bug report, support for another ring, or a question about how something works. For anything bigger than a small fix, opening an issue first is a good idea, so we can agree on the approach before you spend time on it.

## Setup

You need Rust and Node.js. On Linux, also run `scripts/install-linux-deps.sh` for the system libraries.

```bash
git clone https://github.com/wielorzeczownik/pulse-layer.git
cd pulse-layer
cargo run
```

`build.rs` builds the `overlay/` frontend as part of `cargo build` and `cargo run`, so there's no separate step for it.

On macOS, use `./run_macos.sh` instead. It builds an `.app` bundle, because that's what macOS needs to ask for Bluetooth permission.

## Checks

CI runs these on every pull request. You don't have to run them all locally, since CI will point out anything that's off:

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked

cd overlay
npm ci
npm run lint
npm run lint:css
npm run typecheck
npm test
npm run build
cd ..

npx prettier --check .
shfmt --diff scripts/ run_macos.sh
shellcheck scripts/*.sh run_macos.sh
actionlint
markdownlint-cli2 "**/*.md"
```

`npm run fix` in `overlay/` fixes whatever eslint, stylelint and prettier can fix on their own.

## Tests

Rust tests sit next to the code they cover, in a `#[cfg(test)] mod tests` in the same file. Overlay tests are vitest suites in `overlay/tests/`, mirroring `overlay/src/`.

## Commits and pull requests

Commits follow [Conventional Commits](https://www.conventionalcommits.org/) (`feat: ...`, `fix: ...`), because the changelog and version bumps are generated from them. Don't worry if you get the prefix wrong, it can be fixed before merging.

Smaller PRs that do one thing are easier to review. Tests for behavior changes are very welcome. If you're not sure how to test something, open the PR anyway and we'll work it out together.

Bugs and ideas go to [issues](https://github.com/wielorzeczownik/pulse-layer/issues). Security problems go through [SECURITY.md](SECURITY.md).

Contributions are licensed under the [MIT License](LICENSE).
