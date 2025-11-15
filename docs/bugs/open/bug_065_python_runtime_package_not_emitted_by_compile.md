# Bug #065: Python runtime package not emitted by `compile` (frame_runtime_py missing)

## Metadata
```yaml
bug_number: 065
title: "Python runtime package not emitted by `compile` (frame_runtime_py missing)"
status: Resolved
priority: High
category: Tooling
discovered_version: v0.86.31
fixed_version: v0.86.35
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 2025-11-15
```

## Description
When using the CLI `compile -l python_3` to generate a Python module, the expected `frame_runtime_py` package is not emitted alongside the outputs. The generated module imports `from frame_runtime_py import FrameEvent, FrameCompartment`, causing `ModuleNotFoundError` unless a runtime is provided on `PYTHONPATH` or `FRAME_RUNTIME_PY_DIR` is set.

## Reproduction Steps
1. Ensure framec v0.86.34
2. Run: `/tmp/frame_transpiler_repro/bug_065/run_check.sh`
3. Observe `PKG_MISSING: frame_runtime_py not emitted next to outputs`.

## Validation Assets
- FRM: `/tmp/frame_transpiler_repro/bug_065/minimal_runtime_pkg.frm`
- Script: `/tmp/frame_transpiler_repro/bug_065/run_check.sh`

## Expected vs Actual
- Expected: runtime package emitted next to outputs or embedded fallback
- Actual: package not emitted; import fails without manual setup

## Work Log
- 2025-11-15: Verified fix in v0.86.35: `compile -o` and `compile-project` copy `frame_runtime_py` next to outputs; runner v3_cli_project passes. — Codex

---
*Bug tracking policy version: 1.0*
