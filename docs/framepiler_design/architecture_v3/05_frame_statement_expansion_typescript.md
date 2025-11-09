# Stage 5b — Frame Statement Expansion (TypeScript)

Purpose
- Lower MIR Frame statements into TypeScript statements with correct indentation and semantics, obeying TypeScript policy (e.g., `===`/`!==`).

Inputs
- `MixedBody` MIR items; handler context (state names, event params, return policy).

Outputs
- TypeScript snippets (bytes) with indentation preserved from Frame-statement lines.

Expansions
- Transition `-> $State(args?)`:
  - Emit glue to set next state and perform return/exit semantics in the TS runtime.
  - Terminal Frame statement.
- Forward `=> $^`:
  - Emit parent forward glue (dispatch to parent with current event).
  - Terminal Frame statement.
- Stack ops `$$+` / `$$-`:
  - Emit push/pop glue for state stack; both are terminal.

Indentation Rules
- Derive indent from Frame-statement line; avoid disrupting surrounding `else if` chains.

Policy Notes
- Prefer `===`/`!==`; disallow `==`/`!=` in generated code.
- Optional chaining/nullish coalescing remain gated by policy and are not emitted by expansions.

Test Hooks
- Transitions in async handlers (ensure `await` where required by runtime policy).
- Parent forward under nested blocks.
