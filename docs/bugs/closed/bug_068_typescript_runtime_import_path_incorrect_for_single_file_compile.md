# Bug #068: TypeScript runtime import path incorrect for single-file compile output

## Metadata
```yaml
bug_number: 068
title: "TypeScript runtime import path incorrect for single-file compile output"
status: Fixed
priority: Medium
category: CodeGen
discovered_version: v0.86.32
fixed_version: v0.86.35
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 2025-11-15
```

## Description
The generated TS file imports the runtime with a relative path that is too deep for single-file compile outputs:

```
import { FrameEvent, FrameCompartment } from '../../../frame_runtime_ts/index'
```

## Reproduction Steps
1. Ensure framec v0.86.34
2. Run: `/tmp/frame_transpiler_repro/bug_068/run.sh`
3. Script prints the generated path and the incorrect import line.

## Validation Assets
- FRM: `/tmp/frame_transpiler_repro/bug_068/minimal_ts_import.frm`
- Script: `/tmp/frame_transpiler_repro/bug_068/run.sh`

## Expected vs Actual
- Expected: package import (`from 'frame_runtime_ts'`) or correct relative path for OUTDIR
- Actual: uses `../../../frame_runtime_ts/index` which does not resolve

## Work Log
- 2025-11-15: Resolved in v0.86.35: non-demo compile emits `from 'frame_runtime_ts'`; test added (`v3_cli/positive/import_runtime_package.frm`). — Codex

---
*Bug tracking policy version: 1.0*
