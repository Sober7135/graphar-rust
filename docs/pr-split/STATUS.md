# Development Status (Rust-native)

Last updated: 2026-02-23

This tracker records the execution status of the PR split in `docs/pr-split/`.

## Current State

- Foundation work (CI gates, formatting, spelling, dependency policy, license) is in place for iterative development.
- The Rust-native implementation work has not started yet (no core API/types/modules landed).

## Next PR To Execute

**PR-02: Core types and error model**

Rationale:
- It is small and stabilizes shared vocabulary (`FileType`, `AdjListType`, ids) and error semantics for the whole stack.
- It unblocks PR-03/04 (metadata) and PR-05 (Store).

Follow-ups after PR-02:
- Run PR-03 (metadata model/validation) and PR-05 (Store + local FS) in parallel.

## Open Decisions (Non-Blocking)

- Crate layout: single crate vs workspace (can be decided in PR-01/02).
- Storage backend for object stores (PR-13): OpenDAL (`opendal`) vs Arrow `object_store` vs both behind features.
- Arrow scope (PR-08/09): Parquet-only MVP vs multi-format (`csv/json/orc`) support.

## PR Tracker

Legend: `DONE`, `IN_PROGRESS`, `TODO`, `DEFERRED`

| PR | Title | Priority | Depends on | Status | Notes |
|---:|---|:---:|---|---|---|
| 01 | Rust crate scaffold and CI baseline | High | - | DONE | CI/typos/taplo/fmt/clippy/deny/docs gates are present. |
| 02 | Core types and error model | High | 01 | TODO | Recommended next. |
| 03 | Metadata model and validation | High | 02 | TODO | - |
| 04 | YAML metadata I/O | High | 03 | TODO | - |
| 05 | Store abstraction and local filesystem backend | High | 01, 02 | TODO | Consider `read_range` early for Parquet/Arrow. |
| 06 | Graph entry and basic API | High | 03, 04, 05 | TODO | - |
| 07 | Reader API surface | High | 02, 03, 06 | TODO | Must avoid \"seek then undefined\" states by design. |
| 08 | Arrow batch read (vertex) | High | 04, 05, 07 | TODO | Feature-gated; needs representative CI coverage. |
| 09 | Arrow batch read (edge) | High | 04, 05, 07, 08 | TODO | Includes adjList/property/offset reads. |
| 10 | Neighbors API | Medium | 09 | TODO | Only for ordered + offsets; otherwise `MissingOffsets`. |
| 11 | Writer (vertex) | Medium | 02, 03, 04, 05, 06 | TODO | Typed row builder; avoid `Any` in public API. |
| 12 | Writer (edge) | Low | 03, 04, 05, 06, 11 | TODO | Offset generation for ordered layouts. |
| 13 | Object storage backend | Low | 05, 06 | TODO | Prefer feature-gated `opendal` and/or `object_store`. |
| 14 | Docs and examples | Medium | 06, 07 | TODO | Keep examples compiling in CI. |

