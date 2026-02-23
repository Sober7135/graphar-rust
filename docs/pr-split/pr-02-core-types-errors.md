# PR-02: Core types and error model

- Size: Small
- Priority: High
- Depends on: PR-01

## Goal

Land reusable core types and a unified error model to avoid cycles and renames later:
- Core types: `VertexId`, label/name newtypes, `FileType`, `AdjListType`, `Version`, ...
- Error model: expose `Result<T, Error>` from public APIs

## Scope

- `src/error.rs`: `Error` enum + `Result` alias (suggest `thiserror`)
- `src/meta/ids.rs`: `VertexId`, `VertexLabel`, `EdgeLabel`, `PropertyName`, ...
- `src/meta/types.rs`: `FileType`, `AdjListType`, `Version`
- `src/lib.rs`: re-exports

## Implementation Steps

- Define `VertexId(u64)` and newtypes for labels/names (`Box<str>` or `String`).
- Implement `FileType` and `AdjListType` enums + string parsing (for YAML/user input).
- Define `Error`: cover metadata errors, unknown label/triplet, I/O, and feature-gate errors (for example Arrow
  disabled). Consider `#[non_exhaustive]` for forward compatibility.
- Ensure public types/errors are `Send + Sync`.

## Tests

- Round-trip parsing for `FileType`/`AdjListType` strings.
- `Error` formatting and `source()` chaining (if using `thiserror`).

## Acceptance Criteria

- Downstream modules do not define duplicate id/type/error concepts.
- Terminology matches GraphAr (file types, adjList types).
