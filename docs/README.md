
# Frame Documentation — Start Here

This folder is the primary entry point for documentation. If you're new to the repo or returning after a break, start with:

- **HOW_TO (Hands-on guide)**: [docs/HOW_TO.md](HOW_TO.md) - Complete development guide
- **V4 Language Reference**: [docs/framelang/v4/frame_v4_lang_reference.md](framelang/v4/frame_v4_lang_reference.md)

## V4 Architecture (Current)

V4 is a **pure preprocessor** for `@@system` blocks. Native code passes through verbatim ("oceans model").

**V4 Documentation:**
- **Language Reference**: [`docs/framelang/v4/frame_v4_lang_reference.md`](framelang/v4/frame_v4_lang_reference.md) - Authoritative syntax reference
- **Architecture**: [`docs/framelang/v4/frame_v4_architecture.md`](framelang/v4/frame_v4_architecture.md)
- **Runtime**: [`docs/framelang/v4/frame_v4_runtime.md`](framelang/v4/frame_v4_runtime.md)
- **Codegen Spec**: [`docs/framelang/v4/frame_v4_codegen_spec.md`](framelang/v4/frame_v4_codegen_spec.md)
- **Implementation Guide**: [`CLAUDE_V4.md`](../CLAUDE_V4.md)

**V4 Test Infrastructure:**
- Test location: `framepiler_test_env/tests/common/positive/`
- Run tests: `cd framepiler_test_env/tests && ./run_tests.sh`
- Output: `framepiler_test_env/output/{python,typescript,rust,c}/tests/`

**V4 Test Status:**
- Python: 145/145 (100%)
- TypeScript: 127/127 (100%)
- Rust: 130/130 (100%)
- C: 139/139 (100%)
- **Total: 541/541 (100%)**

## Framelang Design

### Target Language Specifications
- **Python**: [`docs/framelang_design/target_language_specifications/python/`](framelang_design/target_language_specifications/python/)
- **TypeScript**: [`docs/framelang_design/target_language_specifications/typescript/`](framelang_design/target_language_specifications/typescript/)
- **Rust**: [`docs/framelang_design/target_language_specifications/rust/`](framelang_design/target_language_specifications/rust/)
- **C**: [`docs/framelang_design/target_language_specifications/c/`](framelang_design/target_language_specifications/c/)

### Common Specifications
- Core Contract: [`docs/framelang_design/target_language_specifications/common/core_frame_contract.md`](framelang_design/target_language_specifications/common/core_frame_contract.md)
- Common Grammar: [`docs/framelang_design/target_language_specifications/common/frame_common_grammar.md`](framelang_design/target_language_specifications/common/frame_common_grammar.md)
- Cross-Language Support Analysis: [`docs/framelang_design/cross_language_support_analysis.md`](framelang_design/cross_language_support_analysis.md)

## Framepiler Design

### Going Native — Key Policies
- Native bodies by default; Frame statements (`->`, `=>`, `push$`/`pop$`, `system.return`) are SOL-anchored and recognized only in handlers.
- Actions/operations are native-only; use `system.return` for returns as needed.
- Per-language boundary detection via DPDAs (TS template/backtick-aware; Py triple-quote/f-string-aware).
- Transitions are terminal: a terminal statement must be last in a handler body.

### Going Native (Authoritative Specs)
- Roadmap: [`docs/framepiler_design/going_native/roadmap.md`](framepiler_design/going_native/roadmap.md)
- Language Support Analysis: [`docs/framepiler_design/going_native/language_support_analysis.md`](framepiler_design/going_native/language_support_analysis.md)
- System Semantics Analysis: [`docs/framepiler_design/going_native/system_semantics_analysis.md`](framepiler_design/going_native/system_semantics_analysis.md)
- Source Map Spec: [`docs/framepiler_design/going_native/source_map_spec.md`](framepiler_design/going_native/source_map_spec.md)

## How To: Compile with V4

Use the main CLI to compile V4 Frame files:

```bash
# Python
./target/release/framec path/to/file.fpy

# TypeScript
./target/release/framec path/to/file.fts

# Rust
./target/release/framec path/to/file.frs

# C
./target/release/framec path/to/file.fc
```

The target language is auto-detected from `@@target` directive in the file.

## V3 Architecture (DEPRECATED)

> **⚠️ V3 is deprecated.** All documentation has been archived with `_` prefix:
> - `docs/framepiler_design/_architecture_v3/`
> - `docs/framelang_design/_architecture_v3/`
>
> Do not read these files unless explicitly instructed.
