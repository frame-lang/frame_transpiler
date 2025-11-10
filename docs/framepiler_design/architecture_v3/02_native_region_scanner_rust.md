# Stage 02 — Native Region Scanner (Rust)

Goal
- Segment Rust bodies and detect SOL Frame statements, respecting nested block comments and raw strings.

Protected Regions
- `//`, nested `/* ... */`, strings `"..."`, chars `'c'`, raw `r#" ... "#` with N `#`.

SOL Patterns
- `-> $State(args?)`, `=> $^`, `$$[+/-]` at SOL only.

Rules
- Must-advance; ignore tokens inside comments/strings/raw; emit NativeText and FrameSegment with indent.

Tests
- Ignore Frame-statement-like tokens in comments/strings/raw; detect transitions/forwards/stacks at SOL.
