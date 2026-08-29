# Contributing to osk-rs

Thank you for your interest in contributing to `osk-rs`!

## Development Setup

1. Clone the repository
2. Install system dependencies: `libgtk-4-dev`, `libglib2.0-dev`, `libgl-dev`, `libdbus-1-dev`
3. Run `cargo build --workspace` to verify the build
4. Run `cargo clippy --all-targets -- -D warnings` to verify linting
5. Run `cargo test --workspace` to verify tests

## Code Style

- Follow `rustfmt` formatting: run `cargo fmt --all` before committing
- One import per line (`imports_granularity = "Item"`)
- `snake_case` for functions and variables, `PascalCase` for types
- All public items must have `///` doc comments
- Avoid `unwrap()` and `expect()` in production code
- Use `Result<T, E>` for error handling. E is not a String

## Pull Requests

1. Create a branch from `main`
2. Make your changes with clear commit messages
3. Ensure `cargo fmt`, `cargo clippy`, and `cargo test` all pass
4. Update `CHANGELOG.md` under `## Unreleased`
5. Open a pull request with a description of the changes
