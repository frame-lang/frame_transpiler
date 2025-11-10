# Stage 07 — Native Parse Facade (C++)

Purpose
- Parse spliced C++ bodies for diagnostics/formatting.

Runtime Optionality
- Execution is runtime-optional (gated by `--validate-native/--strict`).
- Implementation is required to provide strict validation capability across languages.

Design
- `NativeParseFacadeCppV3` trait; pluggable adapters (e.g., clang-based) permitted behind feature flags.
- Preserve mapping of diagnostics through `splice_map`.

Acceptance
- Off by default; produces stable indent hints and syntax diagnostics when enabled.

Tests
- Verify diagnostics mapping; ensure raw string constructs are preserved; formatting hints stable.
