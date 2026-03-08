# Release Notes — v0.86.44 (2025-11-15)

Type: Bug-fix (TypeScript transition temp scoping)

## Highlights

- **Bug #075 — TS2451 for `nextCompartment` in multi-state handlers**
  - TypeScript V3 expander (`TsExpanderV3`) previously emitted transitions as:
    - `const nextCompartment = new FrameCompartment("...");` followed by `_frame_transition(nextCompartment);`.
  - When multiple transitions appeared inside a single handler method (e.g., interface methods routed by `switch (c.state)`), each `case` re-declared `const nextCompartment` in a shared block, causing:
    - `TS2451: Cannot redeclare block-scoped variable 'nextCompartment'`.
  - The expander now:
    - Wraps each transition expansion in its own block:
      - `{ const __frameNextCompartment = new FrameCompartment("..."); ... this._frame_transition(__frameNextCompartment); return; }`
    - This ensures the temp is scoped per expansion and cannot collide across switch cases or other contexts.

## Testing

- Added a focused TypeScript V3 CLI fixture:
  - `language_specific/typescript/v3_cli/positive/redeclare_next_compartment.frm`
    - Uses `@tsc-compile` (with `@skip-if: tsc-missing`) to run `tsc` against the CLI-generated module.
    - Acts as a regression check that multi-state handlers no longer produce TS2451 due to transition temporaries.
- All TypeScript V3 CLI tests, including this new fixture, pass and the generated TS compiles under `tsc`.

## Version

- Workspace version bumped to **0.86.44**; `framec --version` reports `0.86.44`.

