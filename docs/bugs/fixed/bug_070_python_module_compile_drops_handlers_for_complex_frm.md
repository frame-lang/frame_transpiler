# Bug #070: Python module compile drops handlers/actions for complex FRM

## Metadata
```yaml
bug_number: 070
title: "Python module compile drops handlers/actions for complex FRM"
status: Fixed
priority: High
category: CodeGen
discovered_version: v0.86.37
fixed_version: v0.86.39
reporter: Codex
assignee: 
created_date: 2025-11-15
resolved_date: 2025-11-15
```

## Description
Compiling a moderately complex module FRM (adapter/runtime harness) previously yielded a skeleton Python module with only the class boilerplate and no interface handlers or `def _action_*` methods. This blocked debugger/runtime harness tests that expect `_action_handleCommand` and other action methods.

## Root Cause
- The Python import scanner (`ImportScannerPyV3`) scanned from the prolog through the entire file and treated any SOL `import` or `from` statement as a module import, even when it appeared inside the `actions:` section.
- Module partitioning sets the outline start index to the end of the last import span. Because the last `import` in `runtime_protocol.frm` is inside the `actions:` section, the outline scan started near the `domain:` block and skipped all `machine:` and `actions:` headers.
- As a result, `ModulePartitionerV3` produced an empty `bodies` list for this FRM, and the Python module emitter only generated the runtime skeleton methods.

## Fix
- Updated `ImportScannerPyV3` to stop scanning imports once it hits a V3 section or `system` header (`system`, `machine`, `interface`, `actions`, `operations`, `domain`), so imports inside `actions:` no longer affect the outline start.
- Ensured the outline scanner recognizes handlers and actions correctly for the harness FRM, and that the Python emitter always generates handler and `_action_*` methods for those bodies.

## How to Validate
- Repro assets:
  - FRM: `/tmp/frame_transpiler_repro/bug_070/runtime_protocol.frm`
  - Scripts:
    - `/tmp/frame_transpiler_repro/bug_070/run_check_handlers.sh`
- Command:
  - `FRAMEC_BIN=./target/release/framec bash /tmp/frame_transpiler_repro/bug_070/run_check_handlers.sh`
- Expected:
  - Script reports that handlers/actions are present (no `BUG_REPRODUCED`), and the generated `runtime_protocol.py` contains at least one `def run(` handler and multiple `_action_*` methods.

## Work Log
- 2025-11-15: Initial v0.86.38 fix handled simple interface/actions cases but not the full harness FRM; bug was reopened by debugger team.
- 2025-11-15: Tightened Python import scanner boundary and outline scanning; confirmed harness FRM emits handlers and actions with v0.86.39 and marked as Fixed. — Codex

---
*Bug tracking policy version: 1.1*

