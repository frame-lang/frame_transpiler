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

Interfaces
- `ValidatorV3::validate_regions_mir(regions, mir) -> ValidationResultV3` — structural checks (terminal‑last).
- `ValidatorV3::validate_regions_mir_with_policy(regions, mir, ValidatorPolicyV3) -> ValidationResultV3` — expanded checks using body kind.
- `ValidatorPolicyV3 { body_kind: Option<BodyKindV3> }`, `BodyKindV3 = Handler | Action | Operation | Unknown`.

CLI Integration (demo)
- Global flags `--validate` and `--validation-only` apply to demo commands:
  - `demo-multi` validates each provided single‑body file prior to transpile.
  - `demo-project` walks the directory, validates eligible single‑body files, then (unless `--validation-only`) transpiles.
  - Validation prints human messages to stderr; non‑zero exit on failure in `--validation-only` mode.
