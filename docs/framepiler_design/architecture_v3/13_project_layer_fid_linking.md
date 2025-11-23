# Stage 13 — Project Layer (Reserved)

Stage 13 was originally sketched as a FID‑based linking/packaging layer on top
of the core V3 pipeline. That experiment has been retired: V3 now relies
solely on native parsing plus Arcanum for semantics, and there is no FID code
or behavior in the compiler.

This document is kept as a placeholder for any **future** project‑layer design.
For now:

- Project configuration is limited to `frame.toml` / `.framerc.toml` and the
  existing CLI support:
  - `framec init` to create a manifest and `src/main.frm`.
  - `framec project build` to apply V3 module compilation over the manifest’s
    source directories (or the current directory when no manifest is present).
- There is no FID schema, cache, or FID‑aware diagnostics in the V3
  implementation.

Any future Stage 13 work should be specified from scratch, without assuming
the prior FID design.

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
