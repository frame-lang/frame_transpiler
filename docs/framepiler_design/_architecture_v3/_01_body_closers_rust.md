# Stage 01 — Body Closers (Rust)

Goal
- Find the matching `}` for Rust native bodies with nested block comments and raw strings.

Protected Regions
- Line comments: `// ...\n`
- Block comments: `/* ... */` with nesting
- Strings: `"..."` and chars `'c'`
- Raw strings: `r#" ... "#` with N `#`

Algorithm
- DPDA: `brace_depth` outside protected; maintain `block_comment_nest` for nested comments.
- Raw string: detect `r` followed by N `#` and `"`, close on `"` followed by the same N `#`.

Failures
- Unterminated nested comment or raw string; unmatched braces.

Tests
- Nested block comments; raw with 1..3 `#`; braces in strings/comments.
