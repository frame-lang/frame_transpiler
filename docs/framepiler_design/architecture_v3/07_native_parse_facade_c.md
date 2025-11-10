# Stage 07 — Native Parse Facade (C)

Purpose (V3 minimal)
- Provide hermetic validation of wrapper calls in strict mode. No general C parsing; wrapper lines only.

Runtime Optionality
- Gated by `--validate-native` (strict). Off by default.
- Implemented uniformly across all languages with wrapper-only checks.

Inputs
- `SplicedBody { text, splice_map }` with wrapper calls inserted when `FRAME_FACADE_EXPANSION=1`.

Outputs
- Diagnostics on wrapper lines (spliced spans), remapped to Frame/native via `splice_map` in the validator.

Checks (wrapper-only)
- Balanced parentheses on wrapper calls.
- Require trailing semicolon `;` on wrapper lines.

Wrapper arguments (policy)
- Transition wrapper `__frame_transition('<State>'[, <args>...]);`
  - First argument must be a single-quoted state identifier matching `[A-Za-z_][A-Za-z0-9_]*`.
  - Additional arguments are allowed and left uninterpreted (count/shape validated later in Stage 09).
- `__frame_forward();` and `__frame_stack_{push|pop}();` take no arguments.

Errors
- `unbalanced parentheses in wrapper`
- `missing semicolon terminator`
- `transition wrapper: first argument must be quoted state`
- `transition wrapper: invalid state identifier`
- `wrapper takes no arguments` (for forward/stack wrappers)

Complexity
- O(n) over spliced body; no external tooling.

Test Hooks
- Mapping accuracy back to Frame-statement lines; negative wrappers validate as expected.

Native parser integration (optional)
- Structural C parsing (e.g., via Tree-sitter) can be added as an optional adapter behind cargo features and `--validate-native`. Current state is wrapper-only; no external parser compiled by default.
