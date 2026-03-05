# AI Planning - Frame Transpiler

Cross-tool communication document for AI assistants working on the Frame transpiler.

## Current State (v0.96.6, build 72)

### What Just Happened
- **Phase 3 ImportScanner dogfooding COMPLETE**: All 7 import scanners converted to Frame systems (`.frs` → `.gen.rs` → `.rs` wrapper), following the exact same proven 3-file pattern as BodyClosers and SyntaxSkippers.
- **Shared helpers extracted to `import_scanner/mod.rs`**: `starts_kw()` and `is_frame_section_start()` moved from duplicated per-language code to shared module. Called from generated `.gen.rs` files.
- **7 `.frs` Frame specs written**: C, C++, C#, Java, Python, Rust, TypeScript import scanners, each as a 2-state FSM (`$Init` → `$Scanning`).
- **All public API preserved**: `ImportScannerC`, `ImportScannerCpp`, `ImportScannerCs`, `ImportScannerJava`, `ImportScannerPy`, `ImportScannerRust`, `ImportScannerTs` — names unchanged, used by `module_partitioner.rs`.
- **547/547 tests passing**: Python 146/146, TypeScript 128/128, Rust 132/132, C 141/141.
- **Zero compiler warnings**: Clean `cargo build --release`.

### Active Branch
- `claude/exciting-williamson` (worktree of `v4_pure`)

### Architecture
V4 is a **pure preprocessor** — native code passes through verbatim, `@@system` blocks are expanded to classes.

Pipeline: Segmenter → Lexer → Parser → Arcanum → Validator → Codegen → Backend → Assembler

## Dogfooding Roadmap

### Hand-Written State Machines in the Codebase

| Component | States | Instances | Dogfood Priority | Status |
|-----------|--------|-----------|-----------------|--------|
| **BodyClosers** | 5-10 per language | 7 languages | Best candidate | ✅ DONE |
| **SyntaxSkippers** | 3-4 per language | 7 languages | Best candidate | ✅ DONE (refactored to call helpers) |
| **OutlineScanner** | 5 sections + scope stacks | 1 | Good candidate | 📋 Planned |
| **NativeRegionScanner** | 2 states + context | 1 (unified) | Good candidate | 📋 Planned |
| **ImportScanner** | 2 (Init→Scanning) | 7 languages | Best candidate | ✅ DONE |
| **Lexer** | 2 modes | 1 | Questionable | ❓ Evaluate |
| **PragmaScanner** | 2 states | 1 | Low value | ⏸️ Deferred |
| **PrologScanner** | linear | 1 | Too trivial | ⏸️ Skip |

### Phase 1: BodyClosers ✅ COMPLETE

All 7 body closers converted to Frame systems:

| Language   | .frs spec          | .gen.rs (generated) | .rs (glue wrapper) |
|------------|--------------------|--------------------|-------------------|
| C          | c.frs              | c.gen.rs           | c.rs              |
| Java       | java.frs           | java.gen.rs        | java.rs           |
| C++        | cpp.frs            | cpp.gen.rs         | cpp.rs            |
| Rust       | rust_lang.frs      | rust_lang.gen.rs   | rust.rs           |
| Python     | python.frs         | python.gen.rs      | python.rs         |
| TypeScript | typescript.frs     | typescript.gen.rs  | typescript.rs     |
| C#         | csharp.frs         | csharp.gen.rs      | csharp.rs         |

### Phase 2: SyntaxSkippers ✅ COMPLETE (with shared helper calls)

All 7 syntax skippers converted to Frame systems. Each `.frs` spec delegates to shared helpers from `unified.rs` for standard scanning logic, keeping only language-specific constructs inline:

| Language   | .frs spec              | .gen.rs (generated)       | .rs (glue wrapper) | Helpers Used | Inline Logic |
|------------|------------------------|--------------------------|-------------------|--------------|-------------|
| C          | c_skipper.frs          | c_skipper.gen.rs         | c.rs              | All standard | None |
| Java       | java_skipper.frs       | java_skipper.gen.rs      | java.rs           | All standard | `"""..."""` text blocks |
| C++        | cpp_skipper.frs        | cpp_skipper.gen.rs       | cpp.rs            | All standard | `R"delim(...)delim"` raw strings |
| Python     | python_skipper.frs     | python_skipper.gen.rs    | python.rs         | hash, triple, simple, python line end, c-like paren | None |
| Rust       | rust_skipper.frs       | rust_skipper.gen.rs      | rust.rs           | line, raw, simple, c-like paren | Nested `/* */`, raw-aware line end |
| TypeScript | typescript_skipper.frs | typescript_skipper.gen.rs| typescript.rs     | line, block, template, simple | Template-aware line end & paren |
| C#         | csharp_skipper.frs     | csharp_skipper.gen.rs    | csharp.rs         | hash, line, block, simple, c-like line end & paren | `@"..."`, `$"..."`, `$"""..."""` |

### Phase 3: ImportScanner ✅ COMPLETE

All 7 import scanners converted to Frame systems. Each FSM has 2 states (`$Init` → `$Scanning`), with the enter handler containing all scan logic. Shared helpers (`starts_kw`, `is_frame_section_start`) extracted to `import_scanner/mod.rs`.

| Language   | .frs spec             | .gen.rs (generated)        | .rs (glue wrapper) | Keywords Scanned |
|------------|-----------------------|---------------------------|-------------------|-----------------|
| C          | c_import.frs          | c_import.gen.rs           | c.rs              | `#include` |
| C++        | cpp_import.frs        | cpp_import.gen.rs         | cpp.rs            | `#include`, `using`, `import` |
| C#         | csharp_import.frs     | csharp_import.gen.rs      | csharp.rs         | `using`, `#` preprocessor |
| Java       | java_import.frs       | java_import.gen.rs        | java.rs           | `import`, `package` |
| Python     | python_import.frs     | python_import.gen.rs      | python.rs         | `import`, `from` |
| Rust       | rust_import.frs       | rust_import.gen.rs        | rust.rs           | `use`, `extern` |
| TypeScript | typescript_import.frs | typescript_import.gen.rs  | typescript.rs     | `import`, `export` |

### Phase 4: OutlineScanner

Single unified scanner with 5 section states + scope stack tracking. Needs refactoring first — `scan()` and `scan_collect()` have ~80% code duplication, and BodyCloser routing is repeated 3×7=21 times.

### Phase 5: NativeRegionScanner

Single unified scanner with 2 states + context. The core "oceans model" scanner that finds Frame islands in native code. Already has per-language SyntaxSkippers dogfooded (Phase 2).

### Deferred / Skip

- **Lexer**: 2 modes (Structural/NativeAware) — questionable whether Frame adds value here since the lexer is tightly coupled to the token stream
- **PragmaScanner**: Only 2 states, low value from conversion
- **PrologScanner**: Linear scan, too trivial for Frame

### Dogfooding Pattern (3 files per component per language)

1. **`.frs`** — Frame specification using `@@target rust` and `@@system <Name>Fsm { ... }`
2. **`.gen.rs`** — Generated via `./target/release/framec compile -l rust -o /tmp <name>.frs` then `cp /tmp/<name>.rs <name>.gen.rs` (no manual fixes needed)
3. **`.rs`** — Thin glue wrapper: `include!("<name>.gen.rs")` + implements the relevant trait

### Regeneration Checklist

To regenerate after modifying a `.frs`:

```bash
# 1. Transpile
./target/release/framec compile -l rust -o /tmp <name>.frs
cp /tmp/<name>.rs <target_dir>/<name>.gen.rs

# 2. Build and test (no manual fixes needed)
cargo build --release
cd framepiler_test_env/tests && FRAMEC=../../target/release/framec ./run_tests.sh
```

## Test Infrastructure Notes

- **PATH**: `run_tests.sh` auto-detects `$HOME/.cargo/bin` for Rust tests when `cargo` isn't already in PATH (non-login shells, CI).
- **Worktree + submodule**: The `framepiler_test_env/` directory is a git submodule that is NOT checked out in worktrees. Use the main repo's test infrastructure with `FRAMEC=<worktree>/target/release/framec` pointing to the worktree binary.

## What's Next
- **Phase 4: OutlineScanner** — Refactor scan/scan_collect duplication, then convert to Frame system
- **Phase 5: NativeRegionScanner** — Convert core scanning to Frame system
- Additional language backend improvements as needed
- Phase 15 (GraphViz backend) from V4 plan when dogfooding is complete
