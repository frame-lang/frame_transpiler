# Stage 07 — Native Parse Facade (C)

Purpose
- Parse spliced C code for diagnostics and formatting (advisory only).

Runtime Optionality
- Execution is runtime-optional (gated by `--validate-native/--strict`).
- Implementation is required to ensure strict validation capability across languages.

Design
- `NativeParseFacadeCV3` trait, returning opaque `NativeAst`.
- Pluggable adapter (e.g., libclang/tree-sitter) when available.
- Map diagnostics via `splice_map` to Frame lines.

Acceptance
- Disabled by default; no change in behavior.
- When enabled, surfaces syntax errors with correct attribution; may provide indent hints.

Tests
- Diagnostic mapping across splice boundaries; formatting pass produces consistent whitespace.
