# AI Planning - Frame Transpiler

Cross-tool communication document for AI assistants working on the Frame transpiler.

## Current State (v0.96.9, build 75)

### What Just Happened
- **Rust backend state vars moved to compartment**: Fixed fundamental architectural divergence where Rust stored state vars as `self._sv_*` struct fields instead of on the compartment (like Python/TS/C). State vars now live in a typed `StateContext` enum on `compartment.state_context`.
- **Push/pop fixed**: Push uses `mem::replace` (no clone) to move compartment to stack. Pop goes through `__transition()` → kernel, correctly sending `"<$"` exit and `"$>"` enter (was bypassing kernel, sending wrong exit message `"$<"`).
- **HSM state var access fixed**: Added `__sv_comp` compartment chain navigation for Rust state var reads/writes, matching the pattern in Python/TS/C. Child states correctly access parent state vars by walking `parent_compartment` chain.
- **Serialization updated**: Save/restore uses `state_context` directly from compartment instead of syncing `_sv_*` fields.
- **Test runner PATH fix**: Replaced one-off cargo PATH fix with comprehensive PATH init block covering `/usr/local/bin`, `/opt/homebrew/bin`, `$HOME/.cargo/bin` in both `run_tests.sh` and `run_single_test.sh`.
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
| **OutlineScanner** | 5 sections + scope stacks | 1 | Refactored | ✅ DONE (refactored, not dogfooded) |
| **NativeRegionScanner** | 2 states + context | 1 (unified) | Good candidate | ✅ DONE (3 sub-machines) |
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

### Phase 4: OutlineScanner ✅ COMPLETE (Refactoring Only)

Refactored the OutlineScanner to eliminate duplication and the 21× BodyCloser dispatch anti-pattern. Not converted to a Frame FSM — too complex/parser-like for Frame to add clarity.

**Changes:**
- Merged `scan()` and `scan_collect()` into `scan_internal(bytes, start, lang, strict)` — both public methods are now thin wrappers
- Added `close_body()` to `body_closer/mod.rs` as single polymorphic dispatch point
- Fixed 3 bugs: missing body_scopes in scan_collect, inconsistent owner_id, inconsistent kind logic
- Replaced 30-line `close_system()` in `system_param_semantics.rs` with single `close_body()` call
- Line reduction: 637 → 425 lines (−212, 33%)

### Phase 5: NativeRegionScanner ✅ COMPLETE (Hierarchical Decomposition)

Decomposed the NativeRegionScanner's inline parsing logic into 3 Frame sub-machines using the **state manager pattern** — create sub-machine on detection, parse, collect results, let it drop.

**Sub-machines created:**

| Machine | Type | .frs spec | Lines | Purpose |
|---------|------|-----------|-------|---------|
| ExprScanner | PDA | expr_scanner.frs | 75 | Scan RHS expressions (replaces 3× duplication) |
| ContextParser | FSM | context_parser.frs | 254 | Parse all `@@` constructs (7 variants) |
| StateVarParser | FSM | state_var_parser.frs | 89 | Parse `$.varName` access and assignment |

**Hierarchical composition:** ContextParser and StateVarParser both `include!("expr_scanner.gen.rs")` and create `ExprScannerFsm` instances within state handlers when they detect assignment expressions.

**Key design decisions:**
- Frame can't use Rust enums as domain vars → numeric discriminants mapped back to `FrameSegmentKind` by caller
- `@@SystemName()` needs SyntaxSkipper trait → `balanced_paren_end` pre-computed by caller, passed as domain var
- Each sub-machine is a separate Frame system, composed via native Rust code (Frame doesn't support multi-system composition natively)

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

- **PATH**: Both `run_tests.sh` and `run_single_test.sh` have a PATH init block that adds `$HOME/.cargo/bin`, `/usr/local/bin`, `/opt/homebrew/bin` for non-login shells (CI, Claude Code). This covers cargo, node/npx, python3, gcc.
- **Worktree + submodule**: The `framepiler_test_env/` directory is a git submodule that is NOT checked out in worktrees. Use the main repo's test infrastructure with `FRAMEC=<worktree>/target/release/framec` pointing to the worktree binary.

## What's Next
- Additional language backend improvements as needed
- Phase 15 (GraphViz backend) from V4 plan when dogfooding is complete
- Evaluate remaining candidates (Lexer, PragmaScanner) if warranted
