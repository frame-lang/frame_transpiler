# Stage 7b — Native Parse Facade (TypeScript)

Purpose (V3 minimal)
- Provide hermetic validation of facade runtime helper calls in strict mode. No general TS parsing; only these helper-call sites are checked.

Runtime Optionality
- Gated by `--validate-native` (strict). Off by default.
- Present across all languages with call-site checks.

Inputs
- `SplicedBody { text, splice_map }` with helper calls inserted when `FRAME_FACADE_EXPANSION=1`.

Outputs
- Diagnostics on helper-call lines (spliced spans), remapped to Frame/native via `splice_map` in the validator.

Checks
- Balanced parentheses on helper calls.
- Require trailing semicolon `;` on TS helper-call lines.

Helper call arguments (policy)
- Transition helper `__frame_transition('<State>'[, <args>...]);`
  - First argument must be a single-quoted state identifier matching `[A-Za-z_][A-Za-z0-9_]*`.
  - Additional arguments are allowed and left uninterpreted (count/shape validated later in Stage 09).
- `__frame_forward()` and `__frame_stack_{push|pop}()` take no arguments.

Errors
- `unbalanced parentheses in helper call`
- `missing semicolon terminator`
- `transition helper: first argument must be quoted state`
- `transition helper: invalid state identifier`
- `helper call takes no arguments` (for forward/stack helpers)

Complexity
- O(n) over spliced body; no external tooling.

Notes
- Policy checks like `==`/`!=` remain in Stage 09 visitors; facade exists to improve developer feedback without compromising hermeticity.

Native parser integration (optional)
- Real native parsing can be enabled via optional adapters behind cargo features and `--validate-native`.
- Feature flag: `native-ts` (uses SWC `swc_ecma_parser`). Default build keeps this disabled; facades remain call-site-only.
