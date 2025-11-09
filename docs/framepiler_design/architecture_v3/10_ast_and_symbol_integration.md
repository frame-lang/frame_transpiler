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

