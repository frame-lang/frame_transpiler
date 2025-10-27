# LLVM Backend Smoke Tests

These fixtures exercise the experimental LLVM backend added in v0.86.22. The
suite is intentionally small – it mirrors the Phase 1 capabilities (system
struct generation, interface dispatch, print lowering, and state transitions).

## Manual compilation steps

1. Generate LLVM IR:
   ```bash
   ./target/release/framec -l llvm framec_tests/llvm/basic/test_simple_system.frm > simple.ll
   ```
2. Compile and execute (the emitted IR includes a `main` only when your Frame
   spec defines `fn main`—the smoke test does this to allocate the system and
   call its interface; assumes a UTF-8 locale and `puts` from libc):
   ```bash
   clang simple.ll -o simple
   ./simple
   ```
   Expected output:
   ```
   Hello from LLVM backend
   ```

Future phases will wire these specs into the standard test runner and cover
additional features (domain variables, actions, enter/exit handlers, etc.).

## Fixtures

- `basic/test_simple_system.frm` – minimal system instantiation and interface dispatch
- `basic/test_domain_variables.frm` – domain struct layout with default initialization and string access
- `basic/test_actions.frm` – actions block wiring plus domain mutation (bool/string assignments)
- `basic/test_multi_state.frm` – multi-state dispatch and transition handling
