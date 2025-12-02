# Release v0.86.70 — Rust Test Harness (`framec test`) and Persistence Naming

Summary
- Promotes the Rust-native V3 test harness to a first-class `framec test`
  subcommand with validation, compare-with-Python, and basic exec modes for
  PRT V3 categories, and finishes documenting the `@persist` helper naming
  convention for Python, TypeScript, and Rust, while keeping all PRT V3
  transpile-only suites green and publishing a new `framec` release binary.

Highlights
- Rust-native V3 test harness:
  - Added a `test` subcommand to the `framec` CLI
    (`framec/src/frame_c/cli.rs`) that drives the shared Rust harness
    (`framec::frame_c::v3::test_harness_rs`) instead of the Python runner:
    - Validation mode:
      - `framec test --language <python_3|typescript|rust> --category <v3_*>`
      - Uses `run_validation_for_category` to interpret `@expect: Exxx`
        metadata and treat other fixtures as positive.
    - Compare-with-Python mode:
      - `framec test ... --compare-python`
      - For a single `<language>/<category>` slice, runs both the Rust
        harness and `framec_tests/runner/frame_test_runner.py` in
        `--transpile-only --no-run` mode and reports whether both
        succeeded.
    - Exec-smoke mode:
      - `framec test ... --category v3_exec_smoke --exec-smoke`
      - Wraps `run_rust_exec_smoke`, `run_python_exec_smoke`, and
        `run_typescript_exec_smoke` for Rust/Python/TypeScript, mirroring
        the `v3_exec_smoke` marker semantics from the Python runner.
    - Exec-curated mode:
      - `framec test ... --category v3_core --exec-curated`
      - Wraps `run_rust_curated_exec_for_category`,
        `run_python_curated_exec_for_category`, and
        `run_typescript_curated_exec_for_category`, interpreting
        `@run-expect` / `@run-exact` metadata and SKIPping fixtures
        without run expectations.
  - Normalizes language names (`python_3` → `python`, `ts` → `typescript`,
    `rs` → `rust`) so the harness can re-use the existing
    `framec_tests/language_specific/<language>/<category>` layout.
  - Guards flags so `--compare-python` is only valid in validation mode and
    `--exec-smoke` / `--exec-curated` are mutually exclusive.

- Test coverage and alignment:
  - Validation + compare-Python:
    - Confirmed that for PRT V3 categories
      `v3_core`, `v3_control_flow`, `v3_systems`, `v3_persistence`, and
      `v3_systems_runtime` (python/typescript/rust) the Rust harness and
      Python runner agree on success/failure.
    - Added missing `@expect` metadata to Rust v3_systems negatives:
      - `frame_in_actions.frm` → `// @expect: E113`
      - `interface_missing_brace.frm` → `// @expect: E111`
      so that Rust validation behavior matches the documented policies and
      the Python runner’s expectations.
  - Exec-smoke:
    - Fixed `run_python_exec_smoke` so `FRAME_EMIT_EXEC=1` is set for
      `v3_exec_smoke` in addition to the core/control_flow/systems
      categories, restoring the expected TRANSITION/FORWARD/STACK markers
      for the Python exec-smoke fixtures.
    - Verified `framec test --... --exec-smoke` passes for
      `v3_exec_smoke` across Python, TypeScript, and Rust.
  - Exec-curated:
    - Verified `framec test --... --exec-curated` on `v3_core` passes
      (with SKIPs where no `@run-*` metadata is present) for
      python/typescript/rust.

- Persistence naming convention (`@persist` helpers):
  - Updated `docs/framepiler_design/architecture_v3/14_persistence_and_snapshots.md`
    to document the standard helper names that V3 emits for `@persist`
    systems:
    - Python:
      - `TrafficLight.save_to_json(system)` /
        `TrafficLight.restore_from_json(text)`
    - TypeScript:
      - `TrafficLight.saveToJson(system)` /
        `TrafficLight.restoreFromJson(text)`
    - Rust:
      - `impl SnapshotableSystem for TrafficLight` plus inherent
        `TrafficLight::save_to_json(&self)` /
        `TrafficLight::restore_from_json(&str) -> Self`
  - Clarified that these helpers are opt-in (only generated when `@persist`
    is present on the system header) and delegate to the underlying
    `frame_persistence_py`, `frame_persistence_ts`, and
    `frame_persistence_rs` libraries for actual encoding/decoding.

- Documentation updates:
  - `docs/framepiler_design/architecture_v3/PLAN.md`:
    - Marks Stage 19 items for `framec test` (library extraction, minimal
      subcommand, compare-Python, exec modes, and validation coverage
      expansion for PRT V3) as completed.
  - `docs/framepiler_design/architecture_v3/18_rust_native_tooling.md`:
    - Reframes `framec test` as the primary Rust-native harness for PRT V3,
      with `v3_rs_test_runner` documented as a thin developer wrapper.
  - `docs/HOW_TO.md`:
    - Adds a new “Rust-Based V3 Test Harness (`framec test`)" section with
      concrete examples for validation, compare-Python, exec-smoke, and
      exec-curated usage.

- Tests and validation:
  - Rebuilt `framec`:
    - `cargo build --release -p framec`
  - Re-ran the full PRT V3 transpile-only suite using the debug binary:
    - `python3 framec_tests/runner/frame_test_runner.py --languages python typescript rust --categories all_v3 --framec ./target/debug/framec --transpile-only`
    - All Python, TypeScript, and Rust `all_v3` categories remain 100% green.

- Release and bootstrap:
  - Bumped workspace version to `0.86.70` (root `Cargo.toml` and
    `version.toml` via `scripts/sync-versions.sh`), incrementing the patch
    version and build number.
  - Built the release binary:
    - `cargo build --release -p framec`
  - Ran `python3 tools/publish_framec_release.py`:
    - Successfully updated the bootstrap compiler at `boot/framec/framec`
      to `framec 0.86.70`.
    - Published the shared env reference to
      `framepiler_test_env/bug/releases/frame_transpiler/v0.86.70/framec/framec`
      after gaining write access.
- Persistence follow-ups:
  - Rust: `@persist` system parameters are now threaded into generated
    constructors and seeded into `state_args`; annotated systems (e.g.,
    `@persist system ...`) are detected even with header annotations, and
    constructors enforce the FRM-declared arity.
  - Python: `save_to_json` class helper now defaults to the class itself
    when no instance is provided (`system=None`), matching the TS/Rust
    convenience helpers.
