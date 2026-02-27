# Stage 02 — Native Region Scanner (C++)

Goal
- Segment C++ bodies and detect SOL Frame statements, ignoring protected regions.

Protected Regions
- `//`, `/* ... */`, strings/char, raw strings `R"delim( ... )delim"`.

SOL Patterns
- Same as C.

Rules
- Ignore tokens within comments/strings/raw; emit NativeText and FrameSegment as in C.

Inline end markers
- End at LF, first top‑level semicolon `;`, or start of `//` comment; split into FrameSegment then trailing NativeText.

Multi‑statement on a line
- Frame statements may share a line with native C++ statements when separated by a top‑level `;` or `//` (a `/* ... */` block opens a native region and may span lines).
- Examples (valid): `=> $^; native();`, `=> $^ // trailing comment`, `=> $^ /* block */ native();`
- Without a separator, non‑whitespace tokens after a Frame statement are invalid (parser error).

Tests
- `ptr->field` at SOL not a transition; Frame-statement-like tokens in comments/strings/raw are ignored.
