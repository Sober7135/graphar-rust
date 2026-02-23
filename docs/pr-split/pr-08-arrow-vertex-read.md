# PR-08: Arrow batch read (vertex property chunks)

- Size: Large
- Priority: High
- Depends on: PR-04, PR-05, PR-07

## Goal

Implement the "happy path" for vertex batch reads: read chunk files by property group and column projection, producing
Arrow `RecordBatch` values (for example an iterator of `Result<RecordBatch>`). Fold C++ reader capabilities into
`VertexScanBuilder`.

## Scope

- `Cargo.toml`: add `arrow` optional dependencies (and Parquet/CSV/JSON/ORC as needed) behind a feature gate
- `src/read/vertices_arrow.rs` (or `src/read/vertices/arrow.rs`)
- `src/read/vertices.rs`: connect `into_record_batches()` behind `cfg(feature = "arrow")`
- `src/store/*`: enhance `open_reader` / range reads if needed
- `tests/read_vertices_parquet.rs` (or CSV-based tests)

## Implementation Steps

- Define `VertexRecordBatchReader`: `Iterator<Item = Result<RecordBatch>>` (sync is fine for MVP).
- Path resolution: derive chunk paths from vertex info prefix + property group prefix + chunk index.
- Projection: `select([...])` reads only requested columns; append internal id column if required.
- `batch_size`: MVP can emit one batch per chunk; re-chunk later if necessary.
- Error semantics: missing files/chunks return clear errors and do not poison subsequent reads.

## Tests

- Generate a small Arrow table -> write Parquet/CSV to `tempdir` -> place files under GraphAr path rules -> read back and
  assert schema/row count.
- Projection test: selecting a subset yields the expected schema.
- Error test: unknown property group / missing chunk index yields a readable error.

## Acceptance Criteria

- Minimal read path works end-to-end:
  `Graph::open + vertices().scan().select(...).into_record_batches()` returns data.
- With `arrow` feature disabled, Arrow-specific code is not compiled in.
