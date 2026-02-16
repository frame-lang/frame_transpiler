---
id: RFC-0003
title: Interface Guards, Deny Semantics, and Explicit Dispatch (=> $)
authors: ["Project Owner"]
status: draft
created: 2025-11-08
updated: 2025-11-08
tracking: []
supersedes: []
superseded_by: []
---

## Summary
Define interface‑level guards that run before the kernel dispatches an event,
with a standard `fail()` primitive for denial and an explicit handoff symbol
`=> $` to dispatch into the machine after guards pass. Guards can be declared
globally (`*`) and per‑method. This complements RFC‑0002 (event phases) by
gating at ingress, before capture/target/bubble.

## Motivation
- Centralize ingress checks (headers, tokens, coarse policy) for consistency and
  performance (fast fail without entering the kernel).
- Make the “handoff to machine” explicit when desired
  (especially useful with early returns and multiple guard blocks).
- Standardize denial into a consistent error/response across targets.

## Goals / Non‑Goals
- Goals: simple, explicit guard blocks; standard denial; optional explicit
  dispatch (`=> $`); predictable ordering.
- Non‑Goals: transitions or queuing inside guards; stateful, domain‑dependent
  authorization (keep that in capture inside the kernel).

## Syntax

### Wildcard Guard and Explicit Dispatch
```
interface:
  * {
    if not validateHeaders() {
      fail()     # standardized denial; no dispatch
      return     # optional, explicit early return from interface wrapper
    }
  }

  => $           # explicit handoff to kernel (optional; see behavior)
```

### Per‑Method Guard
```
interface:
  foo(): int = 0 {
    if validateUser() => $ else { fail() }
  }
```

Notes:
- `=> $` means “dispatch this interface call into the machine now.” Without
  `=> $`, codegen implicitly dispatches at the end of the interface wrapper if
  control has not returned.
- `fail()` is a built‑in primitive that returns a standardized denial result
  (see Deny Semantics) and prevents dispatch.
- `return` is optional; guards may rely on `fail()` alone.

## Semantics
- Guards run outside the kernel, before event creation/dispatch. `$@` is not in
  scope here.
- Ordering per interface method:
  1) Run the wildcard (`*`) guard if present.
  2) Run the method‑specific guard if present.
  3) If `=> $` appears, dispatch at that point; otherwise, implicit dispatch at
     the end of the wrapper when no `fail()` has executed.
- Prohibited in guards: transitions, queuing events, or any kernel‑dependent
  behavior. Guards should be side‑effect light (logging/audit allowed).

### Deny Semantics (`fail()`)
- `fail()` stops the wrapper and returns a standardized denial:
  - Python/TS: codegen maps to a uniform error/exception or return object, based
    on the interface’s declared return type.
  - Native: codegen wraps a standardized status/result.
- `fail(reason?: string)` MAY be supported later to carry a message or code.
- Deny at interface complements deeper, stateful authorization in capture (RFC‑0002
  pipeline: capture → target → bubble) and avoids TOCTOU with domain state.

## Examples
```
interface:
  * {
    if not validateHeaders() { fail(); return }
  }

  foo(): int = 0 {
    if validateUser() => $ else { fail() }
  }

  bar() {
    // no per‑method guard; wildcard guard still applies
  }
```

## Parsing & Codegen
- Parser: accept guard blocks inside `interface:` under `*` and under specific
  method signatures. Recognize `=> $` as explicit dispatch sites.
- Codegen: emit an interface wrapper per method that runs guards in order and:
  - on `fail()`: return standardized denial.
  - on `=> $`: construct event, dispatch into kernel, then return mapped result.
  - implicit dispatch at wrapper end if not already dispatched or failed.

### Codegen Sketch (Pseudo)

Target‑agnostic wrapper shape:
```
function iface_foo(args...) -> RetType {
  // 1) wildcard guard
  if (has_guard_star) {
    if (!guard_star()) { return deny_result_for(RetType) }
  }

  // 2) per‑method guard body (inline)
  //    supports explicit dispatch via (=> $) or denial via fail()
  if (!validateUser()) { return deny_result_for(RetType) }

  // explicit dispatch sites (=> $) generate:
  //   event = make_event("foo", args)
  //   result = kernel_dispatch(event)
  //   return map_result(result, RetType)

  // 3) implicit dispatch if not already dispatched/failed
  event = make_event("foo", args)
  result = kernel_dispatch(event)
  return map_result(result, RetType)
}
```

Python (conceptual):
```
def foo(self, *args) -> int:
    # wildcard
    if not validateHeaders():
        return deny_int()

    # per‑method
    if not validateUser():
        return deny_int()

    ev = FrameEvent("foo", args)
    out = self._dispatch(ev)  # runs RFC‑0002 phases
    return coerce_to_int(out)
```

TypeScript (conceptual):
```
foo(...args: any[]): number {
  if (!validateHeaders()) return denyNumber()
  if (!validateUser())   return denyNumber()
  const ev = new FrameEvent("foo", args)
  const out = this._dispatch(ev)
  return toNumber(out)
}
```

Native (C/C++/Rust via ABI): create event → push params → call dispatch → map status/result → coerce to return type or return an error wrapper.

Return mapping examples:
- `void`   → return immediately on `fail()`; otherwise ignore kernel result or check status.
- `int`    → `deny_int()` returns a sentinel or raises per‑project policy.
- `Result<T,E>` (preferred) → `fail()` maps to `Err(E::Denied)`; success maps kernel output to `Ok(T)`.

## Interaction with RFC‑0002 (Phases)
- After dispatch, RFC‑0002 applies: capture (root → parent), target (leaf),
  bubble (parent → root). Interface guards run before capture.

## Validation
- Lints:
  - Disallow kernel‑dependent operations in guards.
  - Warn if `=> $` appears after an unconditional `fail()` branch.
  - Ensure guards do not access `$@`.

## Alternatives Considered
- `guard * {}` / `guard foo {}` keywords: more explicit but heavier syntax. The
  `* {}` and method‑header blocks are concise and clear in context.
- `requires expr` clause inline on method header: terse, but less flexible than
  blocks (multiple statements, logs, and branches).

## CEL Policy Integration (Optional)

### Motivation
- Express complex, side‑effect‑free ingress policies declaratively and uniformly
  across targets. Keep interface guards readable and reviewable without embedding
  large boolean logic blocks.

### Scope (v1)
- Allow CEL expressions in interface guards (wildcard and per‑method). Evaluate
  before dispatch; on false, `fail()` without entering the kernel.
- Optionally allow CEL inside capture (RFC‑0002) for stateful authorization using
  the active configuration and domain snapshot. Interface remains stateless/cheap.

### Syntax Options
- Inline predicate:
  ```
  interface:
    foo(): int requires cel("hasClaim('role','admin')") { … }
  ```
- Guard block with CEL:
  ```
  interface:
    * {
      if cel("validateHeaders(headers)") => $ else { fail() }
    }
  ```
- Named policy registry (recommended):
  ```
  cel policy allow_purchase: "resource.owner == subject.id && amount < 1000"
  interface:
    buy() { if cel_policy("allow_purchase") => $ else { fail() } }
  ```

### Implementation Notes
- Value mapping: Frame scalars/lists/maps; context vars `subject` (claims),
  `headers`, `action` (event name), `resource` (domain snapshot), and portions of `$@`.
- Codegen: pre-parse/type CEL at compile time; generate target code (preferred)
  or embed a vetted CEL lib per target. Disallow side effects, user functions,
  or I/O. Freeze environment per dispatch.
- Diagnostics: map policy errors back to source; include policy name/expression
  in denial logs.

### Out of Scope (v1)
- Dynamic policy loading/hot‑reload, user‑defined functions, I/O within CEL.


## Risks / Open Questions
- How to map `fail()` for methods with complex return types; propose a uniform
  Result/Option style across targets or a generated typed error wrapper.
- Whether to permit limited domain reads in guards; recommend “no” in v1 to keep
  guards pure ingress checks and avoid state drift.

## References
- RFC‑0001 — Nested HSM Syntax (lifecycle/LCA foundation)
- RFC‑0002 — Event Phases (Capture/Target/Bubble)
- RFC‑0004 — CEL Policy Integration for Interface Guards and Capture
  (policy registry, compile‑time validation, and per‑target codegen):
  docs/framelang_design/research/RFC-0004-cel-policy-integration.md

## Appendix: CEL Examples

### Registry + Guards
```
cel policy allow_purchase: "resource.owner == subject.id && amount < 1000"
cel policy allow_admin:    "'admin' in subject.roles"

interface:
  * {
    if cel("validateHeaders(headers)") => $ else { fail() }
  }

  buy(amount: int) {
    if cel_policy("allow_purchase") => $ else { fail() }
  }

capture buy(amount: int) {
  if cel_policy("allow_admin") {
    // attach an audit tag; no transitions in capture
  } else {
    $@.stop_propagation()
  }
}
```

### Deny Mapping Across Targets (Conceptual)
- Python: `fail()` → `raise FrameDeny("policy denied")` or `return Denied("policy denied")` (generated wrapper chooses policy per return type).
- TypeScript: `fail()` → `return { ok: false, err: "policy denied" }` or throw a typed error per project convention.
- Native: `fail()` → `Result<T, E>::Err(E::Denied)` or a status code mapped by the wrapper.
