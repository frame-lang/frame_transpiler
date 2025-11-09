# V3 Naming Index — Pipeline Objects and Traits

Purpose
- Provide a single reference for the concrete Rust types (structs/traits) that implement each stage of the V3 pipeline. Names are stable targets for implementation and review.

Conventions
- Suffix `V3` for new/authoritative components.
- Per‑target implementations use `{Py,Ts}` suffixes.
- Traits define stage contracts; structs are concrete implementations.

Stage 01 — Module Partitioning & Body Closers
- Trait: `BodyCloserV3`
  - fn `close(&self, src: &[u8], open_idx: usize) -> Result<usize, CloseError>`
- Structs: `BodyCloserPyV3`, `BodyCloserTsV3`

Stage 02 — Native Region Scanners (Streaming, Protected‑Region Aware)
- Trait: `NativeRegionScannerV3`
  - fn `scan(&self, src: &[u8], open_idx: usize) -> Result<ScanResultV3, ScanError>`
- Structs: `NativeRegionScannerPyV3`, `NativeRegionScannerTsV3`
- Data: `ScanResultV3 { close_byte: usize, regions: Vec<RegionV3> }`
- Data: `RegionV3::{ NativeText{ start, end }, FrameSegment{ start, end, kind_hint: FrameStmtKindV3, indent: usize } }`
- Data: `FrameStmtKindV3::{ Transition, Forward, StackOp }`

Stage 03 — Frame Statement Parser (FIRST‑set)
- Trait: `FrameStatementParserV3`
  - fn `parse(&self, src: &[u8], seg: &RegionV3) -> Result<MirItemV3, ParseError>`
- Helpers: `NativeArgSplitterV3`
  - fn `validate_balanced(&self, text: &str) -> Result<(), ParseError>`
  - fn `split_top_level_commas(&self, text: &str) -> Vec<&str>`
- Structs: `FrameStatementParserPyV3`, `FrameStatementParserTsV3`
- Helpers: `NativeArgSplitterPyV3`, `NativeArgSplitterTsV3`

Stage 04 — MIR Assembly
- Struct: `MirAssemblerV3`
  - fn `assemble(&self, regions: &[RegionV3]) -> Vec<MirItemV3>`
- Data: `MirItemV3::{ Transition{ target: String, args: Vec<String>, span: ByteSpan }, Forward{ span }, StackPush{ span }, StackPop{ span } }`

Stage 05 — Frame Statement Expansion (Per Target)
- Trait: `FrameStatementExpanderV3`
  - fn `expand(&self, mir: &MirItemV3, ctx: &ExpandCtxV3) -> Expansion`
- Structs: `FrameStatementExpanderPyV3`, `FrameStatementExpanderTsV3`
- Optional helpers: `IndentationAnalyzerV3`
  - fn `derive_indent(&self, frame_stmt_span: ByteSpan, native_ast: &NativeAst) -> String`
- Structs: `IndentationAnalyzerPyV3`, `IndentationAnalyzerTsV3`

Stage 06 — Splice & Mapping
- Struct: `SplicerV3`
  - fn `splice(&self, native: &str, expansions: &[Expansion]) -> SplicedBodyV3`
- Struct: `SourceMapComposerV3`
  - fn `compose(&self, spliced: &SplicedBodyV3) -> SourceMapV3`

Stage 07 — Native Parse Facades (Optional, Diagnostics/Formatting)
- Trait: `NativeParseFacadeV3`
  - fn `parse(&self, text: &str) -> Result<NativeAst, NativeParseError>`
- Structs: `NativeParseFacadePyV3`, `NativeParseFacadeTsV3`

Stage 08 — Codegen
- Visitors (existing): `PythonVisitor`, `TypeScriptVisitor`
- Adapters (optional): `TsB2CodegenV3`, `PyB2CodegenV3` (future)

Stage 09 — Validation
- Struct: `ValidatorV3`
  - fn `validate(&self, mixed: &[MirItemV3]) -> Vec<ValidationError>`
- Rules: `TerminalLastRuleV3`, `NoFrameStatementsInActionsOpsRuleV3`, `PythonNativePolicyRuleV3`, etc.

Common Types
- `ByteSpan { start: usize, end: usize }`
- `ExpandCtxV3 { frame_line: usize, sibling_indent: usize, target: Target }`
- `Expansion { text: String, frame_line: usize }`
- `SplicedBodyV3 { text: String, splice_map: Vec<(ByteSpan, Origin)> }`
- `Origin::{ FrameStatement{ frame_line }, NativeText{ start_line, end_line } }`

Notes
- system.return remains native; not a Frame statement. Rewritten during expansion.
- Frame statements are SOL‑anchored and recognized outside protected regions only.

Project / Multi‑File Layer (Planning)
- File and module management
  - `FileLoaderV3` — loads source text per file; provides content hashes for caching
  - `ModuleResolverV3` — resolves imports → `ModuleId`; honors FID locations
  - `ProjectGraphV3` — builds DAG of modules for topological build execution
- FID (Frame Import Descriptor)
  - `FIDIndexV3` — cache of loaded FIDs for imported modules
  - `FIDEmitterV3` — emits FID for compiled modules (exports: systems, interfaces, signatures)
  - `FIDSchemaV3` (doc/schema):
    - `module_id: String`
    - `exports: { systems: [SystemSig], interfaces: [InterfaceSig] }`
    - `SystemSig { name: String, states: [StateSig] }`
    - `InterfaceSig { name: String, methods: [MethodSig] }`
    - `MethodSig { name: String, params: [ParamSig], return_type: Option<String> }`
    - `ParamSig { name: String, type: Option<String> }`
- Cross‑file validation
  - `SemanticAnalyzerV3` — validates cross‑module references against `FIDIndexV3`
  - Rules: `InterfaceCompatRuleV3`, `ImportResolutionRuleV3`
- Linking / Packaging
  - `TsModuleLinkerV3` — normalizes TS import paths; dedupes runtime import
  - `PythonPackagePlannerV3` — emits package layout (__init__.py), relative imports, dedupes runtime import
  - `CodegenDeduperV3` — ensures one shared runtime import per module set
- Incremental build
  - `BuildPlannerV3` — selects changed files + dependents using DAG
  - `RegionScanCacheV3`, `MirCacheV3` — keyed by file content hash
