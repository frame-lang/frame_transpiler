# Bug #062: Python compile appends trailers when --emit-debug is not set

## Metadata
```yaml
bug_number: 062
title: "Python compile appends trailers when --emit-debug is not set"
status: Closed
priority: High
category: Tooling
discovered_version: v0.86.28
fixed_version: v0.86.29
reporter: Codex
assignee: Codex
created_date: 2025-11-14
resolved_date: 2025-11-14
```

## Description
Compilation without `--emit-debug` could still include trailer blocks due to unconditional errors-json emission in the module compile path.

## Fix Summary
- Errors-JSON trailer in module compile is now guarded by `FRAME_ERROR_JSON=1`. The CLI only sets this var when `--emit-debug` (or an equivalent debug flag) is provided.
- No trailers are appended unless explicitly enabled by flags / env vars.

## Verification
- `framec compile -l python_3 file.frm -o outdir` emits runnable code with no trailers; imports cleanly.
- `framec compile -l python_3 --emit-debug file.frm -o outdir` emits runnable code with trailers embedded safely; runner extracts them.

---
*Resolution Owner: Codex*
