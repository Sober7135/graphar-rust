# PR-06: Graph entry and basic API

- Size: Medium
- Priority: High
- Depends on: PR-03, PR-04, PR-05

## Goal

Implement the primary entry point `Graph`: holds `GraphMeta + Store + GraphConfig` and exposes `meta()`,
`vertices(label)`, `edges(src, edge, dst)`. Align with the C++ "Load + GetVertexInfo/GetEdgeInfo" baseline while
Rustifying nullable/pointer-style errors into explicit `Result`.

## Scope

- `src/read/graph.rs`
- `src/read/mod.rs`: exports for `Graph/VertexSet/EdgeSet`
- `src/lib.rs`: re-exports
- `tests/graph_open.rs`

## Implementation Steps

- Define `GraphConfig` (concurrency/prefetch/cache can start as placeholders).
- Implement `Graph::open(path)`: default to local FS store + PR-04 YAML loading; store meta in `Arc`.
- Implement `Graph::from_parts(meta, store, config)` for testing and advanced usage.
- Implement `vertices(label)` and `edges(src, edge, dst)`: return explicit errors on lookup failures.

## Tests

- `tempdir + minimal YAML`: `Graph::open` succeeds and key fields match.
- Unknown label/triplet yields an error (no panics).

## Acceptance Criteria

- Covers the "load + lookup" baseline from GraphAr C++ Getting Started.
