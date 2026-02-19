# Going Native Roadmap (Authoritative)

Status: Draft
Last updated: 2025-11-08
Owner: native backend track

## Objectives
- Unify runtime semantics across native targets behind one C ABI.
- Provide shared debug visibility (source maps, AST dumps) for all native visitors.
- Bring up C, C++, Rust, and Java backends in a phased, low‑risk order with smoke‑suite parity.

## Phased Plan

1) Lock the Runtime ABI
- Add `include/frame_runtime_c.h`, `frame_runtime_abi_version()`, and an ABI conformance test.
- Document string/handle ownership, error model, and platform naming.
- Teach the test runner to locate/build `frame_runtime_llvm` consistently.

2) Shared Debug Infrastructure
- Implement a tiny, target‑agnostic mapping hook in code builders (`map_next(frame_line)`).
- Emit the sidecar source map JSON (source_map_spec.md) for native visitors.
- Add the AST dump JSON (ast_dump_spec.md) from the parsed MixedBody before codegen.
- Wire `--debug-output`/`--ast-dump` across native targets.

3) C Backend (Phase 1)
- Wire `TargetLanguage::C` → `CVisitor`; emit portable C99 calling the C ABI.
- Add `c` to the test runner (clang compile/link/run) and clone the LLVM smoke fixtures under `framec_tests/language_specific/c/basic/`.
- Confirm parity: enter/exit ordering, typed event payloads, parent forward, state stack, prints, main-call support.

4) C++ Backend (Phase 1)
- Implement `CppVisitor` (may reuse C glue). Provide `frame_runtime_cpp.hpp` RAII helpers over the C ABI.
- Add `cpp` to the runner (clang++ compile/link/run) and replicate smoke fixtures under `language_specific/cpp/basic/`.

5) Rust Backend (Phase 1)
- Create a safe wrapper module/crate over the C ABI (opaque newtypes, CString conversions, Drop).
- Implement `RustVisitor` minimal; add `rust` to the runner (rustc/cargo link to C ABI) and smoke fixtures under `language_specific/rust/basic/`.

6) Java Backend (Phase 1)
- Build a JNI shim (`frame_runtime_jni`) forwarding to the C ABI; define the Java surface (see java_backend_plan.md).
- Implement `JavaVisitor`; add `java` to the runner (javac/java with `-Djava.library.path`) and smoke fixtures under `language_specific/java/basic/`.

7) Coverage Lift (All Native Targets)
- Expand assignments (multiple/compound), locals in enter/exit, richer expressions, returns, prints.
- Keep parity with the Core Frame Contract; add tests in lockstep across targets.

8) Packaging, Samples, CI
- Provide pkg‑config/CMake (C/C++), Cargo example (Rust), and Gradle/Maven + loader‑path notes (Java).
- Add cross‑OS CI (macOS/Linux) for all native targets; publish headers/libs as artifacts.

9) FID/Native Import Mapping (Cross‑Target)
- Define a shared FID schema (namespaces, types, functions, async/throws, docs) used across all targets for host API discovery and validation.
- TypeScript (Phase A):
  - Adapter over SWC/TypeScript to harvest imports (e.g., @types/node) and build FIDs.
  - Validate referenced APIs from actions; optional codegen emits real `import` lines and uses mapped Node APIs.
- Python (Phase A):
  - Adapter over inspect/typeshed to extract callable signatures and build FIDs.
  - Validate referenced APIs in actions; emit native Python as-is.
- C#/Java (Phase B):
  - Reflection/AST adapters (Roslyn/JavaParser) to extract public APIs without execution.
  - Validate references; optional helpers for packaging/shading.
- C/C++/Rust (Phase B):
  - Header/clang and Cargo/syn adapters to extract function prototypes and public APIs.
  - Validate references; no network during build.
- Runner/Tooling:
  - Cache FIDs under `.frame/cache/fid/<target>/...`.
  - Add validation gate that uses FIDs when available (off by default in core build).

## References
- Core Contract: docs/framelang_design/target_language_specifications/common/core_frame_contract.md
- Source Map Spec: docs/framepiler_design/going_native/source_map_spec.md
- AST Dump Spec: docs/framepiler_design/going_native/ast_dump_spec.md
- Backend Plans: C / C++ / Rust / Java (going_native directory)
10) MixedBody Parser Normalization (TS/Py)
- Implement FIRST‑set SOLIndex + Frame‑statement mini‑parsers in segmenters:
  - FIRST set: `->`, `=>` `$^`, `push$`, `pop$`, `system.return =`.
  - Streaming DPDA‑protected pass builds Frame‑statement entries; per‑entry mini‑parsers return MIR with precise failures.
- Replace per‑char Frame‑statement checks with SOLIndex; keep DPDA closers authoritative.
- Add Unicode torture fixtures (NBSP indents, emoji in strings/templates/f‑strings) and negative fixtures (unterminated constructs).
- Acceptance:
  - Single‑file TS/Py suites green; Frame‑statement detection strictly SOL; no triggers inside strings/comments/templates; errors are specific.
