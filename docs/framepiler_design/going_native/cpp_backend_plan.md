# C++ Target Plan — Going Native

Status: Draft
Date: 2025‑11‑08
Owner: native backend track

## Goal
Enable a C++ target that generates portable C++ (baseline C++14/C++17) and links against the existing native runtime via a C ABI, matching the LLVM smoke suite first and then growing coverage.

## Non‑Goals (initial)
- Full C++ grammar parsing inside mixed bodies (passthrough only)
- Replacing the kernel in C++ (reuse Rust runtime via `extern "C"`)
- Template‑heavy codegen in v1; keep output straightforward and portable

## Strategy Overview
- Use target‑specific region capture for C++ with passthrough parsing of native blocks.
- Emit straightforward C++ that calls `frame_runtime_llvm` C API (`extern "C"`), with thin RAII wrappers where helpful.
- Mirror LLVM smoke test coverage first.

## Phase 0 — Prerequisites (1–2 days)
- Headers
  - Provide `frame_runtime_c.h` and a small `frame_runtime_cpp.hpp` convenience header with minimal RAII wrappers (unique_ptr deleters for events/compartments).
- CLI/docs
  - Ensure help lists `cpp`; update README/HOW_TO docs.

## Phase 1 — Minimal C++ Emitter (~1 week)
- Compiler wiring
  - Route `TargetLanguage::Cpp` to a `CppVisitor` (new or adapted from C visitor once it exists).
- CppVisitor (minimal)
  - TU emission with includes `<iostream>`, `<string>`, `<vector>`, `<memory>`, and runtime headers.
  - System class/struct: domain fields + runtime/kernel pointers.
  - Methods: init/deinit; interface; handlers with print, transitions, enter/exit, parent forward, state‑stack ops.
  - Main support: interface calls with literals and domain expressions.
- Mixed bodies
  - Interleave C++ native statements with Frame MIR expansions; preserve ordering.

## RAII Wrapper Helpers
- Create lightweight wrappers in `frame_runtime_cpp.hpp`:
  - `struct Event { Event(const char* msg); ~Event(); void push(int32_t); void push(double); void push(bool); void push(const std::string&); /* getters */ void* raw() const; };`
  - `struct Compartment { explicit Compartment(const char* state); ~Compartment(); void set_enter(const char* key, int32_t); void clear_enter(); void* raw() const; };`
  - `struct Kernel { explicit Kernel(Compartment&&); ~Kernel(); int dispatch(const Event&); void set_state(const std::string&); /* next_event, stack ops */ void* raw() const; };`
- Keep wrappers `noexcept`; map C API status codes to integers/enums; do not throw across the C boundary.

## Phase 2 — Runner + Fixtures (2–3 days)
- Runner
  - Add `cpp` language option; compile with `clang++` linking `frame_runtime_llvm`.
- Fixtures
  - Clone LLVM smoke specs into `framec_tests/language_specific/cpp/basic`.

## Phase 3 — Coverage Lift (2–3 weeks)
- Expressions/statements
  - Richer assignments; locals in enter/exit; more call/expr forms.
- Data bridging
  - Strings: prefer `std::string` to `char*` and convert via runtime helpers.
  - Opaque handles as `void*` wrapped in small RAII types.
- Diagnostics
  - Insert comments with Frame line references; consider `#line` for compilers that support it.

## Phase 4 — Packaging & Tooling (1 week)
- Provide CMake sample and pkg‑config snippet to locate runtime + headers.
- Example project template.

## Validation & Policy
- Preserve Core Frame Contract semantics.
- Mixed body rules: Frame drives kernel‑visible effects; native C++ must not bypass these.

## Risks & Mitigations
- ABI/linking across platforms: validate Linux/macOS in CI; ensure `extern "C"` usage and consistent symbol names.
- Exceptions vs. C API errors: keep runtime calls noexcept in wrappers and map to status codes.

## Milestones & DOD
- M1: Minimal emitter; smoke parity green.
- M2: Coverage lift; expanded tests pass.
- M3: Packaging/tooling; examples build out‑of‑tree.

## File/Code Touchpoints
- `framec/src/frame_c/compiler.rs`: route C++ target.
- `framec/src/frame_c/visitors`: add `cpp_visitor.rs` or share logic with C visitor.
- `runtime/llvm`: headers and stable C API.
- `framec_tests/runner/frame_test_runner.py`: add `cpp` compile/execute path.
- Docs: C++ body grammar + this plan.
