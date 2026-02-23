# PR-04: YAML metadata I/O

- Size: Medium
- Priority: High
- Depends on: PR-03

## Goal

Load metadata from `.graph.yml` / `.vertex.yml` / `.edge.yml` and support exporting YAML (dump/save). This includes the
layered structure where graph info references vertex/edge info files, and `prefix` + relative path semantics.

## Scope

- `src/meta/yaml/mod.rs` (or `src/meta/io.rs`)
- `src/meta/yaml/graph.rs`, `src/meta/yaml/vertex.rs`, `src/meta/yaml/edge.rs`
- `src/error.rs`: add YAML parse error variants
- `tests/meta_yaml_roundtrip.rs`

## Implementation Steps

- Pick a YAML library (for example `serde_yaml`). Define YAML-facing DTO structs and convert into internal
  `GraphMeta/VertexMeta/EdgeMeta`.
- Implement `GraphMeta::load(path)`: load graph YAML and recursively load vertex/edge YAML; handle prefix + relative
  paths.
- Implement dump/save: ensure required fields are present (ordering/formatting is not required).
- Call `validate()` (PR-03) after loading and map failures into `Error::InvalidMetadata { ... }`.

## Tests

- Use minimal YAML strings in unit tests; do not depend on external test fixtures (write to `tempdir` if needed).
- Round-trip: `load -> dump -> load` preserves key fields (ordering differences allowed).
- Error paths: missing required fields yield readable errors that identify file/field.

## Acceptance Criteria

- Can load YAML matching the official GraphAr examples (field hierarchy and semantics).
- Errors point to the specific file and field.
