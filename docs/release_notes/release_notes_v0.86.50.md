# Release Notes — v0.86.50

**Date**: 2025-11-18  
**Type**: bug-fix / V3 TypeScript API

## Summary

This release completes the V3 TypeScript CLI surface for systems by adding
public interface wrappers and a functional router, so generated modules can be
used from TypeScript without manually constructing `FrameEvent` or
`FrameCompartment` instances. It builds on v0.86.49’s runtime type alignment
and formally fixes Bug #079.

## Changes

### 1. Public Interface Wrappers (V3 TypeScript)

- For each interface event `foo(payload)` in a V3 TypeScript system, the
  generator now emits:
  - A public wrapper:
    ```ts
    public foo(payload: T): void {
      const __e = new FrameEvent("foo", null);
      this._frame_router(__e, this._compartment, payload);
    }
    ```
  - A private internal handler:
    ```ts
    private _event_foo(__e: FrameEvent, compartment: FrameCompartment, payload: T): void {
      const c = compartment || this._compartment;
      switch (c.state) {
        case "__System_state_A":
          // spliced state body
          break;
        case "__System_state_B":
          // spliced state body
          break;
      }
    }
    ```
- Public methods no longer expose `FrameEvent` or `FrameCompartment` in their
  signatures; these types are now confined to internal methods and the router.

### 2. Functional Router (V3 TypeScript)

- `_frame_router` is now a real dispatcher rather than a stub:
  ```ts
  _frame_router(__e: FrameEvent, c?: FrameCompartment, ...args: any[]): void {
    const _c = c || this._compartment;
    switch (__e.message) {
      case "runtimeMessage":
        this._event_runtimeMessage(__e, _c, args[0]);
        break;
      case "start":
        this._event_start(__e, _c);
        break;
      default:
        break;
    }
  }
  ```
- `$enter` events from the system constructor and `_frame_transition` continue
  to use `_frame_router` as in v0.86.49.

### 3. Bug #079 — V3 TypeScript Interface Wrappers and Router Parity

- **Bug**: Generated V3 TypeScript modules previously:
  - Exposed handler-style signatures like
    `public foo(__e: FrameEvent, compartment: FrameCompartment, payload): void`.
  - Emitted a stub `_frame_router` that simply `void`ed its variables.
- **Fix (v0.86.50)**:
  - Public API now matches the V3 design: consumer-facing wrappers without
    runtime types, plus a functional router that dispatches to private handlers.
  - `docs/bugs/fixed/bug_079_typescript_missing_public_interface_wrappers_and_router_parity.md`
    records this behavior with `fixed_version: v0.86.50`.

## Validation

- **Build**:
  - `cargo build --release` (workspace version `0.86.50`) — success.
- **V3 TypeScript CLI**:
  - `python3 framec_tests/runner/frame_test_runner.py --languages typescript --categories v3_cli --framec ./target/release/framec --transpile-only -v`
  - Result: `language_specific_typescript_v3_cli: 10/10 (100%)`, including:
    - `multi_state_interface_router` (interface wrappers + router dispatch).
    - `actions_and_domain_emit_issues` (actions + domain fields).
- **V3 Transpile-only (Python/TypeScript)**:
  - V3 Python/TS suites remain at the same pass/fail status as in v0.86.49,
    with no new regressions from this change.

## Notes

- This release focuses purely on the V3 TypeScript CLI API; it does not change
  the underlying V3 architecture, scanners, or MIR semantics.
- Combined with v0.86.49’s runtime type alignment, V3 TypeScript systems are
  now callable from TypeScript code using only public methods and the
  `frame_runtime_ts` package types.

