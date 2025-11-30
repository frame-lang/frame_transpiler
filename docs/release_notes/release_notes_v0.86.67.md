# Release v0.86.67 — Rust V3 Test Runner Expansion & Machine Cleanup

Summary
- Expands the Stage 18 Rust-native V3 test runner to more PRT categories, aligns negative fixtures with `@expect` metadata, and relaxes lints for self-hosted machines, while keeping all V3 Python/TypeScript suites fully green.

Highlights
- Rust-based V3 test runner (`v3_rs_test_runner`):
  - Extended coverage beyond the initial core/control-flow sets:
    - Python: `v3_core`, `v3_control_flow`, `v3_systems`, `v3_persistence`, `v3_systems_runtime`.
    - TypeScript: `v3_core`, `v3_control_flow`, `v3_systems`, `v3_persistence` (including curated exec for `traffic_light_persistence`).
  - The runner treats fixtures without `@expect:` as positives (validation must succeed) and fixtures with `@expect:` as negatives (validation must fail and include all listed error codes), matching the Python runner’s semantics.
  - `docs/HOW_TO.md` and `PLAN.md` updated to document the expanded scope and Stage 18 status.

- Fixture alignment for Rust runner semantics:
  - Python `v3_systems/negative`:
    - Added `# @expect` codes to align with the ValidatorV3 taxonomy:
      - `frame_in_actions.frm` → E401 (Frame statements not allowed in actions/operations).
      - `interface_missing_brace.frm` → E111.
      - `duplicate_machine_block.frm` → E114.
      - `system_blocks_out_of_order.frm` → E113.
  - TypeScript `v3_control_flow/negative`:
    - `transition_not_terminal.frm` now declares `// @expect: E400` (transition must be last statement).
  - TypeScript `v3_systems/negative`:
    - Added `// @expect` codes mirroring the Python fixtures where applicable:
      - `frame_in_actions.frm` → E113 (block-order structural error on TS path).
      - `interface_missing_brace.frm` → E111.
      - `duplicate_machine_block.frm` → E114.
      - `system_blocks_out_of_order.frm` → E113.

- Self-hosted machines and lints:
  - IndentNormalizer and TypeScript harness builder machines:
    - Regenerated `indent_normalizer.gen.rs` and `ts_harness_builder.gen.rs` via `tools/gen_v3_machines_rs.py` using the bootstrap compiler.
    - Relaxed non-actionable lints for generated code at the module level:
      - `framec/src/frame_c/v3/machines/mod.rs` and `framec/src/frame_c/v3/ts_harness_machine.rs` now allow `unreachable_patterns` and `dead_code` where appropriate.
  - TypeScript domain scanner:
    - Removed unused locals and ensured `scan_ts_domain_fields` returns an empty `Vec` early when no `domain:` block is found (no behavioral change, just cleanup).

- Docs and plan updates:
  - `docs/HOW_TO.md`:
    - Updated the Rust-based V3 test runner section with the new category coverage list.
  - `docs/framepiler_design/architecture_v3/PLAN.md`:
    - Stage 18 now records the concrete `v3_rs_test_runner` coverage and clarifies that future work includes additional PRT categories and possible CI integration.

- Release:
  - Bumped workspace version to `0.86.67` (root `Cargo.toml`, `version.toml` via `scripts/sync-versions.sh`).
  - Rebuilt `framec` in release mode:
    - `cargo build --release -p framec`
    - Verified `framec --version` reports `0.86.67`.
  - Ran `tools/publish_framec_release.py` to:
    - Update the bootstrap compiler at `boot/framec/framec`.
    - Update the shared env reference at `framepiler_test_env/bug/releases/frame_transpiler/v0.86.67/framec/framec`.
  - Re-ran `all_v3` transpile-only for Python, TypeScript, and Rust using the debug binary; Python and TypeScript remained fully green, with the existing known Rust `v3_persistence/traffic_light_snapshot_dump` E402 negative unchanged by this release.
