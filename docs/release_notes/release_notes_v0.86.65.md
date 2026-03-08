# Release v0.86.65 — IndentNormalizer Machine Integration & Snapshot Helpers

Summary
- Integrates the self-hosted Python IndentNormalizer machine into the V3 pipeline for PRT, and adds snapshot comparison helpers to Python and TypeScript persistence libraries. All V3 PRT suites remain fully green.

Highlights
- Python:
  - The V3 Python emitter now delegates handler indentation to the generated `IndentNormalizer` machine:
    - `framec/src/frame_c/v3/machines/indent_normalizer.frs` defines the machine.
    - `indent_normalizer.gen.rs` is regenerated via `tools/gen_v3_machines_rs.py` using the bootstrap compiler.
    - `framec/src/frame_c/v3/machines/mod.rs` exposes `run_indent_normalizer(...)`.
    - `normalize_py_handler_lines` in `framec/src/frame_c/v3/mod.rs` now calls `run_indent_normalizer` instead of an inline Rust helper.
  - The IndentNormalizer machine no longer prints normalized lines; it only populates `out_lines`, making it safe for production use.
  - Curated exec for Python V3 (`v3_core`, `v3_control_flow`, `v3_systems`, `v3_systems_runtime`) remains 100% green, including `empty_elif_comment_only_exec` and `traffic_light_system_exec`.

- TypeScript:
  - `frame_persistence_ts` gains a snapshot comparison helper:
    - `compareSnapshots(a: SystemSnapshot, b: SystemSnapshot): { equal: boolean; differences: string[] }`.
    - Compares `schemaVersion`, `systemName`, `state`, `stateArgs`, `domainState`, and `stack` using JSON structural comparison.
  - The module is recompiled (`index.ts` → `index.js`/`index.d.ts`) and all V3 TS suites remain green.

- Python persistence:
  - `frame_persistence_py` adds `compare_snapshots(a: SystemSnapshot, b: SystemSnapshot) -> Tuple[bool, List[str]]`:
    - Field-wise comparison over `schemaVersion`, `systemName`, `state`, `stateArgs`, `domainState`, and `stack`.
  - This provides a standard, test-friendly way to assert snapshot equality in Python (and will be used for future cross-language fixtures).

- Plan updates:
  - `PLAN.md` marks Stage 14 (IndentNormalizer design/implementation/tests) as complete and explicitly defers full integration work to the new Stage 16.
  - Stage 16/17 entries are added to track:
    - Further self-hosted machines/harness builders for PRT.
    - Cross-language snapshot semantics and fixtures.

- Release:
  - Bumped workspace version to `0.86.65` (version.toml, Cargo.toml).
  - Built `framec` in release mode and updated:
    - `boot/framec/framec` (bootstrap compiler).
    - Shared env reference at `bug/releases/frame_transpiler/v0.86.65/framec/framec`.
  - `all_v3` transpile-only for Python, TypeScript, and Rust is 100% green with `framec 0.86.65`.

