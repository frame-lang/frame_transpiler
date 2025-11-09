# Stage 7b — Native Parse Facade (TypeScript)

Purpose
- Optionally parse the spliced TypeScript body with SWC (or similar) to improve diagnostics, formatting, and policy enforcement (e.g., disallow `==`/`!=`).

Inputs
- `SplicedBody { bytes, splice_map }`

Outputs
- `NativeAstTs { root, node_spans }` with spans in spliced coordinate space.

Invariants
- Native AST is advisory; Frame semantics remain in MixedBody/MIR.
- Spans preserved for remap to original origins via `splice_map`.

Errors
- Syntax errors: report with mapped locations and policy notes (e.g., usage of `==`).

Complexity
- Linear in body size; native parser dependent.

Test Hooks
- Policy violation mapping; template literal edge cases preserved in AST.

