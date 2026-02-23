# PR-11: Writer (VertexWriter + RowBuilder, replaces std::any)

- Size: Large
- Priority: Medium
- Depends on: PR-02, PR-03, PR-04, PR-05, PR-06

## Goal

Implement vertex writes: write chunk files per property group and provide a schema-driven typed row builder (replacing
C++ `std::any`-based dynamic insertion).

## Scope

- `src/write/vertex_writer.rs`
- `src/write/row_builder.rs`
- `src/write/mod.rs`
- `tests/write_vertices_roundtrip.rs`
- `Cargo.toml`: enable writer dependencies under the `arrow` feature (MVP can support a single format first)

## Implementation Steps

- `VertexWriterBuilder::new(graph, label)`: locate vertex info + target property group; configure overwrite/batch params.
- `VertexRowBuilder`: create Arrow builders from schema; expose `set_i64/set_str/set_null/commit`.
- `write_batch(&RecordBatch)`: split by `chunk_size` and `chunk_index` and write to target paths.
- Metadata files: write at least `vertices_num` (if naming rules are not fixed yet, document TODOs explicitly in-scope).

## Tests

- Round-trip: write to `tempdir`, then read back via PR-08 and compare.
- Errors: missing columns / type mismatches return clear errors and do not write corrupted files.

## Acceptance Criteria

- Minimum viable: write a single property group `chunk0` and read it back consistently.
- No public API exposes `Any` or dynamic type injection.
