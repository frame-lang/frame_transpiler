# Bug #058: Add stable top-level code field in debug-output

## Metadata
```yaml
bug_number: 058
title: "Add stable top-level code field in debug-output"
status: Won't Fix (Superseded)
priority: Medium
category: Tooling
discovered_version: v0.86.25
fixed_version: Superseded by --emit-debug trailers
reporter: Codex
assignee: Framepiler Team
created_date: 2025-11-13
resolved_date: 2025-11-14
```

## Resolution Summary
The project has standardized on emitting debug artifacts as inline trailers (and sidecars) in generated code, rather than maintaining a separate monolithic debug-output JSON envelope. The requested stable `code` alias becomes unnecessary because consumers read the generated code directly and parse the dedicated trailers for metadata.

Artifacts now emitted when `--emit-debug` (or env flags) are used:
- errors-json trailer: `/*#errors-json# … #errors-json#*/`
- frame-map trailer: `/*#frame-map# … #frame-map#*/`
- visitor-map trailer (Py/TS module): `/*#visitor-map# … #visitor-map#*/`
- debug-manifest trailer: `/*#debug-manifest# … #debug-manifest#*/`

These are extracted by the runner as `.frame-map.json`, `.visitor-map.json`, and `.debug-manifest.json` sidecars. This approach is stable across languages and avoids duplicating the code in a debug JSON envelope.

## Rationale
- Removes consumer branching on language-specific top-level fields.
- Keeps artifacts colocated with generated code and versioned by `schemaVersion` inside each trailer payload.
- Aligns with debugger team needs (explicit manifests and maps) and V3 testing strategy.

## Verification
- v3_debugger fixtures (Py/TS) validate presence and minimal shape of visitor-map and debug-manifest.
- Runner asserts errors-json presence for V3 module/demo compiles.

## Notes
- If a consolidated JSON output is desired later, it can be built from the code + sidecars deterministically.

---
*Bug tracking policy version: 1.0*

