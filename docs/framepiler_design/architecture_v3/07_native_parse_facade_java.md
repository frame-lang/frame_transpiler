# Stage 07 — Native Parse Facade (Java)

Purpose
- Parse spliced Java bodies for diagnostics/formatting.

Runtime Optionality
- Execution is runtime-optional (gated by `--validate-native/--strict`).
- Implementation is required to provide strict validation capability across languages.

Design
- `NativeParseFacadeJavaV3` trait; pluggable adapter to a Java parser when available.
- Diagnostics mapped back through `splice_map`.

Acceptance
- Disabled by default; no change to behavior.
- When enabled, stable indentation hints and syntax errors with correct attribution.

Tests
- Diagnostic mapping correctness; indentation suggestions on `if/else/try/catch/finally`.
