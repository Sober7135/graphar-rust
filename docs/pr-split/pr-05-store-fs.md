# PR-05: Store abstraction and local filesystem backend

- Size: Medium
- Priority: High
- Depends on: PR-01, PR-02

## Goal

Decouple I/O (read, list, path resolution) from higher-level APIs to enable Arrow readers and object storage backends.
Prefer dependency-injected `Store` instances over global init/lifetimes.

## Scope

- `src/store/mod.rs`: `Store` trait + core APIs
- `src/store/fs.rs`: local filesystem implementation
- `src/error.rs`: store error mapping
- `tests/store_fs.rs`

## Implementation Steps

- Define a minimal `Store` trait. Recommended baseline:
  - `read(path) -> Result<Bytes>` for small metadata/YAML
  - `open_reader(path) -> Result<impl Read>` for streaming
  - `stat/exists`
  - Optional: `list(prefix)` for chunk discovery
  - Strongly consider `read_range(path, offset, len)` (or an equivalent interface), since Parquet readers often need
    random access / range reads.
- Implement `LocalFsStore` (support plain paths; optionally `file://`).
- Provide a "base prefix + relative path" helper that matches metadata `prefix` semantics.
- Keep implementations `Send + Sync` for parallel scans.

## Tests

- `tempdir` write -> `Store::read` returns the same bytes.
- If `file://` is supported, it resolves identically to plain paths.
- Compile-time `Send + Sync` assertions.

## Acceptance Criteria

- Arrow/parquet readers only depend on `Store` (not `std::fs`).
- Store errors map cleanly into the unified `Error`.

## Notes (OpenDAL)

Do not hard-wire a specific remote backend here. Keep the trait minimal so PR-13 can add `opendal` and/or
`object_store` behind feature flags.
