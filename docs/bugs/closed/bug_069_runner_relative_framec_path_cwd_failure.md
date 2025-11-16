# Bug #069: Runner fails when CWD changes (relative framec path)

## Metadata
```yaml
bug_number: 069
title: "Runner fails when CWD changes (relative framec path)"
status: Closed
priority: Medium
category: Tooling
discovered_version: v0.86.35
fixed_version: v0.86.37
reporter: Debugger Team
assignee: Codex
created_date: 2025-11-15
resolved_date: 2025-11-15
```

## Description
When test fixtures request `@cwd: tmp`, the runner changes the working directory for `framec` invocations. If the runner passes a relative binary path (e.g., `./target/release/framec`), subprocess execution fails with `FileNotFoundError` because the binary path is resolved relative to the temporary cwd.

## Resolution
- Runner absolutizes the `framec` path whenever changing `cwd` in v3_cli/v3_cli_project routes.
- Added @cwd parity fixtures for TS/Py/Rust CLI and project modes.

## Validation
- Validate via runner `validate()` path for v3_cli fixtures with `@cwd: tmp` and a relative `--framec`.
- Confirm: validation succeeds without `FileNotFoundError`.

## Work Log
- 2025-11-15: Implemented absolute path fix and validated on v0.86.37 — Codex

---
*Bug tracking policy version: 1.1*

