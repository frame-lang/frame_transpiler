# Bug #075: TypeScript generator redeclares `nextCompartment` in switch cases (TS2451)

## Metadata
```yaml
bug_number: 075
title: "TypeScript generator redeclares nextCompartment in switch cases (TS2451)"
status: Closed
priority: High
category: CodeGen
discovered_version: v0.86.43
fixed_version: v0.86.45
reporter: vscode_editor (Codex)
assignee: framepiler team
created_date: 2025-11-15
resolved_date: 2025-11-15
```

## Description
For interface handlers that route across multiple states, the TS backend emits a single class method with a `switch (c.state)` and, in each case that performs a transition, emits a temporary `const nextCompartment = ...` then calls `_frame_transition(nextCompartment)`.

Because `case` labels share the same block scope in TypeScript (without braces), redeclaring the same `const` in multiple cases produced `TS2451: Cannot redeclare block-scoped variable` and the module failed to compile.

## Reproduction Steps
1. Use framec v0.86.43–0.86.44.
2. Run: `/tmp/frame_transpiler_repro/bug_075/run.sh`
3. Observe TS2451 errors.

## Test Case
```frame
@target typescript
system RedeclareVar {
  interface:
    runtimeDisconnected()
  machine:
    $A { runtimeDisconnected() { -> $B } }
    $B { runtimeDisconnected() { -> $Terminated } }
    $Terminated { runtimeDisconnected() { /* ignore */ } }
}
```

## Implemented Fix (by framepiler)
- Wrap each `case` in its own block or inline the transition so no `const` is redeclared across cases.

## Acceptance Criteria
- Validator script succeeds with no TypeScript errors:
  - `/tmp/frame_transpiler_repro/bug_075/run_validate.sh` exits 0

## Repro Shortcuts
- `/tmp/frame_transpiler_repro/bug_075/minimal_redeclare_next_compartment.frm`
- `/tmp/frame_transpiler_repro/bug_075/run.sh`
- `/tmp/frame_transpiler_repro/bug_075/run_validate.sh`

## Work Log
- 2025-11-15: Initial report with /tmp repro — vscode_editor
- 2025-11-15: Reopened on v0.86.44; still reproduces (TS2451); added run_validate.sh — vscode_editor
- 2025-11-15: Verified with v0.86.45; run_validate.sh exited 0; closing as owner — vscode_editor
