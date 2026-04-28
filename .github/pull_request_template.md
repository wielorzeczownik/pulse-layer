## What does this PR do?

<!-- Brief description of the change -->

## Type of change

- [ ] Bug fix
- [ ] New feature
- [ ] Refactor
- [ ] Documentation
- [ ] CI/CD

## Checklist

<!-- Check only what applies to this PR -->

- [ ] `cargo fmt --check` passes — if any `.rs` changed
- [ ] `cargo clippy --all-targets -- -D warnings` passes — if any `.rs` changed
- [ ] `cargo check --all-targets --locked` passes — if any `.rs` changed
- [ ] `cargo audit` passes — if `Cargo.lock` changed
- [ ] Shell scripts formatted (`shfmt --diff scripts/ run_macos.sh`) — if any `.sh` changed
- [ ] Markdown lints cleanly — if any `.md` changed
- [ ] Relevant documentation updated (README, comments)

**If `overlay/` was changed:**

- [ ] `npm run format:check` passes
- [ ] `npm run typecheck` passes
- [ ] `npm run lint` passes
- [ ] `npm run lint:css` passes
- [ ] `npm run build` passes

## Related issue

<!-- Closes #123 -->
