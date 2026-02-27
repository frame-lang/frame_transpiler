# Stage 5b — Frame Statement Expansion (TypeScript)

Purpose
- Lower MIR Frame statements into TypeScript statements with correct indentation and semantics, obeying TypeScript policy (e.g., `===`/`!==`).

Inputs
- `MixedBody` MIR items; handler context (state names, event params, return policy).

Outputs
- TypeScript snippets (bytes) with indentation preserved from Frame-statement lines.

Expansions
- Transition `-> $State(args?)`:
  - Production glue: construct a `FrameCompartment` for the target state, call `this._frame_transition(<compartment>)`, then emit a native `return;` to exit the handler immediately. This mirrors the kernel contract and the terminal rule.
  - Terminal within its containing block (validator enforced). The emitted `return;` makes control‑flow explicit even in deeply nested blocks.
  - To avoid `TS2451` redeclarations in multi‑state handlers that use `switch (c.state)`, each transition expansion:
    - Uses a per‑target temporary name (e.g., `__frameNextCompartment_Connected`, `__frameNextCompartment_Terminated`).
    - Is wrapped in its own block:
      ```ts
      {
        const __frameNextCompartment_Terminated = new FrameCompartment("__System_state_Terminated");
        // optional exit/enter/state arg wiring
        this._frame_transition(__frameNextCompartment_Terminated);
        return;
      }
      ```
    - This ensures that even when multiple `case` labels transition to the same state, no `const` is redeclared across cases.
- Forward `=> $^`:
  - Emit parent forward glue (dispatch to parent with current event) via `this._frame_router(__e, compartment.parentCompartment)`.
  - Not mandated terminal; no implicit `return;` is injected. Native statements may follow (subject to TypeScript syntax and inline separators).
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

Runtime Imports (production)
- Insert once at file top: `import { FrameEvent, FrameCompartment } from 'frame_runtime_ts/index'` (workspace local; no network dependency). The expander uses these types to construct transition compartments.

Notes
- Native `return` is terminal by TypeScript semantics; the validator’s terminal rule concerns Frame Transitions. Native-return enforcement is delegated to native parse facades or host compilers.
- Exec harness note: in demo `--emit-exec` mode, the generated wrapper imports the repository runtime (`frame_runtime_ts/index`) rather than inlining primitives to keep execution hermetic.

Test Hooks
- Transitions in async handlers (ensure `await` where required by runtime policy).
- Parent forward under nested blocks.
