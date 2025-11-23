# Stage 7 — Native Parse Facade (Python)

Purpose (V3 minimal)
- Provide hermetic validation of facade runtime calls in strict mode. Lines containing these calls are always validated; optional structural Python parsing is available behind a feature flag.

Runtime Optionality
- Execution of Stage 07 is runtime-optional (gated by `--validate-native`).
- These call-site checks are always available; structural Python parsing uses Tree-sitter when the `native-py` cargo feature is enabled.

Inputs
- `SplicedBody { bytes, splice_map }`

Outputs
- Diagnostics on runtime call lines (spliced spans), remapped to Frame/native via `splice_map`.

Checks (call-sites only)
- Balanced parentheses on runtime calls.
- Forbid trailing semicolon `;` on these Python lines.

Runtime call arguments (policy)
- Transition call `__frame_transition('<State>'[, <args>...])`
  - First argument must be a single-quoted state identifier matching `[A-Za-z_][A-Za-z0-9_]*`.
  - Additional arguments are allowed and left uninterpreted (count/shape validated later in Stage 09).
- `__frame_forward()` and `__frame_stack_{push|pop}()` take no arguments.

Errors
- `unbalanced parentheses in runtime call`
- `semicolon not allowed in Python runtime call`
- `transition call: first argument must be quoted state`
- `transition call: invalid state identifier`
- `runtime call takes no arguments` (for forward/stack helpers)

Complexity
- O(n) over spliced body; no external tooling.

Test Hooks
- Syntax error injection; mapping accuracy back to Frame-statement lines.

Native parser integration (optional)
- Structural Python parsing via Tree-sitter is available behind cargo feature `native-py` and `--validate-native`.
- Implementation detail: we parse the spliced body text as a Python module and surface error/missing nodes as diagnostics; spans map directly to spliced text.
