# Stage 7 — Native Parse Facade (Python)

Purpose (V3 minimal)
- Provide hermetic validation of facade wrapper calls in strict mode. Wrapper lines are always validated; optional structural Python parsing is available behind a feature flag.

Runtime Optionality
- Execution of Stage 07 is runtime-optional (gated by `--validate-native`).
- Wrapper-only checks are always available; structural Python parsing uses Tree-sitter when the `native-py` cargo feature is enabled.

Inputs
- `SplicedBody { bytes, splice_map }`

Outputs
- Diagnostics on wrapper lines (spliced spans), remapped to Frame/native via `splice_map`.

Checks (wrapper-only)
- Balanced parentheses on wrapper calls.
- Forbid trailing semicolon `;` on Python wrapper lines.

Wrapper arguments (policy)
- Transition wrapper `__frame_transition('<State>'[, <args>...])`
  - First argument must be a single-quoted state identifier matching `[A-Za-z_][A-Za-z0-9_]*`.
  - Additional arguments are allowed and left uninterpreted (count/shape validated later in Stage 09).
- `__frame_forward()` and `__frame_stack_{push|pop}()` take no arguments.

Errors
- `unbalanced parentheses in wrapper`
- `semicolon not allowed in Python wrapper`
- `transition wrapper: first argument must be quoted state`
- `transition wrapper: invalid state identifier`
- `wrapper takes no arguments` (for forward/stack wrappers)

Complexity
- O(n) over spliced body; no external tooling.

Test Hooks
- Syntax error injection; mapping accuracy back to Frame-statement lines.

Native parser integration (optional)
- Structural Python parsing via Tree-sitter is available behind cargo feature `native-py` and `--validate-native`.
- Implementation detail: we parse the spliced body text as a Python module and surface error/missing nodes as diagnostics; spans map directly to spliced text.
