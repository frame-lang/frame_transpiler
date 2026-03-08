# Release Notes — v0.86.35 (2025-11-15)

Type: Bug-fix

Highlights
- Python: runtime package copied next to outputs on `compile -o` and `compile-project`.
- Python: actions/operations are emitted as methods in compiled modules.
- TypeScript: non-demo compile now imports from `frame_runtime_ts` by default.

Bug Fixes
- #065 Python runtime package not emitted by compile — Resolved in v0.86.35.
  - CLI copies `frame_runtime_py` to the output directory (override with `FRAME_RUNTIME_PY_DIR`).
- #067 Python actions not emitted in output module — Resolved in v0.86.35.
  - Compiled modules include `def _action_*` and `def _operation_*` methods.
- #068 TypeScript runtime import path incorrect for single-file — Resolved in v0.86.35.
  - Defaults to package import `from 'frame_runtime_ts'` (override with `FRAME_TS_EXEC_IMPORT` for exec-only scenarios).

Testing
- Py/TS suites: v3_cli and v3_debugger 100% passing.
- Rust curated exec: control_flow/core/systems passing.
- New TS fixture to assert default runtime import: `language_specific/typescript/v3_cli/positive/import_runtime_package.frm`.

Compatibility / Notes
- Non-demo compile is the default path for modules (`compile`, `compile-project`).
- Debug trailers are controlled by `--emit-debug` (errors-json, frame-map, visitor-map for Py/TS, debug-manifest).

Version
- Workspace version bumped to 0.86.35; CLI `--version` reflects this.
