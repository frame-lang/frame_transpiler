# v0.86.28 — Runnable Module Compile for Py/TS/Rust, Bug #060 Fixed

Release date: 2025-11-14

## Highlights
- Fixes Bug #060: Python module compile previously emitted annotated Frame text + trailers; now emits runnable modules by default.
- TypeScript and Rust module compile are also runnable by default.
- Debug artifacts (frame-map, visitor-map v2, debug-manifest v2, errors-json) remain appended to outputs.

## Changes
- Compiler (V3 module path):
  - Python: generates system class with runtime stubs and handler methods; splices Frame expansions into methods.
  - TypeScript: generates exported class with runtime stubs and handler methods; runtime import via `FRAME_TS_EXEC_IMPORT` or default path.
  - Rust: generates per-handler free functions with spliced expansions; optional `StateId` enum (set `FRAME_RUST_STATE_ENUM=1`).
  - Opt-out flag: `FRAME_COMPILE_RUNTIMABLE=0` to return to trailer-only output for debugging.
- CLI/Runner:
  - `v3_cli` and `v3_cli_project` suites updated; all green for Py/TS/Rust.
  - Minor runner debug print for CLI invocations when `-v` is used.

## Compatibility
- No breaking changes to demo/exec paths.
- Non-Py/TS/Rust targets remain unchanged; parity work is planned.

## Next Steps
- Bring C/C++/Java/C# compile to runnable parity with lightweight stubs and preserved trailers.
- Consider default-on `StateId` enum for Rust after broader validation.

---
Version: 0.86.28
