# Bug #057: Inconsistent source map schema across languages

## Metadata
```yaml
bug_number: 057
title: "Inconsistent source map schema across languages"
status: Resolved
priority: Medium
category: Tooling
discovered_version: v0.86.25
fixed_version: v0.86.26
reporter: Codex
assignee: Transpiler Team
created_date: 2025-11-13
resolved_date: 2025-11-13
```

## Description
Language-specific docs and emitters drifted on mapping field names.

## Resolution
- Unified on a language-agnostic schema for V3 splice/source maps and published a `schemaVersion` field.
  - Splice map (`SplicerV3.build_trailer_json`): `targetStart`, `targetEnd`, `origin` (frame|native), `sourceStart`, `sourceEnd`, plus `version` and `schemaVersion`.
  - Native mapping docs (`source_map_spec.md`) clarify `targetLine`/`sourceLine` semantics for visitors; both point to the same canonical idea: target ↔ source mapping.

### Fixed Files
- `framec/src/frame_c/v3/splice.rs`: Added `schemaVersion` to trailer JSON and clarified structure.
- `docs/framepiler_design/going_native/source_map_spec.md`: Reflects unified terminology for visitors.

### Verification
- Runner mapping round-trip fixtures remain green.
- Tools can key off `schemaVersion` to decide how to parse.

### Follow-ups
- Native visitors will align their emitted sidecar maps with the doc as we expand codegen stages.

---
*Bug tracking policy version: 1.0*

