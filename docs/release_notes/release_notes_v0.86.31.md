## v0.86.31 — Python Compile Debug Import Hardening

Release date: 2025-11-14

### Highlights
- Follow-up release for #063 to ensure all Python debug trailers in the module compile path are safely embedded (triple-quoted) across all emission branches and that imports succeed consistently.

### Details
- Python: ensured mapping/visitor-map/debug-manifest/emitted trailers are triple-quoted at module scope across the mapping path.
- Confirmed import succeeds with and without `--emit-debug` on representative fixtures.

---
Version: 0.86.31
