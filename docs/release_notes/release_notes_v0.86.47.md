# Release Notes — v0.86.47

**Date**: 2025-11-18  
**Type**: architecture / semantic validation

## Summary

This release completes the V3 outer‑pipeline refactor for Python and TypeScript
by moving system outline, machine/state headers, handler placement, system
parameter semantics, and domain block rules onto dedicated parsers and scanners
that build a `ModuleAst` and `Arcanum` symbol table. It also aligns the V3
documentation set with this architecture and confirms that the PT V3 suites
remain fully green.

## Changes

### 1. AST‑Backed Outer Pipeline (Systems, Machine, Domain)

- **SystemParserV3 → ModuleAst**
  - Parses `system` headers and `system_params` into:
    - `SystemAst { name, params: SystemParamsAst, sections: SystemSectionsAst, section_order: [SystemSectionKind] }`.
  - Locates per‑system spans for:
    - `operations:`, `interface:`, `machine:`, `actions:`, and `domain:`.
  - Drives block ordering and uniqueness checks:
    - **E113** — mis‑ordered blocks (expected operations → interface → machine → actions → domain).
    - **E114** — duplicate block labels within a system.

- **MachineParserV3**
  - Operates within a system’s `machine:` span to:
    - Find `$State` headers and capture their parameters.
    - Locate `$>()` entry handlers for the start state and capture their parameters.
  - Supplies parameter lists for system‑parameter semantics (E416/E417).

- **DomainBlockScannerV3**
  - Operates within `domain:` spans to enforce declaration‑only content:
    - `var ident = <expr>` or `ident = <expr>`.
  - Reports **E419** when a non‑blank, non‑comment line is not a declaration.

- **ModuleAst + Arcanum**
  - `build_arcanum_from_module_ast(bytes, &ModuleAst)` now constructs the V3
    symbol table:
    - Systems → machines → states (`StateDecl { name, parent, params, span }`).
  - The validator uses `Arcanum` to:
    - Resolve transition targets (**E402**).
    - Enforce parent‑forward availability (**E403**).
    - Check state parameter arity vs transitions (**E405**).
    - Provide state spans for handler placement (**E404**).

### 2. Handler Placement and System Parameters (E404/E416–E418)

- **Handler placement (E404)**
  - Replaces the outline‑only `validate_handlers_in_state` rule with
    `validate_handlers_in_state_ast`, which:
    - Collects all state spans from `Arcanum`.
    - Verifies that each handler header (`BodyKindV3::Handler`) lies within
      some state’s span.
    - Emits **E404** when a handler is outside any state block.

- **System parameter semantics (E416–E418)**
  - Uses `ModuleAst` + `Arcanum` + `MachineParserV3` + `DomainBlockScannerV3` to enforce:
    - **E416** — start parameters `$(...)` must match the start state’s parameter list.
    - **E417** — enter parameters `$>(...)` must match the start state’s `$>()`
      handler parameter list; missing `$>()` or mismatched names are errors.
    - **E418** — each domain parameter must correspond to a variable
      declared in the system’s `domain:` block.
  - These rules are documented in `frame_language_guide.md` and covered by
    PT fixtures, including Python’s TrafficLight runtime system.

### 3. Documentation Alignment

- **`architecture_v3/grammar.md`**
  - Introduces the outer AST (`ModuleAst`, `SystemAst`, `SystemSectionKind`)
    and describes the roles of `SystemParserV3`, `MachineParserV3`, and
    `DomainBlockScannerV3` in the V3 pipeline.

- **`architecture_v3/frame_language_guide.md`**
  - Updates:
    - Block ordering and uniqueness semantics to reference `ModuleAst` and
      E113/E114.
    - System parameter semantics to describe E416–E418 and the PT runtime
      status.
    - State hierarchy to note that parent relationships are tracked in
      `Arcanum` and interpreted by the parent‑forward rule (E403).

- **`architecture_v3/frame_runtime.md`**
  - Remains the semantic reference for the runtime model (compartments,
    transitions, parent forward, stack).
  - Clarifies that Python’s V3 runtime now follows the documented system/start
    semantics via the TrafficLight and related fixtures, while TypeScript remains
    compile/validate‑only for this feature.

- **`framelang_design/capability_matrix.md`**
  - Marks handler placement and system parameters as V3‑native features:
    - Handler placement: enforced via `ModuleAst` + `Arcanum` (E404) with
      explicit PT fixtures.
    - System parameters: structural + semantic in both Python and TypeScript,
      with Python runtime coverage (TrafficLight) and TypeScript in
      transpile/validate mode.

### 4. PT Validation

- `cargo build --release` completes successfully.
- PT V3 suites:
  - `python3 framec_tests/runner/frame_test_runner.py --languages python typescript --categories all_v3 --framec ./target/release/framec --transpile-only --include-common`
  - Result:
    - Python: **112/112** tests passing.
    - TypeScript: **101/101** tests passing.

## Notes

- The v2 outer pipeline and legacy tests remain available for historical
  reference but are no longer the primary source of truth for V3 semantics.
- Future work will extend the same AST‑backed model to Rust codegen and
  runtime behavior, and incrementally retire remaining outline/byte‑level
  heuristics where they are still used for non‑PT targets.

