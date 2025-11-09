# Stage 9 — Validation (ValidatorV3)

Purpose
- Enforce structural and policy rules on MixedBody and surrounding Frame context before code emission.

Inputs
- `MixedBody` for each handler
- Frame context (states, events, domain), `Arcanum` symbol table

Rules (non‑exhaustive)
- Terminal Frame statements (Transition, Forward, Stack ops) must be last in a handler.
- No Frame statements in actions/operations (native‑only).
- Python native policy: no `var` declarations, no brace‑style control in native bodies.
- TypeScript policy: disallow `==`/`!=` in generated expansions (and optional scan in native text).
- State/target existence: transition targets must resolve to known states.

Diagnostics
- Report policy violations with precise Frame spans (for MIR) or native spans (for native policy).
- Human‑oriented and machine‑readable formats.

Complexity
- Linear in item count per handler.

Test Hooks
- Negative fixtures per rule and per language.
