# PR-10: Neighbors API (offset-driven)

- Size: Medium
- Priority: Medium
- Depends on: PR-09

## Goal

Provide a safe, composable neighbors traversal API. It is only available when "ordered + offsets exist"; otherwise it
returns `MissingOffsets`.

## Scope

- `src/read/neighbors.rs`
- `src/read/edges.rs`: implement `neighbors_by_source()` / `neighbors_by_dest()` builders
- `src/error.rs`: ensure `MissingOffsets` is fully implemented
- `tests/neighbors.rs`

## Implementation Steps

- Design `NeighborsBySourceBuilder`: fixes `AdjListType::OrderedBySource` and binds to a specific `EdgeSet`.
- Read the offset chunk and the corresponding adjList chunk:
  - Map `VertexId(v)` to its vertex chunk
  - Load the offset chunk to compute the edge range
  - Iterate `dst` values
- Error semantics: missing offsets returns `MissingOffsets` and never creates an undefined state.

## Tests

- Small graph: hand-build ordered_by_source adjList + offset; verify `neighbors(vid)`.
- Edge cases: last vertex offset handling and empty-neighbors behavior.

## Acceptance Criteria

- The API prevents use on unordered layouts at the type/API level.
