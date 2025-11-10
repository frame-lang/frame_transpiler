# Stage 02 — Native Region Scanner (C)

Goal
- Stream segments in C bodies and detect SOL-anchored Frame statements.

SOL Detection
- At SOL only (space/tab allowed):
  - `-> $State(args?)`
  - `=> $^`
  - `$$[+]`, `$$[-]`

Protected Regions
- `//`, `/* ... */`, strings `"..."`, chars `'c'`.

Rules
- O(n), must-advance. Emissions:
  - NativeText spans between Frame statements
  - FrameSegment spans for heads through EOL with captured indent

Tests
- False positives avoided inside comments/strings; transitions at SOL detected.
