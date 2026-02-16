---
id: RFC-0004
title: CEL Policy Integration for Interface Guards and Capture
authors: ["Project Owner"]
status: draft
created: 2025-11-08
updated: 2025-11-08
tracking: []
supersedes: []
superseded_by: []
---

## Summary
Adopt CEL (Common Expression Language) as a declarative, side‑effect‑free policy
language for Frame. v1 integrates CEL into interface guards (RFC‑0003) and
capture (RFC‑0002), with compile‑time validation and per‑target codegen to
ensure cross‑target semantic parity.

## Motivation
- Centralize and simplify complex boolean policy logic (RBAC/ABAC, header claims
  checks, shape validation) without scattering imperative code.
- Keep policies portable and deterministic across Python/TypeScript/native.
- Improve reviewability: policy as data, separate from handler code.

## Scope (v1)
- Allow CEL in:
  - Interface guards (wildcard and per‑method) — fast‑fail before dispatch.
  - Capture handlers — authoritative, stateful authorization using domain state
    and the active configuration; stop propagation on deny.
- No CEL in target/bubble in v1; those phases remain imperative.

## Syntax

### Inline Predicate
```
interface:
  foo(): int requires cel("hasClaim('role','admin') && amount < 1000") { … }
```

### Guard Block with CEL
```
interface:
  * {
    if cel("validateHeaders(headers)") => $ else { fail() }
  }
``>

### Policy Registry (Recommended)
```
cel policy allow_purchase: "resource.owner == subject.id && amount < 1000"
cel policy allow_admin:    "'admin' in subject.roles"

interface:
  buy() { if cel_policy("allow_purchase") => $ else { fail() } }

capture buy() {
  if cel_policy("allow_admin") { /* annotate */ } else { $@.stop_propagation() }
}
```

## Environment & Mapping
- Scalars, lists, maps map directly to CEL values.
- Standard variables (read‑only in v1):
  - `subject` (claims/identity)
  - `headers` (ingress headers; interface guards only)
  - `action` (event name)
  - `resource` (domain snapshot view)
  - optionally selected `$@` fields (e.g., params)
- Whitelisted functions: logical ops, membership (`in`), equality/ordering,
  string/regex, time (if needed). No side effects or I/O.

## Codegen Strategy
- Compile‑time: parse/type CEL expressions, report errors with locations in the
  Frame spec; generate target code for CEL AST to avoid per‑target interpreter
  drift where libs are weak.
- Runtime: evaluate generated functions over frozen environments; cache results
  if needed.

## Deny/Allow Semantics
- Interface: `if cel(...) => $ else { fail() }` fast‑fails denial without
  dispatching.
- Capture: `if !cel(...) { $@.stop_propagation(); /* audit */ }`
- A named policy registry helps reuse and targeted diagnostics (“policy X false
  at Y”).

## Diagnostics
- On compile‑time errors: show policy string/location; halt build.
- On runtime evaluation errors: conservative deny; log policy id and environment
  slice; avoid leaking secrets.
- Source maps include CEL call sites for inspection.

## Out of Scope (v1)
- Dynamic loading/hot reload; user‑defined functions; I/O in CEL.

## Risks
- Library parity: prefer codegen over embedding interpreters to keep semantics
  identical across targets.
- Performance: precompile and cache; keep guards cheap at interface.

## References
- RFC‑0002 — Event Phases (Capture/Target/Bubble)
- RFC‑0003 — Interface Guards
---
