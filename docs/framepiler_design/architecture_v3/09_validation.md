# Stage 9 — Validation (ValidatorV3)

Purpose
- Enforce structural and policy rules on MixedBody and surrounding Frame context before code emission.

Inputs
- `MixedBody` for each handler
- Frame context (states, events, domain), `Arcanum` symbol table

Rules (non‑exhaustive)
- Transition-in-block terminal: a Transition must be the last statement in its containing block. The handler may continue outside that block.
- Forwards and stack ops are not mandated terminal; native lines may follow (subject to target language syntax).
- No Frame statements in actions/operations (native‑only).
- State/target existence: transition targets must resolve to known states.
- Parent forward availability: a parent forward (=> $^) requires that the current state's machine section declares a parent (e.g., `$A => $Parent { … }`). If no parent is declared anywhere in the machine:, validation fails with “Cannot forward to parent: no parent available.” This rule applies to module demos (files with machine: sections). Single‑body demo fixtures without a machine: section are exempt because no parent relationship can be declared in that form.

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

Notes
- Native policy checks (e.g., Python body styles, TS equality operators) are not enforced by default in V3. Any native validation occurs via optional Stage 07 native facades and focuses on wrapper lines only. Generated code policies (e.g., preferring `===`/`!==` in TS) apply to expansions we emit; we do not scan user native text for these by default.

CLI Integration (demo)
- Global flags `--validate` and `--validation-only` apply to demo commands:
  - `demo-multi` validates each provided single‑body file prior to transpile.
  - `demo-project` walks the directory, validates eligible single‑body files, then (unless `--validation-only`) transpiles.
  - Validation prints human messages to stderr; non‑zero exit on failure in `--validation-only` mode.
