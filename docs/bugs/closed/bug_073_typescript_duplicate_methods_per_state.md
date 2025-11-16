# Bug #073: TypeScript generator emits duplicate class methods per state

## Metadata
```yaml
bug_number: 073
title: "TypeScript generator emits duplicate class methods per state"
status: Closed
priority: High
category: CodeGen
discovered_version: v0.86.41
fixed_version: v0.86.42
reporter: vscode_editor (Codex)
assignee: framepiler team
created_date: 2025-11-15
resolved_date: 2025-11-15
```

## Description
When compiling a valid multi-state Frame spec targeting TypeScript, the emitted class contains duplicate method declarations (one per state) for the same interface methods (e.g., `start()` and `runtimeMessage(payload)`). In TypeScript, duplicate method declarations inside a class body are illegal and produce compiler errors, making the generated code unusable.

This also surfaces missing property declarations (e.g., `commandQueue`, `handshakeComplete`) and unbound identifiers (e.g., `payload`), indicating the TS backend is not emitting support fields and action definitions alongside state handlers.

## Reproduction Steps
1. Use framec v0.86.41.
2. Run the following script which generates TS from a minimal FRM and attempts to compile it with `tsc`:
   - Repro path: `/tmp/frame_transpiler_repro/bug_073/`
   - Command: `bash /tmp/frame_transpiler_repro/bug_073/run.sh`

## Test Case
```frame
@target typescript

system DemoTsGen {
  interface:
    start()
    runtimeMessage(payload)

  machine:
    $A {
      start() { /* no-op */ }
      runtimeMessage(payload) { /* ignore */ }
    }
    $B {
      start() { /* still no-op */ }
      runtimeMessage(payload) { /* also ignore */ }
    }

  actions:
    // no actions required for this duplication bug
}
```

## Expected Behavior
- Emitted TS should contain a single class with a state router that dispatches based on an internal compartment/state field (like the Python target), not multiple identical class-level method declarations.
- Class should compile with `tsc` without syntax errors.

## Actual Behavior
- Generated `ts_dup_methods.ts` contains duplicate method declarations for `start` and `runtimeMessage` (one per state), unbound `payload` references, and missing property declarations; `tsc` reports multiple errors including TS2393 (Duplicate function implementation).

```
$ framec --version
framec 0.86.41
$ ./run.sh
...
rebuild/adapter_protocol.ts(9,10): error TS2393: Duplicate function implementation.
...
```

## Impact
- Severity: High (generated TS cannot be compiled or used)
- Scope: Any TypeScript target with multiple states sharing the same interface methods
- Workaround: None (aside from hand-writing TS or using a non-TS target)

## Technical Analysis
Likely the TS backend is emitting per-state method bodies as separate class methods instead of routing within a single method based on state/compartment. Additionally, domain fields and action methods referenced by state handlers are not emitted before use.

### Root Cause
TS codegen does not mirror the runtime/compartment routing pattern used in other targets (e.g., Python); it duplicates interface methods per state, and fails to emit required fields and action wrappers.

### Affected Files
- `framec/src/frame_c/v3/mod.rs` (TypeScript runnable module emitter)
- `framec_tests/runner/frame_test_runner.py` (tsc-backed CLI fixtures via `@tsc-compile`)
- `node_modules/frame_runtime_ts/index.d.ts` (local package shim for `import 'frame_runtime_ts'`)

## Proposed Solution
- Emit a single class with:
  - A `_frame_router(__e, c)` that switches on compartment/state.
  - Interface methods that forward to the router.
  - Properly emitted domain fields and action wrappers before use.
- Ensure identifiers like `payload` are declared as parameters where required.

## Test Coverage
- [x] Unit test: TS target multi-state interface method
  - `framec_tests/language_specific/typescript/v3_cli/positive/multi_state_interface_router.frm`
- [x] Regression test: verify no duplicate method names, class compiles with `tsc`
  - Uses `@tsc-compile` (toolchain-gated with `@skip-if: tsc-missing`) on the CLI-generated module.

## Related Issues
- Bug #068 – TS runtime import path (previous TS target issue, now closed)

## Work Log
- 2025-11-15: Verified with framec v0.86.42; /tmp repro compiles with tsc; closing as owner.

- 2025-11-15: Initial report with /tmp repro – vscode_editor
- 2025-11-15: Implemented grouped handler emission (one public method per interface function with state-based routing) and added a tsc-backed CLI regression fixture; fixed in v0.86.42 – framepiler team

## Resolution
- The TypeScript runnable module emitter now:
  - Groups state handlers by interface method name and emits a single public method per interface function.
  - Dispatches on `c.state` inside each method (using compiled state IDs like `__System_state_A`) instead of emitting one method per state.
  - Reuses spliced Frame expansions inside each case block, preserving indentation and terminal semantics.
- The test runner gained `@tsc-compile` support (with `@skip-if: tsc-missing`) to validate that generated TS compiles cleanly with `tsc` for CLI modules.
