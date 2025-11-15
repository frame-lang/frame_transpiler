# Bug #060: Python module compile outputs non-runnable file (annotated Frame + trailers)

## Metadata
```yaml
bug_number: 060
title: "Python module compile outputs non-runnable file (annotated Frame + trailers)"
status: Closed
priority: High
category: Tooling
discovered_version: v0.86.27
fixed_version: v0.86.28
reporter: Codex
assignee: Codex
created_date: 2025-11-14
resolved_date: 2025-11-14
```

## Description
Running the V3 module compile path for Python with `--emit-debug` previously produced an annotated Frame module (beginning with `@target` and `system …`) followed by debug trailers, which is not runnable Python.

## Root Cause
- The V3 module compile path spliced expansions back into the original Frame text (preserving `@target` and module syntax) and appended trailers, rather than emitting full target-language modules.

## Fix Summary (v0.86.28)
- Compile now emits runnable modules for Python, TypeScript, and Rust by default:
  - Python: generates a class named after the system, runtime stubs, and one method per handler with spliced expansions.
  - TypeScript: generates an exported class with runtime stubs and handler methods; imports runtime via `FRAME_TS_EXEC_IMPORT` or a default path.
  - Rust: generates per-handler free functions with spliced expansions and lightweight runtime stubs; optional `StateId` enum via `FRAME_RUST_STATE_ENUM=1`.
- Debug artifacts (frame-map, visitor-map v2, debug-manifest v2, errors-json) remain appended to the runnable output.
- Added opt-out flag: `FRAME_COMPILE_RUNTIMABLE=0` to restore trailer-only output for debugging purposes.

## Verification
1) Build release: `cargo build --release`.
2) Compile a minimal module for Python/TypeScript/Rust:
   - `./target/release/framec compile -l python_3 --emit-debug path/to/file.frm -o outdir`
   - `./target/release/framec compile -l typescript --emit-debug path/to/file.frm -o outdir`
   - `./target/release/framec compile -l rust --emit-debug path/to/file.frm -o outdir`
3) Inspect output — classes/functions present; trailers included.
4) Runner suites:
   - `v3_cli` and `v3_cli_project` are green for Python/TypeScript/Rust.

## Notes
- This change is non-breaking for demo/exec paths. C/C++/Java/C# will be brought to parity (runnable compile with stubs) next.

---
*Resolution Owner: Codex*
*QA: Runner suites (`v3_cli`, `v3_cli_project`) and manual spot checks*
