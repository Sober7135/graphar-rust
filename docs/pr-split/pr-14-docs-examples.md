# PR-14: Docs and examples (migration guide + feature docs)

- Size: Medium
- Priority: Medium
- Depends on: PR-06, PR-07

## Goal

Provide runnable examples showing how the Rust API covers the C++ Getting Started scenarios, and give reviewers a clear
entry point. Document features and CI usage to reduce onboarding cost.

## Scope

- `README.md`: minimal snippets (open + meta + scan)
- `examples/`:
  - `read_meta.rs`
  - `read_vertices.rs`
  - `read_edges.rs`
  - `neighbors.rs` (once PR-10 lands)
- Optional: `docs/` or a book, depending on repo preference
- `Cargo.toml`: feature documentation comments

## Implementation Steps

- Write a "C++ -> Rust" mapping: `GraphInfo::Load/GetVertexInfo/GetEdgeInfo` vs `Graph::open/vertices/edges`.
- Document "scan + projection": align with C++ `Select()` semantics and the "read only required columns/groups" idea.
- Document features: for example, enabling `arrow` to use `RecordBatch` readers.

## Tests

- Examples compile in CI (running them can be optional).
- Optionally make README snippets doctestable.

## Acceptance Criteria

- New contributors can understand the crate boundaries and the minimal usage path quickly ("examples compile" as the
  lowest bar).
