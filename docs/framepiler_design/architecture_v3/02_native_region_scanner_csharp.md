# Stage 02 — Native Region Scanner (C#)

Goal
- Stream and classify native C# body text into segments: NativeText and SOL-anchored Frame segments (Transition, Forward, Stack ops), ignoring protected regions.

Inputs
- Source bytes and body span
- Byte→(line,col) index (optional) for diagnostics

Outputs
- `ScanResultV3 { close_byte, regions: Vec<RegionV3> }`
- Each `RegionV3` carries precise byte spans and, for Frame segments, computed indent

SOL Detection (C#)
- Recognize directives only at start-of-line (Unicode whitespace allowed)
- Patterns:
  - Transition: `-> $State(args?)`
  - Forward: `=> $^`
  - Stack ops: `$$[+]`, `$$[-]`
- Do not match in protected regions (strings/comments/preprocessor)

Protected Regions
- Same as Body Closers, plus interpolation brace tracking for `$"..."`, `$@"..."`, and raw interpolated strings with `$` count.

States
- Normal, AtSOL
- InStringDouble, InStringSingle, InVerbatim
- InInterpRegular { depth }, InInterpVerbatim { depth }
- InRaw { quotes, dollars }, InRawInterp { quotes, dollars, depth }
- InBlockComment, InLineComment, InPreprocessor

Rules
- Must-advance (no stalling); O(n)
- Record NativeText segments between Frame segments; compute indent as leading whitespace at the Frame line
- Ignore `=>` lambda unless followed by `$^` at SOL

Errors
- Malformed Frame head at SOL (E30x): looks like Frame directive but missing required tokens
- Unterminated protected regions (carried through from Stage 01 if present)

Fixtures
- `framec_tests/v3/02_scanner/csharp/*` covering:
  - Directives in comments/strings/preprocessor
  - Lambdas `() => expr` at SOL (no match)
  - Interpolated raw strings with nested braces
  - Verbatim and normal strings with escaped quotes

Exit Criteria
- Segments match golden fixtures; no false positives in protected regions
