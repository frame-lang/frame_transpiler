# Bug #068: TypeScript runtime import path incorrect for single-file compile output

## Metadata
```yaml
bug_number: 068
title: "TypeScript runtime import path incorrect for single-file compile output"
status: Open
priority: Medium
category: CodeGen
discovered_version: v0.86.32
fixed_version: 
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 
```

## Description
When compiling a minimal `@target typescript` system with the CLI `compile` command, the generated module imports the runtime with a relative path that is too deep for the output location:

```
import { FrameEvent, FrameCompartment } from '../../../frame_runtime_ts/index'
```

For single-file outputs written to a flat `OUTDIR`, this path is incorrect. The generator should either import from the package name `frame_runtime_ts` or compute the correct relative path based on the actual output directory.

## Reproduction Steps
1. Ensure framec v0.86.32
2. Use FRM: `/tmp/frame_transpiler_repro/bug_068/minimal_ts_import.frm`
3. Compile: `framec compile -l typescript /tmp/frame_transpiler_repro/bug_068/minimal_ts_import.frm -o $(mktemp -d)`
4. Inspect the first lines of the emitted `.ts`; observe the `../../../frame_runtime_ts/index` import.

## Test Case
```frame
@target typescript
system ImportPathDemo {
  actions:
    a(){ /* no-op */ }
}
```

## Expected Behavior
- Import should resolve in the emitted context:
  - Prefer `from 'frame_runtime_ts'` (package import), or
  - Use a correct relative path (e.g., `../frame_runtime_ts/index` when OUTDIR contains the runtime).

## Actual Behavior
- Generated import uses `../../../frame_runtime_ts/index`, which does not exist relative to the single-file OUTDIR.

## Impact
- Severity: Medium — TypeScript compilation fails without manual path edits; blocks Frame-only workflows that rely on the generated file as-is.

## Proposed Solution
- Switch to package import `frame_runtime_ts` for TS outputs (consistent across contexts), or
- Adjust relative path logic to match the actual OUTDIR for `compile` outputs.
- Add tests that verify import resolution with `compile -l typescript`.

## Validation Assets
- FRM: `/tmp/frame_transpiler_repro/bug_068/minimal_ts_import.frm`
- Generated file path printed by the repro script; the first import line demonstrates the issue.

## Related Issues
- Local adapter generation (`rebuild/adapter_protocol.frm`) hits the same import path issue.

## Work Log
- YYYY-MM-DD: Initial report with minimal repro — Codex

---
*Bug tracking policy version: 1.0*
