# Stage 07 — Native Parse Facade (C#)

Purpose (V3 minimal)
- Provide hermetic validation of facade wrapper calls in strict mode. No general C# parsing; wrapper lines only.

Runtime Optionality
- Gated by `--validate-native` (strict). Off by default.
- Present across all languages with wrapper-only checks.

Inputs
- Spliced body text + `splice_map` with wrapper calls inserted when `FRAME_FACADE_EXPANSION=1`.

Outputs
- Diagnostics for wrapper lines only (spliced spans), remapped to Frame/native via `splice_map`.

Checks (wrapper-only)
- Balanced parentheses on wrapper calls.
- Require trailing semicolon `;` on wrapper lines.

Wrapper arguments (policy)
- Transition wrapper `__frame_transition("<State>"[, <args>...]);`
  - First argument must be a quoted state identifier matching `[A-Za-z_][A-Za-z0-9_]*` (single or double quotes accepted; double quotes shown here).
  - Additional arguments are allowed and left uninterpreted (count/shape validated later in Stage 09).
- `__frame_forward();` and `__frame_stack_{push|pop}();` take no arguments.

Scanner/Closer Constraints (inputs to facade)
- DPDA body closer and region scanner must handle:
  - Verbatim strings: `@"…"` with doubled `"` escapes.
  - Interpolated strings: `$"…{expr}…"` with brace depth tracking inside strings.
  - Interpolated‑verbatim strings: `$@"…"` with doubled quotes and nested `{…}` expression balance.
  - Raw strings: `"""…"""` (N quotes open/close; content can include quotes/newlines).
  - Character literals: `'x'`, including escapes.
  - SOL preprocessor lines: `#if`, `#elif`, `#endif`, `#define`, etc. (treated as part of the imports/outline when at SOL).

Diagnostics
- `unbalanced parentheses in wrapper`
- `missing semicolon terminator`
- `transition wrapper: first argument must be quoted state`
- `transition wrapper: invalid state identifier`
- `wrapper takes no arguments` (for forward/stack wrappers)

Acceptance
- Disabled by default; zero impact when off.
- When enabled, surfaces wrapper-line errors with correct attribution.

Tests
- v3_facade_smoke negatives (wrapper-only) produce expected failures; diagnostics map through `splice_map`.

Native parser integration (optional)
- Structural C# parsing (e.g., via Tree-sitter or a Roslyn-backed local adapter) can be added behind cargo features and `--validate-native`. Current state is wrapper-only; no external parser compiled by default.
