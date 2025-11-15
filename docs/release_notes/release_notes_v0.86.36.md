# Release Notes — v0.86.36 (2025-11-15)

Type: Bug-fix / Hardening

Highlights
- Python runtime emission hardened
  - compile-project copies `frame_runtime_py` to OUTDIR root.
  - compile `-o` copies `frame_runtime_py` using robust path resolution (FRAME_RUNTIME_PY_DIR, exe-relative repo root/target dir, cwd fallback).
- Actions/Operations emission and validation
  - OutlineScanner recognizes bare IDENT headers for actions/operations; Python emits `def _action_*` / `def _operation_*`.
  - E401 enforced during compile validation when Frame statements appear in actions/operations.
- Runner robustness and coverage
  - @cwd: tmp supported for v3_cli and v3_cli_project; framec path made absolute when cwd changes.
  - v3_cli compile asserts runtime directory presence for Python; v3_cli_project already enforces runtime presence.
  - Visitor-map single-body tests rely on transpile() assertions; no legacy single-body validator.
- TypeScript
  - Non-demo compile imports from `frame_runtime_ts` by default; locked by CLI test.
- Rust
  - Curated exec passing; compile-time E401 wired.

Bugs
- #065 Python runtime package not emitted — Fixed in 0.86.36.
  - Robust runtime copy for compile-project and compile -o; added @cwd: tmp tests.

Tests Added/Updated
- Python CLI: basic_cli_compile_cwd_tmp.frm (tmp cwd) asserts runtime present.
- Python CLI project: simple_project/_cwd_tmp.frm (tmp cwd) asserts runtime present.
- Runner now respects @cwd and asserts runtime presence for Python outputs.

Docs
- PLAN.md records 0.86.36 improvements.
- Bug process standardized to states: Open, Fixed, Closed, Reopen.

Version
- Workspace version bumped to 0.86.36; CLI `--version` reflects this.
