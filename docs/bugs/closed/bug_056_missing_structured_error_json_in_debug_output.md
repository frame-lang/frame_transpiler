# Bug #056: Missing structured error JSON in --debug-output mode

## Metadata
```yaml
bug_number: 056
title: "Missing structured error JSON in --debug-output mode"
status: Closed
priority: High
category: Tooling
discovered_version: v0.86.25
fixed_version: v0.86.26
reporter: Codex
assignee: Transpiler Team
created_date: 2025-11-13
resolved_date: 2025-11-13
```

## Description
When `--debug-output` was set, the compiler emitted only the generated code (optionally with comment trailers). Tool consumers needed a stable JSON envelope.

## Resolution
- Implemented a top-level JSON envelope when `--debug-output` is set (V3 path):
  - Fields: `targetLanguage`, `code` (stable alias), language-specific alias (e.g., `python`/`typescript`), `sourceMap` (object), `errors` (array), and `schemaVersion`.
  - Retained embedded comment trailers (`/*#errors-json#*/`, `/*#frame-map#*/`) when `FRAME_ERROR_JSON=1` / `FRAME_MAP_TRAILER=1` for tools relying on inline scanning.

### Fixed Files
- `framec/src/frame_c/v3/mod.rs`: Added JSON envelope generation for `--debug-output`.

### Verification
- Manual spot checks using `framec --debug-output` on invalid and valid fixtures.
- Runner continues to pass all V3 suites (debug mode is opt-in).

### Notes
- This change is additive and gated by `--debug-output`; non-debug output remains unchanged.

---
*Bug tracking policy version: 1.0*
