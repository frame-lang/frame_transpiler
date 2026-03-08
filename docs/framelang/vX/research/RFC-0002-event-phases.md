---
id: RFC-0002
title: Event Phases (Capture/Target/Bubble) and Propagation
authors: ["Project Owner"]
status: draft
created: 2025-11-08
updated: 2025-11-08
tracking: []
supersedes: []
superseded_by: []
---

## Summary
Introduce DOM-like event phases for Frame hierarchical state machines (HSMs):
capture (down), target (leaf), and bubble (up). Phases are an additive dispatch
model built on RFC‑0001 (nested HSMs with LCA lifecycle). They preserve the Core
Frame Contract (enter/exit ordering, queued dispatch, typed payloads, state stack)
and require no runtime/ABI changes beyond small propagation flags on the current event.

## Motivation / Problem
- Many UI- and protocol-style flows benefit from pre-target (capture) and post-target
  (bubble) processing in ancestors.
- Today’s parent fallback approximates bubble, but lacks a clean capture phase and
  explicit propagation controls.
- A minimal, explicit phase model keeps semantics predictable and aligns with familiar
  DOM patterns while retaining Frame’s deterministic ordering.

## Goals / Non‑Goals
- Goals: capture/target/bubble phases; explicit propagation control; deterministic ordering;
  leaf-only transitions end propagation; no mid-dispatch retargeting.
- Non‑Goals (v1): parent-target transitions, history pseudostates, orthogonal regions
  inside a single HSM (can be modeled as multiple child machines owned by a parent).

## Design (Syntax + Semantics)

### Event Handle `$@` (current event)
`$@` denotes the current event object (already tokenized in the language). This RFC
extends it with propagation fields/methods:

- `$@.handled: bool` — indicates whether the event has been handled in this dispatch.
  - v1 default policy: the dispatcher sets `$@.handled = true` after a target or bubble
    handler for the event name runs (and completes without terminating the phase via an
    error/abort). Capture does not set handled.
  - Handlers may set `$@.handled = true` explicitly for clarity; doing so has no effect
    beyond what the dispatcher already tracks.
- `$@.bubble: bool` (default false) — when set true in target, proceed to bubble phase
  even if the leaf handled the event.
- `$@.stop_propagation()` — end the dispatch pipeline immediately (capture/target/bubble).
- `$@.prevent_default()` — prevent an optional default action (if one is declared).

Notes on precedence: the DOM does not expose a built‑in `handled` flag; it uses
`stopPropagation()` and `preventDefault()`. A `Handled` bit exists in other UI frameworks
such as WPF RoutedEvents. Frame adopts a `handled` flag primarily to enable policies like
“bubble only when unhandled” while keeping explicit `stop_propagation()` for hard stops.

These fields are per-dispatch; visitors/runtimes reset them before each event.

### Handler Phases
Each event may have handlers in one or more phases:

- Capture (pre-target):
  - Keyword: `capture foo() { ... }`
  - Shorthand (optional sugar): `vfoo() { ... }`
  - Runs on ancestors from root → parent-of-leaf.
  - v1 restriction: capture must not transition; it may stop propagation or
    prevent default.

- Target (leaf):
  - Default form: `foo() { ... }` (existing semantics)
  - Can transition; transition ends propagation immediately.
  - May set `$@.bubble = true` to allow bubble even if handled.

- Bubble (post-target):
  - Keyword: `bubble foo() { ... }`
  - Shorthand (optional sugar): `^foo() { ... }`
  - Runs leaf → root along the ancestor chain (nearest parent first).
  - Can transition; transition ends propagation immediately.

### Dispatch Algorithm
1. Compute the ancestor chain once (freeze path for this dispatch).
2. Capture phase: for each ancestor from root → parent-of-leaf, run `capture foo()` if present.
   - If `$@.stop_propagation()` was called, terminate.
3. Target phase: run leaf `foo()` if present.
   - If it transitions, terminate (commit new configuration).
   - Mark `$@.handled = true` after a successful run.
   - If `$@.handled == true` and `$@.bubble` is not set, stop here (handled stops bubbling by default; backward compatible with today’s parent fallback). If the leaf had no handler (handled remains false), proceed to bubble.
4. Bubble phase: for each ancestor from parent-of-leaf → root, run `bubble foo()` if present.
   - If it transitions or calls `$@.stop_propagation()`, terminate.
   - Mark `$@.handled = true` after any bubble handler runs. If `$@.bubble` is not set, stop before invoking the next ancestor (handled stops further bubbling by default). A bubble handler may set `$@.bubble = true` to continue to the next ancestor.
5. Optional default action: if defined at the leaf via `default foo() { ... }` and not
   prevented, run it.

### Ordering Guarantees
- Deterministic order for capture (top-down) and bubble (bottom-up).
- Transitions end propagation immediately and use RFC‑0001 LCA lifecycle.
- No mid-pipeline retargeting; new configuration takes effect on the next event.

### Syntax Notes
- Both keyword and symbolic sugar are accepted in handler headers:
  - Capture: `capture foo()` or `vfoo()`
  - Bubble:  `bubble foo()`  or `^foo()`
  - Target:  `foo()` (existing)
  Visitors lower these to a `phase` tag on the handler.

### Handled & Bubbling Policy
- Default rule: `handled` stops bubbling unless `$@.bubble` is true.
- Starting conditions:
  - If the leaf target runs, `handled = true` → bubble starts only if `$@.bubble == true`.
  - If the leaf has no target handler, bubble starts with `handled = false` and will stop after the first bubble handler unless that handler sets `$@.bubble = true`.
- Hard stop is always available via `$@.stop_propagation()`.

## Backward Compatibility / Migration
- If a state defines only `foo()` (target), semantics are unchanged.
- Today’s parent fallback maps to “bubble if leaf did not handle”; setting `$@.bubble = true`
  maintains a similar upward traversal even when handled.
- No ABI/runtime changes required if propagation flags live in the generated event object;
  alternatively, minimal runtime helpers can be added for portability.

## Parsing & Codegen Impact
- Parser: extend handler headers to accept phase (keyword or sugar). Attach `phase: Capture|Target|Bubble`.
- Visitors: generate per-phase methods and a small dispatcher per event to execute
  capture → target → bubble as specified; obey `$@` flags.
- MIR: no new node types required; phases are a dispatch concern, not a new control statement.

## Source Maps & Validation
- Map emitted code for each phase to the handler’s source line.
- Lints:
  - Disallow transitions in capture (v1 rule).
  - Warn on shadowing: when both child and parent define bubble handlers, document the exact order (child first) and allow `$@.stop_propagation()` to short‑circuit.

## Alternatives Considered
- Bubble‑by‑default: always bubble after target unless stopped. Rejected for v1 to preserve today’s “handled stops here” default; can add a state-level policy later.
- Symbol‑only (`^foo`, `vfoo`) without keywords: less discoverable; dual support preferred.
- Default action as part of bubble: conflates “observer logic” and “default behavior.” Kept separate and optional.

## Comparison To Other Event Models

- DOM Events
  - Phases: capture → target → bubble; controlled via `stopPropagation()` and `stopImmediatePropagation()`; `preventDefault()` cancels default action; no built‑in `handled` flag.
  - Frame: mirrors phases and propagation controls; adds `$@.handled` primarily to implement a clear “handled stops bubbling” default while retaining explicit overrides via `$@.bubble`.
  - Frame freezes the dispatch path; transitions end propagation and commit a new active configuration—DOM has no equivalent to transitions.

- WPF Routed Events
  - Tunnel (Preview) then bubble; `e.Handled` stops routing by default; listeners can opt into handled events (`handledEventsToo`).
  - Frame’s `$@.handled` and the default “handled stops bubbling” policy are analogous. A state‑level policy to “bubble even when handled” would mirror `handledEventsToo` if needed.

- Qt / GTK
  - Event filters/handlers return a boolean to consume events; some systems have default handlers and stop emission semantics.
  - Frame uses explicit flags on a structured event object (`$@`) rather than return values; the semantics are equivalent.

- Middleware Pipelines (e.g., web frameworks)
  - Ordered filters/middleware can short‑circuit or continue; close to Frame’s capture/bubble traversal with `$@.stop_propagation()` and `$@.bubble`.

Key differences / advantages
- Deterministic, compile‑time structure: one handler per phase per state; no dynamic listener lists—simplifies ordering and tooling.
- Transitions integrate naturally: LCA‑based exit/enter and lifecycle hooks (from RFC‑0001) are respected; ending propagation on transitions keeps invariants clear.

## Pros/Cons of Frame vs Other Models

Strengths
- Deterministic semantics: compile‑time structure per state/phase; predictable ordering (LCA exit/enter; capture→target→bubble); transition ends propagation.
- Intent‑first design: hierarchy + explicit states and transitions encode domain logic rather than scattering conditionals and listeners.
- Reuse + lifecycle scoping: parent fallback for shared handlers; ctor/dtor and enter/exit scope resources to composite boundaries.
- Cross‑target parity: one Core Contract with codegen/runtimes for Python/TS/native; with source maps and AST dumps for tooling.
- Testability: run‑to‑completion and explicit hooks produce reproducible traces.

Trade‑offs
- More machinery than flat reducers or pub/sub: active configuration, LCA lifecycle, snapshotting the configuration for push/pop.
- Less dynamic listener wiring by design; structure is part of the spec.
- Concurrency lives outside the core (compose with actors/services as needed).
- Learning curve for HSM concepts and phases.

Comparisons
- DOM: Dynamic listeners and phases; Frame mirrors phases and adds a `handled` bit for clear defaults; deterministic chain; transitions (no DOM analogue) integrate with lifecycle.
- WPF: Routed events with `Handled`; Frame matches defaults closely and offers explicit bubble overrides.
- Actors (OTP/Akka): Concurrency and supervision; combine with Frame for deterministic per‑actor control logic.
- Redux/Elm: Excellent for global state/data; weak at hierarchy and lifecycle; Frame excels there.
- Rx/FRP: Great dataflow; weak for complex control; use FRP around Frame if needed.
- Pub/Sub: Decoupled but prone to drift; Frame offers predictable structure and order; use buses to feed machines, not to encode behavior.

## Risks / Open Questions
- Performance of double walk (capture & bubble) — bounded by depth; acceptable for HSMs.
- Interactions with queued events — clarified by “freeze path” and run-to-completion rules.
- Whether to add `$@.stop_immediate_propagation()` (fine-grained stop within the same phase). Out of scope for v1.

## Test Plan / Rollout
- Unit fixtures covering:
  - Capture stop propagation; no target/bubble executed.
  - Target handled + `$@.bubble=false` → no bubble.
  - Target handled + `$@.bubble=true` → bubble runs (child then parent).
  - Transitions in target/bubble end propagation and commit LCA lifecycle.
  - Optional default action prevented via `$@.prevent_default()`.

## References
- RFC‑0001 — Nested HSM Syntax (lifecycle/LCA foundation)
- Core Contract: docs/framelang_design/target_language_specifications/common/core_frame_contract.md
