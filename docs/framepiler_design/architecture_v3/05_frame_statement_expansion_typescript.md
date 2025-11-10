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
  - Terminal within its containing block (validator enforced).
- Forward `=> $^`:
  - Emit parent forward glue (dispatch to parent with current event).
  - Not mandated terminal; native statements may follow when separated by a valid inline separator.
- Stack ops `$$+` / `$$-`:
  - Emit push/pop glue for state stack.
  - Not mandated terminal; native statements may follow when separated by a valid inline separator.

Indentation Rules
- Derive indent from Frame-statement line; avoid disrupting surrounding `else if` chains.

Inline forms
- When a Frame statement appears with a trailing semicolon and inline native code (e.g., `=> $^; native();`), the scanner splits the line. The expansion is inserted before the semicolon; native code after the semicolon remains in the trailing native segment. No extra semicolons are introduced by expansions.

Policy Notes
- Prefer `===`/`!==`; disallow `==`/`!=` in generated code.
- Optional chaining/nullish coalescing remain gated by policy and are not emitted by expansions.

Notes
- Native `return` is terminal by TypeScript semantics; the validator’s terminal rule concerns Frame Transitions. Native-return enforcement is delegated to native parse facades or host compilers.

Test Hooks
- Transitions in async handlers (ensure `await` where required by runtime policy).
- Parent forward under nested blocks.
