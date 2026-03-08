# AST & Symbol Integration (V3)

Status
- Frame‑side symbols (Arcanum) are integrated and authoritative.
- Native‑side symbols are advisory today; Stage 10 will complete fine‑grained integration in steps 10A–10E.

Purpose
- Define how the authoritative Frame AST and symbol tables integrate with native contexts and MixedBody/MIR.

Authoritative Structures
- Frame AST models modules, systems, states, handlers, actions/ops, and MixedBody.
- `Arcanum` (symbol tables) is built in a dedicated pass for Frame constructs: modules → systems → members → handlers.

Mixed Context (Handlers)
- MixedBody introduces no new Frame symbols. MIR items may reference Frame entities:
  - Transition targets: `$State` → resolve to a `StateId` via `Arcanum`.
  - Event params: available in handler scope for expansion (names/types).
- Resolution occurs at MIR assembly/expansion time using `Arcanum` as read‑only input.

Native Contexts
- Actions/operations are native‑only; they may be optionally parsed into native ASTs for diagnostics/formatting but do not affect `Arcanum`.
- If native AST parsing is enabled, a lightweight `NativeBindings` index may be produced for imports and local declarations to power diagnostics and FID extraction. This index is orthogonal to Frame symbols.

FID & Native Imports
- Native import partitions are collected during partitioning and/or native parsing to form a `NativeImportIndex`.
- FID generation consumes this index to validate and surface external capabilities used by generated code.

Coordinate Spaces
- Byte spans are ground truth in all structures. A `byte→(line,col)` index is maintained for diagnostics only.
- `splice_map` remaps spliced body spans back to original Frame/native origins and is used for native diagnostic attribution and source map composition.

Cross‑Pass Contracts
- No pass mutates `Arcanum` after initial construction; MIR/visitors read from it.
- MixedBody remains the single source of truth for embedded Frame semantics across stages.

Fine‑Grained Plan (Stage 10)
----------------------------

10A — Native Symbol Snapshots (Py/TS)
- Extract safe metadata (names/params) from native ASTs per segmented body.
- Map spans through `splice_map` for any diagnostics.
- Advisory only; does not alter Frame semantics.

10B — Advisory Validation (flag‑gated)
- Add `--validate-native-policy` to enable optional checks (e.g., transition state_args arity vs outline params).
- Diagnostics include mapped spans; disabling the flag reverts to current behavior.

10C — Unified Symbol Query Surface
- Provide helper APIs that return Frame (Arcanum) and Native (snapshots) views for a handler.
- Intended for tools/tests; no semantic coupling to native.

10D — Runner/CI Integration
- Runner preset toggles `--validate-native-policy` for Py/TS suites; JUnit includes mapped spans for new diagnostics.

10E — Documentation
- This document remains authoritative; architecture.md and testing strategy link here.

Risks & Mitigations
-------------------
- Parser drift/versioning: pin and vendor parsers; hermetic defaults (pure‑Rust parsers default‑on; tree‑sitter remains feature‑gated).
- Performance: segment parsing, cache, and limit to validation paths.
- Mapping correctness: assert mapped spans through `splice_map` in fixtures and unit tests.
- Overreach: keep native snapshots advisory; Arcanum + MIR own Frame semantics.

Stage 09 — Symbol Table Migration for State Targets & Parent Forward
-------------------------------------------------------------------
Goal
- Replace coarse, file‑local known‑state scans with proper symbol resolution via `Arcanum` for:
  - Transition target existence (E402)
  - Parent forward availability (E403)

Scope & Phasing
1) Module‑local (single file)
   - Build `Arcanum` from outline: SystemDecl → MachineDecl → StateDecl (with optional parent) → HandlerDecl.
   - MIR validation resolves `$State` names to state symbols; unknown emits E402 with precise spans.
   - Parent forward `=> $^` checks consult state parent metadata; missing parent emits E403.
   - Preserve tolerant (collect‑all) reporting: aggregate resolution + parse/structural issues.

2) Project layer (multi‑file)
   - Promote `Arcanum` to a project graph with imports and cross‑file resolution.
   - Resolve transitions across files; include ref/def context in diagnostics; cache for incremental builds.

AST Nodes (sketch)
- SystemDecl { name, machines[] }
- MachineDecl { name, states[] }
- StateDecl { name, parent?: Ident, span }
- HandlerDecl { name, params[], kind, span }

Integration Points
- v3/mod.rs: build `Arcanum` after outline scan; pass to validator.
- v3/validator.rs: use `Arcanum` for E402/E403; remove known‑state scan when stable.
- v3/mir_assembler.rs: keep producing MIR unchanged; validator performs symbol checks.

Tests
- Port existing E402/E403 negatives to symbol resolution (messages unchanged).
- Add cross‑file target unknown (project layer) later.
- Keep multi‑issue fixtures to ensure aggregation.

Migration Notes
- Keep original known‑state code as a fallback during bring‑up behind a feature flag; remove once symbol path is green across languages.
# Stage 10 — AST & Symbol Integration (V3)

Status: Complete for Python/TypeScript/Rust (advisory native snapshots, symbol‑table validation)

Scope
- Keep Arcanum (Frame symbol table) authoritative for Frame semantics (E402 unknown state, E403 parent‑forward).
- Add hermetic, advisory native parser snapshots to enrich tooling (params, spans), without changing MIR/codegen.
- Maintain tolerant diagnostics and stable error codes.

Components
- Arcanum (authoritative)
  - Built from the outline (systems/machines/states, optional parents, state params from headers).
  - Used in validator for E402/E403 (module path) and in exec contexts for compiled state ids.

- Advisory native symbol snapshot (parser‑backed)
  - Emitted as a trailer comment: /*#native-symbols# … #native-symbols#*/ with schemaVersion.
  - Entry shape: { state, owner, params[], paramSpans[], schemaVersion }.
  - TypeScript (SWC): parses a synthesized function signature (header‑derived) to collect param identifiers; falls back to header extraction when parser is disabled or fails; spans remain header‑derived for stability.
  - Python (RustPython): parses a synthesized def __f(...): pass and conservatively extracts identifier names; header fallback retained; spans remain header‑derived.
  - Advisory only: snapshots never alter MIR, validation, or codegen; they exist to help tools/IDE/debugger and optional policy checks.

- Optional advisory validator policies
  - E405 (state param arity mismatch) compares transition state_args to state header param count (Arcanum). Flag‑gated via FRAME_VALIDATE_NATIVE_POLICY=1; off by default.

Diagnostics & Trailers
- errors‑json: always emitted for V3 demo compiles; includes all validator issues (E400/E401/E402/E403 and advisory E405 when enabled) with schemaVersion.
- frame‑map: splice origin mapping with version + schemaVersion; used for attribution and mapping tests.
- visitor‑map: optional targetLine→sourceLine mappings (single‑body and module path for Py/TS) with schemaVersion; used for stepping/breakpoints tests.

Hermetic Policy
- Use pure‑Rust parsers by default (SWC, RustPython, syn). Tree‑sitter adapters remain feature‑gated for C‑family/Java/C#.
- No external toolchains; no network I/O.

Testing
- Runner asserts presence/shape for errors‑json, frame‑map (map + schemaVersion), and visitor‑map (mappings + schemaVersion) where applicable.
- Per‑test @expect (E‑codes) supports @expect‑mode: equal|superset (equal used for multi‑issue fixtures).
- Curated exec (Py/TS/Rust) validates runtime markers with @run‑expect and, when needed, @run‑exact for precise outputs.

Notes
- Parser‑backed snapshots are intentionally advisory: they help tools and optional checks while keeping the compiler pipeline deterministic and Frame‑semantics led by Arcanum.
