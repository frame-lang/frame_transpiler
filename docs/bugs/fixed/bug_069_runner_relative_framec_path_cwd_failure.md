# Bug #069: Runner fails when CWD changes (relative framec path)

## Metadata
```yaml
bug_number: 069
title: "Runner fails when CWD changes (relative framec path)"
status: Fixed
priority: Medium
category: Tooling
discovered_version: v0.86.35
fixed_version: v0.86.37
reporter: Debugger Team
assignee: Codex
created_date: 2025-11-15
resolved_date: 

## Description
When test fixtures request `@cwd: tmp`, the runner changes the working directory for `framec` invocations. If the runner passes a relative binary path (e.g., `./target/release/framec`), subprocess execution fails with `FileNotFoundError` because the binary path is resolved relative to the temporary cwd.

## Reproduction Steps
1. Use a v3_cli or v3_cli_project fixture with `@cwd: tmp`.
2. Run with `--framec ./target/release/framec`.
3. Observe `FileNotFoundError: [Errno 2] No such file or directory: './target/release/framec'`.

## Expected vs Actual
- Expected: Runner should execute `framec` regardless of cwd.
- Actual: Relative binary path breaks when cwd is changed.

## Technical Analysis
- Runner builds `cmd = [self.config.framec_path, ...]` but does not absolutize it before `subprocess.run(...)` when setting `cwd`.
- Fix: absolutize `cmd[0]` in v3_cli, v3_cli_project, and general V3 transpile/validate paths.

## Resolution
- Runner now absolutizes `cmd[0]` across all execution branches that use cwd changes.
- Added @cwd parity fixtures for TS/Py/Rust CLI and project modes.

## Work Log
- 2025-11-15: Bug filed by Debugger Team.
- 2025-11-15: Implemented absolute path fix; tests added; marking Fixed (awaiting closure by filer).

---
*Bug tracking policy version: 1.1*
