# Stage 7b — Native Parse Facade (TypeScript)

Purpose
- Parse the spliced TypeScript body with SWC (or similar) to improve diagnostics, formatting, and policy enforcement (e.g., disallow `==`/`!=`).

Runtime Optionality
- Execution is runtime-optional (gated by `--validate-native/--strict`).
- Implementation is required to provide strict validation capability across all languages.

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
