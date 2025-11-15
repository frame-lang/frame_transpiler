# Bug #070: Python module compile drops handlers/actions for complex FRM

## Metadata
```yaml
bug_number: 070
title: "Python module compile drops handlers/actions for complex FRM"
status: Fixed
priority: High
category: CodeGen
discovered_version: v0.86.37
fixed_version: v0.86.38
reporter: Codex
assignee: 
created_date: 2025-11-15
resolved_date: 
```

## Description
Compiling a moderately complex module FRM (adapter/runtime harness) yields a skeleton Python module with only the class boilerplate and no interface handlers or `def _action_*` methods. This breaks unit tests that expect `_action_handleCommand` and basic handler emission.

## Reproduction Steps
1. Use the provided FRM copied from the rebuild harness:
   - `/tmp/frame_transpiler_repro/bug_070/runtime_protocol.frm`
2. Run the script:
   - `/tmp/frame_transpiler_repro/bug_070/run.sh`
3. Observe output prints `BUG_REPRODUCED` and shows the generated `.py` with no handlers/actions.

## Validation Assets
- FRM: `/tmp/frame_transpiler_repro/bug_070/runtime_protocol.frm`
- Script: `/tmp/frame_transpiler_repro/bug_070/run.sh`

## Expected vs Actual
- Expected: Generated Python module includes interface handlers and `_action_*` methods per actions section.
- Actual: Only skeleton methods are emitted.

## Impact
- Severity: High — blocks Frame-only generation for runtime validation; unit tests cannot execute without handlers.

## Technical Analysis
- The FRM includes async actions, event-loop helpers, and multiple actions. Generator may be failing to emit handlers when certain async/try/except patterns combine with domain declarations.
- Minimal reproduction still pending; current repro uses the harness FRM. A smaller FRM can be derived if needed.

## Proposed Solution
- Ensure codegen emits interface handlers and action methods regardless of action content complexity.
- Add tests to assert presence of emitted `_action_*` methods and interface handlers for module FRMs.

## Work Log
- 2025-11-15: Initial report with /tmp repro — Codex
- 2025-11-15: Allow bare IDENT headers under interface: in OutlineScanner; added interface handler emission test; validated; marking Fixed (awaiting closure). — Codex

---
*Bug tracking policy version: 1.1*
