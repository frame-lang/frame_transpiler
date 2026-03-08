# V3 Demo Runtime API (Executable Smoke)

Purpose
- Provide a minimal, hermetic runtime shape that expanders can target during the V3 demo “exec smoke” flow so we can compile and run tiny programs across all languages.
- This is not the production runtime; it is intentionally tiny and inlined by the demo compiler when `FRAME_EMIT_EXEC=1`.

Common Concepts
- FrameEvent: message + parameters (Py/TS use a simple class; other languages use a placeholder or omit if unused).
- FrameCompartment: holds the current state string and simple argument/parent fields.
- Runtime hooks: `_frame_transition`, `_frame_router`, `_frame_stack_push`, `_frame_stack_pop`.
- Standardized markers (printed/logged by wrappers):
  - `TRANSITION:<state>` when `_frame_transition` is called.
  - `FORWARD:PARENT` when `_frame_router` is invoked for parent forwarding.
  - `STACK:PUSH` / `STACK:POP` for stack operations.

Per-Language Notes
- Python: wrappers import `frame_runtime_py` for FrameEvent/FrameCompartment; wrapper methods print markers.
- TypeScript: local `FrameEvent`/`FrameCompartment` classes; wrapper methods print markers via `console.log`.
- C/C++: local `FrameCompartment` struct; `__frame_*` hooks print markers via stdio.
- Java/C#: local `FrameCompartment` class; static `__frame_*` methods print markers.
- Rust: local `FrameCompartment` struct (Default); `__frame_*` free functions print markers.

Expander Contract (Demo Exec)
- Transition: construct a `next_compartment` with the compiled state id `__<System>_state_<State>`, call `_frame_transition(next)`, then return.
- Forward (to parent): call `_frame_router(...)` and return.
- Stack push/pop: call `_frame_stack_push()` or `_frame_stack_pop()`; stack pop is terminal and returns.

Scope
- These wrappers exist only in the executable demo path (`FRAME_EMIT_EXEC=1`, `demo-frame`). Production runtimes and full codegen will replace this with proper types and behavior. In normal (non-exec) paths, non-Python/TypeScript expanders remain comment-only.
