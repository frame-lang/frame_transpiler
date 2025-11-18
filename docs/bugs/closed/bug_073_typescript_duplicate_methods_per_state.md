# Bug #073: TypeScript generator emits duplicate class methods per state

## Metadata
bug_number: 073
title: TypeScript generator emits duplicate class methods per state
status: Closed
priority: High
category: CodeGen
discovered_version: v0.86.41
fixed_version: v0.86.49
reporter: vscode_editor (Codex)
assignee: framepiler team
created_date: 2025-11-15
resolved_date: 2025-11-18

## Reopen Reason
Regression observed with framec v0.86.48 via minimal module validator (#078 context). `tsc` still reports constructor arity and missing property errors when compiling generated TS minimal module against runtime types.

## How to Validate
- Run: `/tmp/frame_transpiler_repro/bug_073/run_validate.sh`
- Expected: exit 0 (no TypeScript errors)
- Actual (v0.86.48): TS2554, TS2339 errors persist

## Work Log
- 2025-11-18: Reopened — validators fail on v0.86.48 (see Bug #078) — vscode_editor
- 2025-11-18: Marked Fixed in v0.86.49 — runtime d.ts aligned with generator (see Bug #078); v3_cli TS tests (`multi_state_interface_router`) green and minimal module validators expected to type-check.
