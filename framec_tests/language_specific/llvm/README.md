# LLVM Backend Smoke Tests

These fixtures exercise the experimental LLVM backend added in v0.86.22. The
suite is intentionally small – it mirrors the Phase 1 capabilities (system
struct generation, interface dispatch, print lowering, state transitions, and
the new runtime queue plumbing that landed with the `builder/context` refactor
in v0.86.24).

## Running with the unified test runner

```bash
# Ensure the compiler and runtime are built
cargo build --release

# Execute only the LLVM language-specific suite
python3 framec_tests/runner/frame_test_runner.py \
    --languages llvm \
    --categories language_specific_llvm \
    --framec ./target/release/framec
```

The runner will compile each `.ll`, link it against `libframe_runtime_llvm`,
execute the binary, and report aggregated results alongside the other targets.

> **macOS tip**: The runner exports `DYLD_LIBRARY_PATH` automatically. When
running the smoke tests manually outside the harness, export the same path so
`clang` can locate `libframe_runtime_llvm.dylib`.

## Manual compilation steps

1. Generate LLVM IR:
   ```bash
   ../../../target/release/framec -l llvm basic/test_simple_system.frm > simple.ll
   ```
2. Compile and execute (the emitted IR includes a `main` only when your Frame
   spec defines `fn main`; the smoke tests do this to allocate the system and
   call its interface, assuming a UTF-8 locale and `puts` from libc):
   ```bash
   clang simple.ll -L../../../target/release \
       -lframe_runtime_llvm -Wl,-rpath,../../../target/release \
       -mllvm -opaque-pointers \
       -o simple
   ./simple
   ```
   Expected output:
   ```
   Hello from LLVM backend 2
   ```

Future phases will wire broader coverage (domain variables, actions,
enter/exit handlers, etc.) into the standard test runner configs and introduce
negative/edge-case fixtures as the backend matures.

## Fixtures

- `basic/test_simple_system.frm` – minimal system instantiation and interface dispatch
- `basic/test_domain_variables.frm` – domain struct layout with default initialization and string access
- `basic/test_actions.frm` – actions block wiring plus domain mutation (bool/string assignments)
- `basic/test_action_locals.frm` – action locals mutate typed/untyped domain fields and log intermediate values
- `basic/test_action_returns.frm` – verifies action return values are emitted and consumable
- `basic/test_multi_state.frm` – multi-state dispatch and transition handling
- `basic/test_kernel_interop.frm` – guarantees the runtime kernel is allocated and callable
- `basic/test_parent_forward.frm` – verifies parent forwarding (`=> $^`) re-enters the parent handler (current runtime short-circuits without enqueuing forwarded events)
- `basic/test_parent_hierarchy.frm` – exercises multi-level parent dispatch, typed counters, and inferred domain fields
- *(Coming soon)* `basic/test_enter_exit.frm` & friends to cover enter/exit handler execution once Phase 2 completes
