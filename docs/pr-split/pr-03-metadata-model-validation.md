# PR-03: Metadata model and validation

- Size: Medium
- Priority: High
- Depends on: PR-02

## Goal

Implement the core metadata structures and encode key GraphAr invariants via `validate()`:
- `GraphMeta` / `VertexMeta` / `EdgeMeta` / `PropertyGroupMeta`
- A minimal validation set (chunk size, label uniqueness, adjListType validity, ...)

## Scope

- `src/meta/mod.rs`: `GraphMeta` definition, accessors, lookup helpers
- `src/meta/vertex.rs`, `src/meta/edge.rs`, `src/meta/property_group.rs`: split files for readability
- `src/meta/validate.rs`: centralized validation logic
- `src/meta/builders.rs`: builder placeholder or minimal usable builder
- `src/lib.rs`: exports

## Implementation Steps

- Define `GraphMeta`: `name/prefix/version/vertices/edges/extra`, plus `vertex(label)` and `edge(src, edge, dst)` lookups.
- Define `VertexMeta`: `label/chunk_size/prefix/property_groups`, aligned with GraphAr vertex info semantics.
- Define `EdgeMeta`: `src/edge/dst/directed/edge_chunk_size/src_chunk_size/dst_chunk_size/adj_lists/property_groups/offset`.
- Suggested minimal validation rules:
  - `chunk_size > 0`
  - Unique vertex labels and unique edge triplets within a graph
  - `AdjListType` is limited to the spec-defined variants (multiple adjLists per edge are allowed)
- Use builder pattern where construction would otherwise become parameter-heavy.

## Tests

- Construct a minimal in-memory `GraphMeta`; `validate()` succeeds.
- Duplicate vertex label / edge triplet triggers `InvalidMetadata`.
- `chunk_size = 0` fails validation.

## Acceptance Criteria

- Metadata types are reusable by the YAML layer (PR-04).
- Validation failures point to actionable causes (so users can fix data rather than guess).
