# Release Notes — v0.86.56

**Date**: 2025-11-22  
**Type**: bug-fix / V3 Python codegen (scoping exec)

## Summary

This release bumps the Frame Transpiler version to `0.86.56` and hardens the
V3 Python module code generator so that nested `def` blocks inside handlers
combine correctly with Frame-generated router calls. This closes the nested
`def` indentation issue tracked as Bug #090 in the shared test environment and
allows us to keep Python V3 scoping fixtures enabled under curated exec.

## Changes

### 1. V3 Python handler emission — nested `def` + router calls

- **Problem**: For Python V3 handlers that combined nested function definitions
  with Frame statements (e.g., `=> $^`), the module-path emitter sometimes
  produced invalid Python by:
  - Mis-normalizing indentation inside state-guarded handlers, and
  - Allowing the generated `self._frame_router(__e, ...)` call to be indented
    under a nested `def` or block, leading to inconsistent indentation and
    runtime failures in `v3_scoping` exec fixtures.
- **Fix**:
  - Updated the Python V3 handler emitter in `framec/src/frame_c/v3/mod.rs` to:
    - Compute a base indent per handler (and per state body) and re-emit each
      logical line with:
      - A normalized outer pad under the handler or state guard, and
      - Preserved *relative* indentation for nested `def`/block structures.
    - Rewrite `system.return` to `self._system_return_stack[-1]` inside
      handlers, maintaining the per-call `system.return` semantics.
    - Apply handler-only sugar `return expr` → `system.return = expr; return`
      while leaving bare `return` unchanged.
    - Treat `self._frame_router(__e, ...)` calls generated from Frame
      forward/transition semantics as top-level statements within the state
      guard (never nested inside local defs/blocks), eliminating the mixed
      indentation that previously broke the Python parser.
- **Impact**:
  - Python V3 scoping fixtures now generate syntactically valid code for
    patterns such as:
    - Nested local functions (`def f()`, `def outer()/inner()`),
    - Local shadowing blocks (`if True: x = 2`),
    - Frame forward/transition statements in combination with the above.

### 2. Curated V3 exec coverage (Python/TypeScript)

- The curated `--exec-v3` suite for Python and TypeScript now includes
  `v3_scoping` alongside `v3_core`, `v3_control_flow`, and `v3_systems` with
  full pass rates:
  - Python:
    - `language_specific_python_v3_scoping`: 3/3 passing
      (`function_scope`, `nested_functions`, `shadowing`).
  - TypeScript:
    - `language_specific_typescript_v3_scoping`: 4/4 passing (unchanged from
      previous release; included here for completeness).
- This means that scoping/closure semantics for both runtimes are now validated
  at exec-time under the V3 module path, not just via transpile-only fixtures.

### 3. Bug #090 (shared env) — status

- **Bug**: `Python V3 nested def indentation error in scoping fixture`
  (`bug/bugs/bug_090_python_v3_nested_def_indentation_error_in_scoping_fixture.md`
  in `/Users/marktruluck/projects/framepiler_test_env`).
- **Status**:
  - Marked `Fixed` with `fixed_version: v0.86.55` (the first version that
    contained the indentation fix); `0.86.56` carries the same change forward
    as the released build.
  - Work log updated with:
    - Pointer to the emitter changes in `framec/src/frame_c/v3/mod.rs`.
    - The curated exec command used to verify the fix.
- Note: The shared env bug tracker remains the canonical source of record for
  cross-team validation, including the full repro steps and build path.

## Validation

- **Build**:
  - `cargo build --release` (workspace version `0.86.56`).
- **V3 transpile-only suites (Python + TypeScript)**:
  - `python3 framec_tests/runner/frame_test_runner.py \
       --languages python typescript \
       --categories all_v3 \
       --framec ./target/release/framec \
       --transpile-only`
- **Curated V3 exec (Python + TypeScript)**:
  - `python3 framec_tests/runner/frame_test_runner.py \
       --languages python typescript \
       --categories v3_core v3_control_flow v3_scoping v3_systems \
       --framec ./target/release/framec \
       --run --exec-v3`
  - Result:
    - Python: 68/68 passing (including `v3_scoping`).
    - TypeScript: 72/72 passing.

