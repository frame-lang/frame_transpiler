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
- Recognize Frame statements only at start-of-line (Unicode whitespace allowed)
- Patterns:
  - Transition: `-> $State(args?)`
  - Forward: `=> $^`
  - Stack ops: `push$`, `pop$`
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

Inline end markers
- A Frame segment ends at the earliest of:
  - end of line, or
  - the first top‑level semicolon `;`, or
  - the start of a `//` line comment.
- The scanner splits the line accordingly and emits `FrameSegment` then trailing `NativeText`.

Multi‑statement on a line
- Frame statements may share a line with native C# statements when separated by a top‑level `;` or `//` (a `/* ... */` block opens a native region and may span lines). Preprocessor lines are SOL but do not affect scanning inside bodies.
- Examples (valid): `=> $^; native();`, `=> $^ // trailing comment`, `=> $^ /* block */ native();`
- Without a separator, non‑whitespace tokens after a Frame statement are invalid (parser error).

Errors
- Malformed Frame head at SOL (E30x): looks like a Frame statement but is missing required tokens
- Unterminated protected regions (carried through from Stage 01 if present)

Fixtures
- `framec_tests/v3/02_scanner/csharp/*` covering:
  - Frame-statement-like tokens in comments/strings/preprocessor
  - Lambdas `() => expr` at SOL (no match)
  - Interpolated raw strings with nested braces
  - Verbatim and normal strings with escaped quotes

Exit Criteria
- Segments match golden fixtures; no false positives in protected regions
