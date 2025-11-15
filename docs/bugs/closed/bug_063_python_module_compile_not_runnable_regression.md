# Bug #063: Python module compile still emits non-runnable output (regression) — references Bug #060

## Metadata
```yaml
bug_number: 063
title: "Python module compile still emits non-runnable output (regression) — references Bug #060"
status: Resolved
priority: High
category: Tooling
discovered_version: v0.86.28
fixed_version: v0.86.31
reporter: Codex
assignee: Codex
created_date: 2025-11-14
resolved_date: 2025-11-14
```

## Description
A regression caused Python module compile to emit code with inconsistent indentation inside handler bodies in some cases when `--emit-debug` was in use, which led to import failures.

## Fix Summary
- Normalize indentation in Python handler method bodies: left-strip each spliced line and re-indent to the method block level.
- Ensure Python debug trailers (frame-map, visitor-map, debug-manifest, errors-json) are wrapped in triple-quoted blocks at module scope.
- Errors-JSON is emitted only when enabled by `--emit-debug` (FRAME_ERROR_JSON=1).

## Verification
- Compile without `--emit-debug`: module imports cleanly; no trailers present.
- Compile with `--emit-debug`: module imports cleanly; trailers present but safe; runner extracts sidecars.

---
*Resolution Owner: Codex*
