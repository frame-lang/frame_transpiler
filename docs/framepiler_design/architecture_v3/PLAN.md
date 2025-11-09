# V3 Execution Plan — First Principles Rebuild

Goal
- Rebuild the single‑file pipeline from first principles using the V3 docs as the source of truth, then add the multi‑file project layer. Keep the new code hermetic and deterministic, retire legacy paths once parity is reached.

Scope (MVP → Plus)
- MVP: Stages 01–06 (closers, streaming scanners, Frame Statement parser, MIR assembly, expansion, splice/mapping), per‑file TS/Py. Validation rules enforced. No native parse facades in the critical path.
- Plus: Stage 07 (optional native parse facades for diagnostics/formatting), Stage 08 polish, Project layer (FID, linking/packaging), incremental build.

Repository Mechanics
- Create `framec/src/frame_c/v3/` and implement V3 types alongside legacy.
- Wire CLI feature flag or environment toggle to run V3 pipeline end‑to‑end during migration.
- Once V3 hits parity (MVP acceptance), delete legacy pipeline and flip V3 to default.

01 — Body Closers (per target)
- Objects: `BodyCloserPyV3`, `BodyCloserTsV3` (trait `BodyCloserV3`)
- Deliverable: DPDA closer returns `close_byte` for body starting at `{`.
- Acceptance:
  - Python: handles single/double/triple quotes, f‑strings, `#` comments; returns precise close or characterized failure.
  - TypeScript: handles quotes, block/line comments, templates with nested `${…}`.
- Tests (existing): parser.rs unit tests for TS/Py textual closers; Py triple‑quote/f‑string fixtures; TS templates fixtures.
- New micro‑fixtures (planned): `framec_tests/v3/01_closers/{py,ts}/*.frm` (unterminated cases, deep bodies).

02 — Native Region Scanners (streaming)
- Objects: `NativeRegionScannerPyV3`, `NativeRegionScannerTsV3` (trait `NativeRegionScannerV3`)
- Deliverable: one pass → `ScanResultV3 { close_byte, regions }`; regions are `RegionV3::{NativeText, FrameSegment}` with byte spans and `kind_hint`.
- Acceptance: SOL‑anchored detection only; Unicode whitespace accepted; protected‑region aware; O(n), must‑advance guaranteed.
- Tests (existing): Py event_handler_incremental; TS islands (comments/strings with statement‑like tokens).
- New micro‑fixtures: `framec_tests/v3/02_scanner/{py,ts}/…` to assert segment boundaries with a small dump tool.

03 — Frame Statement Parser (FIRST‑set)
- Objects: `FrameStatementParserV3` (+ `FrameStatementParserPyV3/TsV3`), helpers `NativeArgSplitterPyV3/TsV3`.
- Deliverable: tiny parser validates head/token, balanced parentheses; splits arg list at top‑level commas (string/nesting aware). Produces `MirItemV3::{Transition,Forward,Stack*}` with raw arg strings and byte span.
- Acceptance: clear errors (invalid head; unmatched `)` in args; trailing tokens).
- Tests (existing): negatives for malformed transitions and non‑terminal violations.
- New micro‑fixtures: `framec_tests/v3/03_parser/*.frm` positive/negative per statement kind.

04 — MIR Assembly
- Objects: `MirAssemblerV3` → `[MirItemV3]` from `RegionV3`.
- Deliverable: MixedBody/MIR authoritative for handlers; actions/ops native‑only enforced via validation.
- Acceptance: preserves order, spans; no parser‑level statements in handlers.
- Tests: transitions_terminal rule; language_specific suites.
- New: mapping checks with debug JSON anchors.

05 — Frame Statement Expansion (per target)
- Objects: `FrameStatementExpanderPyV3/TsV3`, optional `IndentationAnalyzerPyV3/TsV3` (AST‑aware when Stage 7 enabled).
- Deliverable: textual glue + early returns injected with correct indentation; preserve elif/else/except/finally (Py) and else if (TS) chains.
- Acceptance: terminal statements suppress following native code (validator + emission behavior); sibling‑based indentation good; AST‑aware optional.
- Tests (existing): Py if_elif_returns, try/except, async_*; forward events; stack ops. TS control_flow/core.
- New: incremental indentation tests `framec_tests/v3/05_expander_py/*.frm`.

06 — Splice & Mapping
- Objects: `SplicerV3`, `SourceMapComposerV3`.
- Deliverable: build spliced body and compose source maps attributing expansions to Frame statement frame lines.
- Acceptance: golden mapping anchors in debug mode; breakpoint alignment samples.
- Tests (new optional): mapping golden files and human JSON trailers.

07 — Native Parse Facades (optional)
- Objects: `NativeParseFacadePyV3/TsV3`, `IndentationAnalyzerPyV3/TsV3`.
- Deliverable: parse spliced body for diagnostics/formatting; refine indentation when present.
- Acceptance: off by default; no regressions when on.
- Tests: formatting/diagnostics mapping samples; off by default in CI.

08 — Codegen (adapters, optional)
- Objects: `TsB2CodegenV3`, `PyB2CodegenV3` (future polish).
- Deliverable: AST‑based emission for deterministic formatting where desired.

09 — Validation
- Objects: `ValidatorV3`, rules: `TerminalLastRuleV3`, `NoFrameStatementsInActionsOpsRuleV3`, `PythonNativePolicyRuleV3`, etc.
- Deliverable: clear diagnostics and rule coverage at MixedBody/MIR level.
- Tests (existing): negatives and policy suites; ensure runner invokes validator post‑transpile.

Project / Multi‑File Layer (after MVP green)
- Objects: `FileLoaderV3`, `ModuleResolverV3`, `ProjectGraphV3`, `FIDIndexV3`, `FIDEmitterV3`, `SemanticAnalyzerV3`, `TsModuleLinkerV3`, `PythonPackagePlannerV3`, `BuildPlannerV3`.
- Deliverables: FID emission/consumption, import resolution, stable linking/packaging, incremental build.
- Acceptance: multi‑file TS/Py suites execute and link correctly; one shared runtime import per module set.
- Tests: import graph positives/negatives, circular detection, missing FID, signature mismatch.

Legacy Retirement Checklist
- Flip default pipeline to V3 in CLI, keep legacy behind a guard temporarily.
- When V3 passes all single‑file + multi‑file suites, delete legacy parser/scanner/visitor paths and dead flags.
- Purge remaining documentation pointing at legacy (mark “V2/legacy” where retained historically).

CI & Tooling
- Gate each stage with per‑stage tests and full language_specific suites.
- Debug flags for mapping/anchors; JSON/human outputs for map inspection.
- Caches: content‑hash keyed `RegionScanCacheV3`/`MirCacheV3` for future incremental builds.

Milestones & Gating
- M1: Stages 01–03 green with micro‑fixtures; scanners return identical close/segment boundaries as legacy on sampled files.
- M2: Stage 04/05: Python language_specific 100% validate + execution ≥95%; TS language_specific 100% validate.
- M3: Stage 06 mapping debug anchors verified on samples.
- M4: Project layer minimum viable linking (TS/Py) + FID round‑trip; multi‑file suites pass.
- M5: Legacy retirement; V3 default.

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

