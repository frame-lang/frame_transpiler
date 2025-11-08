# Rust Target Plan — Going Native

Status: Draft
Date: 2025‑11‑08
Owner: native backend track

## Goal
Deliver a first‑class Rust target that generates idiomatic Rust code while preserving the Core Frame Contract. Start with smoke‑suite parity (enter/exit/parent forward/typed payloads/stack pops/prints, main‑call support), then expand expresison/statement coverage.

## Non‑Goals (initial)
- Full Rust grammar parsing inside mixed bodies (use passthrough for native regions)
- Rewriting the kernel in generated code (reuse an embedded runtime module or thin binding)
- Borrow‑checker‑enforced aliasing across Frame constructs in v1 (align semantics, not type system)

## Strategy Overview
- Use existing target‑specific region capture for Rust; keep a passthrough target parser for mixed bodies.
- Generate a small Rust support module in the output (or depend on a crate) that wraps the native runtime semantics with safe APIs.
  - Option A (shortest path): call the existing C ABI from `frame_runtime_llvm` via `extern "C"` and expose safe wrappers.
  - Option B (longer‑term): factor a Rust‑native runtime crate used directly by generated code.
- Mirror LLVM smoke coverage first to validate semantics.

## Phase 0 — Prerequisites (1–2 days)
- Support module
  - Add a tiny `frame_runtime_rs` shim (or inline module) that provides safe Rust wrappers over the C ABI (events, compartments, kernel, stack, prints).
  - Document crate vs. inline module decision; for single‑file generation, prefer inline; for multifile, prefer crate.
- CLI/docs
  - Ensure help lists `rust`; update README/HOW_TO with usage notes.

## Phase 1 — Minimal Rust Emitter (~1 week)
- Compiler wiring
  - Route `TargetLanguage::Rust` in `compiler.rs` to a `RustVisitor` (existing scaffold may be repurposed).
- RustVisitor (minimal)
  - Emit one `.rs` per input (single‑file mode) with:
    - `mod runtime` (safe wrappers) or `use frame_runtime_rs` crate.
    - System struct: domain fields + runtime/kernel handles.
    - `impl` blocks: init/deinit; interface methods; handler fns.
    - Handlers: print lowering; transitions and enter/exit sequencing via runtime wrappers; parent forward; state‑stack ops.
  - Main function support: interface calls with literal and domain expressions.
- Mixed bodies
  - Interleave Rust native statements (passthrough) with Frame MIR expansions; preserve ordering.

## Phase 2 — Runner + Fixtures (2–3 days)
- Runner
  - Add `rust` language option; compile with `rustc` (or `cargo` for multifile), link to `frame_runtime_llvm` via `cc` if using C ABI.
  - Execute binary; capture stdout.
- Fixtures
  - Clone LLVM smoke specs into `framec_tests/language_specific/rust/basic`.

## Phase 3 — Coverage Lift (2–3 weeks)
- Expressions/statements
  - Assignment variants, locals in enter/exit where safe, broader call/expr coverage.
- Data bridging
  - Strings: prefer `CString`/`String` conversions hidden behind runtime wrappers.
  - Opaque handles as `*mut c_void` newtypes with safe wrappers.
- Diagnostics
  - Attach Frame line comments and/or spans to generated Rust for better error mapping.

## Phase 4 — Packaging & Tooling (1 week)
- Publish `frame_runtime_rs` (optional) and document dependency model.
- Provide example `Cargo.toml` and a template project for out‑of‑tree builds.

## Validation & Policy
- Preserve the Core Frame Contract (enter/exit ordering, queue semantics, typed payloads, domain visibility, state stack behavior).
- Mixed body rules: Frame drives kernel‑visible effects; native Rust must not bypass these.

## Risks & Mitigations
- ABI consistency
  - Prefer a safe Rust wrapper; pin symbol names and validate in CI.
- Borrow checker friction
  - Keep runtime handles as opaque newtypes; avoid exposing interior lifetimes in v1.

## Milestones & DOD
- M1: Minimal emitter; smoke parity green.
- M2: Coverage lift; added tests pass.
- M3: Packaging/tooling; example builds documented.

## File/Code Touchpoints
- `framec/src/frame_c/compiler.rs`: route Rust target.
- `framec/src/frame_c/visitors/rust_visitor.rs`: implement/refresh emitter.
- `runtime/llvm`: confirm C ABI sufficiency or add a thin Rust crate.
- `framec_tests/runner/frame_test_runner.py`: add rust compile/execute path.
- Docs: Rust body grammar + this plan.

