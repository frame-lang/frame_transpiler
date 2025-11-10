# Stage 07 — Native Parse Facade (Rust)

Purpose (V3 minimal)
- Provide hermetic validation of wrapper calls in strict mode. No general Rust parsing; wrapper lines only.

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
- Verify mapping across splice boundaries; ensure nested block comments/raw strings do not break spans.

Native parser integration (optional)
- Structural Rust parsing can be enabled as an optional adapter behind cargo features and `--validate-native`.
- Feature flag: `native-rs` (uses the `syn` crate to parse the spliced body as a block). Default build keeps this disabled; facades remain wrapper-only.
