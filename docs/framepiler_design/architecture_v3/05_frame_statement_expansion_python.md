# Stage 5 — Frame Statement Expansion (Python)

Purpose
- Lower MIR Frame statements into Python statements with correct indentation and semantics. Keep expansions minimal and defer formatting to optional native parse/formatter.

Inputs
- `MixedBody` MIR items; handler context (state names, event params, return policy).

Outputs
- Python snippets (bytes) with indentation preserved from Frame-statement lines.

Expansions
- Transition `-> $State(args?)`:
  - Production glue: construct a `FrameCompartment` for the target state, call `self._frame_transition(<compartment>)`, then emit a native `return` to exit the handler immediately. This mirrors the kernel contract and the terminal rule.
  - Terminal within its containing block (validator enforced). The emitted `return` makes control‑flow explicit even in deeply nested blocks.
- Forward `=> $^`:
  - Emit parent forward glue (dispatch to parent with current event) via `self._frame_router(__e, compartment.parent_compartment)`.
  - Not mandated terminal; no implicit `return` is injected. Native statements may follow (subject to target language syntax and inline separators).
- Stack ops `$$+` / `$$-`:
  - Emit push/pop glue for state stack.
  - Not mandated terminal; native statements may follow when separated by a valid inline separator.

Indentation Rules
- Derive indent from the Frame-statement line’s indent span; preserve block structure.
- Do not break `elif/else/except/finally` chains: expansions should be placed as statements inside the current block without adding superfluous blank lines or mismatched indents.

Inline forms
- Python allows multiple statements on one physical line when separated by `;`. If a Frame statement is followed by a semicolon and native code (e.g., `=> $^; x = 1  # note`), the scanner splits the line and the expansion appears before the semicolon. Trailing text, including `#` comments, remains in the native segment.

system.return
- Remains native; perform a protected‑region aware rewrite to `self.return_stack[-1]` in handlers and actions/ops. Native `return` is terminal by Python semantics; validator’s terminal rule concerns Frame Transitions.

Errors
- Resolution failures (unknown state) are reported with the Frame statement’s Frame span.

Runtime Imports (production)
- Insert once at file top: `from frame_runtime_py import FrameEvent, FrameCompartment` (workspace local). The expander uses these types to construct transition compartments.

Test Hooks
- Nested conditionals with transitions; ensure no `elif` chain breaks.
- Redundant native `return` immediately after a terminal Frame statement is harmless; optional suppression can be applied in later cleanup.
