# Bug #074: TypeScript generator omits actions/domain declarations and drops interface params

## Metadata
```yaml
bug_number: 074
title: "TypeScript generator omits actions/domain declarations and drops interface params"
status: Closed
priority: High
category: CodeGen
discovered_version: v0.86.42
fixed_version: v0.86.43
reporter: vscode_editor (Codex)
assignee: framepiler team
created_date: 2025-11-15
resolved_date: 2025-11-15
```

## Description
The TS backend now avoids duplicate methods per state (fix for #073), but it still:
- Does not emit action methods declared under `actions:`; state bodies call these actions, producing TS2339 (missing methods).
- Does not emit class property declarations for fields used in state/action bodies (`lifecycle`, `commandQueue`, etc.), producing TS2339.
- Drops interface parameters (e.g., `runtimeMessage(payload)`), yet references the undeclared `payload` identifier in generated bodies.

## Reproduction Steps
1. Use framec v0.86.42.
2. Run the repro script:
   - Repro dir: `/tmp/frame_transpiler_repro/bug_074/`
   - Command: `bash /tmp/frame_transpiler_repro/bug_074/run.sh`
3. Observe TypeScript compiler errors (TS2339, TS2304).

## Test Case
```frame
@target typescript

system TsEmitIssues {
  interface:
    start()
    runtimeMessage(payload)

  machine:
    $S {
      start() { this.lifecycle = "starting" }
      runtimeMessage(payload) { this.handle(payload) }
    }
    $T {
      start() { this.enqueueCommand("x", {}) }
    }

  actions:
    handle(payload) { this.last = payload }
    enqueueCommand(action, data) { this.commandQueue.push({ action, data }) }
    flushCommands() { const q = this.commandQueue; this.commandQueue = []; return q }

  domain:
    lifecycle = "idle"
    last = ""
    commandQueue = []
}
```

## Expected Behavior
- Generated TS should include:
  - Action methods with public wrappers (if following Python target pattern) or direct implementations callable from state bodies.
  - Class property declarations for all `domain:` fields referenced in code.
  - Correct method signatures for interface entries (e.g., `runtimeMessage(payload: any)`), and state bodies should reference the declared parameters.
- `tsc` successfully compiles the generated file with a minimal `frame_runtime_ts` type shim.

## Actual Behavior
- Emitted `minimal_ts_emit_issues.ts` references `this.handle`, `this.enqueueCommand`, and `this.lifecycle` but none are declared in the class.
- `runtimeMessage` lacks the `payload` parameter while referencing `payload` inside the body.

### Example Errors
```
TS2339: Property 'handle' does not exist on type 'TsEmitIssues'.
TS2304: Cannot find name 'payload'.
TS2339: Property 'lifecycle' does not exist on type 'TsEmitIssues'.
TS2339: Property 'enqueueCommand' does not exist on type 'TsEmitIssues'.
```

## Impact
- Severity: High. Generated TS for non-trivial machines is not compilable with TypeScript.
- Scope: Any TS target using actions and domain fields or interface params.
- Workaround: None (beyond hand-editing generated TS).

## Technical Analysis
The TS emitter already routes per state via a `switch (c.state)` (fix for #073), but originally omitted:
- Emission of action methods and operations in the runnable module class, so state bodies referenced `this.handle`, `this.enqueueCommand`, etc. without declarations.
- Emission of domain fields declared under `domain:`, so fields like `lifecycle`, `last`, and `commandQueue` were undeclared.
- Propagation of interface parameters into handler signatures, so methods like `runtimeMessage(payload)` referenced `payload` without a declared parameter.

## Implemented Solution
- **Domain fields**
  - Added a minimal, SOL-anchored `domain:` scanner in `compile_module_demo` (TypeScript branch).
  - Domain lines in the TS target are recognized in two forms:
    - `var name: Type` or `var name: Type = expr`
    - `name = expr` (implicit `any`-typed field)
  - For each parsed domain variable, the runnable TS module now emits:
    - `public name: Type = expr;` (or `public name: any = expr;` when no explicit type).
- **Actions and operations**
  - For each `BodyKindV3::Action` and `BodyKindV3::Operation` in `actions:`/`operations:`:
    - Extracts `async` and parameter list from the header.
    - Emits a corresponding class method:
      - Sync: `public actionName(params) { … }`
      - Async: `public async actionName(params) { … }`
    - Bodies use the spliced Frame expansions with correct indentation and semicolon insertion to keep TypeScript syntax valid.
  - State handlers calling `this.handle(payload)` or `this.enqueueCommand("x", {})` now resolve to real methods in the class.
- **Interface parameters**
  - Handler grouping logic (one public method per interface function) now:
    - Extracts the parameter list from each handler header (e.g., `runtimeMessage(payload)`).
    - Chooses a non-empty parameter list per interface method and emits:
      - `public runtimeMessage(__e: FrameEvent, compartment: FrameCompartment, payload) { … }`
    - This ensures identifiers like `payload` are declared parameters and no longer produce TS2304.

## Test Coverage
- [x] TS unit/regression test: actions + domain + interface params compile
  - `framec_tests/language_specific/typescript/v3_cli/positive/actions_and_domain_emit_issues.frm`
    - Uses `@tsc-compile` to run `tsc` on the CLI-generated module.
    - `@compile-expect` asserts:
      - Domain fields: `public lifecycle: any =  "idle"`, `public last: any =  ""`, `public commandQueue: any =  []`.
      - Interface handler signature: `public runtimeMessage(__e: FrameEvent, compartment: FrameCompartment, payload)`.
      - Action methods: `public handle(payload)`, `public enqueueCommand(action, data)`, `public flushCommands()`.
- [x] Regression: no duplicate methods per state (still covered by Bug #073 fixture)
  - `framec_tests/language_specific/typescript/v3_cli/positive/multi_state_interface_router.frm`
    - Updated to assert the new `runtimeMessage` signature includes `payload`.

## Repro Shortcuts
- `/tmp/frame_transpiler_repro/bug_074/minimal_ts_emit_issues.frm`
- `/tmp/frame_transpiler_repro/bug_074/run.sh`

## Work Log
- 2025-11-15: Verified with framec v0.86.43; repro compiles with tsc; closing as owner

- 2025-11-15: Initial report with /tmp repro — vscode_editor
- 2025-11-15: Implemented domain field scanning and emission, action/operation methods, and handler parameter propagation; added `@tsc-compile` regression fixtures; fixed in v0.86.43 — framepiler team

## Acceptance Criteria
- /tmp/frame_transpiler_repro/bug_074/run_validate.sh exits 0 (no TypeScript errors).

## Repro Shortcuts (Validator)
- /tmp/frame_transpiler_repro/bug_074/run_validate.sh
