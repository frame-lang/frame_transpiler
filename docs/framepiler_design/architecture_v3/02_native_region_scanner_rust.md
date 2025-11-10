# Stage 02 — Native Region Scanner (Rust)

Goal
- Segment Rust bodies and detect SOL Frame statements, respecting nested block comments and raw strings.

Protected Regions
- `//`, nested `/* ... */`, strings `"..."`, chars `'c'`, raw `r#" ... "#` with N `#`.

SOL Patterns
- `-> $State(args?)`, `=> $^`, `$$[+/-]` at SOL only.

Rules
- Must-advance; ignore tokens inside comments/strings/raw; emit NativeText and FrameSegment with indent.

Inline end markers
- End at LF, first top‑level semicolon `;`, or start of `//` comment; split into FrameSegment then trailing NativeText.

Multi‑statement on a line
- Frame statements may share a line with native Rust statements when separated by a top‑level `;` or `//` (a `/* ... */` block opens a native region and may span lines).
- Examples (valid): `=> $^; native();`, `=> $^ // trailing comment`, `=> $^ /* block */ native();`
- Without a separator, non‑whitespace tokens after a Frame statement are invalid (parser error).

Tests
- Ignore Frame-statement-like tokens in comments/strings/raw; detect transitions/forwards/stacks at SOL.
