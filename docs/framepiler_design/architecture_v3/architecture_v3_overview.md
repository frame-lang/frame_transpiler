# Architecture V3 — Overview

Scope
- Branch: going_native (V3 architecture)
- Languages: Python, TypeScript, C#, C, C++, Java, Rust — developed in lockstep.
- Model: MixedBody/MIR is authoritative in handlers; actions/operations are native‑only. Frame statements are SOL‑anchored (indentation allowed) and ignored inside strings/comments/templates.

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
  - Inline rule: for eligible languages, a Frame segment ends at the first top‑level semicolon `;` or comment start (Python: `;`/`#`; TS/C#/C/CPP/Java/Rust: `;`/`//`). The remainder of the line is emitted as a trailing native segment.
  - Multi‑statement policy: Frame statements follow the host language’s rules for multiple statements on one physical line. Where the host language requires explicit separators, we end the Frame segment at the first top‑level separator and treat the rest as native. Examples:
    - Python: `=> $^; # comment` and `=> $^; native()` are valid; without a separator, trailing tokens after a Frame statement are invalid.
    - TypeScript/C#/C/C++/Java/Rust: `=> $^; native();` and `=> $^ // comment` are valid; without a top‑level `;` or comment start, trailing tokens after a Frame statement are invalid.

- 03 Frame Segment Parser
  - `FrameStatementParserV3` parses `-> $State(args)`, `=> $^`, `$$[+]`/`$$[-]` with balanced‑paren, string‑aware arg splitting.

- 04 MIR Assembly
  - `MirAssemblerV3` builds `MixedBody` preserving origin spans. Validator enforces “terminal‑last”.

- 05 Frame Statement Expansion (per target)
  - `*ExpanderV3` per language emits minimal native glue with correct indentation; no reformatting beyond what native parsers/formatters later do.

- 06 Splice & Mapping
  - `SplicerV3` produces `SplicedBody { bytes, splice_map }` for dual‑origin mapping (Frame/native).

 - 06.5 Structural Validation (early)
  - `ValidatorV3` enforces transition-as-terminal, disallows Frame statements in actions/ops, and checks state header '{' in machine:.

- 07 Native Parse Facade (runtime‑optional)
  - `NativeParseFacade*V3` parses spliced native bodies to surface native syntax/indent diagnostics; diagnostics remap through `splice_map` to Frame spans. Implemented for all languages; disabled by default.
  - Current state is wrapper-only (validates wrapper lines). Optional native parser adapters (e.g., SWC, rustpython_parser, syn, Tree‑sitter) can be added behind cargo features and `--validate-native`.

- 08 Source Maps & Codegen
  - Compose final maps from AST/text spans + `splice_map`; emit deterministic code.

- 09 Validation
  - `ValidatorV3` rules: terminal‑last; no Frame statements in actions/ops; per‑language native policies.
 
 - 13 Project Layer (Optional — FID/Linking/Packaging)
  - Optional, gated stage for symbol discovery and typed linking; not required for core V3. Provides `.fid` cache generation and project packaging when enabled.

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

## Glossary (V3 Terms)

- **System**  
  A single `system Name { … }` declaration in a module. Each system has at most one `interface:` block, one `machine:` block, and optional `actions:`, `operations:`, and `domain:` blocks.

- **Interface block (`interface:`)**  
  The section inside a system that declares interface methods (e.g., `tick()`, `status(): str = "idle"`). These methods define the public, Frame‑visible API of the system.

- **Interface method**  
  A single method declared under `interface:`. In V3, each interface method has:
  - A Frame header in the `.frm` file, and
  - One or more generated target‑language methods that implement its behavior (wrappers + internal handlers).

- **Interface wrapper (public wrapper)**  
  The generated, consumer‑facing method for an interface method on the target class (Python/TypeScript). It:
  - Accepts only native arguments (no `FrameEvent`/`FrameCompartment` parameters).
  - Constructs a `FrameEvent(message, params)` internally.
  - Calls the system’s router (e.g., `_frame_router(event, compartment, …args)`).
  Wrappers are what user code calls (e.g., `tl.tick()` in Python or `tsSystem.runtimeMessage(payload)` in TypeScript).

- **Internal handler**  
  A generated, implementation detail method that handles a specific `(state, event)` pair. Examples:
  - Python: `_s_Red_tick(self, __e, compartment)`.
  - TypeScript V3 CLI: `_event_runtimeMessage(__e: FrameEvent, compartment: FrameCompartment, payload: T)`.
  Internal handlers are not part of the public API; they are invoked only by the router.

- **Router**  
  The generated dispatch method that routes events to internal handlers based on the current compartment:
  - Python: `_frame_router(self, __e: FrameEvent, compartment: FrameCompartment | None)`.
  - TypeScript: `_frame_router(__e: FrameEvent, c?: FrameCompartment, ...args: any[])`.
  Routers are responsible for consulting `compartment.state` (and `__e.message`) and calling the appropriate internal handler. V3 considers a stubbed router (one that only `void`s variables) incomplete.

- **Multi‑method interface system**  
  A system whose single `interface:` block declares multiple interface methods (e.g., `start`, `runtimeConnected`, `runtimeMessage`, etc.). Some bug reports may refer to these informally as “multi‑interface systems”; V3 does **not** support multiple independent `interface:` blocks per system, so “multi‑interface system” always means “one system with one interface block and several interface methods”.

- **Multi‑entity module**  
  A `.frm` file that contains more than one system and/or top‑level `fn` functions. V3 treats each system independently in the outer pipeline; modules with multiple systems are common in CLI and test fixtures.

- **Facade wrapper (Stage 07)**  
  A synthetic call inserted by the native parse facades (Stage 07) around expanded Frame statements, used only for strict native syntax validation and exec‑smoke (e.g., `__frame_transition("State", …)` in C/C++/Java/C# facades). These wrappers are distinct from interface wrappers:
  - Facade wrappers exist only in spliced demo/module outputs and are used for diagnostics and smoke tests.
  - Interface wrappers are part of the public API for end users in Python/TypeScript.

- **Start state (V3)**  
  The initial state for a system at runtime. In V3, when no explicit start annotation exists, the start state is:
  - The **first state header** discovered in the system’s `machine:` block, in textual order, as recorded by `Arcanum`, and
  - Used to seed the initial `FrameCompartment` in the runtime (`FrameCompartment("__System_state_Start", …)`).
  Hard‑coding `A` as the start state is considered legacy behavior and is replaced in V3 by this AST‑backed selection.
