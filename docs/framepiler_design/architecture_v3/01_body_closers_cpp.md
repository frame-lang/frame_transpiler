# Stage 01 — Body Closers (C++)

Goal
- Find the matching `}` for a C++ native body, handling protected regions.

Protected Regions
- `//` line comments; `/* ... */` block comments (no nesting by standard)
- Strings: `"..."`, chars `'c'`
- Raw strings: `R"delim( ... )delim"` with arbitrary `delim`

Algorithm
- DPDA: `brace_depth` outside protected regions.
- When encountering `R"`, collect delimiter up to `(`; close on `)` + delimiter + `"`.

Failures
- Unterminated block comment or raw string; unmatched braces.

Tests
- Raw strings with embedded braces; comment-embedded directive tokens.
