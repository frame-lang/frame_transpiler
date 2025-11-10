# Architecture V3 — Overview

Scope
- Branch: going_native (V3 architecture)
- Languages: Python, TypeScript, C#, C, C++, Java, Rust — developed in lockstep.
- Model: MixedBody/MIR is authoritative in handlers; actions/operations are native‑only. Frame directives are SOL‑anchored (indentation allowed) and ignored inside strings/comments/templates.

Guiding Principles
- Deterministic, linear‑time passes (O(n)): DPDA‑based body closers and region scanners. No regex for language syntax.
- Operate on byte offsets; lines are for diagnostics only via a byte→(line,col) index.
- Single close of bodies: after partitioning, downstream never re‑closes a body.
- Two‑pass handler pipeline: segment → parse tiny Frame statements → assemble MIR → validate → expand → splice. Native parse facades (Stage 07) are runtime‑optional for stricter diagnostics.

Stages
- 01 Module Partitioning
  - `ModulePartitionerV3` with per‑language DPDA body closers.
  - Output: `ModulePartitions { prolog, imports, bodies }` with exact `{…}` byte spans and `BodyKindV3` (handler/action/operation).

- 02 Native Region Segmentation
  - `NativeRegionScannerV3<{python,typescript,csharp,c,cpp,java,rust}>` scans body byte slices.
  - Output: `[RegionV3::NativeText | RegionV3::FrameSegment]` with spans; SOL‑only detection.

- 03 Frame Segment Parser
  - `FrameStatementParserV3` parses `-> $State(args)`, `=> $^`, `$$[+]`/`$$[-]` with balanced‑paren, string‑aware arg splitting.

- 04 MIR Assembly
  - `MirAssemblerV3` builds `MixedBody` preserving origin spans. Validator enforces “terminal‑last”.

- 05 Directive Expansion (per target)
  - `*ExpanderV3` per language emits minimal native glue with correct indentation; no reformatting beyond what native parsers/formatters later do.

- 06 Splice & Mapping
  - `SplicerV3` produces `SplicedBody { bytes, splice_map }` for dual‑origin mapping (Frame/native).

- 07 Native Parse Facade (runtime‑optional)
  - `NativeParseFacade*V3` parses spliced native bodies to surface native syntax/indent diagnostics; diagnostics remap through `splice_map` to Frame spans. Implemented for all languages; disabled by default.

- 08 Source Maps & Codegen
  - Compose final maps from AST/text spans + `splice_map`; emit deterministic code.

- 09 Validation
  - `ValidatorV3` rules: terminal‑last; no Frame directives in actions/ops; per‑language native policies.

Notes on C# Specifics
- C# scanners/closers must handle: verbatim strings (`@"…"`), interpolated strings (`$"…{"expr"}…"`), interpolated‑verbatim (`$@"…"`), raw triple/long quotes (`"""…"""`), character literals, and SOL preprocessor lines (`#if`, `#endif`, etc.). The V3 C# DPDA implementations model these states to avoid false SOL detections.

Authoritative Specs (this folder)
- Body Closers: `01_body_closers_*.md`
- Native Region Scanner: `02_native_region_scanner_*.md`
- Frame Segment Parser: `03_frame_segment_parser.md`
- MIR Assembly: `04_mir_assembly.md`
- Frame Statement Expansion: `05_frame_statement_expansion_*.md`
- Splice & Mapping: `06_splice_and_mapping.md`
- Native Parse Facade: `07_native_parse_facade_*.md`
- Source Maps & Codegen: `08_source_maps_and_codegen.md`
- Validation: `09_validation.md`
- AST & Symbol Integration: `10_ast_and_symbol_integration.md`
- Error Taxonomy: `11_error_taxonomy.md`
- Testing Strategy: `12_testing_strategy.md`

Testing
- All behavioral tests run via the Python runner (`framec_tests/runner/frame_test_runner.py`).
- Per‑phase, per‑language fixtures: `v3_prolog`, `v3_imports`, `v3_outline`, `v3_demos`, with positive and negative cases. Negative fixtures reside under `negative/` and are expected failures.

Performance
- All scanners/closers/segmenters are O(n) in input size with must‑advance guarantees. Mapping and splicing are O(n) in combined body length.
