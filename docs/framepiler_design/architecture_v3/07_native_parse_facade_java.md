# Stage 07 — Native Parse Facade (Java)

Purpose
- Optional parsing of spliced Java bodies for diagnostics/formatting; not required for core V3 pipeline.

Design
- `NativeParseFacadeJavaV3` trait; pluggable adapter to a Java parser when available.
- Diagnostics mapped back through `splice_map`.

Acceptance
- Disabled by default; no change to behavior.
- When enabled, stable indentation hints and syntax errors with correct attribution.

Tests
- Diagnostic mapping correctness; indentation suggestions on `if/else/try/catch/finally`.
