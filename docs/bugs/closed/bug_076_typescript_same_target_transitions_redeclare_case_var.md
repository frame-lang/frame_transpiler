# Bug #076: TS generator redeclares case-scoped transition var when multiple cases target the same state

## Metadata
```yaml
bug_number: 076
title: "TS generator redeclares case-scoped transition var when multiple cases target the same state"
status: Closed
priority: High
category: CodeGen
discovered_version: v0.86.45
fixed_version: v0.86.45
reporter: vscode_editor (Codex)
assignee: framepiler team
created_date: 2025-11-15
resolved_date: 2025-11-15
```

## Resolution
Validated fix with v0.86.45. Repro:
- /tmp/frame_transpiler_repro/bug_076/run.sh (generates adapter_protocol.ts)
- tsc adapter_protocol.ts — compiles with no TS2451 errors

## Repro Shortcuts
- /tmp/frame_transpiler_repro/bug_076/adapter_protocol.frm
- /tmp/frame_transpiler_repro/bug_076/run.sh
- /tmp/frame_transpiler_repro/bug_076/adapter_protocol.ts

## Work Log
- 2025-11-15: Verified fix; closing as owner — vscode_editor
- /tmp/frame_transpiler_repro/bug_076/run_validate.sh

## Acceptance Criteria
- Running /tmp/frame_transpiler_repro/bug_076/run_validate.sh exits 0 and produces no TypeScript errors.
