# Stage 01 — Body Closers (C)

Goal
- Return the matching `}` for a Frame body embedding C, scanning bytes while ignoring protected regions.

Protected Regions
- Line comments: `// ...\n`
- Block comments: `/* ... */` (no nesting)
- Strings: `"..."` (C escapes), char literals: `'c'`, `'\n'`, `'\''`

Algorithm
- DPDA by bytes, track `brace_depth` outside protected regions.
- Enter/exit strings with escape handling; consume comments fully.
- Return on `brace_depth == 0` when encountering `}` in normal state.

Failures
- Unterminated comment, unterminated string/char; unmatched braces.

Tests
- Unterminated `/*`, braces inside comments/strings, deep nesting.
