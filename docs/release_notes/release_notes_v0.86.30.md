## v0.86.30 — Python Compile Import Fix (Bug #063)

Release date: 2025-11-14

### Highlights
- Fixes #063: normalize indentation in Python handler bodies when emitting runnable modules so compiled modules import cleanly.

### Details
- Python compile module emitter now left-strips each spliced line and re-indents to the method block level, preventing IndentationError.
- Builds on v0.86.29 where trailers became Python-safe (triple-quoted) and errors-json honored --emit-debug.

### Compatibility
- No changes to demo/exec paths.
- No changes to TypeScript/Rust emit behavior in this patch.

---
Version: 0.86.30
