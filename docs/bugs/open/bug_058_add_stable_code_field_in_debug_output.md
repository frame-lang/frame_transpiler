# Bug #058: Add stable top-level code field in debug-output

## Metadata
```yaml
bug_number: 058
title: "Add stable top-level code field in debug-output"
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
The `--debug-output` JSON places generated code under language‑specific keys (e.g., `python`, `typescript`). This forces consumers to branch on the target language for a field name. A stable top‑level field would simplify integration.

## Reproduction Steps
1. Run: `framec -l python_3 --debug-output input.frm`
2. Observe top‑level `{ "python": "...", "sourceMap": { ... } }` structure.

## Test Case
```json
// Current
{ "python": "...", "sourceMap": { ... } }

// Proposed (add stable alias)
{ "targetLanguage": "python_3", "code": "...", "sourceMap": { ... }, "python": "..." }
```

## Expected Behavior
- Emit a stable alias (e.g., `code` or `targetCode`) with the same content as the language‑specific key; include `targetLanguage` for clarity.

## Actual Behavior
- Only language‑specific top‑level code key is present.

## Impact
- **Severity**: Medium — small but pervasive branching in consumers.
- **Scope**: All tools ingesting debug JSON.
- **Workaround**: Branch on `targetLanguage` to read the matching field.

## Technical Analysis
- JSON emitter can add a stable alias without breaking existing consumers.

### Root Cause
- Historical evolution of the debug JSON shape with per‑language fields.

### Affected Files
- JSON emitters for Python/TypeScript visitors.

## Proposed Solution
- Add `code` (alias to `python`/`typescript` content) and `targetLanguage` at the top level.
- Keep existing fields for one minor cycle; deprecate later if desired.

## Test Coverage
- [ ] Unit test: presence of `code` and `targetLanguage`
- [ ] Consumers updated in docs/examples

## Related Issues
- Bug #057 – unify source map schema

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
