# V3 Execution Plan — First Principles Rebuild

## Todo Next
``
- [x] PRT Stages 7–13 Closure Checklist  
  Complete the remaining Stage 7–13 work for the PRT languages (Python,
  TypeScript, Rust): apply native validation (Stage 7) to all Frame-owned
  runtimes/adapters, finalize PRT policy rules (Stage 9), finish Rust runtime
  parity (router + `system.return` + curated exec), and add minimal project
  fixtures for PRT in Stage 13. Keep the checklist below as the authoritative
  list of outstanding Stage 7–13 items for PRT. **All Stage 14 work is paused
  until this checklist is green.**

- [x] Python Indent Normalizer Machine (self‑hosting path) — Phase B (AFTER PRT 7–13)  
  Implement the Python V3 handler indentation algorithm as a Frame system
  (`.frs`) and drive it via the Rust backend once PRT Stage 7–13 parity is
  reached: (1) move the Stage 14 `IndentNormalizer` machine into the V3 code
  tree under `framec/src/frame_c/v3/machines/indent_normalizer.frs` and treat
  it as the authoritative spec for Python handler indentation, (2) add one or
  more fixtures in the shared env (`framepiler_test_env`) that compile this
  machine to Rust and validate it via `py_compile`/exec against the current
  Python indentation helper, (3) refactor the Python V3 emitter (`mod.rs`) so
  handler emission flows through a domain-based normalizer that matches the
  machine semantics, and (4) introduce a **boot compiler** policy for all
  self-hosted FRM machines:
    - The repo MUST contain a single pinned bootstrap compiler under
      `boot/framec/framec` which is the only binary used to regenerate any
      machine-generated Rust from `.frs` sources (e.g. Stage 14+ machines).
    - `cargo build` MUST NOT invoke `boot/framec/framec` automatically; FRM →
      Rust regeneration is an explicit step (e.g. `tools/gen_v3_machines_rs.py`) and the
      build may fail fast if a `.frs` is newer than its generated `.rs`.
    - The bootstrap compiler in `boot/framec/framec` is updated in place when
      semantics for these machines change, and its version is tracked in docs so
      contributors can reproduce the precompile step without access to the
      shared test environment.

Goal
- Rebuild the single‑file pipeline from first principles using the V3 docs as the source of truth, then add the multi‑file project layer. Keep the new code hermetic and deterministic.

Scope (MVP → Plus)
- MVP: Stages 01–06 (closers, streaming scanners, Frame Statement parser, MIR assembly, expansion, splice/mapping), per‑file TS/Py. Validation rules enforced. No native parse facades in the critical path.
- Plus: Stage 07 (optional native parse facades for diagnostics/formatting), Stage 08 polish, Stage 09 policies.
- Project Layer: Stage 13 is reserved and currently has no active design beyond basic project manifests and `framec project build` (no FID/cache layer).
- Self‑Hosting Infra: Stage 14 — Python Indent Normalizer Machine is the first self‑hosting milestone (see below).
- Persistence: Stage 15 — Persistence & Snapshots is **mandatory** for PRT (Py/TS/Rust) workflows and defines the per-language snapshot libraries and schema.

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
- Typed `system.return` across languages (post‑PRT): for Rust (and eventually other statically typed targets), replace untyped return stacks with enums or other concrete types per interface, so `system.return` is represented as a `Vec<MyEnum>`/equivalent rather than an untyped `any`/`object`. Track this as a follow‑up after PRT Stage 7–13 parity is complete and semantics are stable.

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

High‑Level Roadmap Arcs (PRT Focus)
- Arc 1 — Rust V3 Runtime Parity
  - Goal: bring Rust V3 module‑path runtime up to Python/TypeScript semantics (router, `system.return`, curated exec) before relying on additional self‑hosting.
  - Scope: the “Rust Runtime Parity (PRT Progress)” checklist below (router semantics, `system.return` stack/enum, curated Rust exec fixtures and harness).
- Arc 2 — Self‑Hosting Machines (IndentNormalizer and Friends)
  - Goal: move core Stage‑7+ algorithms that are currently Rust helpers in `mod.rs` into Frame machines (`.frs`) and use the boot‑compiler + precompile pipeline to generate their Rust implementations.
  - Scope: Stage 14 (Python Indent Normalizer Machine) and any future V3 machines added under `framec/src/frame_c/v3/machines/`, plus their harnesses in the main repo and shared env.
- Arc 3 — Broader PRT & Ecosystem
  - Goal: once PRT (Python/TypeScript/Rust) is solid through Stage 15, extend V3 and persistence semantics outward (non‑PRT targets, cross‑language snapshots, CI workflows).
  - Scope: non‑PRT language adoption of V3, cross‑language `SystemSnapshot` demonstrations, and CI jobs that gate on `all_v3`, curated exec, and persistence suites for PRT.

Stage 14 — Python Indent Normalizer Machine (Self‑Hosting)
- [x] Design: document the indent normalizer machine (inputs/outputs, state,
      algorithm) and keep it aligned with the current Rust helper logic in
      `framec/src/frame_c/v3/mod.rs` (see `14_indent_normalizer_machine.md`).
- [x] Implementation (Phase A): add an `@target rust` Frame system
      `IndentNormalizer` that accepts a vector of lines + flags and returns
      normalized Python lines. Place the `.frs` in the V3 code tree under
      `framec/src/frame_c/v3/machines/` and precompile it to Rust with the
      bootstrap compiler.
- [x] Tests (Phase A): add harnesses in the main repo and shared env that
      feed known handler bodies (e.g., the `stopOnEntry` and
      `PythonDebugRuntime` cases) into `IndentNormalizer.run(...)` and assert
      that its output matches the Stage 14 algorithm used by
      `normalize_py_handler_lines` / `emit_py_handler_body` in `mod.rs`.
- [ ] Integration (Phase B): replace the remaining ad‑hoc indentation helper
      in `mod.rs` by calling the generated Rust machine directly, wiring
      per‑line flags (`is_frame_expansion`, `is_comment`, etc.) from the
      MIR/expander. (Deferred to Stage 16 for PRT‑only integration.)

Stage 15 — Persistence & Snapshots (PRT Progress)
- [x] Python: `frame_persistence_py` module added with `SystemSnapshot`,
      `snapshot_system`, `restore_system`, and JSON helpers as specified in
      `14_persistence_and_snapshots.md`. V3 Python systems now expose
      class-level helpers `save_to_json` / `restore_from_json` that wrap
      these primitives, covered by
      `language_specific/python/v3_persistence/positive/traffic_light_persistence.frm`.
- [x] TypeScript: `frame_persistence_ts` library mirroring the same snapshot
      shape and helpers (`snapshotSystem`, `restoreSystem`,
      `snapshotToJson`, `snapshotFromJson`). V3 TypeScript systems now
      expose static helpers `saveToJson` / `restoreFromJson` that wrap these
      primitives, exercised by
      `language_specific/typescript/v3_persistence/positive/traffic_light_persistence.frm`.
- [x] Rust: `frame_persistence_rs` helper crate added with serde‑backed
      `SystemSnapshot` / `FrameCompartmentSnapshot` and a `SnapshotableSystem`
      trait, validated by internal unit tests; wiring into V3 Rust codegen is
      tracked separately under the Rust runtime parity work.

Stage 16 — Self‑Hosted Normalizers & Harness Builders (PRT Integration)
- [x] Python: call the generated `IndentNormalizer` machine from the V3
      Python emitter in place of the inline helper, preserving existing
      semantics while moving indentation fully into the self‑hosted machine
      for `v3_core`/`v3_control_flow`/`v3_systems`/`v3_systems_runtime`.
- [x] TypeScript: design and implement a small Frame machine
      that can build façade harnesses or other post‑MIR normalizers used in
      Stage 7 TS tests, mirroring the approach taken for Python indentation.
      This is currently the `TsHarnessBuilder` system under
      `framec/src/frame_c/v3/machines/ts_harness_builder.frs`, precompiled
      via `tools/gen_v3_machines_rs.py` into
      `ts_harness_builder.gen.rs` and wired into the crate as
      `framec::frame_c::v3::ts_harness_machine::run_ts_harness_builder`,
      with a unit test that asserts its output matches the behavior of
      `_execute_ts_harness_from_spliced` in the Python test runner.
- [x] Rust: audit the Rust V3 module path for post‑MIR string‑based
      transforms and confirm that, for current PRT scope, there are no
      additional Rust‑specific harness/normalizer transforms that merit
      extraction into machines beyond the existing runtime generator. Future
      Rust machines (e.g., snapshot diff tools) can be added on demand under
      `framec/src/frame_c/v3/machines/` following the same pattern as
      IndentNormalizer/TsHarnessBuilder.
- [x] Boot/precompile policy: align all PRT machines with a single
      precompile story:
      - FRM machines live under `framec/src/frame_c/v3/machines/*.frs`.
      - Rust sources are regenerated via `tools/gen_v3_machines_rs.py`
        using the pinned bootstrap compiler at `boot/framec/framec`, not as
        part of `cargo build`.
      - `framec/build.rs` emits `rerun-if-changed` hints and a warning when
        a `.frs` is newer than its `.gen.rs`, instructing contributors to
        run the precompile tool explicitly.

Stage 17 — Cross‑Language Snapshot Semantics (PRT)
- [x] Define a small schema-level snapshot comparison tool that can be
      used across PRT languages. This is currently implemented as:
      - Python/TypeScript: `tools/test_cross_language_snapshot_shape.py`,
        which round‑trips a canonical `TrafficLight` snapshot JSON through
        `frame_persistence_py` / `frame_persistence_ts` and asserts
        structural equality via `compare_snapshots`.
      - Rust:
        - `frame_persistence_rs::SystemSnapshot` tests (including
          `system_snapshot_canonical_json_round_trip`) which parse the same
          canonical JSON and validate `SystemSnapshot::compare`.
        - A Rust-native shape harness,
          `framec/src/bin/v3_rs_snapshot_shape.rs`, which constructs the
          same canonical JSON, invokes the Python and TypeScript libraries
          via subprocesses, and verifies that all three PRT targets agree
          on the DTO shape from a single Rust entrypoint.
- [x] Add cross‑language fixtures where the same logical system (TrafficLight)
      is executed in Python and TypeScript, then snapshotted and compared
      via the shared `SystemSnapshot` shape. This is exercised by:
      - `language_specific/python/v3_persistence/positive/traffic_light_snapshot_dump.frm`
      - `language_specific/typescript/v3_persistence/positive/traffic_light_snapshot_dump.frm`
      - `tools/test_cross_language_snapshot_traffic_light.py`, which
        compiles/runs both fixtures via the V3 module path and compares
        the resulting JSON snapshots for structural equality.
- [x] Extend the runtime‑level TrafficLight snapshot comparison to Rust by:
      - [x] Adding a small `@target rust` V3 `TrafficLight` system under
            `framec_tests/language_specific/rust/v3_persistence/positive/`
            that mirrors the Python/TypeScript fixtures (same states,
            parameters, and domain). This is
            `traffic_light_snapshot_dump.frm`.
      - [x] Wiring Rust @persist helpers: V3 Rust codegen now threads system
            parameters into generated constructors (annotated system headers
            are detected even with leading `@persist`) and seeds start params
            into `state_args`; persistence helpers (`save_to_json` /
            `restore_from_json`) remain on the system struct via
            `SnapshotableSystem`.
      - [x] Implementing a minimal Rust harness that:
            - includes the generated Rust module for `TrafficLight`,
            - drives it to the `Green` state with `domain = "red"`, and
            - constructs a `SystemSnapshot` JSON compatible with the
              canonical shape via `frame_persistence_rs::SystemSnapshot`.
            - This is wired via `tools/test_cross_language_snapshot_traffic_light.py`,
              which compiles the harness with `rustc` against the existing
              `frame_persistence_rs` rlib and captures its JSON output.
      - [x] Integrating the Rust JSON snapshot into the existing
            cross‑language comparison flow (`tools/test_cross_language_snapshot_traffic_light.py`)
            so that Python, TypeScript, and Rust snapshots are compared
            structurally against the canonical TrafficLight snapshot.

Stage 18 — Rust-Native Test Runner & Tooling Exploration (Future Work)
- [ ] Evaluate the feasibility and benefits of migrating selected
      Python-based V3 tooling (e.g., `framec_tests/runner/frame_test_runner.py`
      and small helpers under `tools/`) to Rust:
      - [x] Identify high-value candidates where a Rust implementation would
        reduce runtime dependencies, improve performance, or simplify
        integration with the Rust CLI. The initial prototype is the
        `v3_rs_test_runner` binary under `framec/src/bin`, which can run
        validation-only suites directly from Rust.
      - [x] Prototype a Rust-based harness for a small subset of tests and
        compare behavior against the existing Python runner:
        - `v3_rs_test_runner` currently drives:
          - `python v3_core`, `python v3_control_flow`,
            `python v3_systems`, `python v3_persistence`,
            `python v3_systems_runtime`
          - `typescript v3_core`, `typescript v3_control_flow`,
            `typescript v3_systems`, `typescript v3_persistence`
          using `@expect: Exxx` metadata on negative fixtures to match the
          Python runner’s semantics.
      - [ ] Expand coverage to additional categories (e.g., additional PRT
        suites and, later, more Rust language-specific categories such as
        persistence and systems/runtime) and, once behavior is stable,
        consider wiring a Rust-driven validation path into CI alongside the
        Python runner. Initial Rust coverage now includes `rust v3_core`
        and `rust v3_control_flow`.
      - [ ] Document tradeoffs (developer ergonomics, build complexity, CI
        impact) and keep Python tools as the reference until a Rust path
        can fully match their coverage and determinism. This includes a
        follow-up task to explore migrating more Python-based test tooling
        and helpers to Rust-only equivalents once Stage 18 prototypes have
        matured. See `18_rust_native_tooling.md` for details.
      - [ ] Rust test parity (PRT): port missing language_specific fixtures to Rust so counts match Py/TS, keeping truly language-specific cases segregated (e.g., legacy_async/py-runtime quirks stay under python-only; add Rust equivalents where semantics apply such as operators/scoping/systems_runtime/persistence/data_types); wire new fixtures into the runner.
- [ ] Longer-term roadmap for a Rust-first harness with full exec parity:
      - Phase 1 — Validation parity:
        - Generalize `v3_rs_test_runner` into a small Rust library + bin that
          understands the same metadata as the Python runner (`@expect`,
          `@meta`, `@skip-if`, etc.).
        - Expand Rust validation coverage to all PRT V3 categories for
          Python, TypeScript, and Rust (transpile/validation-only).
        - Add a compare mode that runs both the Rust and Python runners for
          a set of categories and reports any differences in pass/fail and
          error codes.
      - Phase 2 — Rust exec harness for Rust targets:
        - Implement a Rust-native exec path that compiles `.frs` to Rust,
          builds with `rustc`, and runs the resulting binary, mirroring the
          current `execute_rust` behavior from the Python runner.
        - Move `v3_exec_smoke` and curated Rust exec suites under this
          harness as the primary path once behavior matches the Python
          runner.
      - Phase 3 — Exec harness for Python/TypeScript targets from Rust:
        - Extend the Rust harness to drive exec for Python and TypeScript
          fixtures by invoking `python3`, `tsc`, and `node` as subprocesses,
          using the same expectations and toolchain skips as the Python
          runner.
      - Phase 4 — Cross-language persistence/snapshot tests in Rust:
        - Port the cross-language TrafficLight snapshot tests to a Rust
          binary that compiles and runs the Py/TS/Rust fixtures and compares
          their JSON snapshots via the shared persistence libraries.
      - Phase 5 — Unified Rust test CLI:
        - Introduce a Rust CLI entry (e.g., `framec test`) that can drive
          validation and exec across languages and categories using the
          Rust harness, with modes for Rust-only, Python-only, or compare.

Stage 19 — Rust-First Tooling Migration (Replace Python Harnesses)
- [ ] Replace Python-based orchestration tools (`framec_tests/runner/frame_test_runner.py`
      and selected helpers under `tools/`) with Rust-native equivalents,
      using the Stage 18 harness library as the foundation.
  - [ ] Persistence helpers driven by `@persist`:
    - [x] Python: update V3 module-path codegen so that `@persist system S`
          emits `@classmethod save_to_json(cls, system)` and
          `@classmethod restore_from_json(cls, text)` on the generated class,
          delegating to `frame_persistence_py.snapshot_system` /
          `frame_persistence_py.restore_system` (plus JSON helpers), and
          update the Python TrafficLight persistence fixtures to call these
          helpers directly.
    - [x] TypeScript: update V3 module-path codegen so that `@persist system S`
          emits static `saveToJson(system: S)` and `restoreFromJson(text: string)`
          methods on the generated class, delegating to
          `frame_persistence_ts.snapshotSystem` /
          `frame_persistence_ts.restoreSystem`, and update the TS TrafficLight
          persistence fixtures accordingly.
    - [x] Rust: for `@persist` V3 systems, derive or synthesize
          `frame_persistence_rs::SnapshotableSystem` implementations and
          emit inherent `save_to_json(&self)` / `restore_from_json(text: &str) -> Self`
          helpers on the generated system struct, then add a Rust persistence
          fixture that exercises these helpers.
  - [ ] Snapshot comparison tooling:
    - [x] Schema-level shape: port the cross-language snapshot shape check
      to a Rust-native tool:
        - Implemented as `v3_rs_snapshot_shape` under `framec/src/bin/`,
          which constructs the canonical TrafficLight snapshot JSON and
          uses `frame_persistence_py`, `frame_persistence_ts`, and
          `frame_persistence_rs` to compare snapshots through the shared
          `SystemSnapshot` shape.
    - [x] Runtime-level TrafficLight snapshots: port
      `tools/test_cross_language_snapshot_traffic_light.py` to a Rust
      binary that:
        - Implemented as `v3_rs_snapshot_traffic_light` under
          `framec/src/bin/`, which:
          - Compiles and runs the Py/TS/Rust fixtures via the V3 pipeline.
          - Uses the per-language persistence libraries to generate JSON
            snapshots at runtime.
          - Normalizes and compares the three JSON snapshots in Rust via
            `serde_json::Value` and exits non-zero on divergence, so all
            PRT runtimes must agree on the canonical TrafficLight snapshot
            without invoking the Python tool as the top-level driver.
  - [ ] Test runner consolidation:
    - [x] Library extraction:
      - Core harness logic lives in `framec::frame_c::v3::test_harness_rs`
        (validation, exec-smoke, curated exec helpers), and
        `v3_rs_test_runner` calls into this module so that future CLI
        entry points (e.g., `framec test`) can share the same behavior.
    - [x] Minimal `framec test` subcommand (validation-only):
      - `framec test` subcommand added to the `framec` CLI with
        `--language` and `--category` filters, delegating to the shared
        harness library for validation-only runs (single category at a
        time), currently exercised for PRT V3 `v3_core` slices.
    - [x] Compare mode in `framec test`:
      - `framec test` supports a `--compare-python` flag that mirrors the
        `compare` mode in `v3_rs_test_runner` for a single slice by
        running both the Rust harness and the Python runner on the same
        `<language>/<category>` and reporting divergences; initial usage
        validated for PRT `v3_core` (python/typescript/rust).
    - [x] Exec modes in `framec test`:
      - `framec test` supports `--exec-smoke` and `--exec-curated` flags
        as thin wrappers around the existing exec helpers in the harness
        library, with the same category gating as `v3_rs_test_runner`
        (currently exercised for `v3_exec_smoke` and curated `v3_core`
        slices across PRT languages).
    - [x] Validation coverage expansion:
      - `framec test` + `--compare-python` has been validated for the PRT
        V3 categories `v3_core`, `v3_control_flow`, `v3_systems`,
        `v3_persistence`, and `v3_systems_runtime` across
        python/typescript/rust, and fixtures have been updated with
        `@expect` where needed so that the Rust harness and Python runner
        agree on validation outcomes.
    - [ ] Gradual deprecation:
      - Gradually deprecate Python-specific runners for V3-only suites
        once the Rust harness reaches coverage and determinism parity,
        keeping the Python path available as a fallback for non-PRT
        languages.
  - [ ] CI integration and docs:
    - Add an optional CI job that runs the Rust-native harness for a
      selected subset of categories alongside the existing Python jobs.
    - Update `HOW_TO.md` and `18_rust_native_tooling.md` with guidance on
      using the Rust-first test CLI, and document which Python tools remain
      as reference implementations (e.g., legacy/compat suites).
  - [ ] Cross-language test parity (PRT fixtures):
    - Add async/await fixtures for TypeScript and Rust (interface/actions/operations/functions) mirroring Python async support; keep Python-only legacy async separate.
    - Add Rust persistence/data_types breadth to match Python/TS counts.
    - Ensure operators/scoping/systems_runtime are aligned across Py/TS/Rust (fixtures added where applicable; keep truly language-specific cases segregated).
  - [ ] Productionalization:
    - Move bulky repo-local fixtures/runners (e.g.,
      `framec_tests/python/src/positive_tests/` and adapter protocol smoke
      assets) into the shared test environment and wire the harnesses to
      consume them from there.

Rust Runtime Parity (PRT Progress)
- [x] Basic Rust V3 runtime scaffold:
  - `StateId` enum with `Default`, a minimal `FrameEvent` struct, and a
    `FrameCompartment` struct with `Default` and a `state: StateId` field.
  - Generated system struct per Frame system with `compartment` and
      `_stack: Vec<FrameCompartment>`.
  - Runtime helpers on the system: `_frame_transition`, `_frame_router`
      (stub), `_frame_stack_push`, `_frame_stack_pop`, and `new()` seeded from
      the Arcanum start state.
- [x] Router semantics: `_frame_router` for Rust V3 now dispatches on
      `FrameEvent.message` by calling internal `_event_<name>()` methods
      that in turn switch on `StateId`, mirroring the Python/TypeScript
      design for module‑path systems.
- [x] `system.return` for Rust: module‑path systems synthesize a typed
      per‑system return enum, a per‑method setter
      (`_set_system_return_for_<name>`) and public wrappers that push a
      per‑call slot, route an event, and pop/return the payload, following
      the semantics in `frame_runtime.md` and `codegen.md` (verified by
      `v3_cli/positive/system_return_cli.frm`).
- [x] Curated Rust exec: the V3 runner executes Rust V3 systems via the
      module‑path runtime for `v3_core`/`v3_control_flow`/`v3_systems`
      (gated by `--exec-v3 --run`), and all curated exec fixtures in these
      categories pass under `framec` v0.86.64.

PRT Stages 7–13 Closure Checklist
- Stage 7 — Native Validation (PRT)
  - [x] Python: Enable native facade / `py_compile` for all Frame‑owned runtimes and adapters (including `PythonDebugRuntime.frm`); syntax errors in native blocks now fail validation/tests via `@py-compile` in V3 fixtures and the shared‑env Bug #091 verification.
  - [x] TypeScript: Ensure all Frame‑owned TS runtimes/adapters are covered by `@tsc-compile` + TS facade; syntax errors fail tests.
  - [x] Rust: Keep V3 CLI and basic facade smoke green; add at least one Rust native compilation probe (`@rs-compile`) that exercises generated runtime code and fails on native syntax errors (wired via `rustc --crate-type lib` in the test runner).
  - [x] Docs/PLAN: State that Stage 7 is mandatory for Frame‑owned PRT runtimes/adapters, optional for user projects.
- Stage 8 — Codegen Adapters (PRT)
  - [x] Python: Stage 8 codegen adapters are explicitly deferred for PRT; V3 Python continues to emit deterministic string‑based code without an external formatter/AST printer. Any future adapter (e.g., `black`/`ruff`) must be gated behind CLI flags and not required in CI.
  - [x] TypeScript: Stage 8 codegen adapters are explicitly deferred; V3 TypeScript relies on the existing deterministic generator. Future adapters (e.g., SWC/TypeScript AST printers) must be optional and off by default.
  - [x] Rust: Stage 8 formatting/codegen adapters are explicitly deferred; V3 Rust uses the embedded struct‑based runtime and string‑based emission. Optional `rustfmt` integration, if added later, must be behind an opt‑in flag and not part of the core pipeline.
- Stage 9 — Policy Validation (PRT)
  - [x] Python: Native policy rules enforced via ValidatorV3 (E400–E405, E406) with negatives in `v3_validator` and `v3_capabilities` (see `09_validation.md`).
  - [x] TypeScript: Same ValidatorV3 policy coverage as Python with TS-specific negatives in `v3_validator`/`v3_capabilities` (including system.calls E406 tied to bugs 73/74/88/89).
  - [x] Rust: Basic Rust policy rules covered by ValidatorV3; explicit Rust‑focused negatives exist for actions/ops (E401), handler/section structure (E111/E113), and `system.method` E406. `_frame_*` misuse remains a future enhancement once more Rust‑specific facade/runtime policies are introduced.
  - [x] Docs: `09_validation.md` updated to enumerate PRT‑specific policies and link to their test categories.
- Stages 10–11 — AST/Symbol Integration & Errors (PRT)
  - [x] Python: All PRT error codes (E1xx/E2xx/E3xx/E4xx/E5xx) observed in V3 Python tests are documented in `11_error_taxonomy.md`. Arcanum‑backed semantics are used for E402/E403/E404/E405 as intended; other policies (e.g., E406) are driven by ModuleAst + interface metadata.
  - [x] TypeScript: Same for TS; the shared taxonomy covers the TS scanner/outline codes and the PRT policy set, including E406 for `system.method` calls.
  - [x] Rust: Same for Rust; the Rust validator now uses the shared taxonomy for structural and policy errors, and any remaining string‑based header parsing has been migrated to OutlineScannerV3/ModuleAst helpers.
- Stage 12 — Testing Strategy (PRT)
  - [x] Python: Curated exec for `v3_core`, `v3_control_flow`, `v3_scoping`, and `v3_systems` is stable under the V3 test runner; Stage‑7 native‑validation negatives live under `language_specific/python/v3_facade_smoke/negative` and are driven by `FRAME_VALIDATE_NATIVE_POLICY` + `py_compile` (see `strict_*` fixtures and Bug #091 in the shared env).
  - [x] TypeScript: Curated exec and `v3_capabilities` are stable; Stage‑7 native‑validation negatives for TS live under `language_specific/typescript/v3_facade_smoke/negative` and are driven by the same V3 runner policy + `@tsc-compile` for CLI fixtures.
  - [x] Rust: V3 Rust exec‑smoke and curated exec use the struct‑based runtime (`StateId`/`FrameCompartment`/system struct) with `_frame_router` wired for handlers; Rust native‑validation probes exist via `v3_facade_smoke/negative` and `@rs-compile` in V3 CLI fixtures. Curated Rust exec for `v3_core/control_flow/systems` is exercised in CI via the `v3_curated_exec` workflow.
  - [x] All PRT: `all_v3` transpile‑only + curated exec are wired into CI as separate jobs (`v3_all.yml`, `v3_curated_exec.yml`), and PRT languages (Python/TypeScript/Rust) participate in both paths.
- Stage 13 — Project Layer (Reserved, PRT)
  - [x] Python: Add at least one small `frame.toml` project fixture (see `language_specific/python/v3_project/positive/project_basic`) and ensure `compile-project` + V3 validator work end‑to‑end for multi‑file projects; runtime layout verified via runner `frame_runtime_py` checks.
  - [x] TypeScript: Same for TS (`language_specific/typescript/v3_project/positive/project_basic`) using `compile-project`; `v3_project` positives/negatives green.
  - [x] Rust: Same for Rust (`language_specific/rust/v3_project/positive/project_basic`) with a simple multi‑file project; `v3_project` positives/negatives green under `compile-project`.
  - [x] Docs: Make clear that, for PRT, Stage 13 currently covers manifest + project build only (no FID), and that this is “complete for 1–13” once these project fixtures are green. See `architecture_v3_overview.md` (“Project Configuration (frame.toml)” and “Project Layer (Stage 13, PRT)”) for the authoritative description.
  - [ ] Production polish (project layer):
    - [x] Robust error handling/diagnostics: validate `frame.toml` schema; emit friendly messages for missing entry points, duplicate systems across modules, conflicting `@target` files (compile-project now enforces single-target and reports missing/mismatched targets and dup systems).
    - [x] Path resolution hardening: ensure `paths.modules` handles relative/absolute paths safely; add fixtures for multiple source dirs (multi-dir v3_project fixtures added).
    - [x] Multi-target stance: explicitly disallow mixed `@target` files in one project; fixtures/diagnostics enforce single-target projects.
    - [x] CI: add a dedicated `compile-project` job for Py/TS/Rust to catch regressions.
    - [x] Packaging: define a stable output layout for project builds (runtime copying, build dirs) and document it.
