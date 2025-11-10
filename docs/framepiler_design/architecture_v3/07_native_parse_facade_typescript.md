# Stage 7b — Native Parse Facade (TypeScript)

Purpose (V3 minimal)
- Provide hermetic validation of facade wrapper calls in strict mode. No general TS parsing; wrapper lines only.

Runtime Optionality
- Gated by `--validate-native` (strict). Off by default.
- Present across all languages with wrapper-only checks.

Inputs
- `SplicedBody { text, splice_map }` with wrapper calls inserted when `FRAME_FACADE_EXPANSION=1`.

Outputs
- Diagnostics on wrapper lines (spliced spans), remapped to Frame/native via `splice_map` in the validator.

Checks
- Balanced parentheses on wrapper calls.
- Require trailing semicolon `;` on TS wrapper lines.

Wrapper arguments (policy)
- Transition wrapper `__frame_transition('<State>'[, <args>...]);`
  - First argument must be a single-quoted state identifier matching `[A-Za-z_][A-Za-z0-9_]*`.
  - Additional arguments are allowed and left uninterpreted (count/shape validated later in Stage 09).
- `__frame_forward()` and `__frame_stack_{push|pop}()` take no arguments.

Errors
- `unbalanced parentheses in wrapper`
- `missing semicolon terminator`
- `transition wrapper: first argument must be quoted state`
- `transition wrapper: invalid state identifier`
- `wrapper takes no arguments` (for forward/stack wrappers)

Complexity
- O(n) over spliced body; no external tooling.

Notes
- Policy checks like `==`/`!=` remain in Stage 09 visitors; facade exists to improve developer feedback without compromising hermeticity.

Native parser integration (optional)
- Real native parsing can be enabled via optional adapters (e.g., SWC) behind cargo features and `--validate-native`. In the current state, the facade is wrapper-only; no SWC dependency is compiled by default.
