# Architecture V3 — Implementation Guide

Purpose
- Orient implementers to stand up the V3 pipeline using the authoritative specs in this folder. Start with the overview, then work stage‑by‑stage using the stage index.

Start Here
- Overview: `architecture_v3_overview.md`
- Stage index: `00_stage_index.md`

How To Use These Docs
- For each stage: read the dedicated spec, implement the named struct(s) in `framec/src/frame_c/v3/…`, and satisfy the Inputs/Outputs/Invariants/Errors/Test Hooks described.
- Keep passes linear and deterministic. Do not re‑close bodies after partitioning. Operate on byte offsets; lines are diagnostics‐only.
- MixedBody/MIR is authoritative for embedded Frame semantics. Only three Frame statements exist in native regions: `-> $State(args)`, `=> $^`, `$$+/-`. `system.return` remains native and is rewritten by visitors.
- Languages in scope from Stage 01: Python, TypeScript, C#, C, C++, Java, Rust. C# is prioritized early due to unique verbatim/interpolated/raw string forms and SOL preprocessor lines.

Implementation Roadmap (Stages)
- 01 Module Partitioning
  - Implement `ModulePartitionerV3` and per‑target body closers.
  - Output exact `{…}` byte ranges; provide optional byte→(line,col) index.
  - Exit criteria: Golden partition fixtures pass; no downstream re‑closing.
- 02 Native Region Scanner (per target)
  - Implement streaming scanners (`NativeRegionScannerV3`) with protected‑region states and SOL detection.
  - Exit criteria: Segments match fixtures; no false positives in strings/comments/templates.
- 03 Frame Statement Parser
  - Implement tiny parser for `-> $State(args?)`, `=> $^`, `$$+/-` only.
  - Exit criteria: Balanced paren handling; malformed Frame statements produce E30x errors.
- 04 MIR Assembly
  - Assemble `MixedBody`; enforce terminal‑Frame‑statement rule.
  - Exit criteria: MixedBody mapping preserved; validator catches trailing natives after terminal.
 - 05 Frame Statement Expansion (per target)
  - Emit minimal native snippets; compute correct indentation from Frame‑statement line.
  - Exit criteria: No broken `elif/else/except/finally` (Py) or `else if` (TS) chains.
- 06 Splice & Mapping
  - Build `SplicedBody` and `splice_map` for dual‑origin mapping.
  - Exit criteria: Round‑trip mapping tests pass for inserted spans.
- 07 Native Parse Facade (optional)
  - Parse spliced body with RustPython/SWC for diagnostics/formatting.
  - Exit criteria: Syntax errors mapped back through `splice_map` correctly.
- 08 Source Maps & Codegen
  - Compose maps from AST/text spans and `splice_map`; emit target code.
  - Exit criteria: Golden maps and breakpoint alignment tests pass.
- 09 Validation
  - Enforce policy (terminal‑last, no Frame statements in actions/ops, language rules).
  - Exit criteria: Negative fixtures report expected E4xx/E5xx/E6xx codes.
- 10 AST & Symbol Integration
  - Keep `Arcanum` authoritative for Frame; native AST/bindings are advisory.
  - Exit criteria: MIR expansion resolves state/param refs via `Arcanum` only.
- 11 Error Taxonomy
  - Implement error classes and message shapes; ensure attribution.
  - Exit criteria: Errors render as `[CODE] message — file:line:col`.
- 12 Testing Strategy
  - Wire per‑stage tests and end‑to‑end transpile‑only suites (Python/TS).
  - Exit criteria: Suites green; performance budgets respected.

Follow‑Up TODOs (Initial)
- Create `framec/src/frame_c/v3/` with module scaffolds matching stage structs.
- Port existing textual closers into `v3/body_closer/{python,typescript}.rs` and adapt to byte‑offset contracts.
- Build segmentation fixtures covering triple‑quotes, f‑strings, and template literals with `${…}`.
- Implement the tiny Frame Segment parser with balanced‑paren, string‑aware arg slicing.
- Add splice mapping round‑trip tests and native parse facades (behind feature flags if needed).
- Integrate validator checks for terminal‑last and language policies.

Notes
- LLVM remains on hold; no V3 work targets LLVM.
- Keep changes hermetic; avoid adding network dependencies to the build.
