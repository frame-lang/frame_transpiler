# Stage 07 — Native Parse Facade (C#)

Purpose
- Optionally parse the spliced C# body for diagnostics/formatting and indentation derivation; off by default and gated behind a feature flag.

Inputs
- Spliced C# body text + splice_map

Outputs
- NativeAst (opaque C# AST) for advisory use (errors/formatting/indent), not authoritative for Frame semantics

Design
- Implement `NativeParseFacadeCsV3` trait with `parse(&str) -> Result<NativeAst, NativeParseError>`.
- Keep integration optional; no network/toolchain dependency in core build. If integrated, use a pluggable adapter to Roslyn/Source Generators in environments where available.
- Map diagnostics positions back through `splice_map` to Frame origin lines.

Acceptance
- Disabled by default; zero impact on pipeline when off.
- When enabled, produces stable indentation hints and surfaces syntax errors with correct attribution.

Tests
- Mapping of diagnostics through splice_map; indentation hints on `if/else/try/catch/finally` blocks.
