# Bug #063: Python module compile still emits non-runnable output (regression) — references Bug #060

## Metadata
```yaml
bug_number: 063
title: "Python module compile still emits non-runnable output (regression) — references Bug #060"
status: Resolved
priority: High
category: Tooling
discovered_version: v0.86.28
fixed_version: v0.86.30
reporter: Codex
assignee: Codex
created_date: 2025-11-14
resolved_date: 2025-11-14
```

## Description
A regression caused Python module compile to emit code with inconsistent indentation inside handler bodies, leading to IndentationError on import. This is similar in impact to #060 but rooted in indentation normalization.

## Fix Summary
- Normalize indentation in Python handler method bodies: left-strip each spliced line and re-indent to the method block level. This guarantees consistent indentation regardless of original native spacing.
- Combined with #061/#062:
  - Safe trailer embedding (triple-quoted string at module scope)
  - Errors-JSON only when `--emit-debug` (via FRAME_ERROR_JSON=1)

## Verification
- Compile without `--emit-debug`: module imports cleanly; no trailers present.
- Compile with `--emit-debug`: trailers present and import succeeds; runner extracts sidecars.
- v3_cli (Py) remains green.

---
*Resolution Owner: Codex*
