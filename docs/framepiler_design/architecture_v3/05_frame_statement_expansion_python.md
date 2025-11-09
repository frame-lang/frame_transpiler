# Stage 5 — Frame Statement Expansion (Python)

Purpose
- Lower MIR Frame statements into Python statements with correct indentation and semantics. Keep expansions minimal and defer formatting to optional native parse/formatter.

Inputs
- `MixedBody` MIR items; handler context (state names, event params, return policy).

Outputs
- Python snippets (bytes) with indentation preserved from Frame-statement lines.

Expansions
- Transition `-> $State(args?)`:
  - Emit glue to set next state and perform return/exit semantics.
  - Terminal Frame statement; ensure no further statements execute.
- Forward `=> $^`:
  - Emit parent forward glue (dispatch to parent with current event).
  - Terminal Frame statement.
- Stack ops `$$+` / `$$-`:
  - Emit push/pop glue for state stack; both are terminal.

Indentation Rules
- Derive indent from the Frame-statement line’s indent span; preserve block structure.
- Do not break `elif/else/except/finally` chains: expansions should be placed as statements inside the current block without adding superfluous blank lines or mismatched indents.

system.return
- Remains native; perform a protected‑region aware rewrite to `self.return_stack[-1]` in handlers and actions/ops.

Errors
- Resolution failures (unknown state) are reported with the Frame statement’s Frame span.

Test Hooks
- Nested conditionals with transitions; ensure no `elif` chain breaks.
- Redundant native `return` immediately after a terminal Frame statement is harmless; optional suppression can be applied in later cleanup.
