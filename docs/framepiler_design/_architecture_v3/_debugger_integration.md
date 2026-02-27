# Debugger Integration — V3 (Python/TypeScript)

This note summarizes the artifacts the transpiler emits to support source-level debugging, breakpoint placement, stack visualizations, and error surface alignment. All artifacts are hermetic (pure Rust) and can be enabled with a single flag in the CLI or by setting environment variables.

## How to enable

- CLI: add `--emit-debug` to the `compile` subcommand for module files. This sets:
  - `FRAME_ERROR_JSON=1` — structured compiler diagnostics trailer
  - `FRAME_MAP_TRAILER=1` — inline splice mapping (frame/native origins)
  - `FRAME_DEBUG_MANIFEST=1` — module manifest of system and compiled state IDs
- Environment (alternative): set the variables above directly before invoking the compiler.

Example (Python):
```
./target/release/framec compile --emit-debug -l python_3 path/to/module.frm > out.py
```

## Artifacts (trailers)

Trailering format is a language-appropriate block comment that begins with a sentinel (e.g., `/*#frame-map#`) and ends with a closing sentinel (e.g., `#frame-map#*/`). These are safe to leave in code for development; the test runner extracts them to sidecars.

- Errors JSON
  - Sentinel: `/*#errors-json# ... #errors-json#*/`
  - Shape: `{ "errors": [ { "code": "E###"|null, "message": "..." }, ... ], "schemaVersion": 1 }`
  - Notes: `code` is taken from the message prefix when present; otherwise `null`. Use to surface compiler diagnostics in tools.

- Frame Map (splice map)
  - Sentinel: `/*#frame-map# ... #frame-map#*/`
  - Shape: `{ "map": [ { "targetStart": n, "targetEnd": n, "origin": "frame"|"native", "sourceStart": n, "sourceEnd": n }, ... ], "version": 1, "schemaVersion": 1 }`
  - Notes: Byte offsets are deterministic over the compilation, but not stable across formatting changes. Consume at unit-of-lines or via visitor-map.

- Visitor Map (line mapping; Python/TypeScript module path)
  - Sentinel: `/*#visitor-map# ... #visitor-map#*/`
  - Shape v2: `{ "mappings": [ { "targetLine": n, "targetColumn": n, "sourceLine": n, "sourceColumn": n, "origin": "frame"|"native" }, ... ], "schemaVersion": 2 }`
  - Notes: Use for stepping/breakpoint alignment; the runner validates presence and schema for v3_visitor_map. Columns are optional for consumers but recommended.

- Debug Manifest (module manifest)
  - Sentinel: `/*#debug-manifest# ... #debug-manifest#*/`
  - Shape v2: `{ "system": "Name"|null, "states": [ { "name": "State", "compiledId": "__System_state_State" }, ... ], "handlers": [ { "state": "State", "name": "handler", "compiledId": "__System_state_State__handler_Handler", "params": [..] } ], "schemaVersion": 2 }`
  - Notes: Helpful to show a compiled state and handler ID map in the debugger UI.

## Sidecars (runner behavior)

When running tests, the runner extracts trailers to sidecars next to the generated output and strips them from the code, so they don’t interfere with toolchains:

- `out.<ext>.frame-map.json`
- `out.<ext>.visitor-map.json`
- `out.<ext>.debug-manifest.json`

Errors JSON is typically consumed by tools directly from the trailer or stderr and is not sidecarred by default.

## Parser-backed symbol snapshots (advisory)

For Python/TypeScript, the compiler can optionally emit a `native-symbols` trailer (when `FRAME_NATIVE_SYMBOL_SNAPSHOT=1`) carrying handler parameter names and spans gathered from the header (and optionally confirmed by native parsers: SWC for TS, RustPython for Py). This is advisory and does not change Frame semantics.

- Sentinel: `/*#native-symbols# ... #native-symbols#*/`
- Shape: `{ "entries": [ { "state": "A"|null, "owner": "e"|null, "params": [..], "paramSpans": [ { "start": n, "end": n }, .. ] } ], "schemaVersion": 1 }`

## Recommended consumption (order of preference)

- Use `visitor-map` for stepping/breakpoints. Pair with `frame-map` for byte-level attribution when needed.
- Display compiler diagnostics from `errors-json`; prefer the E-code when present.
- Show a state map via the `debug-manifest` for quick navigation.
- Use advisory `native-symbols` only for IDE hints and optional validations.

## Stability & Hermeticity

- All maps include `schemaVersion`. Changes will bump this number and be documented.
- No external toolchains required (hermetic). Native parsers used for advisory snapshots are pure Rust (SWC, RustPython).

## CLI quick reference

- `compile` / `compile-project` are the non‑demo V3 entrypoints.
- `--emit-debug`:
  - Enables `errors-json`, `frame-map`, `visitor-map` (module path for Py/TS), and `debug-manifest` for module compiles.
  - Internally maps to `FRAME_ERROR_JSON=1`, `FRAME_MAP_TRAILER=1`, and `FRAME_DEBUG_MANIFEST=1` for that invocation.

Advanced toggles (env‑only; used by tests and tooling):
- `FRAME_MAP_TRAILER=1`: request frame-map/visitor-map trailers even without `--emit-debug` (e.g., single‑body flows).
- `FRAME_ERROR_JSON=1`: force errors-json trailer for V3 demo/single‑body paths.
- `FRAME_EMIT_EXEC=1`: enable exec/demo emission in selected test categories only (not exposed as a CLI flag).

Use `compile --emit-debug` during local development and when producing debugger fixtures. For CI, the test runner already sets the appropriate env flags and asserts presence and schema for relevant categories.
