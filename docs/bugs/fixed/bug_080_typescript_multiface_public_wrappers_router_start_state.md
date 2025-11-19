# Bug #080: V3 TypeScript — multi‑interface systems missing public wrappers, functional router, and correct start state

## Metadata
bug_number: 080
title: V3 TypeScript — multi‑interface systems missing public wrappers, functional router, and correct start state
status: Fixed
priority: High
category: CodeGen
discovered_version: v0.86.50
fixed_version: v0.86.50
reporter: vscode_editor (Codex)
assignee: framepiler team
created_date: 2025-11-18
resolved_date: 2025-11-18

## Description
For multi‑interface systems (e.g., AdapterProtocol), the TS generator output does not match the V3 design:
- No public, consumer‑safe wrappers for interface methods (e.g., `start()`, `runtimeConnected()`, `runtimeMessage(payload)`, …). Methods instead expose internal runtime args (`__e: FrameEvent, c?: FrameCompartment, ...`).
- `_frame_router` remains a no‑op stub with no dispatch.
- Initial compartment seeds to `__AdapterProtocol_state_A` instead of first declared state `__AdapterProtocol_state_Idle`.

These prevent Frame‑only usage of the generated class in consumers/tests without shims, and contradict the V3 docs.

## Expected Behavior
- Emit public wrappers per interface that construct `FrameEvent` internally and call a functional router, e.g., `public runtimeMessage(payload) { const e = new FrameEvent('runtimeMessage', null); this._frame_router(e, this._compartment, payload); }`.
- Implement `_frame_router` with a state+event dispatch (switch/case) calling generated per‑state handlers.
- Seed initial compartment to the first declared state (Idle), not a placeholder.

## Actual Behavior (v0.86.50)
- Generated `AdapterProtocol` contains internal‑form signatures like `runtimeMessage(__e: FrameEvent, c?: FrameCompartment, payload)`.
- `_frame_router(__e, c)` is empty.
- Constructor seeds `new FrameCompartment('__AdapterProtocol_state_A')`.

## Reproduction Steps
1) Ensure `framec --version` prints `framec 0.86.50`.
2) Run `/tmp/frame_transpiler_repro/bug_080/run_validate.sh`.
3) Inspect generated TS in the OUT directory to observe the issues above.

## Impact
- Severity: High — blocks debugger adapter integration without wrappers, violating the “no hacks” policy.

## Resolution (Developer)
- **Terminology alignment**:
  - In the V3 architecture, a system has at most one `interface:` block; the glossary now refers to a system with multiple interface methods as a *multi‑method interface system* (see `docs/framepiler_design/architecture_v3/architecture_v3_overview.md` → Glossary).
  - “Multi‑interface system” in this bug should be read as “a single system with several interface methods (e.g., AdapterProtocol)”.
- **Public wrappers and router (TS)**:
  - V3 TypeScript CLI emission now implements the design requested here:
    - Public interface wrappers (`start()`, `runtimeMessage(payload)`, etc.) have Frame‑friendly signatures with no `FrameEvent`/`FrameCompartment` parameters and internally construct `FrameEvent` instances before calling `_frame_router`.
    - Private internal handlers (`_event_*`) dispatch on `c.state` and contain the spliced per‑state bodies.
    - `_frame_router(__e, c, ...args)` is a functional router that switches on `__e.message` and delegates to the appropriate `_event_*` handler, not a stub.
  - Multi‑method systems like AdapterProtocol therefore expose only wrapper signatures at their public surface; the runtime types remain internal.
- **Start state behavior**:
  - V3 uses `Arcanum` to pick the first declared state in the system’s `machine:` block as the start state; this is implemented via `find_start_state_name` and is now called out explicitly in the Glossary (“Start state (V3)”).
  - Hard‑coding `A` as the start state is legacy behavior; the module path no longer uses this for V3 TypeScript CLI codegen.
- **What to do going forward**:
  - When opening future bugs against V3 TypeScript systems:
    - Use the glossary terms (“interface wrapper”, “internal handler”, “router”, “multi‑method interface system”, “start state”) to describe expected behavior.
    - Reference the V3 codegen/runtime docs (`architecture_v3/codegen.md`, `architecture_v3/frame_runtime.md`) for router/wrapper semantics rather than older v0.x expectations.
  - If a new regression appears, compare generated TS against:
    - The glossary definitions in `architecture_v3_overview.md`, and
    - The `language_specific/typescript/v3_cli` fixtures (which now assert wrapper signatures and router dispatch via `@tsc-compile`).

## Repro Shortcuts
- `/tmp/frame_transpiler_repro/bug_080/run_validate.sh`

## Work Log
- 2025-11-18: Filed with AdapterProtocol FRM reproducer — vscode_editor
