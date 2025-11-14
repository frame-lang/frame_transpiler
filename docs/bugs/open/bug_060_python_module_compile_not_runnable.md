# Bug #060: Python module compile outputs non-runnable file (annotated Frame + trailers)

## Metadata
```yaml
bug_number: 060
title: "Python module compile outputs non-runnable file (annotated Frame + trailers)"
status: Open
priority: High
category: Tooling
discovered_version: v0.86.27
fixed_version: 
reporter: Codex
assignee: 
created_date: 2025-11-14
resolved_date: 
```

## Description
Running the V3 module compile path for Python with `--emit-debug` produces a file whose contents are the annotated Frame module text (beginning with `@target python_3` and `system ...`) plus embedded debug trailers. The output is not runnable Python code (no module-level classes/functions matching the system actions), which breaks downstream tooling and tests that import and execute the generated runtime.

This appears inconsistent with the release notes for v0.86.27 ("CLI Module Compile Restored, Debug Artifacts v2 (Py/TS)") which imply that module compile is ready for Py/TS. The current behavior is useful as a carrier for trailers/sidecars, but does not provide executable Python code.

## Reproduction Steps
1. Use any minimal module with a valid prolog:
   ```frame
   @target python_3
   system Minimal {
       machine:
           $Start {
               e() { # no-op }
           }
   }
   ```
2. Compile with module path:
   ```bash
   framec -l python_3 --emit-debug path/to/minimal_py.frm > out.py
   ```
3. Inspect `out.py` — it starts with `@target python_3` and `system Minimal { ... }` followed by embedded trailers. Importing this file in Python fails; it is not executable Python code.

## Expected Behavior
- For Python module compile, the output should be runnable Python code that executes the system semantics (classes/functions) and optionally includes embedded debug trailers. Alternatively, a clearly documented flag (e.g., `--emit-trailers-only`) should govern trailer-only output, while the default `compile` path produces runnable code.

## Actual Behavior
- The compiled output is a textual representation of the Frame source plus trailers, not executable Python. No classes/functions are emitted; importing the output fails.

## Impact
- **Severity**: High — breaks downstream tools/tests that expect executable Python (e.g., runtime adapters and integration harnesses).
- **Scope**: Python V3 module compile path with `--emit-debug`.
- **Workaround**: Use `demo-frame` for runnable snippets (insufficient for full modules) or keep legacy generated files; neither is a proper module compile replacement.

## Technical Analysis
- CLI shows `compile` subcommand for modules; TS outputs are compilable to JS. For Python, the current module path appears to serialize the Frame AST/module text with trailers rather than invoking the Python visitor to generate code.
- Likely missing/disabled Python visitor step in the v3 module compile pipeline when `--emit-debug` is set, or the pipeline routes to a trailer-only emitter.

## Proposed Solution
- Enable the Python visitor in the V3 module compile path so the output file contains runnable Python code, then append/inline debug trailers.
- If trailer-only mode is needed, add a dedicated flag (e.g., `--emit-trailers-only`) and keep `compile` default as runnable code.
- Ensure the runtime helpers (frame_runtime_py) are emitted or imported correctly so `import out.py` works without manual scaffolding.

## Test Coverage
- [ ] Add a compile test that imports the Python module and executes a minimal handler (smoke).
- [ ] Verify trailers are present and parseable (visitor-map v2, debug-manifest v2) on runnable output.
- [ ] Negative test: trailer-only mode guarded by flag does not emit executable bodies (documented behavior).

## Related Issues
- v0.86.27 release notes (“CLI Module Compile Restored”) — reconcile expectations for Python.

## Work Log
- 2025-11-14: Initial report — Codex

## Resolution
_Pending._

### Fix Summary
_Pending._

### Verification
_Pending._

### Lessons Learned
_Pending._

---
*Bug tracking policy version: 1.0*
