# Release Notes — v0.86.45 (2025-11-15)

Type: Bug-fix (TypeScript transition temp redeclare follow-up)

## Highlights

- **Bug #075 (Reopen) — TS2451 for transition temporaries in multi-state handlers**
  - The first fix in v0.86.44 wrapped each transition expansion in a block and introduced a scoped temporary, but some tooling still observed TS2451 in downstream builds.
  - The TypeScript expander has been updated again to:
    - Generate a **per-state temporary name** for each transition:
      - `__frameNextCompartment_<StateName>` (sanitised to a valid identifier).
    - Emit:
      - `const __frameNextCompartment_State = new FrameCompartment("...");`
      - `this._frame_transition(__frameNextCompartment_State);`
    - This guarantees that a given handler method never redeclares the same identifier across cases, even inside a shared `switch (c.state)` block.
  - Combined with the block-structured Flow, this removes TS2451 for the repro fixture and aligns with the debugger team’s validate script.

## Testing

- `/tmp/frame_transpiler_repro/bug_075/run_validate.sh`:
  - Uses `framec 0.86.45` and the local `tsc` toolchain.
  - Now exits with status 0 and no TS2451 errors in `minimal_redeclare_next_compartment.ts`.
- V3 TypeScript CLI suite:
  - `language_specific/typescript/v3_cli/positive/redeclare_next_compartment.frm`:
    - Still uses `@tsc-compile` to ensure the CLI-generated module compiles cleanly under `tsc`.

## Version

- Workspace version bumped to **0.86.45**; `framec --version` reports `0.86.45`.

