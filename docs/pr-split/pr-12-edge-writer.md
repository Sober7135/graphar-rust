# PR-12: Writer (EdgesBuilder/EdgeWriter)

- Size: Large
- Priority: Low
- Depends on: PR-03, PR-04, PR-05, PR-06, PR-11

## Goal

Implement edge writes: write adjList chunks, property chunks, and generate offset chunks for ordered layouts to support
PR-10 neighbors.

## Scope

- `src/write/edge_writer.rs` (or `src/write/edges_builder.rs`)
- `src/write/mod.rs`
- `tests/write_edges_roundtrip.rs`

## Implementation Steps

- Design `EdgesWriterBuilder`: choose adjListType, directedness, and property groups (from metadata).
- `add_edge(src, dst, props?)`: MVP can start with no properties or a single property group.
- Chunking: split by vertex chunk and `edge_chunk_size`; write into `.../part{}/chunk{}` paths.
- For ordered layouts: generate offset chunks matching PR-10 reading assumptions.

## Tests

- Round-trip: write edges -> read back via PR-09 and compare `(src, dst)` pairs.
- Ordered + offset: after writing, PR-10 neighbors reads back correctly.

## Acceptance Criteria

- The written layout matches GraphAr doc examples (at least at the path-structure level).
