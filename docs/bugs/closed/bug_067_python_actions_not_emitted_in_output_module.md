# Bug #067: Python codegen omits `actions:` methods in output module (no `def _action_*`)

## Metadata
```yaml
bug_number: 067
title: "Python codegen omits actions methods in output module (no def _action_*)"
status: Closed
priority: High
category: CodeGen
discovered_version: v0.86.31
fixed_version: v0.86.35
assignee: 
created_date: 2025-11-14
resolved_date: 2025-11-15
```

## Description
The generated Python module lacks methods for `actions:` entries (e.g., `def _action_helper`). Only interface handlers are present, preventing test harnesses from invoking action helpers.

## Reproduction Steps
1. Ensure framec v0.86.34
2. Run: `/tmp/frame_transpiler_repro/bug_067/run.sh`
3. Script reports missing `_action_` methods and prints the generated file path.

## Validation Assets
- FRM: `/tmp/frame_transpiler_repro/bug_067/minimal_actions_missing.frm`
- Script: `/tmp/frame_transpiler_repro/bug_067/run.sh`

## Expected vs Actual
- Expected: `def _action_<name>` methods emitted for actions
- Actual: Only interface handler methods present

## Work Log
- 2025-11-15: Verified fix in v0.86.35 (action methods emitted)

---
*Bug tracking policy version: 1.0*
framec 0.86.35
warning: frame_runtime_py not found at "frame_runtime_py"; set FRAME_RUNTIME_PY_DIR to override
/var/folders/kl/kn9ds0z918n0bhzr9w40xlt00000gn/T/tmp.EsZfFOmmI3/minimal_actions_missing.py
BUG_REPRODUCED: actions missing in generated module
/var/folders/kl/kn9ds0z918n0bhzr9w40xlt00000gn/T/tmp.EsZfFOmmI3/minimal_actions_missing.py
"""

## How to Validate
- Quick: `FRAMEC_BIN=/Users/marktruluck/projects/frame_transpiler/target/release/framec bash /tmp/frame_transpiler_repro/bug_067/run.sh`
  - Expect: `BUG_REPRODUCED: actions missing in generated module`
- Full: `/tmp/frame_transpiler_repro/bug_067/run_validate.sh`
  - Case1-minimal: expect no `def _action_*` emitted
  - Case2-multiple: expect no `def _action_*` emitted
