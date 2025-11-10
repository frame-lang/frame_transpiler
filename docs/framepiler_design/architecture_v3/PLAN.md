# V3 Execution Plan — First Principles Rebuild

Goal
- Rebuild the single‑file pipeline from first principles using the V3 docs as the source of truth, then add the multi‑file project layer. Keep the new code hermetic and deterministic.

Scope (MVP → Plus)
- MVP: Stages 01–06 (closers, streaming scanners, Frame Statement parser, MIR assembly, expansion, splice/mapping), per‑file TS/Py. Validation rules enforced. No native parse facades in the critical path.
- Plus: Stage 07 (optional native parse facades for diagnostics/formatting), Stage 08 polish, Project layer (FID, linking/packaging), incremental build.

Progress Snapshot
Scaffold brought up (demo‑level; not production‑ready):
- [x] Stage 01 — Body Closers (all 7 languages)
- [x] Stage 02 — Native Region Scanners (all 7 languages)
- [x] Stage 03 — Frame Statement Parser (FIRST‑set)
- [x] Stage 04 — MIR Assembly
- [x] Stage 05 — Minimal comment‑only expansion (all 7 languages)
- [x] Stage 06 — Splice + origin mapping (debug trailer builder present)
- [x] Validator (terminal‑last rule) in demo path
- [x] Demo CLI paths (single‑body; directory; frame‑like multi‑body)

Status Summary — Fixtures and Validation (All Languages)
- [x] v3_prolog fixtures integrated in runner (positive/negative)
- [x] v3_imports fixtures integrated with validation (negatives enforced)
- [x] v3_outline positives integrated
 - [x] v3_outline negatives (missing '{' detection) — enforced via OutlineScannerV3 + validator
 - [x] v3_mapping fixtures (splice map round‑trip)
- [ ] v3_mir parser negatives (malformed heads/args) — pending
- [ ] v3_expansion indentation chain fixtures — pending

Production‑ready criteria (not done unless explicitly checked):
- [ ] Authoritative module outline (prolog/imports/owner_id) with SOL scanners
- [ ] ImportScannerV3 (DPDA) per language:
  - [x] Python
  - [x] TypeScript
  - [x] C#
  - [x] C
  - [x] C++
  - [x] Java
  - [x] Rust
- [ ] Extensive negative fixtures (closers/scanners) per language
- [ ] Validator policies (no Frame in actions/ops; per‑language native policies)
- [ ] Mapping trailer gating/polish and docs
- [ ] Real project build: FID/linking/packaging (TS/Py first), then others
- [ ] Beyond‑comment expansions/glue per language (gated by flags)

Repository Mechanics
- Use `framec/src/frame_c/v3/` as the authoritative implementation.
- No guard/flags: V3 is the default path for demos and ongoing work.

Repository Mechanics Checklist
- [x] V3 is authoritative in v3/ and used by demos
- [x] Demo commands exist for exercising V3
- [x] Validation flags `--validate/--validation-only` (demo) integrated

01 — Body Closers (per target)
- Objects: `BodyCloserPyV3`, `BodyCloserTsV3`, `BodyCloserCsV3` (trait `BodyCloserV3`)
- Deliverable: DPDA closer returns `close_byte` for body starting at `{`.
- Acceptance:
  - Python: handles single/double/triple quotes, f‑strings, `#` comments; returns precise close or characterized failure.
  - TypeScript: handles quotes, block/line comments, templates with nested `${…}`.
  - C#: normal/verbatim/interpolated/raw strings (with `$` arity), char literals, `//`/`/* */`, preprocessor lines.
- Tests (existing): parser.rs unit tests for TS/Py textual closers; Py triple‑quote/f‑string fixtures; TS templates fixtures.
- New micro‑fixtures (planned): `framec_tests/v3/01_closers/{py,ts}/*.frm` (unterminated cases, deep bodies).

Checklist
- [x] Python closer
- [x] TypeScript closer
- [x] C# closer (normal/verbatim/interp/raw)
- [x] C closer
- [x] C++ closer
- [x] Java closer
- [x] Rust closer
- [ ] Negative fixtures complete (all languages)

Per‑Language Test Matrix (01–03 early focus)
- Python
  - [x] Prolog: positive/negative
  - [x] Imports: positive/negative (unterminated paren)
  - [x] Outline: positive
  - [x] Outline: negative (missing '{')
- TypeScript
  - [x] Prolog: positive/negative
  - [x] Imports: positive/negative (missing brace/semicolon)
  - [x] Outline: positive
  - [x] Outline: negative (missing '{')
- C#
  - [x] Prolog: positive/negative
  - [x] Imports: positive/negative (unterminated using)
  - [x] Outline: positive
  - [x] Outline: negative (missing '{')
- C
  - [x] Prolog: positive/negative
  - [x] Imports: positive/negative (unterminated #include)
  - [x] Outline: positive
  - [x] Outline: negative (missing '{')
- C++
  - [x] Prolog: positive/negative
  - [x] Imports: positive/negative (unterminated #include)
  - [x] Outline: positive
  - [x] Outline: negative (missing '{')
- Java
  - [x] Prolog: positive/negative
  - [x] Imports: positive/negative (missing semicolon)
  - [x] Outline: positive
  - [x] Outline: negative (missing '{')
- Rust
  - [x] Prolog: positive/negative
  - [x] Imports: positive/negative (missing semicolon)
  - [x] Outline: positive
  - [x] Outline: negative (missing '{')

02 — Native Region Scanners (streaming)
- Objects: `NativeRegionScannerPyV3`, `NativeRegionScannerTsV3`, `NativeRegionScannerCsV3` (trait `NativeRegionScannerV3`)
- Deliverable: one pass → `ScanResultV3 { close_byte, regions }`; regions are `RegionV3::{NativeText, FrameSegment}` with byte spans and `kind_hint`.
- Acceptance: SOL‑anchored detection only; Unicode whitespace accepted; protected‑region aware; O(n), must‑advance guaranteed.
- Tests (existing): Py event_handler_incremental; TS islands (comments/strings with statement‑like tokens).
- New micro‑fixtures: `framec_tests/v3/02_scanner/{py,ts}/…` to assert segment boundaries with a small dump tool.

Checklist
- [x] Python scanner
- [x] TypeScript scanner
- [x] C# scanner (preprocessor + raw/interp strings)
- [x] C/C++/Java/Rust scanners
- [x] Initial cross‑language fixtures (raw strings/comments)
- [ ] Comprehensive SOL/edge fixtures per language

03 — Frame Statement Parser (FIRST‑set)
- Objects: `FrameStatementParserV3` (+ `FrameStatementParserPyV3/TsV3`), helpers `NativeArgSplitterPyV3/TsV3`.
- Deliverable: tiny parser validates head/token, balanced parentheses; splits arg list at top‑level commas (string/nesting aware). Produces `MirItemV3::{Transition,Forward,Stack*}` with raw arg strings and byte span.
- Acceptance: clear errors (invalid head; unmatched `)` in args; trailing tokens).
- Tests (existing): negatives for malformed transitions and non‑terminal violations.
- New micro‑fixtures: `framec_tests/v3/03_parser/*.frm` positive/negative per statement kind.

Checklist
- [x] Parser implemented (heads/args/balanced parens)
- [ ] Negative fixtures (malformed heads/args) expanded

04–06 — MIR/Expansion/Splice Test Coverage
- [ ] MIR Assembly terminal‑last negatives (runner category v3_mir)
- [ ] Expansion indentation chain fixtures (Py/TS; generalize for C#/Java/C/C++/Rust comments)
- [ ] Mapping round‑trip (runner category v3_mapping)

04 — MIR Assembly
- Objects: `MirAssemblerV3` → `[MirItemV3]` from `RegionV3`.
- Deliverable: MixedBody/MIR authoritative for handlers; actions/ops native‑only enforced via validation.
- Acceptance: preserves order, spans; no parser‑level statements in handlers.
- Tests: transitions_terminal rule; language_specific suites.
- New: mapping checks with debug JSON anchors.

Checklist
- [x] MIR built from regions
- [x] Span preservation
- [x] Mapping anchors sanity checks

05 — Frame Statement Expansion (per target)
- Objects: `FrameStatementExpanderPyV3/TsV3`, optional `IndentationAnalyzerPyV3/TsV3` (AST‑aware when Stage 7 enabled).
- Deliverable: textual glue + early returns injected with correct indentation; preserve elif/else/except/finally (Py) and else if (TS) chains.
- Acceptance: terminal statements suppress following native code (validator + emission behavior); sibling‑based indentation good; AST‑aware optional.
- Tests (existing): Py if_elif_returns, try/except, async_*; forward events; stack ops. TS control_flow/core.
- New: incremental indentation tests `framec_tests/v3/05_expander_py/*.frm`.

Checklist
- [x] Python minimal expansions (comment‑only markers)
- [x] TypeScript minimal expansions (comment‑only markers)
- [x] C/C++/Java/Rust minimal expansions (comment‑only markers)
- [ ] Full glue semantics (per language) gated behind future flags

06 — Splice & Mapping
- Objects: `SplicerV3`, `SourceMapComposerV3`.
- Deliverable: build spliced body and compose source maps attributing expansions to Frame statement frame lines.
- Acceptance: golden mapping anchors in debug mode; breakpoint alignment samples.
- Tests (new optional): mapping golden files and human JSON trailers.

Checklist
- [x] Splice of native + expansions
- [x] Origin→target mapping composed
- [x] Trailer builder (debug) present
- [ ] CLI gating via env flags for map output

07 — Native Parse Facades (pluggable; runtime‑optional — required to implement for all languages)
- Objects: `NativeParseFacade*V3` per language; optional `IndentationAnalyzer*V3`.
- Deliverable: parse spliced body for diagnostics/formatting; validate native arg expressions; map diagnostics to Frame spans via `splice_map`.
- Acceptance: off by default (hermetic core); when enabled (e.g., `--validate-native`), surfaces native syntax errors in args and provides formatting/indent hints; no semantic regressions.
- Tests: `v3_facade_*` categories (runner) with positive/negative fixtures per language; diagnostics correctly mapped to Frame arg spans when enabled.

Checklist
- [ ] Python facade (implemented; off by default)
- [ ] TypeScript facade (implemented; off by default)
- [ ] C# facade adapter (implemented; off by default; Roslyn integration optional)
- [ ] C facade (implemented; off by default)
- [ ] C++ facade (implemented; off by default)
- [ ] Java facade (implemented; off by default)
- [ ] Rust facade (implemented; off by default)

08 — Codegen (adapters, optional)
- Objects: `TsB2CodegenV3`, `PyB2CodegenV3` (future polish).
- Deliverable: AST‑based emission for deterministic formatting where desired.

Checklist
- [ ] Python codegen adapter (optional)
- [ ] TypeScript codegen adapter (optional)
- [ ] C# codegen adapter (optional)
- [ ] C codegen adapter (optional)
- [ ] C++ codegen adapter (optional)
- [ ] Java codegen adapter (optional)
- [ ] Rust codegen adapter (optional)

09 — Validation
- Objects: `ValidatorV3`, rules: `TerminalLastRuleV3`, `NoFrameStatementsInActionsOpsRuleV3`, `PythonNativePolicyRuleV3`, etc.
- Deliverable: clear diagnostics and rule coverage at MixedBody/MIR level.
- Tests (existing): negatives and policy suites; ensure runner invokes validator post‑transpile.

Checklist
- [x] Terminal‑last rule
- [x] Demo CLI `--validate/--validation-only` paths
- [x] No Frame statements in actions/ops (module outline kinds authoritative)
- [ ] Python/TypeScript native policy checks
- [ ] State/target existence checks

Project / Multi‑File Layer (after MVP green)
- Objects: `FileLoaderV3`, `ModuleResolverV3`, `ProjectGraphV3`, `FIDIndexV3`, `FIDEmitterV3`, `SemanticAnalyzerV3`, `TsModuleLinkerV3`, `PythonPackagePlannerV3`, `BuildPlannerV3`.
- Deliverables: FID emission/consumption, import resolution, stable linking/packaging, incremental build.
- Acceptance: multi‑file TS/Py suites execute and link correctly; one shared runtime import per module set.
- Tests: import graph positives/negatives, circular detection, missing FID, signature mismatch.

Checklist
- [x] ModulePartitionerV3 (demo): bodies via body closers
- [ ] Full module outline (prolog/imports/owner_id)
- [ ] FID emission/consumption
- [ ] Linking/packaging per language
- [ ] Incremental build caches

V3 Fixtures (runner; all languages)
- v3_prolog:
  - [x] Python  - seeds added
  - [x] TypeScript - seeds added
  - [x] C# - seeds added
  - [x] C - seeds added
  - [x] C++ - seeds added
  - [x] Java - seeds added
  - [x] Rust - seeds added
- v3_imports:
  - [x] Python  - seeds added (± negatives)
  - [x] TypeScript - seeds added (± negatives)
  - [x] C# - seeds added (± negatives)
  - [x] C - seeds added (± negatives)
  - [x] C++ - seeds added (± negatives)
  - [x] Java - seeds added (± negatives)
  - [x] Rust - seeds added (± negatives)
- v3_outline:
  - [x] Python  - seeds added (+/−)
  - [x] TypeScript - seeds added (+/−)
  - [x] C# - seeds added (+/−)
  - [x] C - seeds added (+/−)
  - [x] C++ - seeds added (+/−)
  - [x] Java - seeds added (+/−)
  - [x] Rust - seeds added (+/−)

Legacy References Cleanup
- Purge remaining documentation pointing at any non‑V3 paths.

Checklist
- [x] V3 is the default for demos
- [x] Legacy pipeline deleted from code
- [x] Remaining docs scrubbed of legacy references

CI & Tooling
- Gate each stage with per‑stage tests and full language_specific suites.
- Debug flags for mapping/anchors; JSON/human outputs for map inspection.
- Caches: content‑hash keyed `RegionScanCacheV3`/`MirCacheV3` for future incremental builds.

Milestones & Gating
- M1: Stages 01–03 green with micro‑fixtures; scanners return identical close/segment boundaries as sampled expectations.
- M2: Stage 04/05: Python language_specific 100% validate + execution ≥95%; TS language_specific 100% validate.
- M3: Stage 06 mapping debug anchors verified on samples.
- M4: Project layer minimum viable linking (TS/Py) + FID round‑trip; multi‑file suites pass.
- M5: Legacy retirement; V3 default.

Milestone Checklist
- [x] M1 — Closers/Scanners/Parser scaffold green
- [ ] M2 — MIR/Expansion coverage per language suites
- [x] M3 — Mapping anchors verified via tests (demo)
- [ ] M4 — Project linking + FID round‑trip
- [ ] M5 — Project layer + policies complete

Per‑Language Production Readiness (matrix)
Python
- [x] 01 Closer DPDA
- [x] 02 Scanner streaming
- [ ] 02 Edge fixtures complete
- [x] 03 Frame Segment Parser
- [x] 04 MIR Assembly
- [x] 05 Minimal expansions
- [ ] 05 Full glue semantics
- [x] 06 Splice/mapping tests
- [ ] 07 Native parse facade (optional)
- [ ] Validation policy (native‑specific)

TypeScript
- [x] 01 Closer DPDA
- [x] 02 Scanner streaming
- [ ] 02 Edge fixtures complete
- [x] 03 Frame Segment Parser
- [x] 04 MIR Assembly
- [x] 05 Minimal expansions
- [ ] 05 Full glue semantics
- [x] 06 Splice/mapping tests
- [ ] 07 Native parse facade (optional)
- [ ] Validation policy (native‑specific)

C#
- [x] 01 Closer DPDA (normal/verbatim/interp/raw)
- [x] 02 Scanner streaming (preprocessor + strings)
- [ ] 02 Edge fixtures complete
- [x] 03 Frame Segment Parser
- [x] 04 MIR Assembly
- [x] 05 Minimal expansions
- [ ] 05 Full glue semantics
- [x] 06 Splice/mapping tests
- [ ] 07 Native parse facade (optional)
- [ ] Validation policy (native‑specific)

C
- [x] 01 Closer DPDA
- [x] 02 Scanner streaming
- [ ] 02 Edge fixtures complete
- [x] 03 Frame Segment Parser
- [x] 04 MIR Assembly
- [x] 05 Minimal expansions
- [ ] 05 Full glue semantics
- [x] 06 Splice/mapping tests
- [ ] 07 Native parse facade (optional)
- [ ] Validation policy (native‑specific)

C++
- [x] 01 Closer DPDA
- [x] 02 Scanner streaming
- [ ] 02 Edge fixtures complete
- [x] 03 Frame Segment Parser
- [x] 04 MIR Assembly
- [x] 05 Minimal expansions
- [ ] 05 Full glue semantics
- [x] 06 Splice/mapping tests
- [ ] 07 Native parse facade (optional)
- [ ] Validation policy (native‑specific)

Java
- [x] 01 Closer DPDA
- [x] 02 Scanner streaming
- [ ] 02 Edge fixtures complete
- [x] 03 Frame Segment Parser
- [x] 04 MIR Assembly
- [x] 05 Minimal expansions
- [ ] 05 Full glue semantics
- [x] 06 Splice/mapping tests
- [ ] 07 Native parse facade (optional)
- [ ] Validation policy (native‑specific)

Rust
- [x] 01 Closer DPDA
- [x] 02 Scanner streaming
- [ ] 02 Edge fixtures complete
- [x] 03 Frame Segment Parser
- [x] 04 MIR Assembly
- [x] 05 Minimal expansions
- [ ] 05 Full glue semantics
- [x] 06 Splice/mapping tests
- [ ] 07 Native parse facade (optional)
- [ ] Validation policy (native‑specific)

Per‑Phase Testing Plan (Must Be In Python Runner)
- General
  - All tests live under `framec_tests/` and run via `framec_tests/runner/frame_test_runner.py`.
  - Add both positive and negative fixtures for every new feature, per language.
  - Use language‑specific v3 categories (e.g., `language_specific/<lang>/v3_*`) and add categories to the runner (e.g., `v3_demos`, `v3_prolog`, `v3_imports`, `v3_outline`, `v3_validator`, `v3_mapping`, `v3_project`).

- Stage 00 — Prolog Scanner (SOL, required)
  - Positives: `@target <lang>` at first non‑whitespace; leading blank lines allowed.
  - Negatives: missing prolog; non‑`@target` token first; malformed head (`@target` without language).
  - Location: `language_specific/<lang>/v3_prolog/*.frm`

- Stage 01 — Body Closers (DPDA per language)
  - Positives: bodies with strings/comments/raw constructs where applicable.
  - Negatives: unterminated strings/templates/comments/raw; unmatched braces.
  - Location: `language_specific/<lang>/v3_closers/{positive,negative}/*.frm`

- Stage 02a — Import Scanners (DPDA per language)
  - Positives: SOL imports (`import`, `export`, `using`, `#include`, `package`/`import`, `use`/`extern`) including multi‑line forms.
  - Negatives: import/include/using tokens inside protected regions (strings/comments/templates) must not be matched; malformed/unterminated continuations.
  - Location: `language_specific/<lang>/v3_imports/{positive,negative}/*.frm`

- Stage 02b — Outline Scanner (SOL artifacts)
  - Positives: handler/action/operation/op/on headers + `{ … }` with correct `owner_id` and `kind`.
  - Negatives: ambiguous or malformed headers (wrong keyword, missing name) must error or classify Unknown consistently.
  - Location: `language_specific/<lang>/v3_outline/{positive,negative}/*.frm`

- Stage 03/04 — Frame Segment Parser + MIR Assembly
  - Positives: well‑formed transitions/forwards/stack ops with balanced args.
  - Negatives: malformed heads, unmatched parens, trailing tokens.
  - Location: `language_specific/<lang>/v3_mir/{positive,negative}/*.frm`

- Stage 06 — Splice & Mapping
  - Positives: mapping anchors present; comment‑only expansions spliced at correct indent.
  - Negatives: inconsistent spans should be detected in tests comparing anchors; ensure debug trailer gating via env only affects printing.
  - Location: `language_specific/<lang>/v3_mapping/{positive,negative}/*.frm`

- Stage 09 — Validator (Policies)
  - Positives: terminal‑last in handlers; no Frame in actions/ops (using Outline kinds); per‑language native policies.
  - Negatives: violations for each rule with precise diagnostics.
  - Location: `language_specific/<lang>/v3_validator/{positive,negative}/*.frm`

- Project Layer — FID/Linking/Packaging
  - Positives: multi‑file imports/linking; FID round‑trip.
  - Negatives: circular imports, missing FID, signature mismatch.
  - Location: `language_specific/<lang>/v3_project/{positive,negative}/*.frm`

Testing Status (Initial)
- [x] `v3_demos` added for all 7 languages (prolog + imports + simple body; transpile‑only via `demo-frame`).
- [x] `v3_prolog` positives/negatives per language.
- [x] `v3_imports` negatives per language (protected‑region masking, malformed cases).
- [x] `v3_outline` positives/negatives with owner_id/kind checks.
- [x] `v3_mir`, `v3_mapping` suites.
- [ ] `v3_validator`, `v3_project` suites.

Test Inventory (existing to reuse)
- Python: event_handler_incremental, if_elif_returns, try/except*, async*, forward events, stack ops, triple‑quotes/f‑strings, torture unicode.
- TypeScript: islands templates/comments, control_flow/core suites.
- Negatives: transitions in actions/ops, terminal‑last violations, malformed heads/args, inline #[target] annotations.

New Tests to Create
- `framec_tests/v3/01_closers/{py,ts}/*.frm`: unterminated strings/templates, deep bodies.
- `framec_tests/v3/02_scanner/{py,ts}/*.frm`: SOL detection vs protected regions; region boundary assertions.
- `framec_tests/v3/03_parser/*.frm`: Frame statement positives/negatives; trailing‑token error.
- `framec_tests/v3/05_expander_py/*.frm`: elif/else/except/finally preservation; redundant native return after terminal.
- Project: `framec_tests/v3/project/*`: FID import/export round‑trip; linking paths; circular import negative.

Owner’s Notes
- Keep objects small and single‑purpose; no monolithic RD parsers.
- The only “parser” in core is `FrameStatementParserV3` (tiny, FIRST‑set). Everything else is scanning/assembly/expansion.
- Default to textual expansions; AST involvement is optional.
