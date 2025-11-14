# Bug #059: Enable full module codegen for TS/Py (non-demo) and restore CLI compile path

## Metadata
```yaml
bug_number: 059
title: "Enable full module codegen for TS/Py (non-demo) and restore CLI compile path"
status: Resolved
priority: Critical
category: Tooling
discovered_version: v0.86.26
fixed_version: v0.86.27
reporter: Codex
assignee: Framepiler Team
created_date: 2025-11-14
resolved_date: 2025-11-14
```

## Resolution Summary
Restored the main CLI compile path for module files. Files containing an `@target` header are now auto-detected and routed to the V3 module compiler in `run_file`/`run_file_debug`, producing production code and debug trailers with `--emit-debug`.

## Fix Details
- Compiler routing (non-demo path):
  - `run_file` / `run_file_debug` now check for `@target` and call `v3::compile_module_demo(content, lang)`.
  - Body-only inputs continue to use the single-body demo path.
- Artifacts: `--emit-debug` works for modules and embeds errors-json, frame-map, visitor-map (Py/TS), and debug-manifest.

## Verification
- Manual CLI runs on Py/TS module fixtures emit all trailers.
- Suites remain green (v3_debugger, v3_visitor_map).

## Notes
- Demo commands remain for CI/tests. Use main CLI compile for production builds.

---
*Bug tracking policy version: 1.0*

