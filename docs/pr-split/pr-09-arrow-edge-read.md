# PR-09: Arrow batch read (edge adjList/offset/property chunks)

- Size: Large
- Priority: High
- Depends on: PR-04, PR-05, PR-07, PR-08

## Goal

Extend edge reads to cover three kinds of data files:
- adjList chunks (two columns: `src`/`dst`)
- property chunks (optional)
- offset chunks (required for neighbors on ordered adjLists)

Provide `into_record_batches()` and `into_edges()` aligned with the Rust builder style.

## Scope

- `src/read/edges_arrow.rs`
- `src/read/edges.rs`: connect `into_record_batches()` and `into_edges()`
- `src/error.rs`: add errors such as `MissingOffsets`
- `tests/read_edges_adjlist.rs`

## Implementation Steps

- Path rules (example semantics):
  - adjList: `edge/<triplet>/<adjlist_type>/adj_list/partX/chunkY`
  - offset: `edge/<triplet>/<adjlist_type>/offset/chunkY` (ordered only)
  - property: `edge/<triplet>/<adjlist_type>/<property_group>/partX/chunkY`
- `EdgeScanBuilder`:
  - `batch_size`: same as PR-08
  - `vertex_chunks(range)`: enumerate edge chunks by vertex-chunk granularity
  - `select_edge_properties([...])`: edge property projection
- `into_edges()`: provide `Iterator<Item = Result<EdgeRef>>` with `src/dst` and optional property view.

## Tests

- Write a minimal two-column adjList as Parquet/CSV -> read back `(src, dst)` pairs.
- Ordered layout + offset: construct a simple offset file as a baseline for PR-10 neighbors.
- Error paths: requesting a missing `AdjListType` returns an error (GraphAr allows multiple adjLists per edge, but missing
  types should fail).

## Acceptance Criteria

- `graph.edges(...).scan_adj_list(AdjListType::OrderedBySource).into_edges()` returns data.
