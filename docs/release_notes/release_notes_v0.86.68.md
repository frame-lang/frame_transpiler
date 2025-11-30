# Release v0.86.68 — TS Persistence Curated Exec Alignment

Summary
- Aligns the TypeScript V3 persistence curated exec path in the Rust-native harness with the Python runner, and adds TypeScript codegen support for top-level Frame functions (`fn main`) in V3 modules, while keeping all PRT V3 transpile-only suites fully green.

Highlights
- TypeScript V3 persistence curated exec:
  - Fixed the Rust harness to set `NODE_PATH` for curated exec runs, so Node can resolve project-local modules such as `frame_runtime_ts` and `frame_persistence_ts`.
  - Adjusted the TypeScript V3 module-path code generator so that top-level Frame functions (including `fn main`) are emitted as `export function main(...) { ... }` after the system class, enabling fixtures like `traffic_light_persistence` to define native entry points.
  - Lifted native `import` statements from within Frame `fn main` bodies into top-level imports in the generated TS module, ensuring that `tsc` sees them at the module level and avoiding “import inside function” errors.
  - With these changes, `traffic_light_persistence` under `language_specific/typescript/v3_persistence/positive/` now passes under both:
    - The Python V3 runner (`--exec-v3 --run`), and
    - The Rust V3 test runner’s curated exec mode:
      `cargo run -p framec --bin v3_rs_test_runner -- exec-curated typescript v3_persistence ./target/debug/framec`.

- Rust V3 test harness alignment:
  - The `run_typescript_curated_exec_for_category` helper in `framec/src/frame_c/v3/test_harness_rs.rs` now:
    - Mirrors the exec-smoke behavior by prepending the repo root to `NODE_PATH` before invoking Node.
    - Treats `v3_persistence` fixtures consistently with the Python runner by appending `main();` when a generated `function main(...)` is present but no explicit call is emitted in the source.
  - The `v3_rs_test_runner` `exec-curated typescript all_curated` path now reports `passed=70 failed=0`, matching the Python runner’s curated exec behavior for the covered PRT categories.

- Tests and validation:
  - Rebuilt `framec` in release mode:
    - `cargo build --release -p framec`
    - Verified `framec --version` reports `0.86.68`.
  - Re-ran the full V3 PRT transpile-only suite using the release binary:
    - `python3 framec_tests/runner/frame_test_runner.py --languages python typescript rust --categories all_v3 --framec ./target/release/framec --transpile-only`
    - All Python, TypeScript, and Rust `all_v3` categories remain 100% green.
  - Re-ran the Rust-native curated exec harness for all PRT languages:
    - `cargo run -p framec --bin v3_rs_test_runner -- exec-curated python all_curated ./target/debug/framec`
    - `cargo run -p framec --bin v3_rs_test_runner -- exec-curated typescript all_curated ./target/debug/framec`
    - `cargo run -p framec --bin v3_rs_test_runner -- exec-curated rust all_curated ./target/debug/framec`
    - All three report `failed=0`, with fixtures lacking `@run-expect`/`@run-exact` treated as exec-gated skips, matching the Python semantics.

- Release and bootstrap:
  - Bumped workspace version to `0.86.68` (root `Cargo.toml`, `version.toml` via `scripts/sync-versions.sh`).
  - Rebuilt and verified the release binary as above.
  - Ran `tools/publish_framec_release.py --framec-binary ./target/release/framec`:
    - Successfully updated the bootstrap compiler at `boot/framec/framec` (version `0.86.68`).
    - Attempted to create the shared env release at
      `framepiler_test_env/bug/releases/frame_transpiler/v0.86.68/framec/framec`, but this step failed in this environment with a permission error when creating the `v0.86.68` directory.
    - On a developer machine with access to the shared test environment, re-running `tools/publish_framec_release.py` should complete the shared env publish.

