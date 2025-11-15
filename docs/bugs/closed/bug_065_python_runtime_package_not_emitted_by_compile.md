# Bug #065: Python runtime package not emitted by `compile` (frame_runtime_py missing)

## Metadata
```yaml
bug_number: 065
title: "Python runtime package not emitted by `compile` (frame_runtime_py missing)"
status: Closed
priority: High
category: Tooling
discovered_version: v0.86.31
fixed_version: v0.86.33
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 2025-11-15
```

## Resolution
- The CLI `compile -l python_3 -o OUTDIR` now copies the `frame_runtime_py` package into `OUTDIR/frame_runtime_py` (override via `FRAME_RUNTIME_PY_DIR`). This ensures generated modules import and run without external packaging steps.

