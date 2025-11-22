# Bug #083: V3 TypeScript — stopped event does not set isPaused in generated output

## Metadata
bug_number: 083
title: V3 TypeScript — stopped event does not set isPaused in generated output
status: Fixed
priority: High
category: CodeGen
discovered_version: v0.86.51
fixed_version: v0.86.53
reporter: vscode_editor (Codex)
assignee: framepiler team
created_date: 2025-11-19
resolved_date: 2025-11-19

## Description
Routing a 'stopped' event via runtimeMessage(payload) with event: 'stopped' does not set isPaused to true in the generated class (observed failure in unit tests). Expect: isPaused === true after 'stopped', matching FRM action logic.

## Reproduction Steps
1) Ensure framec is v0.86.51.
2) Run /tmp/frame_transpiler_repro/bug_083/run_validate.sh.

## Build/Release Artifacts
- framec binary used for validation:
  `/Users/marktruluck/projects/frame_transpiler/target/release/framec`
- External validator harness:
  `/tmp/frame_transpiler_repro/bug_083/run_validate.sh` (compiles AdapterProtocol
  FRM to TypeScript/JS and runs Node assertions).

## Repro Shortcuts
- /tmp/frame_transpiler_repro/bug_083/run_validate.sh

## Work Log
- 2025-11-19: Filed with minimal FRM and validator — vscode_editor
 - 2025-11-19: Verified against a minimal in-repo fixture (AdapterProtocolMinimal) and the shared `framepiler_test_env/adapter_protocol` harness that `runtimeMessage` with `event: 'stopped'` sets `isPaused === true` and updates stopped metadata in the generated TS. The external validator currently fails earlier on TypeScript environment issues (`frame_runtime_ts` module and Node typings), not on isPaused semantics. Bug is therefore fixed in v0.86.53 from the compiler’s perspective; external tests should either reuse the shared harness or add `frame_runtime_ts` and Node typings to their TS setup.
