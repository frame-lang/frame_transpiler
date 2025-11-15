# V3 Open Questions and Follow‑Ups

Purpose
- Track ambiguities or gaps discovered while validating recent V3 behavior (v0.86.36–v0.86.40), and propose specific doc and implementation follow‑ups.

Scope
- Applies to the V3 pipeline only (docs under `docs/framepiler_design/architecture_v3/`).
- Prioritized items are those that affected recent fixes (#065, #067, #068, #069, #070, #071) and day‑to‑day developer workflows.

## 1) Python Actions/Handlers: Async + Indentation Guarantees

Observations
- Prior to v0.86.40, actions and handlers with `try/except` and/or `await` sometimes emitted invalid Python (IndentationError/SyntaxError).
- v0.86.40 adds fixes; the docs should codify invariants so regressions are caught early.

Open Questions
- What are the exact invariants for indentation normalization of action/operation bodies?
- How does the emitter decide between `def` vs `async def` across handlers, actions, and operations?

Proposal (Docs + Tests)
- Update `05_frame_statement_expansion_python.md` to include:
  - “Async‑aware signature” rule: If the header begins with `async`, emit `async def`; otherwise `def`. Add note that emitting `await` in a non‑async body should be caught by native validation.
  - “Indentation normalization” algorithm for actions/ops:
    - Compute minimal common indent across non‑blank lines of the spliced body.
    - Re‑indent under the method suite while preserving relative structure of `try/except/finally`.
    - Emit `pass` for comment‑only/empty suites.
  - Two minimal examples (sync + async `try/except`), showing FRM → emitted Python.
- Add V3 CLI fixtures for `try/except` (sync + async) to guard generator behavior.

## 2) Import Scanning Boundaries (Python)

Observations
- Earlier issues stemmed from import scanning that consumed `import` inside `actions:` bodies and shifted outline start, skipping `machine:`/`actions:`.

Open Questions
- Where is the authoritative boundary defined – Partitioning vs Native Scanner? Which stage enforces “stop at first section header”?

Proposal (Docs)
- Update `01_module_partitioning.md` and `02_native_region_scanner_python.md`:
  - Define “top‑level imports” as those before the first module header (`system`, `machine`, `interface`, `actions`, `operations`, `domain`).
  - Scanners must stop import collection at the first header; imports inside actions/handlers are not module imports.
  - Include a short example demonstrating the boundary.

## 3) Compile vs Build: Python Runtime Packaging

Observations
- The CLI now emits `frame_runtime_py` for `compile -o` when configured; teams still ask how to import generated modules.

Open Questions
- What is the canonical guidance for `FRAME_RUNTIME_PY_DIR` vs adding the package to `PYTHONPATH`? Should compile always copy the runtime by default in `-o` mode?

Proposal (Docs)
- In `13_project_layer_fid_linking.md` (and a short callout in `README.md`):
  - Document: `compile -o OUTDIR` emits `frame_runtime_py/` (or honor `FRAME_RUNTIME_PY_DIR`).
  - Provide a minimal “import validator” snippet using `sys.path.insert(0, OUTDIR)`.
  - Note parity with `compile-project`.

## 4) Validation Flags and Native Parse Policy

Observations
- The runner toggles stricter native parse validation (e.g., `FRAME_VALIDATE_NATIVE_POLICY=1`) for some categories; the docs allude to native parse facades but the toggles are not easily discoverable.

Open Questions
- Which categories turn on native validation by default, and what guarantees should be expected from `--validate` vs `--validate-native`?

Proposal (Docs)
- Update `09_validation.md` and `07_native_parse_facade_python.md`:
  - List validation flags and environment variables that affect validation behavior.
  - Provide a matrix: category → validation route (single‑body vs module), whether native parse is engaged, and the expected diagnostics surface.

## 5) Demo‑Frame vs Module Validation (Single‑Body vs Module)

Observations
- The runner skips some single‑body validation paths for module files and relies on module trailers/sidecars. The stage docs still mention single‑body validators prominently.

Open Questions
- For which categories are single‑body validators authoritative, and for which should we rely on the module pipeline and trailers?

Proposal (Docs)
- In `06_splice_and_mapping.md` and `09_validation.md`, add a “Validation Routes” section:
  - Enumerate routes the runner uses today, and when trailers/sidecars are considered canonical.
  - Include brief examples of asserting the presence and shape of `frame-map`, `visitor-map`, `debug-manifest`, `native-symbols`.

## 6) Trailer/Sidecar Matrix by Language

Observations
- Trailer handling has evolved; each language emits some combination of trails and sidecars from `--emit-debug` or default paths.

Open Questions
- What is the canonical set per target for module outputs? For example: `frame-map`, `visitor-map`, `debug-manifest`, `native-symbols`, `errors-json`.

Proposal (Docs)
- Extend `08_source_maps_and_codegen.md` with a per‑language matrix:
  - When emitted (compile vs compile‑project, with/without `--emit-debug`).
  - Where extracted (sidecar filenames) and expectations (minimal schema keys).

---

Implementation Notes (Non‑normative)
- We validated recent fixes:
  - 0.86.36: Python runtime packaging and runner @cwd handling.
  - 0.86.35: TS import path for compile outputs.
  - 0.86.39: Python handler/action emission for module FRMs.
  - 0.86.40: Python async/indentation for `try/except` in actions/handlers.
- The above proposals lock these behaviors into the spec and reduce churn.

