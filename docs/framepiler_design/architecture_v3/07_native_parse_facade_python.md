# Stage 7 — Native Parse Facade (Python)

Purpose
- Optionally parse the spliced Python body with a native parser (e.g., RustPython) to improve diagnostics, enable formatting, and validate that expansions form syntactically coherent Python.

Inputs
- `SplicedBody { bytes, splice_map }`

Outputs
- `NativeAstPy { root, node_spans }` where `node_spans` are byte spans in spliced coordinate space.

Invariants
- Native AST is advisory: Frame semantics remain in MixedBody/MIR; errors in native AST are surfaced for developer feedback but do not change MIR.
- Spans are preserved for later remap to original Frame/native origins using `splice_map`.

Errors
- Syntax errors: include native parser diagnostics; remap locations back through `splice_map` and byte→(line,col).

Complexity
- Linear in body size; dependent on native parser performance characteristics.

Test Hooks
- Syntax error injection; mapping accuracy back to Frame-statement lines.
