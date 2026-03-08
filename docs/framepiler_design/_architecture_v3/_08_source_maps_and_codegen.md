# Stage 8 — Source Maps & Codegen

Purpose
- Compose precise source maps that attribute emitted code back to either original native text or originating Frame statements, and generate the final target code.

Inputs
- Native AST (preferred) or `SplicedBody` bytes
- `splice_map` and a precomputed byte→(line,col) index for the original source

Outputs
- Target code (Python/TypeScript)
- Source maps with dual‑origin attribution

Mapping Model
- Two‑step mapping for inserted regions:
  1) Emitted code span → spliced body span (from native AST or concatenation offsets)
  2) Spliced body span → original origin via `splice_map` (Frame statement vs native text)
- For native text, `splice_map` has no entry; default origin is the original native span.

Invariants
- Deterministic output and mapping for identical inputs.
- No gaps in mapping for emitted statements; missing spans are internal errors.

Codegen Choices
- With native AST: regenerate target code using the AST and a stable printer; attach mapping by walking AST nodes and consulting span remaps.
- Without native AST: concatenate text (native runs + expansions) and synthesize mapping ranges from known offsets.

Complexity
- Linear in node/text count.

Test Hooks
- Breakpoint alignment tests; stepping through mixed handlers lands on Frame-statement lines and native lines as expected.
- Golden source maps for representative fixtures.
## Addendum: Trailer/Sidecar Matrix (Per Language)

This section summarizes common trailers and sidecars emitted by module codegen/CLI. Exact emission can vary with flags (`--emit-debug`) and subcommands; the runner extracts these to assert mapping correctness.

| Language   | Trailer in code     | Sidecar(s) written                 | Notes                                 |
|------------|---------------------|------------------------------------|---------------------------------------|
| Python     | frame-map, visitor-map, debug-manifest, errors-json | `*.frame-map.json`, `*.visitor-map.json`, `*.debug-manifest.json`, `*.errors.json` | For CLI `compile -o`, code may be stripped and written as sidecars. |
| TypeScript | frame-map, visitor-map, debug-manifest              | Same naming pattern                    | Import path for runtime uses package name (`frame_runtime_ts`).      |
| Rust/Java/C/C++/C# | (varies)     | Mapping sidecars when applicable    | Language‑specific runners assert shape |

Guideline
- For module tests, prefer sidecar extraction, stripping the trailer from code for readability and importability. The runner demonstrates this flow.
