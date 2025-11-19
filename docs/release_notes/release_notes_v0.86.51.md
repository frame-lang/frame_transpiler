# Release Notes — v0.86.51

**Date**: 2025-11-19  
**Type**: bug-fix / V3 TypeScript adapter semantics

## Summary

This release tightens V3 TypeScript adapter semantics for guard/deferral and
stopped state handling, and introduces a shared, hermetic adapter test
environment for the Frame Transpiler (PT) and Debugger Adapter (DAP) teams.
It fixes **Bug #081** by ensuring that adapter-level rules are expressed in a
minimal in-repo FRM fixture and preserved by code generation.

## Changes

### 1. Minimal AdapterProtocol Fixture (V3 TypeScript CLI)

- Added `framec_tests/language_specific/typescript/v3_cli/positive/adapter_protocol_minimal.frm`:
  - Encodes guarded commands (`continue`, `next`, `stepIn`, `stepOut`, `pause`)
    that must defer before handshake+ready and enforce a single in-flight
    action.
  - Models `setBreakpoints` deferral until handshake/ready.
  - Tracks stopped state with `isPaused`, `lastStoppedReason`, and
    `lastThreadId`.
- The fixture is covered by `@tsc-compile` in the V3 TypeScript CLI suite and
  compiles cleanly under the shared `frame_runtime_ts` types.

### 2. Guard/Deferral Semantics Fix (Bug #081)

- Updated the minimal AdapterProtocol fixture’s `handleConnectedMessage`
  implementation so that on `eventType === "ready"`:
  - `isReady` is set to `true`.
  - `deferredQueue` entries are replayed into `commandQueue` with the
    invariant that only the first guarded command sets `pendingAction = true`;
    additional guarded commands are dropped if `pendingAction` is already
    `true`.
  - Non-guarded commands (e.g., `setBreakpoints`) are replayed as-is.
- This ensures that the `hello` + `ready` + `continue` scenario yields a
  single in-flight guarded command after readiness, matching the adapter’s
  intended behavior.

### 3. Shared Adapter Test Environment (`framepiler_test_env`)

- Established `framepiler_test_env/adapter_protocol` as a shared home for PT
  and DAP adapter semantics tests:
  - `adapter_protocol_minimal.frm` mirrors the in-repo fixture.
  - `runtime/frame_runtime_ts.d.ts` is copied from the PT repo’s
    `frame_runtime_ts/index.d.ts`.
  - `scripts/run_adapter_smoke.sh`:
    - Compiles the minimal FRM to TypeScript using `framec`.
    - Compiles TS → JS with a local `tsconfig.json` that includes Node
      typings and a `paths` alias for `frame_runtime_ts`.
    - Locates the compiled adapter and harness JS, provides a tiny
      `frame_runtime_ts` stub module, and executes the Node harness.
  - `scripts/node_harness.ts` drives a minimal sequence equivalent to the
    external validator (start, runtimeConnected, guarded pre-ready command,
    hello/ready, stopped) using a host-level drain of the adapter’s
    `commandQueue`.
- `package.json` in `adapter_protocol` pins `typescript` and `@types/node` and
  exposes `npm run smoke` as the primary entrypoint.

## Validation

- **Build**:
  - `cargo build --release` (workspace version `0.86.51`).
- **V3 TypeScript CLI (targeted)**:
  - `python3 framec_tests/runner/frame_test_runner.py --languages typescript --categories v3_cli --framec ./target/release/framec --transpile-only`
  - Result: `language_specific_typescript_v3_cli: 11/11 (100%)`, including
    `adapter_protocol_minimal`.
- **Shared Adapter Smoke (Node)**:
  - `cd framepiler_test_env/adapter_protocol`
  - `npm install`
  - `npm run smoke`
  - Result: `ADAPTER_SMOKE_OK`.

## Notes

- The original `/tmp` validator for Bug #081 remains environment-specific due
  to its dependency on a workspace-local `frame_runtime_ts` implementation and
  missing Node typings. The shared `framepiler_test_env` harness is now the
  recommended, hermetic way for both PT and DAP teams to validate adapter
  semantics across machines.

