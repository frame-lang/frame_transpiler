# Target Parsers (TypeScript SWC, Python rustpython_parser)

Purpose
- Parse native target language bodies when they contain only native statements (no Frame directives), producing structured AST blocks for validation and emission. Target parsers are validators and structure providers; they do not own Frame semantics.

Roles per target
- TypeScript (SWC)
  - Input: a contiguous native body text slice with `TargetSourceMap` (frame_start_line + offsets).
  - Parsing: we wrap the body into a temporary function or module context to satisfy grammar requirements (no external symbols resolved here).
  - Output: `TypeScriptTargetAst` (subset of swc_ecma_ast) recorded in `ParsedTargetBlock` with source locations.
  - Emission: In the current B2 strategy, we keep custom visitors for directive glue and emit native spans verbatim. The SWC AST remains available for optional validation and future codegen improvements.

- Python (rustpython_parser)
  - Input: a native body text slice (suite) and `TargetSourceMap`.
  - Parsing: tolerant of common Python constructs used in Frame modules; does not attempt to resolve runtime or external imports.
  - Output: Python AST stored in `ParsedTargetBlock` for diagnostics; native‑only bodies are emitted verbatim by the visitor.

Boundary contract with Frame
- The scanner/segmenter ensures Frame directives (e.g., `->`, `=> $^`, `$$[+/-]`) are never passed to target parsers; native bodies that contain directives are handled by the NativeRegionSegmenter + MixedBody/MIR path.
- Mixed bodies: target parsers are skipped; segmented native spans are carried as `MixedBodyItem::NativeText` for deterministic custom emission.

Diagnostics & mapping
- Target parser errors are mapped back to Frame lines via `TargetSourceMap` (frame_start_line + per‑line offsets). Error reports include the target language name, target line, and mapped frame line.
- For native‑only bodies emitted verbatim, visitors may use the parser’s span information to enrich diagnostics (optional).

Limitations & guardrails
- Target parsers do not resolve external modules or perform symbol binding; that belongs to runtime or future analysis passes.
- Target parsers do not interleave Frame semantics; any directive presence defers to segmenter/MIR.

Validation
- Compile‑only fixtures exercising native constructs (arrows, callbacks, imports, comprehensions) without directives validate parser acceptability.
- Negative fixtures ensure directive tokens cause segmentation rather than target parsing.
