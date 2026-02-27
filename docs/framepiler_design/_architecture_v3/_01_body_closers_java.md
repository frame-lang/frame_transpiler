# Stage 01 — Body Closers (Java)

Goal
- Find the matching `}` for Java native bodies.

Protected Regions
- Line comments: `// ...\n`
- Block comments: `/* ... */`
- Strings: `"..."` with escapes; char literals `'c'`

Algorithm
- DPDA: track `brace_depth`; treat strings/comments as protected and skip their content.

Failures
- Unterminated comment/string; unmatched braces.

Tests
- Braces in strings/comments; unterminated comment; nested blocks.
