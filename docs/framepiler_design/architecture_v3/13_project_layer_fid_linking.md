# Stage 13 — Project Layer (Optional): FID / Linking / Packaging

Purpose
- Provide optional, hermetic project-level facilities for symbol discovery, typed linking, and packaging. This stage is not part of the core single-file V3 pipeline and can be disabled in normal development and tests.

Scope and Optionality
- Optional final stage after Stage 09 (Validation).
- Gated behind explicit CLI flags and config; no impact on the core stages (01–09).
- No network access during normal builds; any external tooling must be pinned and opt-in.

Inputs
- Project manifest (e.g., `frame.toml` or CLI args) describing sources and optional FID manifests.
- Optional FID manifests pointing at native declarations for targets (TS TypeDoc JSON, Python introspection, etc.).

Outputs
- Cached `.fid` files under `.frame/cache/fid/<target>` (content-addressed and pinned by a lock file).
- Linkage metadata used by the compiler to surface “unknown symbol” diagnostics or to enrich generated type surfaces (e.g., `.d.ts`).

Responsibilities
- Generate and cache FID artifacts from target-specific sources.
- Provide symbol existence/shape metadata to the compiler when enabled.
- Package/link multi-file projects with deterministic outputs and pinned dependencies.

CLI and Gating
- `framec fid import --config <manifest>.json [--allow-missing]` generates/refreshes `.fid` caches.
- Project build commands accept flags to enable project linking and FID consumption; defaults keep this stage disabled for the core V3 demos/suites.

Determinism & Operations
- No ad-hoc network during builds; any external tool invocations (e.g., `typedoc`) are version-pinned and behind explicit commands.
- FID caches are build artifacts (ignored by VCS); reproducibility ensured via lock files and pinned tool versions.

Interaction with Core V3
- Core scanning/segmentation/parsing/MIR/validation (Stages 01–09) do not depend on FID.
- Stage 13 can add additional diagnostics (e.g., unknown external symbol) and inform codegen for typed surfaces when requested.

Testing Strategy
- Separate project-level suites under `v3_project/{positive,negative}` gated by CLI flags.
- Keep all language-specific V3 suites green without Stage 13 enabled.

## Addendum: Python Runtime Packaging for CLI Outputs

When using the CLI on V3 module files:

- `compile -l python_3 -o OUTDIR …`
  - Emits the generated module(s) and the `frame_runtime_py/` package alongside them.
  - If the runtime location must be overridden, set `FRAME_RUNTIME_PY_DIR=/path/to/frame_runtime_py`.

Minimal import validator:

```bash
OUTDIR=$(mktemp -d)
FRAME_RUNTIME_PY_DIR=/path/to/frame_runtime_py framec compile -l python_3 file.frm -o "$OUTDIR"
PY=$(ls "$OUTDIR"/*.py | head -n 1)
python3 - << PY "$PY" "$OUTDIR"
import importlib.util, sys
p, out = sys.argv[1], sys.argv[2]
sys.path.insert(0, out)
spec = importlib.util.spec_from_file_location('m', p)
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
print('IMPORT_OK')
PY
```

Notes
- `compile-project -l python_3 DIR -o OUTDIR` maintains the same runtime‑emission guarantees.
- For single‑file workflows without `-o`, prefer a project or explicitly set `FRAME_RUNTIME_PY_DIR`.
