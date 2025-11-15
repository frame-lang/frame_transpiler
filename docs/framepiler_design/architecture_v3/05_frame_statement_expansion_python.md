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

Exec harness
- In demo `--emit-exec` mode, the generated wrapper imports the runtime from `frame_runtime_py` and emits a minimal program that calls a single handler so we can validate output markers (TRANSITION:/FORWARD:PARENT/STACK:PUSH|POP) without inlining runtime primitives.

Test Hooks
- Nested conditionals with transitions; ensure no `elif` chain breaks.
- Redundant native `return` immediately after a terminal Frame statement is harmless; optional suppression can be applied in later cleanup.
## Addendum: Async + Indentation Guarantees for Actions/Handlers

This section clarifies emission rules for Python actions/operations/handlers that surfaced during recent validation (v0.86.39–v0.86.40).

### Async‑aware signatures

- If a header begins with `async`, emit `async def …` for the corresponding Python method:
  - Action header: `async demo(x)` → `async def _action_demo(self, x):`
  - Handler header inside a state: `async run()` → `async def run(self, __e: FrameEvent, compartment: FrameCompartment):`
- If a body contains `await` but the header is not `async`, native validation should surface a diagnostic; codegen does not attempt to auto‑promote.

### Indentation normalization (actions/operations)

For actions and operations (not state handlers), normalize the body as follows:

1. Compute the minimal leading indentation across non‑blank, non‑trailer lines of the spliced body.
2. Left‑trim that baseline from every non‑blank line to preserve relative structure.
3. Re‑indent lines under the Python method suite (8 spaces by convention in emitted modules).
4. If the normalized body is comment‑only or empty, emit a `pass` to form a valid suite.

This preserves the semantics of nested `try/except/finally` suites, `with`, `for/while`, and nested defs/lambdas within actions.

### Handlers (state blocks)

- Handlers are emitted as thin methods that call into actions. Each non‑blank line is re‑indented under the method suite, and `pass` is emitted for an otherwise empty/comment‑only handler.

### Examples

Sync `try/except` in an action (FRM → Python):

```frame
@target python_3
system Example {
  actions:
    demo() {
      try {
        self.x = 1
      } except Exception as e {
        self.x = 2
      }
    }
}
```

```python
class Example:
    # …
    def _action_demo(self):
        try:
            self.x = 1
        except Exception as e:
            self.x = 2
```

Async variant with await:

```frame
@target python_3
system Example {
  actions:
    async demo() {
      try {
        await self.io()
      } except Exception as e {
        self.err = str(e)
      }
    }
}
```

```python
class Example:
    # …
    async def _action_demo(self):
        try:
            await self.io()
        except Exception as e:
            self.err = str(e)
```

These guarantees are validated by v3_cli fixtures and unit repros.
