# Release Notes — v0.86.52

**Date**: 2025-11-19  
**Type**: bug-fix / documentation alignment (V3 Frame semantics)

## Summary

This release clarifies V3 Frame semantics for TypeScript with respect to
SOL‑anchored Frame statements and the Bug #081 adapter validator. It introduces
no compiler code changes relative to v0.86.51 but updates bug documentation and
versioning to reflect the intended behavior.

## Changes

### 1. Bug #081 — Validator Alignment and SOL‑Anchored Transitions

- Updated `docs/bugs/fixed/bug_081_typescript_adapter_semantics_guard_deferral_stopped_state.md` to
  capture the reopened investigation and final resolution:
  - The environment‑independent validator at
    `/tmp/frame_transpiler_repro/bug_081/run_validate.sh` embeds its own minimal
    AdapterProtocol FRM that uses inline transitions such as:
    - `this.lifecycle = "terminated"; -> $Terminated;`
    - `if (this.lifecycle === "terminated") { -> $Terminated }`
  - Under the V3 architecture, Frame statements (including `-> $State`) are
    **strictly SOL‑anchored**:
    - They must begin at the start of a logical statement line (after the
      Frame indent), not after a semicolon or inside an inline block.
    - Inline forms are treated as native target‑language text and will not be
      rewritten into `_frame_transition` calls, which in turn leads to invalid
      TypeScript (`TS1109`) when compiled.
  - The bug is therefore resolved by:
    - Keeping the in‑repo minimal AdapterProtocol fixture and shared
      `framepiler_test_env/adapter_protocol` harness as the canonical adapter
      semantics tests, and
    - Requiring external validators to rewrite their FRM to use SOL‑anchored
      transitions that match the documented V3 rules and the minimal fixture’s
      semantics.

## Validation

- **Build**:
  - `cargo build --release` (workspace version `0.86.52`).
- **V3 TypeScript CLI (targeted)**:
  - `python3 framec_tests/runner/frame_test_runner.py --languages typescript --categories v3_cli --framec ./target/release/framec --transpile-only`
  - Result: unchanged from v0.86.51 (all v3_cli tests, including
    `adapter_protocol_minimal`, remain green).

## Notes

- No compiler code paths changed between v0.86.51 and v0.86.52; this is a
  documentation/bug‑tracking alignment release that makes the SOL‑anchored
  requirement explicit in the context of Bug #081 and external validators.

