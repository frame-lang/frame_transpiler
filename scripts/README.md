# Frame Transpiler Scripts

This directory contains utility scripts for maintaining the Frame transpiler.

## Version Management

### Single Source of Truth: Cargo Workspace Version
The canonical version now lives in the root `Cargo.toml` under `[workspace.package]`.  
Updating that value automatically keeps every crate aligned because individual manifests declare `version.workspace = true`, and the compiler reads the same value at build time.

```toml
[workspace.package]
version = "0.86.17"
```

### Keeping `version.toml` in Sync
`version.toml` remains as a convenience artifact for tooling and release notes, but it mirrors the workspace version. Use the sync script whenever the workspace version changes:

```bash
# Preview the changes
./scripts/sync-versions.sh --dry-run

# Apply updates to version.toml
./scripts/sync-versions.sh
```

The script extracts the workspace version via `cargo metadata` and rewrites the `major`, `minor`, `patch`, and `full` fields in `version.toml`. No Cargo manifests or Rust source files require manual edits.

### Update Workflow

1. Edit `Cargo.toml`:
   ```toml
   [workspace.package]
   version = "0.86.17"
   ```
2. Sync auxiliary metadata:
   ```bash
   ./scripts/sync-versions.sh
   ```
3. Build (updates `Cargo.lock` if needed):
   ```bash
   cargo build --release
   ```
4. Verify:
   ```bash
   ./target/release/framec --version
   # Output: framec 0.86.17
   ```

### Files Automatically Updated

| File | Content | How it stays current |
|------|---------|----------------------|
| `Cargo.toml` (root) | Workspace version | Manual edit |
| `framec/Cargo.toml`, `frame_build/Cargo.toml`, `frame_runtime/Cargo.toml` | Inherit workspace version | `version.workspace = true` |
| `framec/src` (CLI, visitors, source maps) | Build-time constants | `env!("FRAME_VERSION")` set from Cargo |
| `version.toml` | Release metadata | `./scripts/sync-versions.sh` |
| `Cargo.lock` | Dependency snapshots | `cargo build` |

### CLI Version Output

`framec/build.rs` injects `FRAME_VERSION` using `CARGO_PKG_VERSION`, so the CLI and generated code comments automatically reflect the workspace version:

```bash
framec --version
# framec 0.86.16
```

### Troubleshooting

- **Version mismatch warning during build?** Run `./scripts/sync-versions.sh` to refresh `version.toml`.
- **Need to double‑check the active version?**
  ```bash
  cargo metadata --no-deps --format-version 1 | python3 -c 'import json,sys; print(next(pkg["version"] for pkg in json.load(sys.stdin)["packages"] if pkg["name"]=="framec"))'
  ```
- **Pre-commit hooks?** If you maintain a custom hook, you can simply invoke the sync script instead of writing bespoke sed logic.
