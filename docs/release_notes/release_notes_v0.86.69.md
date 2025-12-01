# Release v0.86.69 — Persistence Docs and Rust Tooling Plan

Summary
- Documents the `@persist`-driven, type-centric persistence helpers for PRT languages in the Stage 15 design, and records the Stage 19 Rust-only tooling plan in the V3 architecture PLAN, while keeping all PRT V3 transpile-only suites green and publishing a new `framec` release binary.

Highlights
- Persistence design updates:
  - Extended `docs/framepiler_design/architecture_v3/14_persistence_and_snapshots.md` with a new “System-Level Helpers and `@persist`” section.
  - Clarified that, in addition to the per-language persistence libraries (`frame_persistence_py`, `frame_persistence_ts`, `frame_persistence_rs`), V3 systems can opt into type-centric helpers via an `@persist` annotation on the system header.
  - Recorded the intended helper shape per language as thin wrappers over the libraries:
    - Python: `save_to_json` / `restore_from_json` classmethods.
    - TypeScript: `saveToJson` / `restoreFromJson` static methods.
    - Rust: inherent `save_to_json` / `restore_from_json` methods on the generated system struct.
  - Noted that the library-level APIs are implemented today, and that wiring `@persist` into codegen for these helpers is planned work tied to the Stage 19 Rust-first persistence/tooling effort.

- Rust tooling roadmap:
  - The V3 PLAN already includes Stage 18 (Rust-native test runner exploration) and Stage 19 (Rust-first tooling migration); the new persistence documentation explicitly calls out that `@persist`-driven helpers will be wired through that pipeline, keeping the Rust-first tooling work and persistence evolution aligned.

- Tests and validation:
  - Rebuilt `framec` in debug mode:
    - `cargo build -p framec`
  - Re-ran the full V3 PRT transpile-only suite using the debug binary:
    - `python3 framec_tests/runner/frame_test_runner.py --languages python typescript rust --categories all_v3 --framec ./target/debug/framec --transpile-only`
    - All Python, TypeScript, and Rust `all_v3` categories remain 100% green.

- Release and bootstrap:
  - Bumped workspace version to `0.86.69` (root `Cargo.toml`, `version.toml` via `scripts/sync-versions.sh`) and updated release metadata (`release_date`, `build_number`).
  - Built the release binary:
    - `cargo build --release -p framec`
  - Ran `python3 tools/publish_framec_release.py`:
    - Successfully updated the bootstrap compiler at `boot/framec/framec` (version `0.86.69`).
    - Attempted to create the shared env release at
      `framepiler_test_env/bug/releases/frame_transpiler/v0.86.69/framec/framec`, but this step failed in this environment with a permission error when creating the `v0.86.69` directory.
    - On a developer machine with access to the shared test environment, re-running `tools/publish_framec_release.py` should complete the shared env publish for `0.86.69`.
