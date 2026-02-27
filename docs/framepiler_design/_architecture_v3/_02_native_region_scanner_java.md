# Stage 02 — Native Region Scanner (Java)

Goal
- Detect SOL Frame statements in Java bodies while skipping protected regions.

Protected Regions
- `//`, `/* ... */`, strings `"..."`, chars `'c'`.

SOL Patterns
- `-> $State(args?)`, `=> $^`, `push$`, `pop$` at SOL only.

Rules
- Must-advance; emit NativeText and FrameSegment with indent.

Inline end markers
- End at LF, first top‑level semicolon `;`, or start of `//` comment; split into FrameSegment then trailing NativeText.
 

Multi‑statement on a line
- Frame statements may share a line with native Java statements when separated by a top‑level `;` or `//` (a `/* ... */` block opens a native region and may span lines).
- Examples (valid): `=> $^; native();`, `=> $^ // trailing comment`, `=> $^ /* block */ native();`
- Without a separator, non‑whitespace tokens after a Frame statement are invalid (parser error).

Tests
- Ignore Frame-statement-like tokens in comments/strings; detect transitions at SOL.
