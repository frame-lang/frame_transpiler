# V3 Execution Plan — First Principles Rebuild

## Todo Next

- [ ] Docs — PRT V3 expansion semantics  
  Align the V3 per-language expansion docs for Python and TypeScript with the
  implemented semantics (system.return stack, interface wrappers, handler
  return sugar, adapter glue). Once the docs match the current codegen, mark
  the relevant sections in `05_frame_statement_expansion_python.md` and
  `05_frame_statement_expansion_typescript.md` as complete and add brief
  cross-references from the architecture overview.

Goal
- Rebuild the single‑file pipeline from first principles using the V3 docs as the source of truth, then add the multi‑file project layer. Keep the new code hermetic and deterministic.

Scope (MVP → Plus)
- MVP: Stages 01–06 (closers, streaming scanners, Frame Statement parser, MIR assembly, expansion, splice/mapping), per‑file TS/Py. Validation rules enforced. No native parse facades in the critical path.
- Plus: Stage 07 (optional native parse facades for diagnostics/formatting), Stage 08 polish, Stage 09 policies.
- Optional Final: Stage 13 — Project Layer is reserved and currently has no active design beyond basic project manifests and `framec project build`.

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
 - [x] Stage 07 — Native parse facades (wrapper‑only everywhere; strict native adapters feature‑gated for TS/Rust/C/C++/Java/C#)

Status Summary — Fixtures and Validation (All Languages)
- [x] v3_prolog fixtures integrated in runner (positive/negative)
- [x] v3_imports fixtures integrated with validation (negatives enforced)
- [x] v3_outline positives integrated
- [x] v3_outline negatives (missing '{' detection) — enforced via OutlineScannerV3 + validator
- [x] Outline header detection requires 'fn'/'async fn' for function headers; fixtures updated accordingly
  - [x] Header grammar aligned across all 7 languages: actions/operations/interface use `fn`/`async fn`; machine handlers allow bare `IDENT()`; outline/systems/validator suites green
- [x] v3_mapping fixtures (splice map round‑trip)
- [x] v3_mir parser negatives (malformed heads/args) — extended across Transition/Forward/Stack
- [x] v3_expansion indentation chain fixtures — completed
- [x] v3_validator (early structural) — terminal‑last; no Frame statements in actions/ops; state header '{' check
- [x] v3_validator (early structural) — terminal‑last; no Frame statements in actions/ops; state header '{' check; parent‑forward availability (module demos)
- [x] v3_validator — section marker refinement for state collection (ignore non‑section tokens like `else:`); fixes spurious E402 on valid states
 - [x] v3_prolog negatives mapped to E105 (missing/invalid/not-first @target)
 - [x] Outline/validator negatives for handler scope
   - [x] `handler_outside_state` and `handler_in_nonstate_block` added across languages; surfaces E404 alongside any E403 where applicable
 - [x] Docs: inline separators and multi‑statement policy per language
- [x] Docs: facade wrapper‑only checks and transition‑arg policy across languages
- [x] Exec-smoke: all 7 languages emit and run tiny programs with standardized markers (TRANSITION, FORWARD:PARENT, STACK:PUSH/POP). Non-Py/TS use minimal wrapper stubs in exec mode; main suites remain transpile/validate-only.
 - [x] Curated exec (Python/TS): runner supports `--exec-v3` with gating via `@run-expect` / `@run-exact` / `@exec-ok` for selected categories (`v3_core`, `v3_control_flow`, `v3_systems`). Seeded fixtures under control_flow.
 - [x] Curated exec expanded (Python/TS): added nested/loops, inline-forward+transition, try/catch/finally/else chains, and full-bucket transition fixtures; all green.
- [x] TypeScript exec import stabilized: runner supplies FRAME_TS_EXEC_IMPORT and exec handler relaxes compartment type; curated exec 100%.
- [x] Curated exec (Rust): control_flow/core/systems seeded and running under exec-v3; forward/stack non-terminal semantics fixed in expander.
- [x] Curated exec (Java/C#): control_flow/core/systems seeded and running under exec-v3 with toolchain skips.
- [x] Curated exec (C/C++): control_flow/core/systems seeded and running under exec-v3; runner compiles C++ with -std=c++11.
- [x] CI artifacts: V3 workflows upload JUnit XML reports for v3_all, v3_exec_smoke, and curated exec.
- [x] Visitor-map (module path): emit frame-map and visitor-map trailers for module demos when FRAME_MAP_TRAILER=1 (Py/TS); runner routes v3_mapping module fixtures via demo-frame; basic module_map fixtures added and green.

v0.86.36–0.86.39 — Runtime & Testing Hardening (Py/TS/Rust)
- Python runtime emission
  - compile-project copies `frame_runtime_py` to OUTDIR root.
  - compile `-o` copies `frame_runtime_py` (robust path resolution: env, exe-relative repo root/target, cwd).
- Actions/Operations emission
  - OutlineScanner recognizes bare IDENT headers in `actions:` and `operations:`; Python emits `def _action_*` / `def _operation_*`.
  - Import scanners for Python/TypeScript/Rust now stop at `system`/section headers so imports/uses inside `actions:` do not shift the module outline start; avoids dropping handlers/actions in complex FRMs.
  - OutlineScanner supports `async` headers in `machine:`/`actions:` across languages; `interface` headers without `{` are treated as prototypes (no E111), while `interface` bodies with `{` emit handlers.
- E401 policy enforcement
  - Compile validation enforces E401 for Frame statements inside actions/operations.
- Runner robustness
  - @cwd: tmp supported for v3_cli and v3_cli_project; runner makes framec path absolute when cwd changes.
  - v3_cli compile asserts presence of `frame_runtime_py` in OUTDIR for Python; v3_cli_project continues runtime assertion.
  - Visitor-map single-body validation uses transpile() assertions; no legacy single-body validator.
- TypeScript
  - Non-demo compile imports from `frame_runtime_ts` by default; locked by CLI test.
- Rust
  - Curated exec positives/negatives green; compile-time E401 wired.

Non-PRT Languages (C/C++/Java/C#) — Roadmap and Parity Tasks
- CWD parity
  - Add @cwd: tmp CLI and CLI-project fixtures (compile-only) to ensure runner/path robustness (planned next).
- Visitor-map parity
  - Add module-path visitor-map trailers (where supported) and minimal shape checks.
- CLI scaffolds
  - Ensure compile emits code with debug trailers (errors-json, frame-map; visitor-map when available) consistently.
- Validation parity
  - Keep E400/E401/E404 aligned; add targeted negatives as needed.
- Exec-smoke
  - Maintain exec-smoke markers; expand curated exec only when minimal runtime wrappers are stable.

Stage 10 — AST & Symbol Integration (Debugger Readiness)
- [x] 10A Native-symbol snapshot (module path)
  - Emit native-symbols trailer when FRAME_NATIVE_SYMBOL_SNAPSHOT=1.
  - Entries include: state, owner (handler name), params, paramSpans, schemaVersion.
  - Param extraction from outline headers with byte-accurate spans; trailer JSON shape stabilized.
  - Fix: corrected paramSpans double-bracket bug; emit a proper JSON array.
- [x] 10B Advisory policy checks (flag‑gated)
  - E405: State parameter arity mismatch (transition state_args vs state header params) — advisory under FRAME_VALIDATE_NATIVE_POLICY=1.
  - Runner enables the flag for v3_validator; negatives added and green for Py/TS.
- [x] Errors‑JSON alignment
  - errors-json trailer now aggregates module validator issues including E400/E401/E402/E403 and advisory E405 consistently with validate_module_demo.
  - Runner asserts presence/shape of errors-json for V3 module/demo outputs.
- [x] Robustness
  - Splicer hardened against parse-collection underflow when MIR items are fewer than detected Frame segments (no panic). Empty expansion used to preserve mapping.

Curated Exec (verification focus)
- [x] Python/TypeScript curated exec for v3_core, v3_control_flow, v3_systems passing under --exec-v3 --run.
- [x] Exec-smoke remains green across all 7 languages; markers standardized.

Stage 10 — Completion
- [x] 10C Parser-backed native snapshots (advisory)
  - TypeScript: SWC-backed extraction of handler parameter identifiers from a synthesized function signature (header-derived). Falls back to header-based extraction if parsing fails or feature disabled.
  - Python: header-based extraction remains authoritative (RustPython-backed variant tracked for future; not required for debugger readiness).
- [x] 10D Module visitor-map parity
  - Visitor-map trailer emitted and asserted (shape) for Py/TS module compiles; runner routes v3_visitor_map modules via demo-frame.
- [x] 10E Symbol threading polish
  - Arcanum threaded through module validation and exec contexts; unknown-state and parent-forward checks driven by symbols.

Stage 11 — Full AST/Symbol Integration (scoped)
- [x] Replace remaining coarse known-state checks with Arcanum for PRT (Py/TS/Rust) in the V3 module/CLI paths; keep non‑PRT languages on the coarse known‑state set for now.
- [x] Prepare optional native AST hooks (advisory only) for future param-bound checks (no codegen changes).
  - Native facade parsers (SWC/RustPython/syn) are already wired via `NativeFacadeRegistryV3` in the V3 module path for Py/TS/Rust, mapping advisory diagnostics onto `ValidationIssueV3` without affecting codegen. CLI flags/environment (`strict_native`, `FRAME_VALIDATE_NATIVE_POLICY`) gate these checks so they remain optional.

Stage 12 — Native Import / Project Wiring
- [ ] Reserved for future native import and project‑level wiring work that does not require any external metadata caches.

Stage 13 — Project Layer (Reserved)
- [x] 13A — Project manifest + CLI scaffolding (PRT)
  - Minimal project manifest format (`frame.toml`) and CLI entrypoints for project builds in `framec` (e.g. `framec init`, `framec project build`).
  - Project-level commands are gated behind explicit flags/env and are no-ops for existing single-file workflows when not used.
- [ ] 13B–13E — Future project-layer features
  - Any future project‑layer behavior (e.g., richer manifest semantics, packaging, or advisory cross‑file checks) should be specified from scratch.
  - The earlier project‑layer experiment that relied on external metadata caches has been removed from V3; there is no such mechanism in the current implementation.

Milestone — Py/TS/Rust to 100%
- [x] TypeScript: native parsing default‑on in validation (SWC); curated exec expanded across core/control_flow/systems; runner asserts errors-json trailers and runtime output markers.
- [x] Python: strict/native adapter via RustPython parser (pure Rust) enabled in CI; curated exec breadth expanded; runner asserts errors-json trailers and runtime output markers.
- [x] Rust: syn default‑on validation; curated exec expanded across control_flow/core/systems with markers; parity with Py/TS subsets validated; promotion to real runtime glue deferred until Py/TS stabilization.

Stage 10 — AST & Symbol Integration (Fine‑Grained)

10A — Native Symbol Snapshots (Py/TS)
- Goal: Extract safe native metadata (names/params) from parsed native bodies; keep advisory.
- Implementation:
  - TypeScript (SWC): collect function/handler param lists in segmented bodies.
  - Python (RustPython parser): collect function/handler param lists (positional/kw‑only/defaults).
- Mapping: All spans mapped back through `splice_map` to original source for diagnostics.
- Acceptance:
  - Snapshots available via API for selected fixtures; no behavior changes; mapped spans verified in tests.

10B — Advisory Validation (flag‑gated)
- Goal: Add optional policy checks that use native snapshots; keep Frame semantics authoritative.
- Implementation:
  - Add CLI/runner flag (e.g., `--validate-native-policy`).
  - Checks: param arity/name presence for transition state_args vs outline params (initial); more later.
- Acceptance:
  - Positives/negatives for Py/TS; diagnostics carry mapped spans; disabling flag reverts to current behavior.

Note: Arcanum now captures state parameter lists from outline headers (e.g., `$B(x, y, k)`), enabling the initial arity check without native parsing.

10C — Unified Symbol Query Surface
- Goal: Provide a single query API to obtain Frame (Arcanum) and Native (snapshots) metadata per handler.
- Implementation: expose `get_frame_state_signature()` + `get_native_handler_params()` style helpers.
- Acceptance: tooling/tests use the unified API; no semantic coupling to native.

10D — Runner/CI Integration
- Goal: Make it easy to run advisory checks in targeted suites.
- Implementation: runner preset to enable `--validate-native-policy` for Py/TS; JUnit includes mapped spans for new diagnostics.
- Acceptance: jobs green locally/CI, with clean skips when toolchains are missing.

10E — Documentation
- Update 10_ast_and_symbol_integration.md with scope, approach, advisory‑only policy, span mapping, risks/mitigations, flags.
- Cross‑link from architecture.md and testing strategy.
- Acceptance: docs clearly state that Frame semantics (Arcanum + MIR) remain authoritative; native snapshots enrich diagnostics only.

Deferred Improvements (postpone until TS/Py are 100% debugger‑ready)
- Rust target: switch state identity from strings to an enum (StateId), add Display/FromStr, update FrameCompartment to use enum; keep facade wrappers string‑based; preserve marker text via Display.
- TS/Py state typing: add TS literal union type for state ids; optional Python Enum for state ids (Display for compiled id).
- Validation policies (optional): arg/param arity checks for transition state_args vs outline params; rich error notes with related spans; suggestion diagnostics for unknown states.
- Mapping/Debugging: finalize visitor‑map sidecars (targetLine/sourceLine) for Py/TS module + single‑body; add small golden tests; later add optional columns and AST dump JSON.
- Native parsing hermetic defaults: keep Py/TS/Rust default‑on; consider tree‑sitter prebuilt artifacts for C/C++/Java/C# to avoid system C compiler (feature‑gated).
- Exec harness parity: expand Rust curated exec to Py/TS breadth (multi‑handler, deeper nesting); consider pilot real glue for non‑Py/TS later.
- Project layer (Phase A, retired): earlier sketch of a metadata‑driven linking layer; removed in favour of native parsing + Arcanum. No current implementation.
- Debug JSON envelope: add stable top‑level `code` alias and `targetLanguage` in debug output; keep language‑specific key for one minor cycle; ensure runner/tooling supports both.
- Performance/Robustness: small‑buffer reuse in scanners/closers; fuzz/torture suites for protected regions and SOL anchoring; no panics.

Production‑ready criteria (not done unless explicitly checked):
- [x] Authoritative module outline (prolog/imports/owner_id) with SOL scanners
- [x] ImportScannerV3 (DPDA) per language:
  - [x] Python
  - [x] TypeScript
  - [x] C#
  - [x] C
  - [x] C++
  - [x] Java
  - [x] Rust
- [ ] Extensive negative fixtures (closers/scanners) per language
- [x] Validator policies (structural only by default; no Frame in actions/ops). Per‑language native policies are optional and provided via Stage 07 facades when enabled.
- [x] Mapping trailer gating/polish and docs
- [x] Optional native parser adapters (facade strict): TS/Rust/C/C++/Java/C# behind `native-*` features
- [ ] Real project build: project linking/packaging (TS/Py first), then others
- [ ] Beyond‑comment expansions/glue per language (gated by flags)

Repository Mechanics
- Use `framec/src/frame_c/v3/` as the authoritative implementation.
- No guard/flags: V3 is the default path for demos and ongoing work.

Repository Mechanics Checklist
- [x] V3 is authoritative in v3/ and used by demos
- [x] Demo commands exist for exercising V3
- [x] Validation flags `--validate/--validation-only` (demo) integrated
- [x] Curated exec (`--exec-v3`) for Python/TypeScript with `@run-expect` assertions
- [x] CI JUnit uploads for v3_all, v3_exec_smoke, curated exec

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
- [x] Initial cross-language fixtures (raw strings/comments)
- [x] Import scanner diagnostics standardized (E110)
- [x] TS: support Transition with leading exit args at SOL `(<exit>) -> ( <enter> ) $State( <state> )`
- [x] Comprehensive SOL/edge fixtures per language
  - [x] Block comments (C/C++/Java/C#/Rust): tokens inside `/* ... */` ignored
  - [x] Raw strings (C++): tokens inside `R"(...)"` ignored
  - [x] Interpolated strings (C#): tokens inside `$"...{...}..."` ignored
  - [x] Java text blocks: tokens inside `""" ... """` ignored
  - [x] TypeScript templates: tokens inside multi-line backticks with nested `${...}` ignored

03 — Frame Statement Parser (FIRST‑set)
- Objects: `FrameStatementParserV3` (+ `FrameStatementParserPyV3/TsV3`), helpers `NativeArgSplitterPyV3/TsV3`.
- Deliverable: tiny parser validates head/token, balanced parentheses; supports full Transition buckets `(exit_args)? -> (enter_args)? $State(state_params?)`; splits each arg list at top‑level commas (string/nesting aware). Produces `MirItemV3::{Transition{target,exit_args,enter_args,state_args},Forward,Stack*}` with raw arg strings and byte span.
- Acceptance: clear errors (invalid head; unmatched `)` in args; trailing tokens).
- Tests (existing): negatives for malformed transitions and non‑terminal violations.
- New micro‑fixtures: `framec_tests/v3/03_parser/*.frm` positive/negative per statement kind.

Checklist
- [x] Parser implemented (heads/args/balanced parens)
- [x] Negative fixtures (malformed heads/args) expanded

04–06 — MIR/Expansion/Splice Test Coverage
  - [x] MIR Assembly terminal‑last negatives (runner category v3_mir)
  - [x] Expansion indentation chain fixtures (Py/TS; generalize for C#/Java/C/C++/Rust comments)
  - [x] Inline separators for Frame statements (multi‑statement line support)
    - [x] Scanners split at `;` / comment start (per language specifics)
    - [x] Parser tolerates optional trailing `;` and ignores inline comment markers
    - [x] Positive fixtures: `=> $^; native()` (TS/C#/C/CPP/Java/Rust), `=> $^; x = 1  #` (Py)
    - [x] Negative fixtures: `=> $^ native()` (no separator)

- 06.5 — Structural Validation (early)
- Objects: `ValidatorV3` (early rules)
- Deliverable: lightweight, hermetic checks before facades (Stage 07).
- Rules:
  - Terminal‑last on MIR items (Transition/Forward/Stack*).
  - No Frame statements in actions/operations (Outline kinds authoritative).
  - Machine state header must include '{' on same logical line.
- Tests: `v3_validator/{positive,negative}` per language in Python runner.

Checklist
- [x] Transition‑as‑terminal rule
- [x] No Frame statements in actions/ops
- [x] Machine state header '{' check
  - [x] Mapping round‑trip (runner category v3_mapping)

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
- [x] Python facade (implemented; off by default; tree-sitter; strict facade smoke green)
- [x] TypeScript facade (implemented; off by default; SWC + wrapper checks; strict facade smoke green)
- [x] C# facade adapter (implemented; off by default; tree-sitter-c-sharp; strict facade smoke green)
- [x] C facade (implemented; off by default; tree-sitter-c; strict facade smoke green)
- [x] C++ facade (implemented; off by default; tree-sitter-cpp; strict facade smoke green)
- [x] Java facade (implemented; off by default; tree-sitter-java; strict facade smoke green)
- [x] Rust facade (implemented; off by default; syn/strict checks on wrappers; strict facade smoke green)

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
- Objects: `ValidatorV3`, rules: policy‑level and semantic checks (beyond 06.5), e.g., `PythonNativePolicyRuleV3`, `TypeScriptPolicyRuleV3`, state/target existence.
- Deliverable: clear diagnostics and rule coverage for native policies and cross‑artifact checks.
- Tests: negatives and policy suites; ensure runner invokes validator post‑transpile.

Checklist
- [x] Demo CLI `--validate/--validation-only` paths
- [ ] Python/TypeScript native policy checks
- [x] State/target existence checks
  - [x] E402/E403 backed by symbol table (Arcanum) in module validation path
  - [x] Parent‑forward availability checks (module demos)
  - [x] Test policy: parent‑forward fixtures only in system context (module files); do not author single‑body tests for this rule
  - [x] Known‑state collection honors only real sections (machine/actions/operations/interface); no interference from control‑flow labels
  - [x] Exec-smoke parity: Python/TS real glue; C/C++/Java/C#/Rust emit wrapper calls and print markers; runner treats missing toolchains as clean skip
- [x] Handler scope enforcement
  - [x] E404: handler must be inside a state (negatives added across languages: `handler_outside_state`, `handler_in_nonstate_block`)

Project / Multi‑File Layer (after MVP green; reserved)
- Objects: `FileLoaderV3`, `ModuleResolverV3`, `ProjectGraphV3`, `SemanticAnalyzerV3`, `TsModuleLinkerV3`, `PythonPackagePlannerV3`, `BuildPlannerV3`.
- Deliverables: import resolution, stable linking/packaging, incremental build.
- Acceptance: multi‑file TS/Py suites execute and link correctly; one shared runtime import per module set.
- Tests: import graph positives/negatives, circular detection, signature mismatch.

Checklist
- [x] ModulePartitionerV3 (demo): bodies via body closers
- [ ] Full module outline (prolog/imports/owner_id)
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

Exec‑Smoke Parity (All Languages)
- Goal: each language runs tiny, hermetic programs that print standardized markers for Frame statements.
- Status: [x] Python [x] TypeScript [x] C [x] C++ [x] Java [x] C# [x] Rust
- Coverage:
  - [x] transition_basic, forward_parent (system context with declared parent)
  - [x] stack_ops (PUSH/POP), mixed_ops (stack + transition)
  - [x] if_forward_else_transition (forward or transition path)
  - [x] stack_then_transition, nested_stack_then_transition

Next Steps (Validation & CI)
- [ ] Tolerant diagnostics parity sweep (module path): ensure multi‑issue fixtures surface all problems; codegen remains strict.
- [ ] CI polish: ensure v3_exec_smoke runs on push/PR; keep v3_all transpile‑only suites as a separate job.
- [x] Plan Stage 09 symbol‑table migration for E402/E403 to replace coarse known‑state collection.

Milestones & Gating
- M1: Stages 01–03 green with micro‑fixtures; scanners return identical close/segment boundaries as sampled expectations.
- M2: Stage 04/05: Python language_specific 100% validate + execution ≥95%; TS language_specific 100% validate.
  - Status: Python/TypeScript v3_exec_smoke 100% run/validate; C/C++/Java/C#/Rust exec-smoke 100% via wrapper markers; runner emits `--emit-exec` for Py/TS and validates markers consistently across languages.
- M3: Stage 06 mapping debug anchors verified on samples.
- Next: Stage 07 (optional) — Native parser adapter scaffolding behind cargo features (`native-ts`, `native-py`, `native-rs`, etc.) and `--validate-native`.
- M4 (Optional): Project layer minimum viable linking (TS/Py); multi‑file suites pass when enabled.
- M5: Legacy retirement; V3 default.

Milestone Checklist
- [x] M1 — Closers/Scanners/Parser scaffold green
- [ ] M2 — MIR/Expansion coverage per language suites
- [x] M3 — Mapping anchors verified via tests (demo)
- [ ] M4 — Project linking (Optional)
- [ ] M5 — Project layer + policies complete

Production Readiness — Python & TypeScript (P‑track)
Purpose
- Define concrete steps to move Python and TypeScript targets from façade/demo to production use.

Scope
- Replace comment‑only expansions with real runtime glue; execute non‑façade suites; wire CI; lock runtime APIs; document and package.

Checklist (Py/TS)
- [x] Expanders: full glue semantics
  - [x] Python: implement Transition/Forward/Stack expansions that call stable runtime instance methods on the machine (kernel Option B) with correct control‑flow semantics. Transitions emit native `return` after `_frame_transition(...)`. (Implemented in `PyExpanderV3`; covered by v3_exec_smoke + curated exec.)
  - [x] TypeScript: implement Transition/Forward/Stack expansions that call stable runtime instance methods on the machine (kernel Option B); preserve inline multi‑statement splits; avoid double semicolons. Transitions emit native `return;` after `this._frame_transition(...)`. (Implemented in `TsExpanderV3`; covered by v3_exec_smoke + curated exec.)
- [x] Runtime API surface (stabilize and document)
  - [x] Python runtime: `frame_runtime_py` exposes `FrameEvent`/`FrameCompartment`; generated systems import them and implement `_frame_transition/_frame_router` as described in `frame_runtime.md`. Runtime semantics are exercised by Python v3_exec_smoke and curated exec suites.
  - [x] TypeScript runtime: `frame_runtime_ts` exposes `FrameEvent`/`FrameCompartment` via `index.ts/index.d.ts`; generated systems import from `"frame_runtime_ts"` and implement `_frame_transition/_frame_router` per `frame_runtime.md`. Semantics are exercised by TypeScript v3_exec_smoke and curated exec suites.
- [x] Executable test suites (beyond façade)
  - [x] Runner: enable execute/run mode for v3_core, v3_control_flow, v3_scoping, v3_systems for Python/TypeScript using real runtime (build = transpile; validate/run = execute). (Implemented via `--exec-v3` gating in `framec_tests/runner/frame_test_runner.py`; verified with `--languages python typescript --categories v3_core v3_control_flow v3_scoping v3_systems --run --exec-v3`.)
  - [x] Fixtures: add positives/negatives that assert real behavior for Transition (terminal), Forward (non‑terminal), and Stack operations. (Covered by curated exec fixtures under v3_core/v3_control_flow/v3_systems and newly exec‑enabled v3_scoping fixtures.)
  - [x] SOL/inline multi‑statement cases executing correctly (e.g., `=> $^; native()` keeps ordering). (Exercised by scoping/control‑flow fixtures such as `block_scope.frm`, `closure_scope.frm`, `function_block_scope.frm`, and their Python equivalents.)
- [ ] Validation rules (production)
  - [x] Transition terminal in containing block (structural) — implemented.
  - [x] Unknown state target diagnostics — implemented.
  - [x] Parent forward without parent (module demos) — implemented.
  - [ ] Forward‑to‑parent compile‑time rule refined for production modules (ensure metadata is sourced from outline; single‑body demos remain exempt).
- [ ] Source maps & diagnostics
  - [ ] Verify splice map quality for expansions (origin↔target for decision points) on sample programs.
  - [ ] Ensure runtime error surfaces map to Frame lines when feasible.
- [ ] Packaging & distribution
  - [ ] Python: package runtime with versioning; minimal install doc; avoid network deps in build.
  - [ ] TypeScript: package runtime as npm module (local workspace usage documented); avoid adding network deps to core build.
- [ ] CI gating
  - [ ] Add CI job to build and run executable Py/TS suites with real runtime (skips if toolchains missing).
  - [ ] Keep façade strict jobs (native parsers) as an additional signal, not a blocker for production runs.
- [ ] Documentation
  - [ ] Update 05_frame_statement_expansion_{python,typescript}.md with concrete runtime API calls and examples (done for return‑on‑transition and imports).
  - [ ] Update HOW_TO with "Build vs Run" for production, per‑language runtime setup, and troubleshooting.
  - [ ] Call out multi‑statement line policy per language (already covered; ensure consistency with examples).

Execution Order (recommended)
1. TypeScript expander + runtime API stabilization
2. Python expander + runtime API stabilization
3. Executable suites in runner for Py/TS
4. CI job for Py/TS executable suites
5. Source‑map spot checks and doc polish
6. Packaging notes (without introducing network deps to core build)

Post‑P‑track (optional)
- [ ] Codegen adapters (Stage 08) for formatting stability
- [ ] Extend executable suites gradually to other languages as runtimes mature

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
- [x] 07 Native parse facade (optional)
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
- [x] 07 Native parse facade (optional)
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
- [x] 07 Native parse facade (optional)
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
- [x] 07 Native parse facade (optional)
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
- [x] 07 Native parse facade (optional)
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
- [x] 07 Native parse facade (optional)
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
- [x] 07 Native parse facade (optional)
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

- Stage 06.5 — Structural Validation (early)
  - Positives: terminal‑last in handlers; actions/ops without Frame statements.
  - Negatives: multiple Frame statements after terminal; Frame statements in actions/ops; missing '{' after state header.
  - Location: `language_specific/<lang>/v3_validator/{positive,negative}/*.frm`

- Stage 06 — Splice & Mapping
  - Positives: mapping anchors present; comment‑only expansions spliced at correct indent.
  - Negatives: inconsistent spans should be detected in tests comparing anchors; ensure debug trailer gating via env only affects printing.
  - Location: `language_specific/<lang>/v3_mapping/{positive,negative}/*.frm`

- Stage 09 — Validator (Policies)
  - Positives: per‑language native policies and cross‑artifact checks.
  - Negatives: violations for each rule with precise diagnostics.
  - Location: `language_specific/<lang>/v3_validator/{positive,negative}/*.frm`

- Project Layer — Linking/Packaging (reserved)
  - Positives: multi‑file imports/linking.
  - Negatives: circular imports, signature mismatch.
  - Location: `language_specific/<lang>/v3_project/{positive,negative}/*.frm`

Testing Status (Initial)
- [x] `v3_demos` added for all 7 languages (prolog + imports + simple body; transpile‑only via `demo-frame`).
- [x] `v3_prolog` positives/negatives per language.
- [x] `v3_imports` negatives per language (protected‑region masking, malformed cases).
- [x] `v3_outline` positives/negatives with owner_id/kind checks.
- [x] `v3_mir`, `v3_mapping` suites.
- [x] `v3_validator` early structural suite.
- [ ] `v3_project` suites.

Test Inventory (existing to reuse)
- Python: event_handler_incremental, if_elif_returns, try/except*, async*, forward events, stack ops, triple‑quotes/f‑strings, torture unicode.
- TypeScript: islands templates/comments, control_flow/core suites.
- Negatives: transitions in actions/ops, terminal‑last violations, malformed heads/args, inline #[target] annotations.

New Tests to Create
- `framec_tests/v3/01_closers/{py,ts}/*.frm`: unterminated strings/templates, deep bodies.
- `framec_tests/v3/02_scanner/{py,ts}/*.frm`: SOL detection vs protected regions; region boundary assertions.
- `framec_tests/v3/03_parser/*.frm`: Frame statement positives/negatives; trailing‑token error.
- `framec_tests/v3/05_expander_py/*.frm`: elif/else/except/finally preservation; redundant native return after terminal.
- Project: `framec_tests/v3/project/*`: project import/linking paths; circular import negative.

Owner’s Notes
- Keep objects small and single‑purpose; no monolithic RD parsers.
- The only “parser” in core is `FrameStatementParserV3` (tiny, FIRST‑set). Everything else is scanning/assembly/expansion.
- Default to textual expansions; AST involvement is optional.
