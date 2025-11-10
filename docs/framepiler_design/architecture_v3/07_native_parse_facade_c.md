# Stage 07 — Native Parse Facade (C)

Purpose (V3 minimal)
- Provide hermetic validation of wrapper calls in strict mode. Wrapper lines are always validated; optional structural C parsing is available behind a feature flag.

Runtime Optionality
- Gated by `--validate-native` (strict). Off by default.
- Wrapper-only checks are always available; structural C parsing uses Tree-sitter when the `native-c` cargo feature is enabled.

Inputs
- `SplicedBody { text, splice_map }` with wrapper calls inserted when `FRAME_FACADE_EXPANSION=1`.

Outputs
- Diagnostics on wrapper lines (spliced spans), remapped to Frame/native via `splice_map` in the validator.

Checks (wrapper-only)
- Balanced parentheses on wrapper calls.
- Require trailing semicolon `;` on wrapper lines.

Wrapper arguments (policy)
- Transition wrapper `__frame_transition("<State>"[, <args>...]);`
  - First argument must be a quoted state identifier matching `[A-Za-z_][A-Za-z0-9_]*` (single or double quotes accepted; double quotes shown here).
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
- Structural C parsing via Tree-sitter is available behind cargo feature `native-c` and `--validate-native`.
- Implementation detail: the spliced body is wrapped in a minimal function `void __framec_facade(){ ... }` to parse statement lists. Diagnostics from error nodes are remapped to spliced offsets.
- Default builds do not enable this feature; CI and local runs can opt in with `cargo build --features native-c`.
