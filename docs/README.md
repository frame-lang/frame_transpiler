
# Frame Documentation — Start Here

This folder is the primary entry point for documentation. If you’re new to the repo or returning after a break, start with:

- HOW_TO (Hands‑on guide): docs/HOW_TO.md
- Architecture (Big picture): docs/framepiler_design/architecture.md

Both are authoritative and kept current with the “Going Native” work.

Below is the living index of the most relevant specs and plans across `framelang_design` and `framepiler_design`.

## Framelang Design
- Core Contract: docs/framelang_design/target_language_specifications/common/core_frame_contract.md
- Common Grammar: docs/framelang_design/target_language_specifications/common/frame_common_grammar.md
- Native Imports & FID: docs/framelang_design/frame_interface_definition/native_imports_and_fid.md
- Cross‑Language Support Analysis: docs/framelang_design/cross_language_support_analysis.md
 - Research RFCs: docs/framelang_design/research/README.md (RFC index)

### Target Body Grammars
- Python: docs/framelang_design/target_language_specifications/python/python_body_grammar.md
- TypeScript: docs/framelang_design/target_language_specifications/typescript/typescript_body_grammar.md
- C: docs/framelang_design/target_language_specifications/c/c_body_grammar.md
- C++: docs/framelang_design/target_language_specifications/cpp/cpp_body_grammar.md
- Java: docs/framelang_design/target_language_specifications/java/java_body_grammar.md
- Rust: docs/framelang_design/target_language_specifications/rust/rust_body_grammar.md

## Framepiler Design
- Architecture (Authoritative): docs/framepiler_design/architecture.md
- MixedBody FIRST‑Set Indexing: docs/framepiler_design/stages/native_mixed_body_first_set_indexing.md

### Going Native — Key Policies
- Native bodies by default; Frame statements (->, => $^, $$[+/-], system.return) are SOL‑anchored and recognized only in handlers.
- Actions/operations are native‑only; use `system.return` for returns as needed.
- Per‑language boundary detection via DPDAs (TS template/backtick‑aware; Py triple‑quote/f‑string‑aware).
- Transitions are terminal: a terminal MIR statement must be last in a handler body (validator enforced).

### Going Native (Authoritative Specs)
- Roadmap: docs/framepiler_design/going_native/roadmap.md
- Language Support Analysis: docs/framepiler_design/going_native/language_support_analysis.md
- System Semantics Analysis: docs/framepiler_design/going_native/system_semantics_analysis.md
- Source Map Spec: docs/framepiler_design/going_native/source_map_spec.md
- AST Dump Spec: docs/framepiler_design/going_native/ast_dump_spec.md

### Going Native (Plans)
- C Backend Plan: docs/framepiler_design/going_native/c_backend_plan.md
- C++ Backend Plan: docs/framepiler_design/going_native/cpp_backend_plan.md
- Rust Backend Plan: docs/framepiler_design/going_native/rust_backend_plan.md
- Java Backend Plan: docs/framepiler_design/going_native/java_backend_plan.md
