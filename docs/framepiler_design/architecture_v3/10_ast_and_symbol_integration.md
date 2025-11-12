# AST & Symbol Integration (V3)

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
