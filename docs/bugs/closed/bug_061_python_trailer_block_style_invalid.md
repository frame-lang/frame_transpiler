# Bug #061: Python compile appends C-style trailer blocks that break Python syntax

## Metadata
```yaml
bug_number: 061
title: "Python compile appends C-style trailer blocks that break Python syntax"
status: Resolved
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
Python compile previously appended debug trailers using C-style block markers directly (/*#name# … #name#*/), which is invalid Python and could cause syntax/indentation errors on import.

## Fix Summary
- For Python target, trailers are now embedded inside a harmless triple-quoted string at module scope using the existing sentinel markers inside the string literal. The runner still detects and strips the trailer content reliably.
- Errors-JSON emission in module compile is now flag-gated by `FRAME_ERROR_JSON`, preventing unintended trailers.

## Verification
- Compiled Python modules import successfully without `--emit-debug` (no trailers).
- With `--emit-debug`, trailers are present and do not break import; runner extracts and strips to sidecars.
- v3_cli suites remain green.

---
*Resolution Owner: Codex*
