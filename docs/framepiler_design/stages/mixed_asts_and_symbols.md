# Mixed ASTs and Symbol Linking

Overview
The Frame compiler maintains a clear separation between Frame semantics and target‑native structure while allowing them to coexist in the same body. This document explains how MixedBody/MIR, target parser ASTs (SWC/rustpython), and the Frame symbol tables (Arcanum) relate.

Components
- Frame AST: Systems, states, handlers, actions, operations, expressions, etc.
- Arcanum: Frame symbol tables (scopes, names, types) built in Parser Pass 1 and consulted in Pass 2.
- Target AST (optional): SWC AST for TypeScript or rustpython AST for Python, attached as `ParsedTargetBlock` for native‑only bodies.
- MixedBody/MIR: Ordered sequence of `NativeText`/`NativeAst` spans and `MirStatement` nodes when native bodies contain Frame statements.
- Native Symbol Index (sidecar, optional): A lightweight index of native identifiers discovered in target ASTs for diagnostics; not used for codegen or semantic binding.

Linking strategy
1) Frame symbols remain Frame‑owned
   - Arcanum holds system/state/action/operation/domain symbols and call chains. We do not merge native symbols into Arcanum.
   - Frame statements inside native bodies (MIR) reference Frame symbols only (e.g., `$State`, stack ops). Statement arguments are currently captured as strings; future work may associate parsed target expressions.

2) Native symbols live in a sidecar index (optional)
   - When a target AST is available (native‑only bodies), we can index top‑level names and imports to provide advisory diagnostics (e.g., undefined native name).
   - This index is separate from Arcanum and never blocks codegen.

3) Pseudo‑symbol translation (early rewrite) [planned]
   - Certain cross‑target conveniences like `system.return` or `self` shorthands can be rewritten in an early pass:
     - TS: `system.return` → `this.returnStack[this.returnStack.length - 1]`
     - Py: `system.return` → `self.return_stack[-1]`
   - Rewrites happen before MixedBody/MIR emission to keep Frame‑statement args target‑native and avoid Frame‑specific leakage in native code.

4) MixedBody drives emission; target AST validates
   - Mixed bodies: visitors emit `NativeText` verbatim (or `NativeAst` when available) and expand `MirStatement` using deterministic glue; we do not need a native code printer for glue.
   - Native‑only bodies: visitors may emit verbatim or later leverage native ASTs; today we prefer verbatim emission for determinism.

5) Diagnostics and source maps
   - Native parser spans map to Frame via `TargetSourceMap`.
   - Mixed bodies synthesize spans for Frame statements at their Frame lines; native text spans record start/end Frame lines.
   - Error messages include both domains where useful (Frame+target).

Resolution rules for statement arguments [future]
- If MIR arguments contain native expressions (e.g., `-> $S1(a, this.count)`), we keep them as strings in MIR, and emit them directly in glue. In future we may parse these expressions via the target parser to enable better diagnostics without changing emission.

Security & stability goals
- No dependence on external printers for glue; custom emission ensures formatting stability across versions.
- Keep symbol binding deterministic by separating Frame symbols (Arcanum) from native symbol indices.

Appendix: Typical flows
- Native‑only TS handler: Target parser → `ParsedTargetBlock` (SWC AST) → visitor emits verbatim; optional native symbol index created.
- Mixed TS handler: Segmenter → MixedBody { NativeText/NativeAst + MirStatement } → visitor emits verbatim + glue; Arcanum resolves Frame symbols used by MIR.
- Python mirrors TS flow with rustpython_parser and target‑specific glue.
