# PR-13: Object storage backend (S3/OSS) without global init

- Size: Medium
- Priority: Low
- Depends on: PR-05, PR-06

## Goal

Support URIs like `s3://...` while avoiding C++-style global `Initialize/Finalize`. All credentials and clients live
inside `Store` instances (shareable via `Arc`) and do not leak into global state.

## Scope

- `src/store/opendal.rs` (or `src/store/object_store.rs` / `src/store/s3.rs`)
- `Cargo.toml`: add `opendal` and/or `object_store` features + optional dependencies
- `tests/store_object_store_mock.rs`: mock-based tests (prefer trait-level mocks if infrastructure is unavailable)

## Implementation Steps

- URI parsing: handle `s3://bucket/path` (and optionally `oss://`, etc.).
- Implement a `Store` backend using one of:
  - OpenDAL: wrap an `opendal::Operator` (use the blocking API for sync I/O) and keep all config in the `Store`
    instance.
  - Arrow `object_store`: implement `Store` as a thin adapter around `object_store::ObjectStore`.
- Ensure no secrets are logged; document credential and endpoint configuration.
- Make sure required methods (especially range reads) are supported for Parquet/Arrow.

## Tests

- Validate `read/stat` behavior via mock or local compatible service.
- Concurrency: parallel reads of multiple keys.

## Acceptance Criteria

- Users can read GraphAr data from object storage with no global init/finalize.
- Provider-specific types do not leak into public APIs.
