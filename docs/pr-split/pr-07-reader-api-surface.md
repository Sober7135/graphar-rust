# PR-07: Reader API surface (scan builders, FilterExpr, Row view)

- Size: Medium
- Priority: High
- Depends on: PR-02, PR-03, PR-06

## Goal

Land the read API "shape" first: stabilize types and the builder pattern so later Arrow implementations mainly fill in
internals.

Explicitly avoid C++-style "seek then undefined state": errors must not push readers into a permanently unusable state,
or the API should model seeking as producing a new cursor/iterator.

## Scope

- `src/read/vertices.rs`: `VertexSet` + `VertexScanBuilder`
- `src/read/edges.rs`: `EdgeSet` + `EdgeScanBuilder` (neighbors builders can start as placeholders)
- `src/read/filter.rs`: `FilterExpr` / `Scalar`
- `src/read/row.rs`: row view (`VertexRow` / `ValueRef`)
- `src/error.rs`: feature-gate errors (for example Arrow not enabled)

## Implementation Steps

- `VertexSet`: provide `meta()` and `scan()` returning a builder.
- `EdgeSet`: provide `scan_adj_list(type)` returning a builder; keep neighbors entry points gated to "ordered + offsets".
- `FilterExpr`: minimal set `And/Or/Not/Eq/IsNull` + `Scalar`.
- Row view: typed getters (for example `get_i64/get_str`) and `ValueRef`; the backing storage can be stubbed for now.
- Invariant: errors must be explicit `Result` and must not corrupt iterator state.

## Tests

- The crate compiles without `arrow` feature (read methods return feature-gate errors).
- Basic construction tests for `FilterExpr`/`Scalar`.
- `VertexRow` tests for missing column / wrong type errors (no real Arrow required).

## Acceptance Criteria

- The API surface can represent projection/filter/chunk access, so PR-08/09 do not need public API reshaping.
