# Repository Guidelines

## Project Structure & Module Organization
This repository is a Rust library crate (`graphar-rust`) targeting edition 2024. Core code lives in `src/` (currently `src/lib.rs`). Keep new modules under `src/` and expose only stable APIs from `lib.rs`.

Project-level configuration lives at the root:
- `Cargo.toml` / `Cargo.lock`: package metadata and dependency lockfile.
- `taplo.toml`: TOML formatting rules.
- `deny.toml`: dependency, license, and source policy.
- `.github/workflows/ci.yml`: CI checks and build matrix.
- `rust-toolchain.tool`: pinned Rust toolchain.

Use `tests/` for integration tests when behavior spans multiple modules.

## Build, Test, and Development Commands
Use these commands locally before opening a PR:
- `cargo build --all-targets --all-features` - compile all targets.
- `cargo test --all-targets --all-features` - run unit and integration tests.
- `cargo fmt --all -- --check` - verify Rust formatting.
- `cargo clippy --all-targets --all-features -- -D warnings` - lint with warnings denied.
- `taplo fmt --check --diff` - verify TOML formatting.
- `cargo doc --all-features --no-deps` - build docs.
- `cargo deny check` - run license/advisory/source policy checks.

## Coding Style & Naming Conventions
Follow idiomatic Rust:
- `snake_case` for modules/functions/variables.
- `PascalCase` for structs/enums/traits.
- `UPPER_SNAKE_CASE` for constants.
- Keep functions focused and avoid hidden side effects.

Rely on `rustfmt` and `clippy` as the source of truth; do not merge code that fails either check.

## Testing Guidelines
Place unit tests next to implementation with `#[cfg(test)]`. Name tests by behavior, e.g. `parses_empty_schema` or `returns_error_on_invalid_edge`.

For non-trivial logic, add both success and failure-path cases. Run `cargo test --all-targets --all-features` before each push.

## Commit & Pull Request Guidelines
Current history uses short, imperative subjects (for example: `init`, `first commit`). Keep commit titles concise and action-oriented; one logical change per commit.

PRs should include:
- what changed and why,
- key design decisions,
- verification commands you ran,
- linked issue(s) when applicable.

Ensure CI is green (`typos`, `lint`, `deny`, `build`, `test`, `docs`) before requesting review.
