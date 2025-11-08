# C Target Plan — Going Native

Status: Draft
Date: 2025‑11‑08
Owner: native backend track

## Goal
Enable a first‑class C target that generates portable C99 code and links against the existing native runtime (frame_runtime_llvm) via a C ABI, achieving parity with the current LLVM smoke suite and establishing a path to broader feature coverage.

## Non‑Goals (initial)
- Full C grammar parsing inside mixed bodies (passthrough only at first)
- Re‑implementing the kernel in C (reuse Rust runtime via FFI)
- Complete Python/TypeScript feature parity in the first milestone

## Strategy Overview
- Reuse the scanner/parser’s target‑specific region capture for C; keep a passthrough target parser.
- Generate straightforward C that calls the Rust runtime’s `extern "C"` API (events, compartments, kernel, stack, prints).
- Mirror the LLVM smoke coverage first (enter args, typed events, parent forwarding, stack multi‑pop, prints, main‑call support).
- Add a test runner path to compile and run generated C with `clang`, linking the prebuilt `frame_runtime_llvm` library.

## Phase 0 — Prerequisites (1–2 days)
- Runtime header
  - Produce `include/frame_runtime_c.h` declaring the exported functions already exposed by `frame_runtime_llvm` (ffi.rs `#[no_mangle] extern "C"`).
  - Document platform naming (macOS `.dylib`, Linux `.so`, Windows `.dll`/`.lib`).
- CLI/docs
  - Ensure help mentions `c` target; align README/HOW_TO and design docs.

## Phase 1 — Minimal C Emitter (LLVM smoke parity, ~1 week)
- Compiler wiring
  - Route `TargetLanguage::C` in `compiler.rs` to a `CVisitor`.
- CVisitor (minimal)
  - Emit one `.c` translation unit per input (single‑file mode):
    - `#include <stdio.h>`, `<stdlib.h>`, `<stdbool.h>`, `<string.h>` and `"frame_runtime_c.h"`.
    - System struct: domain fields + runtime/kernel pointers as needed.
    - Init/deinit functions.
    - Interface methods: build `FrameEvent`, push typed params, `frame_runtime_kernel_dispatch`.
    - Handlers: print lowering; transitions to `$State` and `-> (args) $State` via compartment + enter arg setters; `$>()`/`$<()` calls; parent forward `=> $^` via `frame_runtime_compartment_set_forward_event`.
    - State stack ops using `frame_runtime_kernel_state_stack_push/pop`.
  - Main function support: interface calls with literal and domain expressions.
- Mixed bodies
  - Respect segmenter: interleave C statements (passthrough) with Frame statements; preserve ordering.
- Types
  - Map Frame types: `int→int`, `float→double`, `bool→bool`, `string→char*` (copy/ownership via runtime where needed), opaque handles as `void*`/`char*` by convention until declarations land.

## Linking Notes (Runner)
- Compile with `clang`:
  - `clang -I <include_dir> -L <runtime_dir> -lframe_runtime_llvm generated.c -o a.out`
- Ensure rpath or `DYLD_LIBRARY_PATH`/`LD_LIBRARY_PATH` is set so the loader finds `libframe_runtime_llvm` at runtime.

## Phase 2 — Test Runner + Fixtures (2–3 days)
- Runner
  - Add `c` as a language option; compile C with `clang` into an executable next to the generated code.
  - Link against `frame_runtime_llvm` found via the existing `_ensure_llvm_runtime()` (reuse logic from LLVM path for locating the library/output dir).
  - Run executable; capture stdout; enforce timeouts.
- Fixtures
  - Seed `framec_tests/language_specific/c/basic/` by cloning LLVM smoke specs:
    - `test_enter_args.frm`, `test_event_parameters.frm`, `test_parent_forward*.frm`, `test_state_stack_*`, `test_actions.frm`, `test_action_returns.frm`, `test_domain_variables.frm`, `test_simple_system.frm`.
  - Update CI to run the C suite (Mac/Linux toolchains).

## Phase 3 — Coverage Lift (2–3 weeks, iterative)
- Expressions/statements
  - Assignment forms: multiple/compound assignments emitted where supported; fall back to simple forms as needed.
  - Locals in `$>()/$<()` where safe (visitor emits C locals only; Frame locals remain restricted per existing LLVM notes).
  - Broader call/expr coverage in handlers and operations.
- Data bridging
  - Clarify string ownership boundaries; prefer copying into runtime‐owned storage when passed through events/compartments.
  - Support opaque handles via `native module` declarations (map to `void*`).
- Diagnostics
  - Improve error surfaces by attaching Frame line info as comments and optional pragmas for better back‑mapping.

## Phase 4 — Tooling & Packaging (1 week)
- Provide `pkgconfig` or CMake snippet to locate `frame_runtime_llvm` and headers.
- Add example Makefile and a one‑file “hello Frame (C)” sample.
- Document cross‑platform notes (MSVC vs clang/clang‑cl).

## Validation & Policy
- Core contract
  - Preserve enter/exit ordering, queue semantics, typed payload fidelity, domain visibility, and state stack behavior (see `core_frame_contract.md`).
- Mixed body rules
  - Frame drives kernel‑visible effects (transitions, events, forwards); native C must not bypass these.
  - Only scalars and opaque handles cross the boundary; collections require opaque handles.

## Risks & Mitigations
- Semantics drift
  - Reuse the same runtime API as LLVM; keep the smoke suite in lockstep.
- String lifetime bugs
  - Centralize copying/freeing in the runtime FFI helpers; avoid borrowing transient buffers from C locals.
- Linking/portability
  - Standardize include/lib search in the runner and docs; CI validates Linux/macOS.

## Milestones & DOD
- M0: Header prepared; docs updated; `@target c` recognized. DOD: header compiles and links in a trivial C program.
- M1: Minimal C emitter lands; parity with LLVM smoke tests. DOD: 100% green on C smoke suite.
- M2: Coverage lift on assignments/locals/exprs; expanded tests. DOD: added tests pass on C and LLVM.
- M3: Packaging/tooling; examples. DOD: out‑of‑tree sample builds with documented steps.

## File/Code Touchpoints
- `framec/src/frame_c/compiler.rs`: route `TargetLanguage::C` to visitor.
- `framec/src/frame_c/visitors/c_visitor.rs`: implement minimal emitter.
- `runtime/llvm/`: expose C header; ensure exported symbols stable (`ffi.rs`).
- `framec_tests/runner/frame_test_runner.py`: add compile/link/execute path for `c`.
- `framec_tests/language_specific/c/basic`: smoke fixtures.
- Docs: HOW_TO, README, target language specs (C body grammar), this plan.

## Open Questions
- Do we want a dedicated C runtime shim later (smaller ABI) or keep the single Rust runtime for all native targets?
- Should the visitor emit one TU per system or a combined TU for multifile projects initially?
- How far do we push validation for C native blocks without adding a real C parser?
