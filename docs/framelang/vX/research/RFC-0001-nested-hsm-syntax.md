---
id: RFC-0001
title: Nested HSM Syntax for Frame
authors: ["Project Owner"]
status: draft
created: 2025-11-08
updated: 2025-11-08
tracking: []
supersedes: []
superseded_by: []
---

## Summary
Introduce a nested syntax for hierarchical state machines (HSMs) where parent/child relationships are expressed lexically. The proposal desugars to the existing Core Frame Contract and MIR so backends and runtimes require no semantic changes.

## Motivation / Problem
- Current HSMs declare states flat and bind parents via separate mapping lines (`$Child => $Parent`). This splits context and invites drift.
- Nested syntax makes hierarchy obvious and co-locates handlers and relationships, improving readability and refactors.

## Goals / Non‑Goals
- Goals: lexical parent declaration, optional defaults, no runtime semantic changes, minimal parser churn, MixedBody compatibility.
- Non‑Goals: new control operators, altered enter/exit ordering, new runtime APIs.

## Design (Syntax + Semantics)
### State Nesting
```
system S {
  machine:
    $Parent(x, y) {
      $Child1(a, b) { ... }
      $Child2(c, d) { ... }
    }
}
```
- Nesting declares `Parent` as the parent of `Child1` and `Child2`.

### Defaults (named)
Option A (header):
```
$Child1(a, b) defaults(state_args: (0, 1), parent_enter: (self.name, "E5")) { ... }
```
Option B (block):
```
$Child1(a, b) {
  defaults:
    state_args: (0, 1)
    parent_enter: (self.name, "E5")
  ...
}
```
- `state_args`: default arguments for entering `Child1` when unspecified by caller.
- `parent_enter`: default arguments forwarded to the parent on child enter/exit when applicable.

### Handlers
Unchanged. SOL Frame statements (`-> $State(...)`, `=> $^`, `$$[±]`, `return`) remain valid inside `$>()`, `$<()`, and event handlers.

### Constructors and Destructors (New Hooks)
- `$>>()` — state compartment constructor; runs when a state’s compartment is created while entering a new ancestor chain (one‑time per creation, not per event).
- `<<$()` — state compartment destructor; runs when the state’s compartment is permanently dropped while leaving its ancestor chain.
- Enter/Exit remain event‑driven hooks for the leaf: `$>()` / `$<()`.

Ordering (LCA‑based):
- On transition to target leaf T (with target chain A0→…→T):
  1. Evaluate state/default arguments (`state_args`, `parent_enter`).
  2. Compute LCA with the current chain.
  3. For states added below the LCA (top‑down): run `$>>()`.
  4. Run `$>()` for T only (parents’ enter do not run on every event).
- On leaving the current leaf:
  1. Run `<$()` on the current leaf.
  2. For states removed below the LCA (bottom‑up): run `<<$()`.

These rules keep lifecycle scoping at composite boundaries while preserving the familiar leaf‑enter/leaf‑exit model.

### Initial Child of a Composite
Two options are considered:
- Explicit (recommended):
  - `initial: $Child1` inside the composite block.
  - Pros: refactor‑safe (reordering does not change semantics), merge‑safe, grep‑friendly, aligns with UML “initial pseudostate”.
- Implicit by position (“first child wins”):
  - Meaning is tied to layout; cosmetic reorders change semantics.
  - Acceptable only when a composite has a single child.

Policy (proposed): require explicit `initial: $Child` when a composite has >1 child; allow implicit when there is exactly one child. Provide a codemod to insert `initial:` equal to the current first child.

### State Name Uniqueness
State names must be unique within the module (or globally, per repository policy). Qualified references like `$Parent.$Child` remain available for clarity even if uniqueness is enforced.

### Leaf‑Only Targets (v1 Scope)
Transitions target leaf states only in v1. This avoids the complexity of targeting composites while still unlocking hierarchical benefits:
- Active configuration = leaf + ancestors (parents retain a reference to current child).
- LCA‑based exit/enter, plus constructor/destructor ordering as specified above.
- Parent fallback for handler factoring (leaf‑first, then parent) is encouraged.

Note: If nesting were treated as mere sugar (no ancestor lifecycle, no active configuration), semantics would collapse to the flat model. This RFC opts for a “statechart‑lite” subset: real hierarchy (active configuration + LCA) with leaf‑only targets.

## Backward Compatibility / Migration
- Keep flat parent mapping (`$Child => $Parent(...)`) for compatibility.
- When both forms exist, nested declaration wins; emit a diagnostic on conflicting mappings.
- Provide codemods to convert nested ↔ flat forms.

## Parsing & Codegen Impact
- Parser adds hierarchical state scopes and qualified names (`Parent.Child1`).
- Nested forms desugar to the existing MIR:
  - `Transition { state, args }`
  - `ParentForward`
  - `StackPush/StackPop`
  - `Return`
- No runtime/ABI changes. Visitors see the same MIR with qualified state names.

### Runtime Model & Push/Pop
- Runtime keeps a leaf pointer and parent links (or `parent.current_child`) to represent the active configuration.
- Push snapshots the full configuration (leaf + ancestors). Pop restores it without re‑running constructors/destructors (compartments were not destroyed). Leaf `$>()` on restore is a policy choice to be kept consistent across targets.

## Source Maps & Validation
- Map defaults‑related glue to the state header or `$>()` line as appropriate.
- New validation rules: duplicate sibling names rejected; ambiguous flat references flagged.

## Alternatives Considered
- Keep only flat mappings — retains status quo but poorer locality.
- Table‑style transition specs — readable but awkward for MixedBody and interleaving native code.

Leaf‑only vs Sugar‑only:
- A) Statechart‑lite (this RFC): leaf‑only targets, active configuration, LCA lifecycle, explicit initial child.
- B) Sugar‑only: nested blocks as pretty syntax; single current state; no ancestor lifecycle. Simpler, but loses lifecycle scoping and predictability.

## Risks / Open Questions
- Name resolution for flat references when duplicates exist under different parents (recommend qualified usage or explicit export).
- Exact evaluation timing of `parent_enter` defaults (enter vs exit glue): proposal keeps current semantics — forward before user code in `$>()`, after in `$<()`.
 - Whether leaf `$>()` should run on pop (restoring a snapshot). Must be uniform across targets.
 - Event bubbling semantics (leaf‑first then parent) — recommended for handler factoring; keep predictable and diagnosable.

## Test Plan / Rollout
- Add nested HSM fixtures mirroring existing parent‑mapping tests.
- Ensure parity across Python/TypeScript/native smoke suites.

## References
- Core Contract: docs/framelang_design/target_language_specifications/common/core_frame_contract.md
- Architecture (MixedBody/MIR): docs/framepiler_design/architecture.md
 - RFC‑0002 — Event Phases (Capture/Target/Bubble) and Propagation: docs/framelang_design/research/RFC-0002-event-phases.md

### Related Guidance (v1 out of scope but compatible)
- Orthogonal Regions: model as multiple child machines owned by a parent orchestrator (deterministic routing order). No special runtime needed; each machine keeps its own active configuration.
- History Pseudostates (H/H*): add later to restore last active child or deep subtree.
