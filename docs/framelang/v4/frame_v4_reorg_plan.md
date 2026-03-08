# Frame V4 Codebase Reorganization Plan

**Version:** 1.0
**Date:** March 2026
**Companion to:** `frame_v4_pipeline_architecture.md`

---

## 1. Current State

The V4 codebase under `framec/src/frame_c/v4/` contains **two independent compilation paths** and a collection of dead modules:

### 1.1 Primary Pipeline (compile_ast_based)

The active compilation pipeline invoked by `framec` for code generation:

```
cli.rs → compiler.rs → compile_ast_based()
  Uses: frame_parser, frame_ast, arcanum, frame_validator
  Uses: codegen/ (system_codegen, backends, ast)
  Uses: pipeline/ (compiler, config, traits)
  Uses: native_region_scanner/, body_closer/, pragma_scanner
  Uses: splice (Splicer only)
```

### 1.2 Validation Demo Pipeline (validate_module_demo)

A secondary pipeline invoked by `framec --validate`. This is the **old V4 pipeline** that predates the AST-based rewrite. It uses a completely different code path:

```
cli.rs → validate_module_demo_with_mode()
  Uses: module_partitioner → prolog_scanner, import_scanner/, outline_scanner
  Uses: system_parser, interface_parser, machine_parser
  Uses: validator, mir, mir_assembler, frame_statement_parser
  Uses: expander/, splice, facade
  Uses: system_param_semantics, ast (old ast.rs)
```

### 1.3 Dead Code (No Callers)

Modules declared in `v4/mod.rs` but not called by either pipeline:

| Module | Description |
|--------|-------------|
| `domain_scanner` | Domain block scanner (legacy) |
| `native_symbol_snapshot` | Incomplete snapshot builder |
| `python_transpiler` | Unfinished Frame-to-Python transpiler |
| `rust_domain_scanner` | Domain field scanner for Rust |
| `machines/` | Generated state machines (indent_normalizer, ts_harness_builder) |
| `ts_harness_machine` | Wrapper for ts_harness_builder |
| `parser_debug` | Debug utility with `debug_parse()` |
| `system_transformer` | Partial system transformation |

### 1.4 Legacy Directories (Empty)

Empty directories under `framec/src/frame_c/` from V3 era:

| Directory | Status |
|-----------|--------|
| `body_interleaver/` | Empty — 0 files |
| `body_segmenter/` | Empty — 0 files |
| `declaration_importers/` | Empty — 0 files |
| `llvm/` | Empty — 0 files |
| `modules/` | Empty — 0 files |
| `native_partition_scanner/` | Empty — 0 files |
| `native_region_segmenter/` | Empty — 0 files |
| `target_parsers/` | Empty — 0 files |
| `tools/` | Empty — 0 files |
| `validation/` | Empty subdirectories only (analysis/, reporters/, rules/, targets/) — 0 .rs files |

---

## 2. File Inventory

### 2.1 Active in Primary Pipeline (KEEP)

These files are used by `compile_ast_based()` — the main code generation path:

| File | Role |
|------|------|
| `frame_parser.rs` | Hand-rolled byte parser (to be rewritten per pipeline arch) |
| `frame_ast.rs` | AST type definitions |
| `arcanum.rs` | Symbol table builder |
| `frame_validator.rs` | Semantic validator |
| `pragma_scanner.rs` | Pragma/system block detection |
| `splice.rs` | Splicer (used by system_codegen for handler body assembly) |
| `pipeline/mod.rs` | Pipeline module |
| `pipeline/compiler.rs` | Pipeline orchestrator |
| `pipeline/config.rs` | Pipeline configuration |
| `pipeline/traits.rs` | RegionScannerTrait (dead after rewrite) |
| `codegen/mod.rs` | Codegen module + entry points |
| `codegen/ast.rs` | CodegenNode IR definitions |
| `codegen/system_codegen.rs` | AST → CodegenNode transformation |
| `codegen/backend.rs` | LanguageBackend trait + EmitContext |
| `codegen/backends/python.rs` | Python emitter |
| `codegen/backends/typescript.rs` | TypeScript emitter |
| `codegen/backends/rust_backend.rs` | Rust emitter |
| `codegen/backends/c_backend.rs` | C emitter |
| `codegen/backends/cpp_backend.rs` | C++ emitter |
| `codegen/backends/java_backend.rs` | Java emitter |
| `codegen/backends/csharp_backend.rs` | C# emitter |
| `native_region_scanner/mod.rs` | Scanner module + shared types |
| `native_region_scanner/unified.rs` | Unified scan algorithm + SyntaxSkipper trait |
| `native_region_scanner/python.rs` | Python SyntaxSkipper + NativeRegionScanner |
| `native_region_scanner/typescript.rs` | TypeScript SyntaxSkipper + NativeRegionScanner |
| `native_region_scanner/rust.rs` | Rust SyntaxSkipper + NativeRegionScanner |
| `native_region_scanner/c.rs` | C SyntaxSkipper + NativeRegionScanner |
| `native_region_scanner/cpp.rs` | C++ SyntaxSkipper + NativeRegionScanner |
| `native_region_scanner/java.rs` | Java SyntaxSkipper + NativeRegionScanner |
| `native_region_scanner/csharp.rs` | C# SyntaxSkipper + NativeRegionScanner |
| `body_closer/mod.rs` | BodyCloser trait |
| `body_closer/python.rs` | Python BodyCloser |
| `body_closer/typescript.rs` | TypeScript BodyCloser |
| `body_closer/rust.rs` | Rust BodyCloser |
| `body_closer/c.rs` | C BodyCloser |
| `body_closer/cpp.rs` | C++ BodyCloser |
| `body_closer/java.rs` | Java BodyCloser |
| `body_closer/csharp.rs` | C# BodyCloser |

**Total: 37 active files**

### 2.2 Validation Demo Only (DEPRECATE with `_` prefix)

These files are used **only** by `validate_module_demo_with_mode()`, not by the main compilation pipeline. They represent the old V4 pipeline before the AST-based rewrite:

| File | Role |
|------|------|
| `ast.rs` | Old AST types (ModuleAst, SystemAst — different from frame_ast.rs) |
| `module_partitioner.rs` | File segmentation into prolog/imports/system/epilog |
| `prolog_scanner.rs` | Native prolog detection |
| `outline_scanner.rs` | Section outline scanning |
| `system_parser.rs` | System block parser (pre frame_parser.rs) |
| `interface_parser.rs` | Interface section parser |
| `machine_parser.rs` | Machine section parser |
| `validator.rs` | Old validator (pre frame_validator.rs) |
| `mir.rs` | Mid-level intermediate representation |
| `mir_assembler.rs` | MIR builder from scan regions |
| `frame_statement_parser.rs` | Frame statement parser for MIR |
| `expander/mod.rs` | MIR → native code expander |
| `facade.rs` | Native facade registry for syntax checking |
| `system_param_semantics.rs` | System parameter semantic analysis |
| `import_scanner/mod.rs` | Import scanner module |
| `import_scanner/python.rs` | Python import scanner |
| `import_scanner/typescript.rs` | TypeScript import scanner |
| `import_scanner/rust.rs` | Rust import scanner |
| `import_scanner/c.rs` | C import scanner |
| `import_scanner/cpp.rs` | C++ import scanner |
| `import_scanner/java.rs` | Java import scanner |
| `import_scanner/csharp.rs` | C# import scanner |

**Total: 22 files (validation demo pipeline)**

### 2.3 Dead Code (DEPRECATE with `_` prefix)

These files have zero callers in any pipeline:

| File | Role |
|------|------|
| `domain_scanner.rs` | Unused domain scanner |
| `native_symbol_snapshot.rs` | Incomplete, unused |
| `python_transpiler.rs` | Unfinished, unused |
| `rust_domain_scanner.rs` | Unused |
| `machines/mod.rs` | Unused generated state machines |
| `machines/indent_normalizer.frs` | Frame source for generated machine |
| `machines/indent_normalizer.gen.rs` | Generated, unused |
| `machines/ts_harness_builder.frs` | Frame source for generated machine |
| `machines/ts_harness_builder.gen.rs` | Generated, unused |
| `ts_harness_machine.rs` | Unused wrapper |
| `parser_debug.rs` | Unused debug utility |
| `system_transformer.rs` | Unused, self-referencing only |

**Total: 12 files (completely dead)**

### 2.4 Test Infrastructure (KEEP)

| File | Role |
|------|------|
| `test_harness_stub.rs` | Stub for test harness (used by CLI `--test` commands) |
| `arcanum_tests.rs` | Unit tests for arcanum (cfg(test) only) |
| `compile_tests.rs` | Unit tests for compiler (cfg(test) only) |
| `frame_parser_tests.rs` | Unit tests for parser (cfg(test) only) |

### 2.5 Non-V4 Active Files (KEEP)

Files directly under `framec/src/frame_c/`:

| File | Role |
|------|------|
| `mod.rs` | Module root |
| `cli.rs` | CLI entry point |
| `compiler.rs` | Compilation orchestration |
| `config.rs` | Project configuration |
| `utils.rs` | Utility types (RunError, etc.) |
| `visitors/mod.rs` | TargetLanguage enum |

---

## 3. Deprecation Plan (Phase 1: `_` Prefix)

### 3.1 Immediate: Dead Code (`_` prefix)

Rename these files/directories with `_` prefix to mark as deprecated. Update `mod.rs` declarations accordingly.

**Files to rename:**
```
v4/domain_scanner.rs           → v4/_domain_scanner.rs
v4/native_symbol_snapshot.rs   → v4/_native_symbol_snapshot.rs
v4/python_transpiler.rs        → v4/_python_transpiler.rs
v4/rust_domain_scanner.rs      → v4/_rust_domain_scanner.rs
v4/machines/                   → v4/_machines/
v4/ts_harness_machine.rs       → v4/_ts_harness_machine.rs
v4/parser_debug.rs             → v4/_parser_debug.rs
v4/system_transformer.rs       → v4/_system_transformer.rs
```

**Empty directories to delete:**
```
frame_c/body_interleaver/
frame_c/body_segmenter/
frame_c/declaration_importers/
frame_c/llvm/
frame_c/modules/
frame_c/native_partition_scanner/
frame_c/native_region_segmenter/
frame_c/target_parsers/
frame_c/tools/
frame_c/validation/
```

### 3.2 After Pipeline Rewrite: Validation Demo Pipeline (`_` prefix)

Once the new pipeline (from `frame_v4_pipeline_architecture.md`) replaces the validation demo path, these files become dead and should be `_` prefixed:

```
v4/ast.rs                      → v4/_ast.rs
v4/module_partitioner.rs       → v4/_module_partitioner.rs
v4/prolog_scanner.rs           → v4/_prolog_scanner.rs
v4/outline_scanner.rs          → v4/_outline_scanner.rs
v4/system_parser.rs            → v4/_system_parser.rs
v4/interface_parser.rs         → v4/_interface_parser.rs
v4/machine_parser.rs           → v4/_machine_parser.rs
v4/validator.rs                → v4/_validator.rs
v4/mir.rs                      → v4/_mir.rs
v4/mir_assembler.rs            → v4/_mir_assembler.rs
v4/frame_statement_parser.rs   → v4/_frame_statement_parser.rs
v4/expander/                   → v4/_expander/
v4/facade.rs                   → v4/_facade.rs
v4/system_param_semantics.rs   → v4/_system_param_semantics.rs
v4/import_scanner/             → v4/_import_scanner/
```

### 3.3 After Pipeline Rewrite: Superseded by New Stages

Once the new Segmenter + Lexer + Parser are implemented, these current pipeline files become dead:

```
v4/frame_parser.rs             → v4/_frame_parser.rs   (replaced by lexer/ + parser/)
v4/pipeline/traits.rs          → v4/_pipeline_traits.rs (RegionScannerTrait no longer needed)
v4/pipeline/compiler.rs        → rewritten in place (new orchestrator)
```

Note: `native_region_scanner/` and `body_closer/` are **retained** — their `SyntaxSkipper` and `BodyCloser` traits are reused by the new Segmenter and Lexer.

---

## 4. Multi-System Support

### 4.1 Current Implementation

The current pipeline supports multi-system files via:
- `FrameParser::count_systems()` — counts `@@system ` occurrences (full-text search)
- `FrameAst::Module(ModuleAst)` — wraps multiple `SystemAst` nodes
- `compile_ast_based()` — iterates `module.systems` with native code between them

### 4.2 New Pipeline Architecture

The new architecture (per `frame_v4_pipeline_architecture.md`) handles multi-system cleanly:

1. **Segmenter** identifies ALL `@@system` blocks in the source, producing multiple `Segment::System` entries in the `SourceMap`
2. **Pipeline Orchestrator** iterates `source_map.systems()`, running Lexer → Parser → Arcanum → Validator → Codegen → Backend for each system independently
3. **Assembler** reassembles the full output: `Native₁ + Generated₁ + Native₂ + Generated₂ + ... + NativeN`

This is cleaner than the current approach because:
- No `count_systems()` full-text search — the Segmenter finds systems structurally
- No `FrameAst::Module` wrapper — each system is parsed independently
- Native code between systems is tracked by the SourceMap, not extracted ad-hoc in compiler.rs

### 4.3 Cross-System References

Tagged instantiation (`@@SystemName()`) in native code can reference any system in the file. The Assembler handles this:
- The Segmenter extracts system names from all `@@system` blocks
- The Assembler passes the full set of defined system names to `expand_tagged_instantiations()`
- This works identically to the current approach

### 4.4 Shared Arcanum

Currently, `build_arcanum_from_frame_ast()` builds a single Arcanum from the full `FrameAst` (which may contain multiple systems). In the new architecture, each system gets its own Arcanum. If cross-system validation is needed in the future (e.g., verifying `@@SystemName()` references), the orchestrator can build a module-level name registry from the Segmenter's system names.

---

## 5. Implementation Sequence

### Phase 1: Dead Code Cleanup (Now)
1. `_` prefix dead code files (Section 3.1)
2. Delete empty legacy directories
3. Update `v4/mod.rs` to comment out dead module declarations
4. Verify: `cargo build` passes, all 539 tests pass

### Phase 2: New Pipeline Implementation (Per `frame_v4_pipeline_architecture.md`)
1. Implement Source Segmenter (Stage 0)
2. Implement Frame Lexer (Stage 1)
3. Implement Frame Parser (Stage 2)
4. Update Codegen to remove `source: &[u8]` dependency (Stage 5)
5. Implement Output Assembler (Stage 7)
6. Implement Pipeline Orchestrator (Stage 11)
7. Verify: all 539 tests pass

### Phase 3: Validation Demo Cleanup
1. Migrate `--validate` to use new pipeline (or remove validation demo mode)
2. `_` prefix old validation demo files (Section 3.2)
3. Update `v4/mod.rs`
4. Verify: `cargo build` passes

### Phase 4: Final Cleanup
1. `_` prefix superseded parser files (Section 3.3)
2. Remove all `_` prefixed files
3. Delete empty directories
4. Final verification: all tests pass

### Phase 5: Warning & Test Cleanup
1. Fix unit test compilation errors (missing struct fields in test constructors)
   - `frame_ast.rs` test: add `library` field to `PersistAttr` constructor
   - `system_codegen.rs` test: add `raw_code` field to `DomainVar` constructors
   - `arcanum_tests.rs`: add `raw_code` to `DomainVar`, `is_static` to `OperationAst`
2. Fix all remaining compiler warnings (`cargo build --release` with zero warnings)
   - Unused imports, unused variables, unused functions, dead code
3. Verify: `cargo build --release` produces 0 warnings
4. Verify: `cargo test --release` passes all unit tests
5. Verify: integration tests pass via `framepiler_test_env`:
   ```bash
   cd /Users/marktruluck/projects/frame_transpiler/framepiler_test_env/tests
   ./run_tests.sh
   ```
   Expected: 539 tests passing (Python 144, TypeScript 126, Rust 130, C 139)

---

## 6. Risk Assessment

| Risk | Mitigation |
|------|-----------|
| Dead code identification is wrong | Verify with `cargo build` after each rename; `_` prefix is reversible |
| Validation demo mode breaks | Phase 1 only touches truly dead code; validation demo files preserved |
| Multi-system regression | Existing multi-system tests validate; new pipeline tested against same suite |
| `splice.rs` is shared | splice.rs remains active (used by primary pipeline); only deprecated after pipeline rewrite |
