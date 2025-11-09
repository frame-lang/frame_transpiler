# Stage 07 — Native Parse Facade (C)

Purpose
- Optionally parse spliced C code for diagnostics and formatting (advisory only). Off by default; gated.

Design
- `NativeParseFacadeCV3` trait, returning opaque `NativeAst`.
- Pluggable adapter (e.g., libclang/tree-sitter) when available; not required for core.
- Map diagnostics via `splice_map` to Frame lines.

Acceptance
- Disabled by default; no change in behavior.
- When enabled, surfaces syntax errors with correct attribution; may provide indent hints.

Tests
- Diagnostic mapping across splice boundaries; formatting pass produces consistent whitespace.
