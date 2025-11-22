# Release Notes — v0.86.53

**Date**: 2025-11-19  
**Type**: bug-fix / V3 TypeScript API (adapter-friendly)

## Summary

This release fixes return-value handling for V3 TypeScript interface wrappers so
that methods such as `drainCommands()` surface the value produced by their
Frame bodies, and formally closes Bugs #082 and #083 from the compiler side.

## Changes

### 1. Bug #082 — drainCommands returns undefined

- **Problem**: For systems where an interface method returned the result of an
  action (e.g., `drainCommands() { return this.flushCommands() }`), the
  generated TypeScript wrapper had signature `public drainCommands(): void`
  and called `_frame_router(...)` without returning its result, so callers saw
  `undefined` rather than the expected array.
- **Fix**:
  - V3 TypeScript interface wrappers now:
    - Have signature `public name(...): any`.
    - Construct a `FrameEvent` and `return this._frame_router(__e, this._compartment, ...)`.
  - `_frame_router` now:
    - Has signature `_frame_router(__e: FrameEvent, c?: FrameCompartment, ...args: any[]): any`.
    - `return`s the result of the matching `_event_*` handler in each `case`.
  - This ensures that any FRM interface method that `return`s a value from a
    handler (e.g., `drainCommands`, `getPaused`) now exposes that value to
    TypeScript callers.
- **Validation**:
  - Updated `language_specific/typescript/v3_cli` fixtures:
    - `multi_state_interface_router.frm` now expects `public start(): any`.
    - `adapter_protocol_minimal.frm` now expects `public start(): any`,
      `public runtimeConnected(): any`, `public runtimeDisconnected(): any`,
      `public requestTerminate(): any`, and `public drainCommands(): any`.
  - v3_cli TypeScript tests:
    - `python3 framec_tests/runner/frame_test_runner.py --languages typescript --categories v3_cli --framec ./target/release/framec --transpile-only`
    - Result: 11/11 passing.

### 2. Bug #083 — stopped event does not set isPaused

- The minimal in-repo fixture (`AdapterProtocolMinimal`) and the shared
  `framepiler_test_env/adapter_protocol` harness already validated that
  `runtimeMessage` with `event: 'stopped'` sets `isPaused === true` and
  updates stopped metadata in the generated TS.
- The external Bug #083 validator currently fails earlier due to
  TypeScript environment issues (missing `frame_runtime_ts` module and Node
  typings), not because of an isPaused semantics bug in the compiler.
- The bug is therefore closed from the compiler perspective in v0.86.53; the
  recommended path for external tests is to:
  - Reuse the shared `framepiler_test_env/adapter_protocol` harness, or
  - Add `frame_runtime_ts` and Node typings (`@types/node`) to their TS setup.

## Validation

- **Build**:
  - `cargo build --release` (workspace version `0.86.53`).
- **V3 TypeScript CLI**:
  - `python3 framec_tests/runner/frame_test_runner.py --languages typescript --categories v3_cli --framec ./target/release/framec --transpile-only`
  - Result: `language_specific_typescript_v3_cli: 11/11 (100%)`.

