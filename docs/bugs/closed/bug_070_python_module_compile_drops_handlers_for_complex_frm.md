# Bug #070: Python module compile drops handlers/actions for complex FRM

## Metadata
```yaml
bug_number: 070
title: "Python module compile drops handlers/actions for complex FRM"
status: Closed
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
- Module partitioning sets the outline start index to the end of the last import span. Because the last `import` in `runtime_protocol.frm` is inside the `actions:` section (e.g., `import sys` in `log(message)`), the outline scan started near the `domain:` block and skipped all `machine:` and `actions:` headers.
- As a result, `ModulePartitionerV3` produced an empty `bodies` list for this FRM, and the Python module emitter only generated the runtime skeleton methods.

## Fix
- **Import scanner boundary:**
  - Updated `ImportScannerPyV3` to stop scanning imports once it encounters a V3 section or module header:
    - `system`, `machine`, `interface`, `actions`, `operations`, or `domain`.
  - This ensures that only true top-level imports (before `system`) are treated as module imports; `import` statements inside `actions:` bodies are no longer consumed by the import scanner.
- **Outline scanner async support:**
  - Enhanced `OutlineScannerV3` to recognize `async` function headers inside `machine:` and `actions:` sections:
    - Supports both `run()` and `async run()` inside state blocks.
    - Supports `async` action headers like `async runtimeMain()`, `async handleCommand(message)`, etc.
  - Ensures the outline scan produces `BodyKindV3::Handler` entries for state handlers and `BodyKindV3::Action` entries for all action bodies in the harness FRM.
- **Module emit:**
  - With correct partitions, the Python module emitter now:
    - Emits state handlers such as `def run(self, __e: FrameEvent, compartment: FrameCompartment):` for `$Idle` and `$Terminated`.
    - Emits `_action_*` methods for each action body (`_action_runtimeMain`, `_action_handleCommand`, etc.), preserving the original body content.

## Validation Assets
- FRM: `/tmp/frame_transpiler_repro/bug_070/runtime_protocol.frm`
- Scripts:
  - `/tmp/frame_transpiler_repro/bug_070/run.sh`
  - `/tmp/frame_transpiler_repro/bug_070/run_check_handlers.sh`

## How to Validate
- Quick smoke:
  - `FRAMEC_BIN=/Users/marktruluck/projects/frame_transpiler/target/release/framec bash /tmp/frame_transpiler_repro/bug_070/run_check_handlers.sh`
  - Expected: script exits non-zero with a line containing `Unexpected success: handlers/actions present`, and the printed `runtime_protocol.py` includes both `def run(` and multiple `_action_*` methods.
- Manual inspection:
  - `FRAMEC_BIN=./target/release/framec bash /tmp/frame_transpiler_repro/bug_070/run_check_handlers.sh || true`
  - Open the generated `runtime_protocol.py` and confirm:
    - Two `run` handlers (for `$Idle` and `$Terminated`), and
    - `_action_runtimeMain`, `_action_handleCommand`, `_action_sendOutput`, etc.

## Work Log
- 2025-11-15: Initial v0.86.38 fix handled simpler interface/actions cases but not the full harness FRM; bug was reopened by the debugger team.
- 2025-11-15: Tightened Python import scanner boundary and OutlineScanner async handling; revalidated with `run_check_handlers.sh` and confirmed handlers/actions are emitted. Tagged in v0.86.39. — Codex

---
*Bug tracking policy version: 1.1*

