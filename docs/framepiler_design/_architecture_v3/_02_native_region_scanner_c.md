# Stage 02 — Native Region Scanner (C)

Goal
- Stream segments in C bodies and detect SOL-anchored Frame statements.

SOL Detection
- At SOL only (space/tab allowed):
  - `-> $State(args?)`
  - `=> $^`
  - `push$`, `pop$`

Protected Regions
- `//`, `/* ... */`, strings `"..."`, chars `'c'`.

Rules
- O(n), must-advance. Emissions:
  - NativeText spans between Frame statements
  - FrameSegment spans for heads through EOL with captured indent

Inline end markers
- A Frame segment ends at the earliest of LF, the first top‑level semicolon `;`, or the start of a `//` line comment. The scanner splits the line, emitting the Frame segment first and then a trailing NativeText segment.

Multi‑statement on a line
- Frame statements may share a line with native C statements when separated by a top‑level `;` or `//` (a `/* ... */` block opens a native region and may span lines).
- Examples (valid): `=> $^; native();`, `=> $^ // trailing comment`, `=> $^ /* block */ native();`
- Without a separator, non‑whitespace tokens after a Frame statement are invalid (parser error).

Tests
- False positives avoided inside comments/strings; transitions at SOL detected.
