# Stage 07 — Native Parse Facade (C#)

Purpose (V3 minimal)
- Provide hermetic validation of facade runtime helper calls in strict mode. Lines containing these calls are always validated; optional structural C# parsing is available behind a feature flag.

Runtime Optionality
- Gated by `--validate-native` (strict). Off by default.
- Call-site checks are always available; structural C# parsing uses Tree-sitter when the `native-csharp` cargo feature is enabled.

Inputs
- Spliced body text + `splice_map` with helper calls inserted when `FRAME_FACADE_EXPANSION=1`.

Outputs
- Diagnostics for helper-call lines only (spliced spans), remapped to Frame/native via `splice_map`.

Checks (call-sites only)
- Balanced parentheses on helper calls.
- Require trailing semicolon `;` on these lines.

Helper call arguments (policy)
- Transition helper `__frame_transition("<State>"[, <args>...]);`
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
- `unbalanced parentheses in helper call`
- `missing semicolon terminator`
- `transition helper: first argument must be quoted state`
- `transition helper: invalid state identifier`
- `helper call takes no arguments` (for forward/stack helpers)

Acceptance
- Disabled by default; zero impact when off.
- When enabled, surfaces helper-call errors with correct attribution.

Tests
- v3_facade_smoke negatives (call-sites only) produce expected failures; diagnostics map through `splice_map`.

Native parser integration (optional)
- Structural C# parsing via Tree-sitter is available behind cargo feature `native-csharp` and `--validate-native`.
- Implementation detail: the spliced body is wrapped in a minimal `class __FramecFacade { void M(){ ... } }` to parse statement lists. Diagnostics from error nodes are remapped to spliced offsets.
- Default builds do not enable this feature; CI and local runs can opt in with `cargo build --features native-csharp`.
