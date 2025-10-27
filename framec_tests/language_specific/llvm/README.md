# LLVM Backend Smoke Tests

These fixtures exercise the experimental LLVM backend added in v0.86.22. The
suite is intentionally small – it mirrors the Phase 1 capabilities (system
struct generation, interface dispatch, print lowering, and state transitions).

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
- `basic/test_multi_state.frm` – multi-state dispatch and transition handling
- `basic/test_kernel_interop.frm` – guarantees the runtime kernel is allocated and callable
