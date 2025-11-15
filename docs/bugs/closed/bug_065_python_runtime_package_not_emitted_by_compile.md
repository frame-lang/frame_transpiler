# Bug #065: Python runtime package not emitted by `compile` (frame_runtime_py missing)

## Metadata
```yaml
bug_number: 065
title: "Python runtime package not emitted by `compile` (frame_runtime_py missing)"
status: Closed
priority: High
category: Tooling
discovered_version: v0.86.31
fixed_version: v0.86.32
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 2025-11-15
```

## Resolution
- The CLI `compile -l python_3 -o OUTDIR` now copies the `frame_runtime_py` package into `OUTDIR/frame_runtime_py` (override via `FRAME_RUNTIME_PY_DIR`). This ensures generated modules import and run without external packaging steps.

## Validation
- Added/used v3_cli fixtures to import/execute compiled modules.
- Manually verified `framec compile -l python_3 --emit-debug file.frm -o outdir` emits `outdir/frame_runtime_py/` and the module imports successfully.

## Notes
- No changes needed for demo paths; release path is compile/compile-project.

