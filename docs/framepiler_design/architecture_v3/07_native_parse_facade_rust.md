# Stage 07 — Native Parse Facade (Rust)

Purpose
- Optional parsing of spliced Rust bodies to improve diagnostics/formatting/indent analysis; not required in the critical path.

Design
- `NativeParseFacadeRustV3` trait; adapters to rustc/syn/rust-analyzer are optional and must be gated.
- Diagnostics mapped via `splice_map` to Frame sources.

Acceptance
- Off by default; zero impact when disabled.
- When enabled, produces syntactically aware indentation hints and accurate error mapping.

Tests
- Verify mapping across splice boundaries; ensure nested block comments/raw strings do not break spans.
