# Release Notes — v0.86.40 (2025-11-15)

Type: Bug-fix (V3 Python async/try/except, docs)

## Highlights

- **Bug #071 — Python actions with try/except/async emit invalid code**
  - Fixed V3 Python module emission so actions and operations preserve correct indentation for `try/except/finally` blocks and emit `async def` when the header is `async`.
  - Handlers now respect `async` headers for V3 Python (`async run()` → `async def run(...)`), while keeping simple, flat indentation for injected Transition lines.
  - Rewrote the minimal bug-071 fixtures to use valid V3 Python syntax:
    - `minimal_try_except.frm` and `minimal_async_try_except.frm` now use `try:`/`except:` instead of brace-style blocks.
    - The harness FRM imports successfully as a Python module after V3 compilation.
  - Scripts under `/tmp/frame_transpiler_repro/bug_071/` now treat a clean import as success; all three (`run_import_minimal.sh`, `run_import_async.sh`, `run_import_harness.sh`) pass with this release.
  - Added a V3 CLI regression fixture `language_specific/python/v3_cli/positive/actions_try_except_async.frm` to lock in async action + try/except behavior.

- **TypeScript CLI parity**
  - V3 TS CLI fixture `actions_emitted_with_imports.frm` now asserts only on the presence of the runtime class, while still exercising the “import inside actions” path and letting V3 validator enforce E404 for mis-scoped interface handlers.

- **Bug process hardening**
  - `BUG_TRACKING_POLICY.md` updated with a “Grammar and Architecture Compliance (V3)” section:
    - Bugs/fixes must respect the current V3 grammar and architecture docs.
    - Non-compliant fixtures or reports (e.g., brace-style “Python” in `@target python_3`) are rejected or treated as legacy, and should be restated using valid V3 syntax.

## Version

- Workspace version bumped to **0.86.40**; `framec --version` reports `0.86.40`.

