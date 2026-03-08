# Stage 01 — Body Closers (C#)

Goal
- Find the matching closing `}` of a Frame body that contains C# native code, scanning bytes and respecting protected regions.

Inputs
- Source bytes for the file
- Byte index of the `{` opening the body

Outputs
- `close_byte`: byte index of the matching `}` for the body
- On failure: characterized error (unterminated string/comment/raw string, unmatched braces, etc.)

Protected Regions (C#)
- Line comments: `//` (consume until newline)
- Doc comments: `///` (treated same as `//`)
- Block comments: `/* ... */` (no nesting)
- Strings:
  - Normal: `"..."` with `\` escapes
  - Verbatim: `@"..."` where `""` escapes a single quote
  - Interpolated: `$"..."` with `{ ... }` interpolation (balanced braces)
  - Interpolated+verbatim: `$@"..."` or `@$"..."` (both orders)
  - Raw (C# 11+): `""" ... """` (N≥3 quotes) optionally prefixed by one or more `$` for interpolation; close requires the same N quotes. Interpolation braces require as many `{`/`}` as the number of leading `$` characters (e.g., `$$` ⇒ `{{` / `}}`).
- Char literals: `'c'`, `'\n'`, `\uXXXX` (single-quoted; brace depth ignored inside)
- Preprocessor lines: `#region`, `#if`, etc., SOL-only; consume to end-of-line as native

Algorithm (DPDA-style)
- Maintain: `brace_depth`, `at_sol`, `state`
- `state` covers: Normal/AtSOL, InStringDouble, InStringSingle, InVerbatim, InInterp{Regular,Verbatim}{brace_depth}, InRaw{quote_count,dollar_count}, InRawInterp{quote_count,dollar_count,brace_depth}, InBlockComment, InLineComment, InPreprocessor
- Increment/decrement `brace_depth` only in Normal/AtSOL states
- Openers/closers for strings/comments transition to protected states; interpolation states track nested braces
- Body closes when `brace_depth` returns to zero and `}` is encountered in Normal/AtSOL

Failure Characterization
- Unterminated raw string (missing N quotes on a subsequent line)
- Unterminated block comment
- Unterminated string/verbatim/interpolated/char literal

Test Hooks
- Fixtures under `framec_tests/v3/01_closers/csharp/`
  - Verbatim with doubled quotes; multi-line
  - Interpolated with nested braces and nested strings
  - Interpolated+verbatim with path/backslash content
  - Raw strings with/without interpolation; different `$` counts; SOL closer
  - Comments containing Frame-statement-like tokens
  - Preprocessor at SOL

Exit Criteria
- Closer returns correct byte index across fixtures
- No rescans/re-closing later stages; byte indices are authoritative
