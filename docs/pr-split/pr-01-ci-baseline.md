# PR-01: Rust crate scaffold and CI baseline

- Size: Small
- Priority: High
- Depends on: None

## Goal

Establish the engineering baseline: crate layout, a minimal buildable `lib.rs`, CI gates, and placeholder features (for
example `arrow`, `opendal`/`object_store`, `arrow_compute`).

## Scope

- `Cargo.toml`: package metadata, features, optional dependency placeholders
- `src/lib.rs`: module declarations (`error/meta/store/read/write`)
- `README.md`: minimal usage snippet
- `.github/workflows/ci.yml`: fmt/clippy/test/doc + feature matrix
- Optional: `deny.toml`, `rustfmt.toml`, `clippy.toml`

## Implementation Steps

- Create the crate skeleton and a minimal `src/lib.rs`; ensure `cargo test` passes.
- Define a feature plan: default `std` (if `no_std` matters, decide explicitly), plus feature flags for Arrow and storage
  backends.
- CI should at least run: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`, `cargo doc`.
- Add a feature matrix (for example `--no-default-features`, `--features arrow`, `--features opendal`) to prevent
  "default passes, feature breaks".

## Tests

- `cargo test` passes with default features.
- If planned: `cargo test --no-default-features` passes (or is explicitly out-of-scope for MVP).
- `cargo fmt --check` and `cargo clippy` are green.

## Acceptance Criteria

- Future PRs can rely on CI gates and avoid accumulating breakage.
- Feature structure is clear and extensible without hidden coupling.
