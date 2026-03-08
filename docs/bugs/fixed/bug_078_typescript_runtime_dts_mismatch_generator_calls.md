# Bug #078: TypeScript runtime d.ts mismatch with generator calls (FrameCompartment ctor, FrameEvent/Compartment fields)

## Metadata
bug_number: 078
title: TypeScript runtime d.ts mismatch with generator calls (FrameCompartment ctor, FrameEvent/Compartment fields)
status: Fixed
priority: High
category: CodeGen
discovered_version: v0.86.47
fixed_version: v0.86.49
reporter: vscode_editor (Codex)
assignee: framepiler team
created_date: 2025-11-18
resolved_date: 

## Description
With v0.86.47, the generated TS modules invoke the runtime with multi‑argument `new FrameCompartment(state, enterArgs, …)` and reference fields like `n.enterArgs` and `__e.message`. When compiling these minimal modules against the published runtime types, `tsc` reports signature and property errors:

- TS2554: Expected 1 arguments, but got 4 (FrameCompartment constructor)
- TS2339: Property 'enterArgs' does not exist on type 'FrameCompartment'
- TS2339: Property 'message' does not exist on type 'FrameEvent'

Our workspace runtime implementation (`frame_runtime_ts/index.ts`) does define a multi‑arg constructor and those fields. The failure occurs in standalone minimal module compilation because the surfaced type shape from the resolved runtime types does not match the generator’s usage in this context.

## Reproduction Steps
1) Ensure `framec` is v0.86.47.
2) Use either of the existing /tmp validators that compile a minimal TS module and run `tsc`:
   - `bash /tmp/frame_transpiler_repro/bug_073/run_validate.sh`
   - `bash /tmp/frame_transpiler_repro/bug_074/run_validate.sh`
3) Observe TS2554/TS2339 errors about FrameCompartment args and FrameEvent/Compartment fields.

## Expected Behavior
- Generated TS modules should compile with `tsc` when using the published runtime types (no argument count or missing property errors).
- The `frame_runtime_ts` types (or the generator’s usage) should be aligned so that constructor signatures and commonly used fields (`message`, `enterArgs`) are consistent.

## Actual Behavior
- `tsc` reports constructor arity and missing property errors during minimal module compilation.

## Impact
- Severity: High — prevents validating minimal TS modules without custom stubs; undermines confidence in TS target consistency.
- Scope: TS target minimal module compile; likely affects users compiling generated TS against the published runtime types.

## Resolution (Developer)
- Implemented an explicit `frame_runtime_ts/index.d.ts` in the workspace that matches the actual runtime implementation in `frame_runtime_ts/index.ts`:
  - `FrameEvent` now has a declared `message: string` property and constructor `(message: string, parameters: FrameEventParameters | null)`.
  - `FrameCompartment`'s constructor is declared with `(state: string, enterArgs?, exitArgs?, stateArgs?, stateVars?, enterArgsCollection?, exitArgsCollection?, forwardEvent?)`.
  - All fields used by the TS generator (`enterArgs`, `exitArgs`, `stateArgs`, `enterArgsCollection`, `exitArgsCollection`, `forwardEvent`) are declared on `FrameCompartment`.
- Kept the V3 TS generator behavior aligned with the runtime and docs:
  - System constructors call `new FrameCompartment("__Name_state_A", enterArgs, undefined, stateArgs)` and seed `_compartment.enterArgs`/`_compartment.stateArgs`.
  - `_frame_transition` uses `n.enterArgs` to build `$enter` events.
  - MIR expansion for transitions and forwards continues to wire `exitArgs`, `enterArgs`, `stateArgs`, and `compartment.parentCompartment` directly.
- Effect:
  - Generated TS modules now type‑check cleanly against the bundled runtime types (and any consumers using the updated d.ts) without constructor or missing‑property errors.
  - The runtime implementation, docs, and generator are consistent for `FrameEvent`/`FrameCompartment`.

## Repro Shortcuts
- #073 validator: `/tmp/frame_transpiler_repro/bug_073/run_validate.sh`
- #074 validator: `/tmp/frame_transpiler_repro/bug_074/run_validate.sh`

## Work Log
- 2025-11-18: Initial report with /tmp validators — vscode_editor
