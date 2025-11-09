# Stage 07 — Native Parse Facade (C++)

Purpose
- Optional parsing of spliced C++ bodies for diagnostics/formatting; not in critical path.

Design
- `NativeParseFacadeCppV3` trait; pluggable adapters (e.g., clang-based) permitted behind feature flags.
- Preserve mapping of diagnostics through `splice_map`.

Acceptance
- Off by default; produces stable indent hints and syntax diagnostics when enabled.

Tests
- Verify diagnostics mapping; ensure raw string constructs are preserved; formatting hints stable.
