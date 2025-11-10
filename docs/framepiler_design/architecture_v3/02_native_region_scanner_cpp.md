# Stage 02 — Native Region Scanner (C++)

Goal
- Segment C++ bodies and detect SOL Frame statements, ignoring protected regions.

Protected Regions
- `//`, `/* ... */`, strings/char, raw strings `R"delim( ... )delim"`.

SOL Patterns
- Same as C.

Rules
- Ignore tokens within comments/strings/raw; emit NativeText and FrameSegment as in C.

Tests
- `ptr->field` at SOL not a transition; Frame-statement-like tokens in comments/strings/raw are ignored.
