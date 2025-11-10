# Stage 02 — Native Region Scanner (Java)

Goal
- Detect SOL Frame statements in Java bodies while skipping protected regions.

Protected Regions
- `//`, `/* ... */`, strings `"..."`, chars `'c'`.

SOL Patterns
- `-> $State(args?)`, `=> $^`, `$$[+/-]` at SOL only.

Rules
- Must-advance; emit NativeText and FrameSegment with indent.

Tests
- Ignore Frame-statement-like tokens in comments/strings; detect transitions at SOL.
