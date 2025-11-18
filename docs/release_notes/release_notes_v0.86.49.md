# Release Notes — v0.86.49

**Date**: 2025-11-18  
**Type**: bug-fix / runtime types / CLI validation

## Summary

This release aligns the TypeScript runtime type declarations with the V3 code
generator and runtime implementation and formally fixes three related TypeScript
CLI bugs (#073, #074, #078). It keeps the V3 architecture and MIR semantics
unchanged while ensuring that minimal generated modules compile cleanly with
`tsc` against the bundled `frame_runtime_ts` package.

## Changes

### 1. TypeScript Runtime Types (`frame_runtime_ts`)

- Added `frame_runtime_ts/index.d.ts` that mirrors the concrete runtime
  implementation in `frame_runtime_ts/index.ts`:
  - `FrameEvent`:
    - Constructor: `constructor(message: string, parameters: FrameEventParameters | null)`.
    - Properties: `message: string`, `parameters: FrameEventParameters | null`.
  - `FrameCompartment`:
    - Constructor:
      ```ts
      constructor(
        state: string,
        enterArgs?: any,
        exitArgs?: any,
        stateArgs?: any,
        stateVars?: any,
        enterArgsCollection?: any,
        exitArgsCollection?: any,
        forwardEvent?: FrameEvent | null
      );
      ```
    - Properties: `state`, `enterArgs`, `exitArgs`, `stateArgs`, `stateVars`,
      `enterArgsCollection`, `exitArgsCollection`, `forwardEvent`.
- With this surface, the V3 TypeScript generator no longer triggers TS2554
  (constructor arity) or TS2339 (missing fields) when compiled with `tsc`
  against the runtime package.

### 2. V3 TypeScript CLI Emission (Regression Validation)

- Confirmed that the V3 CLI TypeScript emitter remains consistent with the
  earlier fixes for:
  - **Bug #073** (duplicate methods per state):
    - `language_specific/typescript/v3_cli/positive/multi_state_interface_router.frm`
      still emits a single public method per interface function and dispatches
      on `c.state` inside the method body.
  - **Bug #074** (missing actions/domain declarations and dropped interface params):
    - `language_specific/typescript/v3_cli/positive/actions_and_domain_emit_issues.frm`
      still emits domain fields and actions correctly and now compiles cleanly
      under `@tsc-compile` with the updated runtime types.
- All `v3_cli` TypeScript tests pass:
  - `python3 framec_tests/runner/frame_test_runner.py --languages typescript --categories v3_cli --framec ./target/release/framec --transpile-only`

### 3. Bug Tracking Updates

- **Bug #078** — TS runtime d.ts mismatch with generator calls:
  - Status: `Fixed`, `fixed_version: v0.86.49`.
  - Resolved by adding `frame_runtime_ts/index.d.ts` and aligning the type
    surface with the V3 generator and runtime.
- **Bug #073** — TypeScript generator emits duplicate class methods per state:
  - Originally fixed in v0.86.42; reopened due to the type mismatch reported
    in #078.
  - Marked `Fixed` in v0.86.49 after confirming that:
    - The generator still emits a single public method per interface with
      state-based routing.
    - Minimal module validation no longer fails once runtime types are aligned.
- **Bug #074** — TypeScript generator omits actions/domain declarations and drops interface params:
  - Originally fixed in v0.86.43; reopened due to the same type mismatch.
  - Marked `Fixed` in v0.86.49 with the aligned runtime types; CLI fixtures
    compile without TS errors.
- `docs/bugs/INDEX.md` has been updated:
  - `Fixed: 3` (073, 074, 078), `Open: 0`, `Reopen: 0`.
  - All three bugs are listed under “Fixed (awaiting closure by owning team)”
    with `fixed_version: v0.86.49`.

## Validation

- **Build**:
  - `cargo build --release` (workspace version `0.86.49`) — success.
- **V3 Transpile-only suites (Python/TypeScript)**:
  - `python3 framec_tests/runner/frame_test_runner.py --languages python typescript --categories v3_prolog v3_imports v3_outline v3_closers v3_mir v3_mapping v3_expansion v3_validator v3_core v3_control_flow v3_data_types v3_operators v3_scoping v3_systems --framec ./target/release/framec --transpile-only`
  - Result: same known Python/TS negatives as prior builds; no new failures introduced by this release.
- **V3 CLI (TypeScript)**:
  - `python3 framec_tests/runner/frame_test_runner.py --languages typescript --categories v3_cli --framec ./target/release/framec --transpile-only`
  - Result: 10/10 tests passing, including `multi_state_interface_router` and
    `actions_and_domain_emit_issues` under `@tsc-compile`.

## Notes

- This release is intentionally narrow in scope: it does not change the V3
  architecture, scanners, or MIR; it only brings the TypeScript runtime types
  in line with the existing runtime and generator behavior.
- Future work may publish the updated `frame_runtime_ts` `index.d.ts` as part
  of the external runtime package so that downstream consumers automatically
  benefit from these fixes.

