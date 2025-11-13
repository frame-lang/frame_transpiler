# Bug #057: Inconsistent source map schema across languages

## Metadata
```yaml
bug_number: 057
title: "Inconsistent source map schema across languages"
status: Open
priority: Medium
category: Tooling
discovered_version: v0.86.25
fixed_version: 
reporter: Codex
assignee: 
created_date: 2025-11-13
resolved_date: 
```

## Description
The debug output schema differs by language: Python examples/documentation use `pythonLine` in `sourceMap.mappings`, whereas native backend specs use `targetLine`. This inconsistency complicates cross‑language tooling (e.g., IDEs, debuggers) that aim to consume a uniform schema.

## Reproduction Steps
1. Compare the Python integration doc vs. native source map spec:
   - `/docs/vscode_dap_integration.md` (uses `pythonLine`)
   - `/docs/framepiler_design/going_native/source_map_spec.md` (uses `targetLine`)
2. Attempt to write a single mapping consumer for both.

## Test Case
```json
// Python‑centric example (current)
{
  "sourceMap": {
    "mappings": [
      { "frameLine": 3, "pythonLine": 12 }
    ]
  }
}

// Native spec example (current)
{
  "version": 1,
  "mappings": [
    { "sourceLine": 3, "targetLine": 12, "kind": "MirTransition" }
  ]
}
```

## Expected Behavior
- Single, language‑agnostic field: `targetLine` (and optional `targetColumn`) for all targets, including Python and TypeScript.

## Actual Behavior
- Python docs/tools prefer `pythonLine`; native spec defines `targetLine`.

## Impact
- **Severity**: Medium — adds branching logic and risk of drift.
- **Scope**: All consumers of `--debug-output` across targets.
- **Workaround**: Support both keys in consumers.

## Technical Analysis
- Documentation drift and/or emitter differences between visitors.

### Root Cause
- Lack of a single canonical target‑line field name in shared code/docs.

### Affected Files
- `docs/vscode_dap_integration.md`
- `docs/framepiler_design/going_native/source_map_spec.md`
- JSON emitters in the Python/TypeScript visitors (paths vary under `framec/src/frame_c/...`).

## Proposed Solution
- Standardize on `targetLine` across all languages.
- For compatibility, emit both fields for one minor cycle (Python/TS emit `targetLine` and legacy `pythonLine`/`tsLine` or keep the language‑named field as an alias).
- Update docs and sample code accordingly.

## Test Coverage
- [ ] Unit tests for mapping emitters ensure `targetLine` is present
- [ ] Update fixtures/examples in docs to the unified schema

## Related Issues
- Bug #056 – JSON envelope for error paths

## Work Log
- 2025-11-13: Initial report — Codex

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
