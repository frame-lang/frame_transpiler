# Release Notes — v0.86.58

**Date**: 2025-11-22  
**Type**: bug-fix / plan alignment

## Summary

This release bumps the Frame Transpiler to `0.86.58` and aligns the V3
architecture plan and documentation with the current PRT (Python/TypeScript/
Rust) implementation. It keeps all PRT V3 suites green while clarifying that
Stage 11 (Arcanum-backed semantics) and Stage 12 Phase A FID work are
complete for PRT, and that Stage 13 project-layer/FID linking is the next
focus area.

## Changes

### 1. V3 PLAN “Todo Next” cursor updated

- `docs/framepiler_design/architecture_v3/PLAN.md`:
  - Collapsed the duplicate `## Todo Next` sections into a single cursor.
  - Marked Stage 11 (PRT Arcanum semantics) and Stage 12 Phase A FID/schema
    work as complete and reflected in the detailed stage sections.
  - Set “Todo Next” to Stage 13 — Project Layer / FID Linking (PRT-first),
    focusing next on:
    - 13A/13B: project manifests and CLI scaffolding (`framec project build`,
      `framec fid import`) wired to the Phase A FID loaders.
    - Early project-level tests for PRT that keep Stage 13 optional and
      disabled by default.

### 2. Adapter protocol verification scaffolding (tracked, no behavior change)

- `adapter_protocol/tests/README.md`:
  - Documented the role of shared adapter verification tests in the main repo:
    - Bug-specific repros remain in the shared `framepiler_test_env` bug
      tracker.
    - Stable adapter semantics are captured here as verification tests so
      regressions are caught automatically.
  - Clarified that the canonical smoke test remains the shared
    `adapter_protocol/scripts/run_adapter_smoke.sh` harness, which both
    teams use for AdapterProtocol validation.

## Validation

- **Build**:
  - `cargo build --release -p framec` (workspace version `0.86.58`).
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
- **V3 curated exec (PRT core/control_flow/scoping/systems)**:
  - `python3 framec_tests/runner/frame_test_runner.py \
       --languages python typescript \
       --categories v3_core v3_control_flow v3_scoping v3_systems \
       --framec ./target/release/framec \
       --run --exec-v3`
  - Result:
    - Python: 68/68 passing.
    - TypeScript: 72/72 passing.

