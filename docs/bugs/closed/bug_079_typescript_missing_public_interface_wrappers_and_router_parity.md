# Bug #079: V3 TypeScript — missing public interface wrappers and router parity (generated systems expose internal args; router stub)

## Metadata
bug_number: 079
title: V3 TypeScript — missing public interface wrappers and router parity (generated systems expose internal args; router stub)
status: Closed
priority: High
category: CodeGen
discovered_version: v0.86.49
fixed_version: v0.86.50
reporter: vscode_editor (Codex)
assignee: framepiler team
created_date: 2025-11-18
resolved_date: 2025-11-18

## Description
Per V3 docs, TypeScript targets should emit consumer‑safe public interface methods that construct a `FrameEvent` and call a functional `_frame_router`. Current generated TS modules expose handler‑style signatures that require internal runtime types (e.g., `__e: FrameEvent, c?: FrameCompartment`) and emit a structurally stubbed router.

This prevents Frame‑only usage of generated systems without local shims and violates the V3 design.

## Expected Behavior
- Public interface methods (e.g., `foo(payload)`):
  - Accept only domain/payload args; do not require `FrameEvent` or `FrameCompartment` parameters.
  - Construct a `FrameEvent` internally and call a functional `_frame_router(event)` that dispatches by `(state, event.message)`.

## Actual Behavior (v0.86.49, pre-fix)
- Generated methods included signatures like `public foo(__e: FrameEvent, compartment: FrameCompartment, payload): void`.
- `_frame_router` body was a no‑op stub: referenced `__e.message` and `this._compartment` but only `void`ed them, with no dispatch.

## Reproduction Steps
1) Ensure `framec` prints `framec 0.86.49`.
2) Run `/tmp/frame_transpiler_repro/bug_079/run_validate.sh`.
3) Observe that generated TS contains:
   - `public foo(__e: FrameEvent` (internal args exposed)
   - `_frame_router(__e: FrameEvent` followed by a stub body that `void`s variables, indicating no dispatch.

## Impact
- Severity: High — blocks “Frame‑only → generate” adoption for TS systems without local shims.
- Scope: TS target V3 generator output for systems with interface methods.

## Technical Analysis
- V3 docs (architecture_v3/codegen.md, frame_runtime.md) specify public interface wrappers and a functional router. Current TS generator provides only a structural scaffold in `_frame_router`, and interface methods mirror internal handler signatures requiring runtime types.

## Resolution (Developer)
- Implemented public interface wrappers and a functional `_frame_router` for V3 TypeScript systems:
  - **Public interface wrappers**:
    - For each interface method `foo(payload)` the generator emits:
      - A public method `foo(payload)` with no `FrameEvent`/`FrameCompartment` parameters.
      - The wrapper constructs `new FrameEvent("foo", null)` and calls `_frame_router(__e, this._compartment, payload)`.
  - **Internal event handlers**:
    - The previous grouped `public foo(__e: FrameEvent, compartment: FrameCompartment, payload)` method is now a private internal handler:
      - `_event_foo(__e: FrameEvent, compartment: FrameCompartment, payload)` that:
        - Normalizes `compartment || this._compartment`.
        - Switches on `c.state` and executes the spliced state‑specific body (`case "__System_state_A": …` / `case "__System_state_B": …`).
  - **Router dispatch**:
    - `_frame_router(__e: FrameEvent, c?: FrameCompartment, ...args: any[]): void` dispatches on `__e.message` and calls internal handlers:
      - `switch (__e.message) { case "foo": this._event_foo(__e, _c, args[0]); … }`.
    - This keeps router semantics simple (first positional payload) while providing a real entry point for wrappers and `$enter` events.
- Public API no longer exposes `FrameEvent`/`FrameCompartment` in interface signatures; internal handlers remain private.

## Validation
- V3 TypeScript CLI suite (`v3_cli`) with `@tsc-compile`:
  - `language_specific/typescript/v3_cli/positive/multi_state_interface_router.frm`:
    - Now emits `public start(): void` and `public runtimeMessage(payload)`, plus private `_event_start` / `_event_runtimeMessage`.
    - `_frame_router` dispatches by `__e.message` and delegates to the appropriate `_event_*` handler.
  - `language_specific/typescript/v3_cli/positive/actions_and_domain_emit_issues.frm`:
    - Emits `public runtimeMessage(payload)` wrapper and internal `_event_runtimeMessage` that calls actions and uses domain fields.
  - All 10 v3_cli TypeScript tests pass with `tsc` validation:
    - `python3 framec_tests/runner/frame_test_runner.py --languages typescript --categories v3_cli --framec ./target/release/framec --transpile-only -v`
- Core V3 TypeScript suites (transpile‑only) remain at the same pass/fail status as before; no new regressions were introduced by this change.

## Test Cases
- Minimal FRM with a single interface method `foo(payload)` produces public `foo(payload)` and a functional router.

## Repro Shortcuts
- `/tmp/frame_transpiler_repro/bug_079/minimal_ts_interface.frm`
- `/tmp/frame_transpiler_repro/bug_079/run_validate.sh`

## Work Log
- 2025-11-18: Filed with minimal FRM + validator (0.86.49) — vscode_editor
