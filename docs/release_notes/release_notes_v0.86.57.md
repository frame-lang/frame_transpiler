# Release Notes — v0.86.57

**Date**: 2025-11-22  
**Type**: bug-fix / V3 PRT validation polish

## Summary

This release bumps the Frame Transpiler to `0.86.57` and finishes the
Stage 11 “PRT-only” semantic integration work for Python, TypeScript, and Rust.
It tightens Arcanum-backed validation for transitions, parent forward, and
handler placement in the V3 module/CLI paths, and aligns the documentation
with the implemented behavior, without changing target language codegen
signatures.

## Changes

### 1. Arcanum-backed diagnostics (PRT only) — E402/E403/E404

For the PRT languages (Python, TypeScript, Rust), the V3 module and CLI
pipelines now consistently rely on the `ModuleAst + Arcanum` symbol table for
the following diagnostics:

- **E402: unknown state 'TargetState'**
  - Transitions `-> $TargetState(...)` are validated against the Arcanum
    state set for the enclosing system’s `machine:` section.
  - If `$TargetState` is not present in the system’s states, validation emits
    E402 in both the main V3 module path and the V3 CLI path for Py/TS/Rust.
  - Non‑PRT targets (C/C++/Java/C#) continue to use the existing structural
    known‑state set until their V3 paths are upgraded.

- **E403: Cannot forward to parent: no parent available**
  - Parent forward `=> $^` is only allowed when the enclosing state has a
    parent in the Arcanum (i.e., `$Child => $Parent { … }`).
  - In the PRT V3 module/CLI paths, any handler that contains a `Forward` MIR
    item but whose state has no parent in the Arcanum produces E403.

- **E404: handler body must be inside a state block**
  - Handlers in `machine:` must reside inside some `$State { … }` span; this
    is checked via the `ModuleAst` machine span and Arcanum state spans.
  - Handlers whose headers lie outside any `machine:` section (e.g., interface
    handlers under `interface:`) are explicitly excluded from this check and
    no longer trigger false positives.

### 2. Optional native-AST hooks marked complete for PRT

- Native facade parsers (SWC/RustPython/syn) were already wired for the V3
  module path via `NativeFacadeRegistryV3`. In this release:
  - Stage 11’s “optional native AST hooks” item is marked complete in
    `PLAN.md` for PRT:
    - Facades remain purely advisory and are driven by `strict_native` /
      `FRAME_VALIDATE_NATIVE_POLICY`.
    - Diagnostics from native parsers are mapped into `ValidationIssueV3` and
      do not alter codegen behavior.
  - Non‑PRT targets continue to rely on structural checks only.

### 3. Documentation alignment and cleanup

- `docs/framelang_design/architecture_v3/frame_runtime.md`:
  - Documented that E402/E403/E404 for Py/TS/Rust are Arcanum-backed in the
    V3 module/CLI paths, and that non‑PRT targets stay on structural checks.
  - Added a new section describing handler placement rules for `machine:`
    vs `interface:` and their relationship to E404.
- `docs/framelang_design/architecture_v3/codegen.md`:
  - Clarified that the PRT codegen/validation flows derive E402/E403/E404 from
    `ModuleAst + Arcanum`, and that structural coarse checks remain in place
    for non‑PRT languages.
- Minor internal cleanup:
  - Removed unused imports from `framec/src/frame_c/v3/validator.rs`.
  - Marked the unused `start` parameter in
    `build_arcanum_from_outline_bytes` as `_start` to document that it is
    intentionally unused in the PRT path.

## Validation

- **Build**:
  - `cargo build --release` (workspace version `0.86.57`).
- **V3 transpile-only suites (PRT)**:
  - `python3 framec_tests/runner/frame_test_runner.py \
       --languages python typescript rust \
       --categories all_v3 \
       --framec ./target/release/framec \
       --transpile-only`
  - Result:
    - Python: 112/112 passing.
    - TypeScript: 102/102 passing.
    - Rust: 53/53 passing.

