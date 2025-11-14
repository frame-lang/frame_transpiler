# Bug #059: Enable full module codegen for TS/Py (non-demo) and restore CLI compile path

## Metadata
```yaml
bug_number: 059
title: "Enable full module codegen for TS/Py (non-demo) and restore CLI compile path"
status: Open
priority: Critical
category: Tooling
discovered_version: v0.86.26
fixed_version: 
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 
```

## Description
With v0.86.26, the `framec` CLI in this environment appears to be built in V3 "demo" mode. It supports only `demo-*` subcommands and rejects direct compilation of full `.frm` modules used by the VS Code extension (Frame Debug Adapter and Python Debug Runtime specs). This blocks regeneration of adapter/runtime from their `.frm` sources.

Symptoms:
- Running `framec -l python_3 <spec.frm>` fails with: `V3 demo expects body starting at '{' (single-body debug mode)`.
- `demo-frame` accepts tiny `@target` fixtures but not real module specs (multiple states/handlers, enter handlers, actions/operations).
- `--emit-debug` did not embed debug trailers (errors-json, frame-map, visitor-map, debug-manifest) in this build, even when env flags were set. (May be feature-gated; please confirm expected behavior.)

## Reproduction Steps
1. Use the VS Code extension repo specs:
   - `/Users/marktruluck/vscode_editor/src/debug/state_machines/FrameDebugAdapter.frm`
   - `/Users/marktruluck/vscode_editor/src/debug/state_machines/PythonDebugRuntime.frm`
2. Attempt compilation:
   - `framec -l typescript /Users/marktruluck/vscode_editor/src/debug/state_machines/FrameDebugAdapter.frm`
   - `framec -l python_3 /Users/marktruluck/vscode_editor/src/debug/state_machines/PythonDebugRuntime.frm`
3. Observe error: `V3 demo expects body starting at '{' (single-body debug mode)`.
4. Optional: For small demo fixtures with `demo-frame`, try `--emit-debug` + env:
   - `FRAME_ERROR_JSON=1 FRAME_MAP_TRAILER=1 FRAME_DEBUG_MANIFEST=1 FRAME_NATIVE_SYMBOL_SNAPSHOT=1 FRAME_EMIT_EXEC=1 framec demo-frame --emit-debug -l python_3 <fixture.frm>`
   - In this environment, no trailer markers were embedded; confirm build feature flags.

## Expected Behavior
- Provide a non-demo compile path that accepts full `.frm` modules (multi-state, event handlers and enter/exit handlers) and generates:
  - TypeScript for the adapter
  - Python for the runtime
  - Debug artifacts when requested (`--emit-debug` trailers/sidecars; or a documented stdout JSON mode for compatibility)
- Restore/introduce a stable CLI that works with existing automation:
  - Either `framec -l <lang> <file.frm>` (legacy behavior), or
  - A `build`/`compile` subcommand that handles full module codegen.

## Actual Behavior
- CLI rejects full module input with demo-only constraints; only `demo-*` subcommands work on single-body fixtures.
- Debug trailers not observed via `--emit-debug` in this local build.

## Impact
- **Severity**: Critical — blocks regenerating the Frame Debug Adapter and Python Runtime from source `.frm` files in the editor project.
- **Scope**: TS/Py codegen path and debugger artifact consumption (trailers/sidecars).
- **Workaround**: Continue using legacy generated files and a stdio JSONL harness for connectivity; cannot advance adapter/runtime rebuild.

## Technical Analysis
- CLI help shows only `demo-*` commands in this build. The direct compile path (`framec -l <lang> <file.frm>`) errors out with a single-body demo guard.
- Release notes (`v0.86.26`) indicate debugger artifacts are ready behind `--emit-debug`; this binary did not embed trailer markers, suggesting feature flags or a different build profile.

## Proposed Solution
- Expose a non-demo codegen path for TS/Py in V3:
  - Option A: Restore direct compile: `framec -l typescript <file.frm>` and `framec -l python_3 <file.frm>` for full modules.
  - Option B: Add `build`/`compile` subcommand for full module codegen with output to stdout or `-o` directory.
- Ensure `--emit-debug` is functional in the non-demo path (embed trailers and/or write sidecars).
- Maintain backward compatibility for tools that consume `--debug-output` stdout JSON (document deprecation plan if moving to trailers-only).
- Document build features/flags required to enable trailers in local builds.

## Test Coverage
- [ ] CLI compile full module (TS/Py) generates code successfully for complex specs (states, enter/exit, actions).
- [ ] `--emit-debug` produces errors-json, frame-map, visitor-map (module path), and debug-manifest.
- [ ] Integration: compile `FrameDebugAdapter.frm` and `PythonDebugRuntime.frm` in CI and verify artifacts.

## Related Issues
- Release notes v0.86.26 claim debugger artifacts ready; align CLI with docs.

## Work Log
- 2025-11-14: Initial report — Codex

## Resolution
_Pending._

### Fix Summary
_Pending._

### Verification
_Pending._

### Lessons Learned
_Pending._

---
*Bug tracking policy version: 1.0*
