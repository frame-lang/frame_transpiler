## v0.86.29 — Python Trailer Safety + Debug Flag Honor (Fixes #061/#062)

Release date: 2025-11-14

### Highlights
- Python-safe trailer embedding: trailers are placed inside a triple-quoted string at module scope, preserving Python syntax.
- Errors-JSON is now emitted only when explicitly enabled (e.g., via `--emit-debug` which sets `FRAME_ERROR_JSON=1`).
- Fixes:
  - #061 Python trailer block style invalid
  - #062 Python emit-debug flag ignored

### Details
- Compiler (V3 module path):
  - For Python, debug trailers (frame-map, visitor-map v2, debug-manifest v2, errors-json) are embedded inside a harmless triple-quoted string using existing sentinels.
  - Errors-JSON trailer is now gated by `FRAME_ERROR_JSON=1` to correctly honor `--emit-debug`.
- No changes to TypeScript/Rust trailer style in this patch; both remain unchanged and continue to work with runner extraction.

### Compatibility
- No changes to demo/exec paths.
- Python modules import cleanly both with and without `--emit-debug` (no unintended syntax issues).

---
Version: 0.86.29
