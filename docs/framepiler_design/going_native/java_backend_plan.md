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

## JNI Surface (Initial)
- Java class `org.frame.runtime.FrameRuntime` (package tbd) exposing:
  - `static native long eventNew(String message);`
  - `static native void eventFree(long eventPtr);`
  - `static native void eventPushI32(long eventPtr, int v);`
  - `static native void eventPushDouble(long eventPtr, double v);`
  - `static native void eventPushBool(long eventPtr, boolean v);`
  - `static native void eventPushCString(long eventPtr, String s);`
  - `static native int eventGetI32(long eventPtr, int index);` (and Double/Bool/String variants)
  - `static native long compartmentNew(String stateName);`
  - `static native void compartmentFree(long compPtr);`
  - `static native void compartmentEnterSetI32(long compPtr, String key, int v);` (and Double/Bool/String)
  - `static native int compartmentEnterGetI32(long compPtr, String key);` (and Double/Bool/String)
  - `static native void compartmentEnterClear(long compPtr);`
  - `static native long kernelNew(long compPtr);`
  - `static native void kernelFree(long kernelPtr);`
  - `static native int kernelDispatch(long kernelPtr, long eventPtr); // 0=Continue, 1=Halt`
  - `static native void kernelSetState(long kernelPtr, String stateName);`
  - `static native long kernelNextEvent(long kernelPtr); // returns event handle or 0`
  - `static native void compartmentSetForwardEvent(long compPtr, long eventPtrOr0);`
  - `static native void kernelStateStackPush(long kernelPtr, int stateIndex);`
  - `static native long kernelStateStackPop(long kernelPtr, int[] stateIndexOut);`
  - `static native void printLine(String s);`
  - `static native void printInt(int v);`
  - `static native void printDouble(double v);`
  - `static native void printBool(boolean v);`
- Native side maps 1:1 to `frame_runtime_llvm` C API; strings are copied into runtime‑owned storage; return values are safe for the call lifetime or explicitly freed by the JNI stub when needed.

## Packaging and Loading
- Build native lib `libframe_runtime_jni.{so|dylib|dll}` that links `-lframe_runtime_llvm`.
- Load via `static { System.loadLibrary("frame_runtime_jni"); }` in the Java class.
- Runner sets `-Djava.library.path=` or `jna.library.path` as needed.
- Provide Gradle/Maven snippets and OS‑specific notes (rpath/loader_path/ORIGIN, Windows PATH).

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
- Exceptions: never throw across the ABI; return status codes and rethrow Java exceptions at the boundary when desired.

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
