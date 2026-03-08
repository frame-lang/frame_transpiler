# Release v0.86.64 — Stage 15 PRT Persistence Hooks and Rust Library

Summary
- Finalizes Stage 15 persistence helpers for the PRT languages (Python, TypeScript, Rust) and aligns the docs and PLAN with the implemented behavior. Adds explicit hooks for complex domain encode/decode and introduces a dedicated Rust persistence crate.

Highlights
- Python:
  - `frame_persistence_py` now exposes:
    - `snapshot_system(system, *, system_name=None, domain_keys=None, domain_encoder=None)`
    - `restore_system(snapshot, system_factory, *, domain_keys=None, domain_decoder=None)`
  - `domain_encoder` / `domain_decoder` allow applications to project complex domain graphs into a stable DTO and re-hydrate them on restore.
  - Existing traffic-light persistence fixture under `language_specific/python/v3_persistence` remains green.

- TypeScript:
  - Added `frame_persistence_ts` module with:
    - `snapshotSystem(system, opts?: SnapshotSystemOptions)`
    - `restoreSystem(snapshot, factory, opts?: RestoreSystemOptions)`
    - `snapshotToJson` / `snapshotFromJson`
  - `SnapshotSystemOptions` / `RestoreSystemOptions` support `encodeDomain` / `decodeDomain` hooks for complex domains.
  - A TS traffic-light persistence fixture in `language_specific/typescript/v3_persistence` passes under the V3 exec harness.

- Rust:
  - New crate `frame_persistence_rs` added to the workspace:
    - `SystemSnapshot` / `FrameCompartmentSnapshot` with `serde` / `serde_json`.
    - `SnapshotableSystem` trait for systems that want to integrate with the snapshot model.
  - Includes an internal `traffic_light_snapshot_round_trip` unit test to validate snapshot → JSON → restore → continue semantics.
  - Wiring generated V3 Rust systems to implement `SnapshotableSystem` is planned as part of the Rust runtime parity work, not this release.

- Docs & PLAN:
  - `architecture_v3/14_persistence_and_snapshots.md` updated to:
    - Reflect Python and TypeScript domain encode/decode hooks.
    - Describe `frame_persistence_rs`, `SystemSnapshot`, `FrameCompartmentSnapshot`, and `SnapshotableSystem` for Rust.
  - `PLAN.md` marks all PRT Stage 15 bullets as `[x]`, with Rust generator integration called out separately.

- Tooling & Release:
  - Version bumped to `0.86.64` (`version.toml`, workspace `Cargo.toml`).
  - `framec` built in release mode and published to:
    - `boot/framec/framec` (bootstrap compiler).
    - Shared test env reference at `bug/releases/frame_transpiler/v0.86.64/framec/framec`.
  - `all_v3` transpile-only tests remain 100% green for Python, TypeScript, and Rust.

