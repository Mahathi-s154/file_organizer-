# Contributing

This project is being shaped into a safe, well-tested file organization CLI. Contributions should improve reliability, clarity, and cross-platform behavior before adding broad feature surface.

## Local Setup

```bash
cargo test
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
```

Run the CLI locally:

```bash
cargo run -- --help
```

## Contribution Guidelines

- Keep pull requests focused. Small, reviewable changes are easier to maintain.
- Add or update tests for user-visible behavior changes.
- Update the README or design docs when CLI behavior, architecture, or workflows change.
- Prefer deterministic behavior over cleverness. File-moving tools need to be predictable.
- Preserve cross-platform compatibility. Avoid assumptions that only hold on Unix-like systems.

## Suggested Areas

- Planner and executor refactors that improve testability.
- Cross-platform filesystem edge cases.
- Machine-readable reporting and automation workflows.
- Documentation, examples, and contributor ergonomics.

## Pull Request Checklist

- Tests pass locally.
- `cargo fmt` and `cargo clippy` are clean.
- New behavior is documented.
- Edge cases and failure modes are called out in the PR description.
