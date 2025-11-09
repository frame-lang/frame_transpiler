Python Parsing — Interim Status and Plan (Going Native)

Context
- Branch: going_native
- Scope: single‑file Python target with native bodies and SOL‑anchored Frame statements inside event handlers only.
- Model: MixedBody/MIR is authoritative in handlers; actions/operations are native‑only. Pseudo‑symbol `system.return` is allowed in actions/ops/handlers and rewritten in visitors.

Current Implementation (Parser)
- Handler path: `event_handler_mixed(..)` (Parser)
  - Parses header: message, parameters, optional `: return_type` and optional `= default_return`.
  - Enters handler‑local scope; consumes `{` and locates matching `}` via a textual detector for Python that is aware of quotes/triple‑quotes and `#` comments.
  - Slices the body and partitions it via the Python native segmenter into BodySegments: Native and SOL‑anchored FrameStmt (->, => $^, $$[+/-], system.return).
  - Builds MixedBody from segments; no parser‑level statement parsing inside handlers (MixedBody drives emission).
- Body closer (Python): Parser methods `detect_py_close_or_failure(..)` and `scan_py_closing_brace_line(..)` implement a deterministic pushdown scan (brace depth + protected regions) with failure characterization (e.g., unterminated triple quote). `consume_close_brace_at_line(..)` advances the token cursor exactly to that brace.
- Segmenter (Python): `native_region_segmenter/python.rs` performs an SOL‑aware scan that ignores strings/triple‑quotes/f‑strings and yields BodySegments. Today this is line‑loop based; a streaming region scanner is planned.
- Validator: `transitions_terminal` enforces that terminal Frame statements (Transition/Forward/Stack ops) are last in a handler MixedBody. `Return` is not terminal.

Current Implementation (Visitor)
- Python visitor emits MixedBody in order:
  - NativeText: emitted verbatim with indentation preserved.
  - MIR: emits transition/forward/stack glue and early returns; rewrites `system.return` to `self.return_stack[-1]`.
- Source maps: anchors exist; mapping polish is tracked separately.

Observed Problems
- Indentation/elif chain breaks in nested conditionals:
  - In fixtures like `language_specific/python/control_flow/test_if_elif_returns.frm`, inserted MIR `return` lines disrupt `elif/else` alignment. Needs consistent indentation computation and suppression of extra `return` lines when the next native line is itself a `return`.
- Body close/ownership with nested states:
  - In `language_specific/python/control_flow/test_forward_events.frm`, parser reports `Expected '}' at actions:`. Root cause is interaction between handler body close detection/consumption and the loops that parse nested states after handlers. We need to keep the “guarded closer” policy: only use the textual closer when triple‑quotes/f‑string markers are present; otherwise use the stable token‑depth path to avoid mis‑alignment with subsequent state parsing.
- Suite run timeouts (previously stalls):
  - Legacy handler loops could stall; we’ve replaced them with must‑advance guards and a MixedBody‑only handler path. Single‑file invocations return promptly; long category runs are now bounded by the number of fixtures rather than a spin, but we still need to avoid redundant rescans.
- Line‑based segmentation:
  - Current Python segmenter walks line‑by‑line for SOL checks. This complicates precise region slicing and remapping; a streaming region scanner by byte offsets is preferable for correctness and performance symmetry with TS.

Outstanding Work (Ordered)
1) Reinstate guarded closer policy for Python handlers
   - Pre‑scan for triple‑quotes/f‑string markers within the candidate body; if present, use the textual DPDA closer; otherwise prefer token‑depth close to avoid over/under‑consumption. Always consume the final `}` via `consume_close_brace_at_line` to keep ownership precise.

2) Fix Python visitor indentation and post‑terminal suppression
   - Compute indent from the nearest native sibling; insert MIR `return` at the correct depth; suppress our bare `return` if the next native line is an explicit `return`. Preserve `elif/else/except/finally` chains.

3) Streaming region scanner (Python)
   - Add `native_partition_scanner/python.rs` with a DPDA that tracks quotes/triple‑quotes/f‑strings and a single brace counter. Produce region spans by byte offsets with FIRST‑set detection at SOL, ignoring protected regions. Precompute byte→line offsets so MixedBody carries accurate line info. Replace the line‑based segmenter.

4) Frame‑statement mini‑parsers (FIRST‑set)
   - Implement minimal DFAs for: `-> $State(args)`, `=> $^`, `$$[+/-]`, and `system.return = expr`. Integrate with the region scanner so MixedBody contains MIR directly (not `FrameStmt` placeholders).

5) Must‑advance invariants and early‑out
   - Ensure all handler continuation and post‑handler loops advance on no‑progress; add SOL fast‑path/early‑out when no FIRST‑set tokens are present in a region.

6) Tests and fixtures
   - Keep language‑specific Python fixtures fully native and validator‑verified. Add incremental handler fixtures (00→10) and torture cases (strings/triple‑quotes/f‑strings, Unicode). Lock negatives for legacy constructs in Python bodies.

7) Cleanup and legacy removal
   - Remove/dead‑code legacy handler parsing once Python suite is green. Keep docs and test index consistent with “handlers only” MixedBody policy and “actions/ops native‑only”.

Architectural Notes
- DFA vs DPDA: A pure DFA cannot count braces; the closer uses a deterministic pushdown scan (single counter) plus protected‑region flags to work in O(n). FIRST‑set detection for Frame statements is DFA‑like at SOL and combined with the DPDA’s protected‑region state.
- Region‑based partitioning: Move from line slicing to region spans. Lines remain useful for diagnostics but should derive from byte offsets, not control scanning.
- Unicode: SOL detection must honor all Unicode whitespace. Scanning operates on bytes for ASCII delimiters (‘{’, ‘}’, quotes) while SOL uses `char.is_whitespace()`; retain byte→line mapping for accuracy.

Acceptance Criteria
- Python language_specific suite: 100% transpile + validate; execution ≥95% with known service/infinite‑loop exclusions.
- No handler stalls; O(n) scanning with must‑advance guards; no mis‑owned braces between handler bodies and enclosing state blocks.
- MixedBody contains only Native and MIR (no parser‑level statements); actions/ops remain native‑only with `system.return` rewrite.

Short‑Term Plan
- Implement guarded closer + visitor indentation fixes → re‑run language_specific_python.
- Replace Python line‑segmenter with streaming region scanner + FIRST‑set mini‑parsers.
- Clean up remaining fixtures and negatives; then broaden to TypeScript polish.

This file is a temporary status and plan checkpoint for the Python path under the “Going Native” effort. It will be folded into the permanent docs once the implementation stabilizes.

Architecture Update — Region-Based Partitioning (No Line Slicing)

- Problem statement
  - Prior segmentation logic “sliced by lines” for convenience (easy SOL checks and line-based diagnostics). This is not a sound boundary for Python native code (triple quotes, f-strings, multi-line expressions) and creates brace-ownership and indentation artifacts.

- Why line slicing is a liability
  - Multi-line constructs cross line boundaries; lines aren’t syntactic units.
  - Divergence between the body closer and the line-based segmenter causes mismatched ownership of the final ‘}’ and follow-on blocks (e.g., actions: after handlers).
  - Requires repeated joins/splits and duplicate scans; complicates accurate source mapping.

- Decision
  - Move to a single streaming, region-based scanner per language. Emit regions by byte offsets; compute line/column only for diagnostics/source maps via a precomputed byte→line index. Never partition by line boundaries.

- Region scanner shape (Python)
  - Deterministic pushdown scan tracks: single/double quotes, triple quotes, f-strings (brace-aware), and `#` comments; plus a single brace-depth counter for the Frame body.
  - Maintain `at_sol` (true after newline and before first non-whitespace outside protected regions). FIRST-set detection (->, => $^, $$[+/-], system.return) triggers a flush of the preceding Native span and invokes a small Frame‑statement mini‑parser.
  - One pass yields: (a) the closing ‘}’ location, (b) a sequence of regions [(Native|FrameStmt/MIR, start_byte, end_byte)].

- Parser integration
  - Use the scanner’s computed close location to consume the exact CloseBrace token. Build MixedBody directly from emitted regions (prefer MIR items once mini-parsers land). No second pass and no line slicing.

- Implications
  - Simplifies correctness and performance (still O(n)); eliminates mismatches between closer and segmenter; improves robustness with Unicode and complex native constructs.

- Action items
  - Implement `native_partition_scanner/python.rs` (streaming DPDA + FIRST-set detection) and remove the line-based segmenter.
  - Integrate scanner output into `event_handler_mixed(..)` and retire legacy handler-body parsing paths.
  - Keep diagnostics stable by mapping byte offsets to lines for messages and source maps.

Mini‑Report — Review and Plan (2025‑11‑09)

Summary
- Scope and direction are solid: handlers use MixedBody/MIR; actions/ops are native‑only; SOL‑anchored Frame statements only in handlers; DPDA‑style boundary detection.
- The most impactful improvement is replacing line‑based segmentation with a single streaming, region‑based scanner that yields byte‑spanned regions and the exact closing brace.
- Visitor correctness needs targeted work around indentation and terminal Frame statement suppression to avoid breaking `elif/else/except/finally` chains.

Gaps / Clarifications
- system.return semantics
  - Current text implies a FIRST‑set mini‑parser for `system.return = expr`. Recommendation: treat `system.return` strictly as a native pseudo‑symbol (usable in expressions/assignments) and rewrite during visiting; do not model it as a MIR Frame statement.
- Closer policy
  - “Guarded closer” suggests token‑depth unless triple‑quotes/f‑strings are detected. For Python, non‑string multi‑line constructs can still confound a token‑depth approach. Recommendation: make the textual DPDA closer the default for Python bodies; only keep a fast‑path if formally proven equivalent and measurably faster.
- Multi‑line `system.return`
  - If allowed (via parentheses or line continuation), define handling explicitly. If not allowed, document as single‑line only.
- Error taxonomy
  - Document concrete error classes (e.g., unterminated triple quote, stray closing brace, unmatched `)` in transition args) and message shapes to align diagnostics with scanner states.
- Validation coverage
  - Ensure negatives for forbidden Frame constructs inside Python native bodies (`var`, brace‑style control, nested `def`), and false‑positive Frame statement lexemes inside strings/comments.

First‑Principles Plan (Refined)
- Architecture principles
  - Parser owns Frame; per‑target scanners own native body slicing and Frame statement detection.
  - MixedBody is authoritative; MIR contains only true Frame Frame statements: transition (`-> $State(args)`), parent forward (`=> $^`), and stack ops (`$$[+/-]`).
  - `system.return` remains native and is rewritten in visitors; never emitted as MIR.
- Python body boundary detection
  - Use an always‑on textual DPDA closer tracking single/double/triple quotes, f‑strings, and `#` comments with a single brace counter; O(n) scan.
- Streaming region scanner (Python)
  - One pass from body open to close; maintain protected‑region states and `at_sol` flag; when at SOL and not protected, test FIRST‑set.
  - Emit regions as byte spans: NativeText and FrameDirective(kind, slice/meta). Mini‑parsers at SOL for:
    - Transition: `-> $Name(args)` with balanced `(`…`)` and basic string awareness.
    - Forward: `=> $^`.
    - Stack ops: `$$+` / `$$-`.
- Parser integration
  - Consume the exact `}` using the scanner’s close location; build MixedBody directly from regions; no line slicing or rescans.
- Visitor correctness (Python)
  - Compute indentation from the nearest native sibling at the same depth; don’t break `elif/else/except/finally` chains.
  - Suppress redundant `return` when a terminal MIR is followed by an explicit native `return`.
  - Rewrite `system.return` via protected‑region aware pass (or native AST for actions/ops) to avoid touching strings/comments.
- Errors and diagnostics
  - Enumerate and map scanner states to human messages; include byte→line mapping for locations.
- Validation and tests
  - Positive: strings, triple‑quotes, f‑strings with braces, dict/set literals with nested braces, unicode whitespace at SOL, large handlers, terminal Frame statements at end.
  - Negative: Frame statement‑like tokens in strings/comments, nested `def`, `var` in Python body, unterminated strings, unmatched `)` in transition args.
- Incremental rollout
  - Phase 1: Always‑on DPDA closer + visitor indentation/suppression fixes; rerun Python language_specific (transpile‑only + validator).
  - Phase 2: Streaming region scanner + mini‑parsers; remove line‑based segmenter.
  - Phase 3: Pure native `system.return` rewrite; remove any MIR handling for it.
  - Phase 4: Harden diagnostics and negatives; finalize acceptance criteria.

Suggested Doc Updates
- Clarify: `system.return` is native‑only, rewritten in visitors; not a MIR Frame statement.
- Make closer policy explicit: textual DPDA as the default for Python; define any fast‑path conditions if retained.
- Add an “Errors and Diagnostics” section enumerating error classes and message shapes.
- Specify mini‑parser tokens and whitespace rules precisely (transition/forward/stack ops).
- State the mapping contract: scanners produce byte spans; diagnostics/source maps derive line/column via a precomputed index.

Open Questions
- Should actions/ops leverage a native AST (e.g., RustPython) for better diagnostics, or remain passthrough with protected‑region rewrites only?
- Do we support multi‑line `system.return` assignments? If yes, define the allowed forms and detection; if no, document prohibition.
- What is the exact grammar for state names in Frame statements (e.g., allow dotted paths vs. `$CamelCase` only)? Align with the common grammar and document here.
V3 Parser Rebuild — Authoritative Overview

Purpose
- Define the authoritative plan to rebuild the Frame parser from first principles with a clear separation of concerns, deterministic scanning, and multi‑pass processing that is overall O(n).

Core Principles
- Partitioning owns body bounds: The ModulePartitioner computes exact byte offsets for every `{ … }` body using per‑target textual closers (brace/string/comment aware). Later stages never “search for the closing brace” again; they trust the recorded range.
- Byte offsets drive algorithms: Scanning, segmentation, and parsing operate on bytes and protected‑region state. Line/column exists only for diagnostics and source maps via a precomputed byte→(line, col) index.
- MixedBody/MIR is authoritative: Handlers are modeled as a sequence of Native segments and MIR Frame statements. MIR includes only true Frame Frame statements embedded in native regions: transition (`-> $State(args)`), parent forward (`=> $^`), and stack ops (`$$+` / `$$-`).
- system.return is native: `system.return` is a native pseudo‑symbol usable in handlers and actions/ops; visitors rewrite it to target‑specific return storage. It is not a MIR Frame statement and is not parsed by the Frame Segment parser.
- SOL‑anchored detection: Frame Frame statements are recognized only at start‑of‑line (ignoring indentation) and only when outside strings/comments/templates. Per‑language segmenters must be string/comment/brace aware; do not use regex for language syntax.
- Multi‑pass is encouraged: We perform multiple simple, linear passes (scan → segment → parse tiny Frame statements → expand → splice → optional native parse) rather than forcing clever single‑pass contortions.

Authoritative Pipeline (V3)
1) Module Partitioning
   - Detect `@target` and outline the module.
   - For each member body (handlers, actions, ops), compute `{ … }` byte range using the per‑target textual body closer:
     - Python: triple‑quote + f‑string aware DPDA closer.
     - TypeScript: template/backtick aware DPDA closer.
   - Record body start/end byte offsets in BodyPartition metadata.

2) Native Region Segmentation (per body range)
   - Streaming DPDA over bytes with protected‑region flags (strings/templates/comments) and a single brace counter for the Frame body.
   - Maintain `at_sol` (true after newline and before first non‑whitespace outside protected regions).
   - Emit segments with byte spans: `NativeText { start, end }` and `FrameSegment { start, end, kind_hint }` where `kind_hint` is derived from FIRST‑set at SOL.

3) Frame Segment Parser (tiny, specialized)
   - Runs only on `FrameSegment` byte slices; builds MIR items:
     - Transition: `-> $State(args?)` with balanced parentheses (string‑aware inside the arg list).
     - Forward: `=> $^`.
     - Stack ops: `$$+` / `$$-`.
   - No `system.return` here (native rewrite only).

4) MIR Assembly
   - Construct MixedBody as an ordered sequence: `NativeText` + MIR items, preserving original byte spans for mapping.
   - Validate structural rules at MIR level (e.g., terminal Frame statements must be last in a handler body).

5) Directive Expansion (language‑specific visitors)
   - Expand MIR into target‑native statements (text or AST) with correct indentation derived from the Frame statement’s line context.
   - Keep expansions pure and minimal; rely on runtime helpers for shared semantics where needed.

6) Splice + Optional Native Parse
   - Splice expansions into the native text using a splice map from original Frame byte spans to inserted native spans.
   - Optionally parse the full spliced body with the native parser (e.g., RustPython/SWC) to produce a uniform native AST for formatting and later passes.

7) Codegen + Source Maps
   - Emit code from the native AST (preferred) or from concatenated text while maintaining source maps.
   - Use byte→line index and the splice map to attribute diagnostics to original Frame or native origins.

8) Diagnostics & Validation
   - Error taxonomy includes: unterminated string/triple‑quote, unbalanced template/f‑string, stray close brace, invalid Frame statement at SOL, unbalanced transition args.
   - Present clear messages with mapped locations; validation (e.g., transitions terminal) runs over MIR before expansion.

Complexity
- Each pass is linear in the size of the body (`O(n)`): partitioning scan, segmentation, small MIR parses, one native parse. Memory use is bounded by the size of the body and mapping tables.

Cross‑Language Expectations
- Python: Always use the textual DPDA closer in partitioning; segmenter tracks single/double/triple quotes, f‑strings, and `#` comments; indentation from nearest native sibling; avoid breaking `elif/else/except/finally` chains.
- TypeScript: Textual template/backtick‑aware closer in partitioning; segmenter tracks strings/templates/comments; require `===`/`!==`; optional chaining and nullish coalescing remain policy‑gated.

Notes
- This section supersedes older mentions of “re‑searching for the closing brace” in later passes. Body bounds are determined exactly in ModulePartitioning and trusted thereafter.
- MixedBody/MIR remains the single source of truth for embedded Frame semantics; splicing is an implementation detail to enable native AST/formatting.

Related Detailed Docs
- Stage index: `00_stage_index.md`
- Module Partitioning: `01_module_partitioning.md`
- Body Closers (Python/TS): `01_body_closers_python.md`, `01_body_closers_typescript.md`
- Native Region Scanner (Python/TS): `02_native_region_scanner_python.md`, `02_native_region_scanner_typescript.md`
- Frame Segment Parser: `03_frame_segment_parser.md`
- MIR Assembly: `04_mir_assembly.md`
- Directive Expansion (Python/TS): `05_Frame statement_expansion_python.md`, `05_Frame statement_expansion_typescript.md`
- Splice & Mapping: `06_splice_and_mapping.md`
- Native Parse Facade (Python/TS): `07_native_parse_facade_python.md`, `07_native_parse_facade_typescript.md`
- Source Maps & Codegen: `08_source_maps_and_codegen.md`
- Validation: `09_validation.md`
- AST & Symbol Integration: `10_ast_and_symbol_integration.md`
- Error Taxonomy: `11_error_taxonomy.md`
- Testing Strategy: `12_testing_strategy.md`

Pipeline Stages & Dedicated Structs

Overview
- Each stage is implemented by a focused struct with a clear contract. Stages exchange immutable artifacts (no in‑place mutation across boundaries). All stages are deterministic and overall O(n) for the body size.
- Proposed module root for V3: `framec/src/frame_c/v3/`.

Stage 1 — Module Partitioning
- Struct: `ModulePartitionerV3`
- Files (proposed): `v3/partitioner.rs`, `v3/body_closer/{python.rs,typescript.rs}`
- Input: source bytes, path, `Target`
- Output: `ModulePartitions { prolog, imports, outline, bodies: Vec<BodyPartition> }` where each `BodyPartition` carries `{ open_byte, close_byte, target }`
- Invariants:
  - Uses per‑target textual DPDA closers (string/comment/template aware).
  - Records exact `{ … }` byte offsets; later stages never re‑close bodies.
- Errors: unterminated string/template inside body; stray closing `}`; inconsistent target.
- Complexity: O(n) over the file.
- Test hooks: golden partitions for tricky strings/templates; negative fixtures for unterminated literals.

Stage 2 — Native Region Segmentation
- Struct: `NativeRegionScannerV3<{Python,TypeScript}>`
- Files (proposed): `v3/native_scan/{python.rs,typescript.rs}`
- Input: `BodyPartition` slice `[open_byte+1, close_byte)`
- Output: `Segments: Vec<Segment::{NativeText(ByteSpan), FrameSegment{span, kind_hint, indent}}>`
- Invariants:
  - Streaming DPDA over bytes; maintains protected‑region flags and `at_sol`.
  - Detects FIRST‑set at SOL only; ignores strings/comments/templates.
- Errors: invalid Frame statement tokens (e.g., `->` without `$`); malformed SOL markers (policy‑driven).
- Complexity: O(n) over the body.
- Test hooks: SOL lexeme negatives inside strings/comments; unicode whitespace at SOL.

Stage 3 — Frame Segment Parser (Tiny)
- Struct: `FrameSegmentParserV3`
- File: `v3/frame_segment_parser.rs`
- Input: `FrameSegment` byte slice + indent string
- Output: `MirItem::{Transition{target,args,span}, Forward{span}, StackPush{span}, StackPop{span}}`
- Invariants:
  - Balanced `(`…`)` for transition args with string awareness inside args.
  - No `system.return` here (native rewrite).
- Errors: unbalanced parentheses; invalid `$State` token; unexpected trailing tokens on the line.
- Complexity: O(length of segment) per segment; sum O(n).
- Test hooks: transitions with nested strings/commas; empty args; whitespace variants.

Stage 4 — MIR Assembly
- Struct: `MirAssemblerV3`
- File: `v3/mir_assembler.rs`
- Input: `Segments` + parsed MIR items
- Output: `MixedBody { items: [NativeText | MirItem], mapping }`
- Invariants:
  - Preserve original byte spans; keep stable item order.
  - Validate terminal Frame statement rule for handlers.
- Errors: non‑terminal Frame statement followed by native statements (validator violation).
- Complexity: linear in item count.
- Test hooks: terminal‑Frame statement fixtures; mixed native/MIR sequences.

Stage 5 — Directive Expansion (Per Target)
- Structs: `DirectiveExpanderPyV3`, `DirectiveExpanderTsV3`
- Files: `v3/expand/{python.rs,typescript.rs}`
- Input: `MirItem` + handler context (state name, event params, return policy)
- Output: target‑native snippet (text or minimal native AST) + indent
- Invariants:
  - Indentation derived from the Frame statement’s indent; do not break `elif/else/except/finally` chains.
  - Reuse runtime helpers; no reformatting beyond what the native parser/formatter later applies.
- Errors: context resolution failures (e.g., unknown target state); report with Frame spans.
- Complexity: O(items).
- Test hooks: nested conditionals; forward/transition glue correctness.

Stage 6 — Splice & Map
- Struct: `SplicerV3`
- File: `v3/splice.rs`
- Input: `MixedBody` + expansions
- Output: `SplicedBody { bytes, splice_map: Vec<Mapping{frame_span -> inserted_span}> }`
- Invariants:
  - Stable ordering; exact byte accounting; expansions inserted at Frame statement spans.
  - Maintain mapping for dual‑origin diagnostics.
- Errors: overlapping spans; inconsistent offsets.
- Complexity: O(n) over combined length.
- Test hooks: mapping round‑trip checks; overlapping‑span negatives.

Stage 7 — Native Parse (Optional, Per Target)
- Structs: `NativeParseFacadePyV3`, `NativeParseFacadeTsV3`
- Files: `v3/native_parse/{python.rs,typescript.rs}`
- Input: `SplicedBody`
- Output: native AST + node→byte spans (spliced coordinate space)
- Invariants:
  - Parsing is best‑effort for better diagnostics/formatting; not required for MIR semantics.
  - Preserve spans to allow remapping through `splice_map` to original Frame/native origins.
- Errors: syntax errors in native code (report mapped locations).
- Complexity: linear in body; native parser dependent.
- Test hooks: syntax error mapping; formatting stability.

Stage 8 — Codegen & Source Maps
- Struct: `SourceMapComposerV3` and target codegens
- Files: `v3/source_map.rs`, `v3/codegen/{python.rs,typescript.rs}`
- Input: native AST (preferred) or `SplicedBody` text + `splice_map` + byte→line index
- Output: target code + source maps with dual origin attribution
- Invariants:
  - Accurate mapping for both MIR expansions and native text.
  - Deterministic output for identical inputs.
- Errors: mapping gaps; report missing spans as internal errors (never silent).
- Complexity: linear in nodes/text.
- Test hooks: mapping goldens; debugging breakpoints land on expected lines.

Stage 9 — Validation
- Struct: `ValidatorV3`
- File: `v3/validator.rs`
- Input: `MixedBody` + AST context
- Output: diagnostics (or success)
- Invariants:
  - Enforce: transitions terminal; no Frame Frame statements in actions/ops; Python native policy (no `var`, no brace‑style control).
- Errors: policy violations with precise spans.
- Complexity: linear in item count.
- Test hooks: negative fixtures per policy.

AST & Symbol Table Integration

High‑Level Model
- Frame AST (authoritative) models modules, systems, states, handlers, actions/ops, and MixedBody items. Symbol tables (`Arcanum`) are built in Frame contexts (Pass 1) and referenced by later stages.
- Native ASTs (optional) model the spliced native bodies for formatting/diagnostics only; they do not participate in Frame semantic resolution.

Scopes and Symbols
- Frame scopes:
  - Module scope: functions, systems, modules, enums, vars.
  - System scope: actions, operations, domain fields, machine states.
  - Handler local scope: event parameters, locals (Frame declarations only).
- Native scopes:
  - If parsed, a lightweight binder builds `NativeBindings` for imports and local decls to aid diagnostics and FID extraction.
  - Native bindings never affect Frame resolution; they are orthogonal.

Mixed Context (Handlers)
- MixedBody does not introduce new Frame symbols. MIR items may reference Frame entities (e.g., target state names, event params) which are resolved against `Arcanum` at MIR assembly/expansion time.
- Visitors use Frame symbol info (e.g., state IDs, param names/types) to craft native expansions; no native name lookup is required beyond indentation and structural placement.

FID & Imports
- During segmentation or native parse, imports are collected into a `NativeImportIndex` for FID generation and policy checks. This index is attached to the module/system, not to MIR.

Coordinate Spaces & Mapping
- Byte spans are the ground truth. The `splice_map` remaps spliced native byte ranges back to original Frame/native origins for diagnostics and source maps.
- A precomputed byte→(line, col) index renders human locations; no algorithmic decisions depend on lines.

Error Reporting Contract
- Frame errors (grammar/policy/MIR) report Frame spans resolved via byte→line mapping.
- Native errors (post‑splice parse) report mapped spans using `splice_map` to attribute to the originating Frame statement/native text.

Testing Strategy
- Unit tests per stage using hermetic fixtures; end‑to‑end tests via existing language‑specific suites in transpile‑only mode.
- Golden tests for partitions, segments, MIR assembly, mapping, and diagnostics.
