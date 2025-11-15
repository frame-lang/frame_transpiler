# Release Notes — v0.86.37 (2025-11-15)

Type: CI / Parity / Runner

Highlights
- Runner CWD parity finalized
  - Absolutizes framec binary path in all branches (v3_cli, v3_cli_project, v3 validate paths) when changing cwd.
  - Ensures @cwd: tmp works uniformly for Python, TypeScript, and Rust (CLI and project).
- New tests
  - TypeScript: v3_cli/positive/basic_cli_compile_cwd_tmp.frm and v3_cli_project/simple_project/_cwd_tmp.frm
  - Rust: v3_cli/positive/basic_cli_compile_cwd_tmp.frm and v3_cli_project/simple_project/_cwd_tmp.frm
- CI
  - all_v3 preset now includes v3_cli and v3_cli_project, so CWD tests run in v3_all workflow.
- Docs/Plan
  - PLAN updated with Non‑PRT roadmap (C/C++/Java/C#): CWD parity, visitor‑map parity, CLI scaffolds, validation parity, exec‑smoke.

Notes
- 0.86.36 contained the primary runtime and policy hardening; 0.86.37 finalizes parity and CI integration.
