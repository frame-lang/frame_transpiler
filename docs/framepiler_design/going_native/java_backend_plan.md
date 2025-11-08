# Java Target Plan — Going Native

Status: Draft
Date: 2025‑11‑08
Owner: native backend track

## Goal
Provide a Java target that generates portable Java code and bridges to the native runtime via JNI for the initial milestone, achieving smoke‑suite parity (enter/exit, parent forward, typed payloads, state stack, prints, main‑call support).

## Non‑Goals (initial)
- Full Java grammar parsing for mixed bodies (passthrough only)
- Reimplementing the kernel in Java
- Rich collections bridging in v1 (prefer opaque handles or scalars)

## Strategy Overview
- Use target‑specific region capture with passthrough Java blocks for mixed bodies.
- Emit Java classes that call a small Java runtime shim (JNI) which forwards to `frame_runtime_llvm` C API.
- Mirror LLVM smoke coverage first.

## Phase 0 — Prerequisites (3–4 days)
- JNI shim
  - Create `frame_runtime_jni` (Java class + native library) exposing: event creation, param push/get, kernel lifecycle, compartment arg accessors, print, state stack ops.
  - Build with `javac` + `javah`/`javac -h` and `clang` to produce a shared lib linking `frame_runtime_llvm`.
- CLI/docs
  - Ensure help lists `java`; document JDK/JNI requirements.

## Phase 1 — Minimal Java Emitter (~1 week)
- Compiler wiring
  - Route `TargetLanguage::Java` in `compiler.rs` to `JavaVisitor`.
- JavaVisitor (minimal)
  - Emit one `.java` per system with:
    - Domain fields + private kernel/compartment members
    - Constructors/destructors (load JNI lib; init/deinit runtime state)
    - Interface methods: construct events, push params, dispatch via JNI
    - Handlers: prints; transitions (enter/exit sequencing via JNI); parent forward; state stack ops
  - Main support: interface calls with literals/domain expressions.
- Mixed bodies
  - Interleave Java native statements with Frame MIR expansions; preserve ordering.

## Phase 2 — Runner + Fixtures (3–4 days)
- Runner
  - Add `java` language option; compile with `javac`, run with `java -Djava.library.path=…` to find JNI lib.
- Fixtures
  - Clone LLVM smoke specs into `framec_tests/language_specific/java/basic`.

## Phase 3 — Coverage Lift (2–3 weeks)
- Expressions/statements: assignment variants, locals in enter/exit, richer calls/exprs.
- Data bridging: strings/booleans/ints/doubles; opaque handles as `long` or `ByteBuffer` mapped to native pointers in JNI.
- Diagnostics: attach Frame line comments; consider source maps via annotations.

## Phase 4 — Packaging & Tooling (1 week)
- Gradle/Maven examples; platform classifiers for JNI library.

## Validation & Policy
- Preserve Core Frame Contract; Frame drives kernel‑visible effects.

## Risks & Mitigations
- JNI portability: test Linux/macOS; ship prebuilt JNI for CI.
- GC vs native lifetimes: centralize lifetimes in JNI glue and avoid sharing raw pointers with user code.

## Milestones & DOD
- M1: JNI shim + minimal emitter; smoke parity green.
- M2: Coverage lift; expanded tests pass.
- M3: Packaging/tooling; examples.

## File/Code Touchpoints
- `framec/src/frame_c/compiler.rs`: route Java target.
- `framec/src/frame_c/visitors/java_visitor.rs` (new): implement emitter.
- `runtime/llvm`: reuse C ABI under JNI shim.
- `framec_tests/runner/frame_test_runner.py`: add java compile/run path.
- Docs: Java body grammar + this plan.

