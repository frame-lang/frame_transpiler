
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
- Architecture (Authoritative): docs/framepiler_design/architecture.md (includes V3 glossary and native parser policy)
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

## Quick Reading List — Python and TypeScript

Python
- docs/framepiler_design/architecture_v3/01_body_closers_python.md:1
- docs/framepiler_design/architecture_v3/02_native_region_scanner_python.md:1
- docs/framepiler_design/architecture_v3/05_frame_statement_expansion_python.md:1
- docs/framepiler_design/architecture_v3/07_native_parse_facade_python.md:1
- frame_runtime_py/__init__.py:1

TypeScript
- docs/framepiler_design/architecture_v3/01_body_closers_typescript.md:1
- docs/framepiler_design/architecture_v3/02_native_region_scanner_typescript.md:1
- docs/framepiler_design/architecture_v3/05_frame_statement_expansion_typescript.md:1
- docs/framepiler_design/architecture_v3/07_native_parse_facade_typescript.md:1
- frame_runtime_ts/index.ts:1

Shared V3 Core (applies to both; read in this order)
- docs/framepiler_design/architecture_v3/architecture_v3_overview.md:1
- docs/framepiler_design/architecture_v3/03_frame_segment_parser.md:1
- docs/framepiler_design/architecture_v3/04_mir_assembly.md:1
- docs/framepiler_design/architecture_v3/06_splice_and_mapping.md:1
- docs/framepiler_design/architecture_v3/06_5_structural_validation.md:1
- docs/framepiler_design/architecture_v3/09_validation.md:1
- docs/framepiler_design/architecture_v3/10_ast_and_symbol_integration.md:1
- docs/framepiler_design/architecture_v3/11_error_taxonomy.md:1
- docs/framepiler_design/architecture_v3/08_source_maps_and_codegen.md:1
- docs/framepiler_design/architecture_v3/07_runtime_api_demo.md:1
- docs/framepiler_design/architecture_v3/00_stage_index.md:1
- docs/framepiler_design/architecture_v3/PLAN.md:1

Testing, Runner, and CI
- docs/framepiler_design/architecture_v3/12_testing_strategy.md:1
- framec_tests/runner/frame_test_runner.py:1
- .github/workflows/v3_all.yml:1
- .github/workflows/v3_curated_exec.yml:1
- .github/workflows/v3_exec_smoke.yml:1
