# Release v0.86.66 — Cross-Language TrafficLight Snapshots & Rust Runtime Fixes

Summary
- Extends Stage 15/17 persistence work with a runtime-level TrafficLight snapshot comparison across Python, TypeScript, and Rust, and fixes a Rust V3 runtime constructor bug. All V3 PRT suites remain fully green.

Highlights
- Python:
  - Added `language_specific/python/v3_persistence/positive/traffic_light_snapshot_dump.frm`:
    - Defines a minimal `TrafficLight` system.
    - `main()` drives the system from Red to Green, snapshots via `frame_persistence_py.snapshot_system`, and prints `snapshot_to_json(...)`.
  - `tools/test_cross_language_snapshot_traffic_light.py` now:
    - Compiles the Python fixture via the V3 module path.
    - Executes its generated module and captures the JSON snapshot.

- TypeScript:
  - Added `language_specific/typescript/v3_persistence/positive/traffic_light_snapshot_dump.frm` with the same logical `TrafficLight` machine as the Python fixture.
  - The cross-language harness:
    - Compiles the TS fixture via `framec compile -l typescript`.
    - Uses `tsc` + Node and `frame_persistence_ts.snapshotSystem/snapshotToJson` to produce a matching JSON snapshot.

- Rust:
  - Added `language_specific/rust/v3_persistence/positive/traffic_light_snapshot_dump.frm`:
    - `@target rust` `TrafficLight` system mirroring Python/TS semantics.
    - Domain parameter `domain: &'static str = "red"` to match the canonical snapshot.
  - Fixed the V3 Rust runtime constructor in `framec/src/frame_c/v3/mod.rs`:
    - `fn new()` now seeds `compartment.state` via `StateId::default()` instead of hard-coding `StateId::A`, aligning with the Arcanum-derived start state.
  - Extended `tools/test_cross_language_snapshot_traffic_light.py` with a Rust harness:
    - Compiles the Rust fixture via `framec`.
    - Generates a small Rust program that includes the module, drives the system to Green, and builds a canonical `SystemSnapshot` JSON via `frame_persistence_rs::SystemSnapshot`.
    - Compiles the harness with `rustc` against the already-built `frame_persistence_rs` rlib and captures its JSON output.

- Cross-language snapshot semantics (Stage 17):
  - `tools/test_cross_language_snapshot_traffic_light.py` now compares snapshots from:
    - Python (`frame_persistence_py`), TypeScript (`frame_persistence_ts`), and Rust (`frame_persistence_rs`).
  - All three snapshots are structurally identical for the TrafficLight scenario:
    - `state == "__TrafficLight_state_Green"`, `stateArgs == ["green"]`, `domainState.domain == "red"`, `stack == []`.
  - `PLAN.md` Stage 17 is updated to mark schema-level and runtime-level Py/TS/Rust TrafficLight checks as complete.

- Stage 16 completion and future work:
  - `PLAN.md` Stage 16 marks the Rust bullet as complete: the Rust module path was audited for post-MIR transforms, and no additional PRT-era Rust machines were deemed necessary beyond the existing runtime generator.
  - Added Stage 18 in `PLAN.md` for future exploration of Rust-native runners and tooling (potential migration paths for selected Python test tools).

- Release:
  - Bumped workspace version to `0.86.66` (version.toml, Cargo.toml).
  - Built `framec` in release mode and updated:
    - `boot/framec/framec` (bootstrap compiler).
    - Shared env reference at `bug/releases/frame_transpiler/v0.86.66/framec/framec`.
  - Re-ran `all_v3` transpile-only for Python, TypeScript, and Rust with `framec 0.86.66` (all tests passing), and validated `v3_persistence` exec for Python/TypeScript.

