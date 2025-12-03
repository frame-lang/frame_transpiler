# Stage 18 — Rust-Native Test Runner & Tooling Exploration

This document describes the current state of Rust-native V3 tooling, the
relationship to the existing Python-based tools, and the tradeoffs involved
in moving more of the test/tooling surface to Rust.

It is intentionally descriptive rather than prescriptive: the Python runner
remains the source of truth, and Rust-native tooling is exploratory.

## 1. Current Tooling Surface

### 1.1 Python-Based Tools (Authoritative Path)

- **V3 test runner**
  - `framec_tests/runner/frame_test_runner.py`
  - Drives all language-specific V3 categories for Python, TypeScript, and
    Rust.
  - Responsibilities:
    - Discovers `.frm` fixtures under `framec_tests/language_specific`.
    - Interprets metadata (`@target`, `@expect`, `@py-compile`, `@tsc-compile`,
      `@rs-compile`, `@meta: rs_compile`, etc.).
    - Runs `framec compile` (and where configured, language compilers /
      runtime executables) and compares results to expectations.

- **Self-hosted machine precompile helper**
  - `tools/gen_v3_machines_rs.py`
  - Uses the committed bootstrap compiler (`boot/framec/framec`) to compile
    Frame machines under `framec/src/frame_c/v3/machines/*.frs` to Rust
    sources (`*.gen.rs`).
  - Called explicitly (not via `cargo build`), with build hints coming from
    `build.rs`.

- **Release helper**
  - `tools/publish_framec_release.py`
  - Copies `target/release/framec` to:
    - `boot/framec/framec` (bootstrap compiler).
    - `framepiler_test_env/bug/releases/frame_transpiler/<version>/framec/framec`
      (shared env reference).

These Python tools are the canonical path: CI and local workflows rely on
them, and any new language, category, or policy is first wired through this
pipeline.

### 1.2 Rust-Based Tools (Current State)

- **Rust V3 test harness (`framec test`)**
  - CLI entry:
    - `framec/src/frame_c/cli.rs` exposes a `test` subcommand on the main
      `framec` binary.
  - Usage (validation-only):
    ```bash
    cargo run -p framec --bin framec -- \
      test --language <python_3|typescript|rust> --category <v3_category>
    ```
  - Compare-with-Python mode:
    ```bash
    cargo run -p framec --bin framec -- \
      test --language python_3 --category v3_core --compare-python
    ```
    - Runs the Rust harness and the Python V3 runner on the same slice and
      reports whether both succeeded.
  - Exec modes:
    - `--exec-smoke`:
      - Drives the `v3_exec_smoke` category for python/typescript/rust using
        FRAME_EMIT_EXEC and the same marker semantics as the Python runner.
    - `--exec-curated`:
      - Drives curated exec for `v3_core` (and will be extended to other
        curated categories) for python/typescript/rust, interpreting
        `@run-expect` / `@run-exact` metadata in fixtures.
  - Implementation details:
    - All test logic lives in `framec::frame_c::v3::test_harness_rs`:
      - `run_validation_for_category`
      - `run_rust_exec_smoke`, `run_python_exec_smoke`,
        `run_typescript_exec_smoke`
      - `run_rust_curated_exec_for_category`,
        `run_python_curated_exec_for_category`,
        `run_typescript_curated_exec_for_category`
    - The older `v3_rs_test_runner` binary still exists as a thin,
      developer-facing wrapper around this library but is no longer the
      primary documented entry point.
  - Current PRT V3 coverage:
    - Validation + compare-Python:
      - python/typescript/rust: `v3_core`, `v3_control_flow`,
        `v3_systems`, `v3_persistence`, `v3_systems_runtime`.
    - Exec-smoke:
      - python/typescript/rust: `v3_exec_smoke`.
    - Exec-curated:
      - python/typescript/rust: `v3_core` (additional curated categories are
        available via `v3_rs_test_runner` and will be folded into
        `framec test` over time).

- **Rust snapshot shape tool**
  - `framec/src/bin/v3_rs_snapshot_shape.rs`
  - Usage:
    ```bash
    cargo run -p framec --bin v3_rs_snapshot_shape
    ```
  - Behavior:
    - Constructs the canonical `SystemSnapshot` JSON for a TrafficLight
      system (same shape as described in `14_persistence_and_snapshots.md`).
    - Uses `frame_persistence_rs::SystemSnapshot` to validate Rust-side
      parse/encode/compare semantics.
    - Invokes the Python and TypeScript persistence helpers via
      subprocesses (`frame_persistence_py`, `frame_persistence_ts`) and
      asserts that all three agree on the DTO shape.

The Rust tools above are primarily focused on validation and DTO shape
checks; they do not yet attempt to mirror the full execution harnesses
provided by the Python runner.

## 2. Tradeoffs: Python vs Rust Tooling

### 2.1 Advantages of Rust-Native Tooling

- **Single-language toolchain for Rust users**
  - Developers primarily working in Rust can run a subset of validation
    tests using `cargo run` without needing Python installed, once Rust
    coverage is sufficient for their workflow.

- **Stronger static checks in tooling**
  - Tooling written in Rust benefits from the same type system and error
    checking as the compiler itself, reducing certain classes of scripting
    bugs that are easy to introduce in ad-hoc Python code.

- **Closer integration with the CLI**
  - Rust code can directly use internal APIs and types (e.g., Arcanum,
    validators) without going through the CLI boundary, if/when we choose
    to expose such APIs, enabling richer tooling over time.

### 2.2 Advantages of Python Tooling (Why It Remains Authoritative)

- **Faster iteration for complex test harnesses**
  - Python is well-suited for glue logic:
    - Orchestrating external tools (e.g., `tsc`, `node`, `py_compile`,
      `rustc`).
    - Managing temporary directories and process orchestration.
  - Adding or adjusting harness behavior is generally faster in Python,
    especially during heavy test suite evolution.

- **Cross-language dependencies**
  - Many V3 tests exercise interactions with:
    - Python runtime (`frame_runtime_py`), `py_compile`, etc.
    - TypeScript toolchain (`tsc`, Node).
    - Rust toolchain (`rustc`).
  - The Python runner already understands how to range over these tools,
    interpret their outputs, and align them with Frame error codes and
    expectations.

- **Existing CI and developer muscle memory**
  - CI workflows and local scripts are wired to the Python runner and
    helpers.
  - Contributors across multiple languages are more likely to have Python
    available than a full Rust toolchain, especially when only running
    tests.

### 2.3 Practical Tradeoff Summary

- Short term:
  - Keep Python as the source of truth for all V3 tests and release
    workflows.
  - Use `v3_rs_test_runner` as a focused, fast validation harness for a
    subset of categories (primarily PRT V3) and for exploring Rust-native
    test orchestration.

- Medium term:
  - Expand `v3_rs_test_runner` coverage where it adds clear value (e.g.,
    Rust-facing categories, simple validation-only suites for Python and
    TypeScript that do not require external language toolchains).
  - Add more small Rust helpers where they naturally fit (e.g., future
    self-hosted machines and post-MIR inspectors).

- Long term:
  - Consider a Rust-only toolchain mode as an *optional* path:
    - Build + run a curated subset of V3 tests and tools using only Rust
      binaries (no Python), for users who want this.
    - Keep the Python-based path fully supported and authoritative, to
      avoid locking contributors into a single language/toolchain.

## 3. Next Steps (Stage 18 / Future Work)

- **Increase Rust runner coverage (carefully)**
  - Evaluate extending `v3_rs_test_runner` to additional categories:
    - More Rust V3 suites (e.g., persistence and systems/runtime) once the
      desired semantics are nailed down.
    - Possibly selected Python/TypeScript categories that are purely
      validation-only and do not rely on external toolchains.

- **Document CI integration options**
  - Consider adding an optional CI job that runs `v3_rs_test_runner` for a
    small, focused subset of categories alongside the Python runner, to
    exercise the Rust tooling without making it mandatory.
  - Suggested pilot: `framec test --language rust --category v3_core` (and
    a single python/typescript slice) using the release binary; gate the job
    to avoid blocking when toolchains are missing.

- **Explore migration candidates**
  - Identify Python tools that could benefit most from a Rust
    implementation (e.g., simple validators, index comparers) and record
    them in `PLAN.md` as explicit follow-up items, keeping in mind the
    tradeoffs above.

## 4. Roadmap to a Rust-First Harness (Exec Parity)

This section mirrors the high-level Stage 18 roadmap from `PLAN.md` in more
detail. The intent is not to remove the Python runner, but to reach a point
where a Rust-first harness can cover the full PRT surface, including exec.

1. **Phase 1 — Validation parity**
   - Refactor `v3_rs_test_runner` into a small library + bin that:
     - Parses metadata from `.frm` headers and comments in a way that
       matches the Python runner (negative tests, `@expect`, `@meta`,
       `@skip-if`, torture/infinite loop detection).
     - Can run validation-only tests for arbitrary `<language>/<category>`
       pairs.
   - Expand validation coverage to all PRT V3 categories for Python,
     TypeScript, and Rust.
   - Add a “compare with Python” mode that runs both runners on the same
     fixture set and reports divergences in pass/fail and error codes.

2. **Phase 2 — Rust exec harness for Rust targets**
   - Implement a Rust-native exec path that:
     - Compiles `.frs` to Rust via `framec`.
     - Builds the generated Rust via `rustc` (or `cargo` for multi-file).
     - Runs the resulting binary and captures stdout/stderr.
   - Mirror the Python runner’s `v3_exec_smoke` semantics:
     - Expected markers in output.
     - Toolchain-missing skips.
   - Gradually move Rust exec-smoke and curated exec suites under this
     harness as the primary path once behavior is proven equivalent.

3. **Phase 3 — Exec harness for Python/TypeScript from Rust**
   - Extend the Rust harness to drive exec for Py/TS fixtures by:
     - Compiling `.frm` to `.py`/`.ts` using the same V3 module paths and
       `FRAME_EMIT_EXEC` policies as the Python runner.
     - Invoking `python3`, `tsc`, and `node` as subprocesses, with the same
       treatment of `@py-compile`, `@tsc-compile`, and toolchain-skips as
       in the Python runner.
   - Keep the Python runner as the reference while iterating on this path.

4. **Phase 4 — Cross-language persistence/snapshot exec in Rust**
   - Port cross-language persistence tests (e.g., TrafficLight snapshots)
     into a Rust binary that:
       - Compiles and runs the Py/TS/Rust snapshot fixtures.
       - Uses `frame_persistence_py`, `frame_persistence_ts`, and
         `frame_persistence_rs` to compare JSON snapshots.

5. **Phase 5 — Unified Rust test CLI**
   - Introduce a CLI entry (e.g., `framec test`) that:
     - Accepts language/category filters and modes (`--transpile-only`,
       `--exec`, `--validation-only`).
     - Internally uses the Rust harness library to drive tests across
       languages and categories.
     - Can optionally run in “compare with Python” mode for extra safety.

Rust-native tooling is a complement to, not a replacement for, the Python
tooling. Any migration or expansion should preserve determinism, keep DFA /
DPDA analyzers as the ground truth, and remain fully reflected in the V3
PLAN and docs.
