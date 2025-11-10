# Stage 07 — Native Parse Facade (C#)

Purpose
- Parse the spliced C# body for diagnostics/formatting and indentation derivation.

Runtime Optionality
- Execution is runtime-optional (gated by `--validate-native/--strict`).
- Implementation is required so strict validation is available across languages.

Inputs
- Spliced C# body text + splice_map

Outputs
- NativeAst (opaque C# AST) for advisory use (errors/formatting/indent), not authoritative for Frame semantics

Design
- Implement `NativeParseFacadeCsV3` trait with `parse(&str) -> Result<NativeAst, NativeParseError>`.
- Keep integration optional; no network/toolchain dependency in core build. If integrated, use a pluggable adapter to Roslyn/Source Generators in environments where available.
- Map diagnostics positions back through `splice_map` to Frame origin lines.

Scanner/Closer Constraints (inputs to facade)
- DPDA body closer and region scanner must handle:
  - Verbatim strings: `@"…"` with doubled `"` escapes.
  - Interpolated strings: `$"…{expr}…"` with brace depth tracking inside strings.
  - Interpolated‑verbatim strings: `$@"…"` with doubled quotes and nested `{…}` expression balance.
  - Raw strings: `"""…"""` (N quotes open/close; content can include quotes/newlines).
  - Character literals: `'x'`, including escapes.
  - SOL preprocessor lines: `#if`, `#elif`, `#endif`, `#define`, etc. (treated as part of the imports/outline when at SOL).

Diagnostics
- When enabled via `--validate-native` (or equivalent), parse the fully spliced C# body and surface:
  - Syntax errors mapped to Frame segments (via `splice_map`).
  - Indentation/formatting hints for control structures (if/else/try/catch/finally, switch), without altering semantics.
  - Optional advisory binding (namespaces/usings) for improved error clarity; does not affect Frame semantics.

Acceptance
- Disabled by default; zero impact on pipeline when off.
- When enabled, produces stable indentation hints and surfaces syntax errors with correct attribution.

Tests
- Mapping of diagnostics through splice_map; indentation hints on `if/else/try/catch/finally` blocks.
 - Negative fixtures including tricky string forms and preprocessor lines (expected failures) in `v3_facade_csharp` (runner category to be added when Stage 07 is enabled).
