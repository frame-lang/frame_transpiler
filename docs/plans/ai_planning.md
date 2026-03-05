# AI Planning - Frame Transpiler

Cross-tool communication document for AI assistants working on the Frame transpiler.

## Current State (v0.96.4, build 70)

### What Just Happened
- **All 7 SyntaxSkippers dogfooded**: Converted all per-language syntax skippers from hand-written Rust to Frame-generated state machines. Each language now has a `.frs` spec → `.gen.rs` (generated) → `.rs` (glue wrapper) triple.
- **Shared helpers in `unified.rs` are now dead code** but retained for reference / future Phase 2b work.
- **Fixed `@@:data`/`@@:params` quote preservation bug**: `extract_bracket_key` was stripping user quotes then re-wrapping in `"..."`, breaking Python f-strings. Now preserves original quote style for Python/TypeScript.
- **Fixed test runner PATH for Rust**: Added `$HOME/.cargo/bin` to PATH in `run_tests.sh` so Rust tests work in non-login shells and CI.
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
| **SyntaxSkippers** | 3-4 per language | 7 languages | Best candidate | ✅ DONE |
| **OutlineScanner** | 5 sections + scope stacks | 1 | Good candidate | 📋 Planned |
| **NativeRegionScanner** | 2 states + context | 1 (unified) | Good candidate | 📋 Planned |
| **ImportScanner** | 5+ (quotes, parens) | 7 languages | Medium | 📋 Planned |
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

### Phase 2: SyntaxSkippers ✅ COMPLETE

All 7 syntax skippers converted to Frame systems:

| Language   | .frs spec              | .gen.rs (generated)       | .rs (glue wrapper) |
|------------|------------------------|--------------------------|-------------------|
| C          | c_skipper.frs          | c_skipper.gen.rs         | c.rs              |
| Java       | java_skipper.frs       | java_skipper.gen.rs      | java.rs           |
| C++        | cpp_skipper.frs        | cpp_skipper.gen.rs       | cpp.rs            |
| Rust       | rust_skipper.frs       | rust_skipper.gen.rs      | rust.rs           |
| Python     | python_skipper.frs     | python_skipper.gen.rs    | python.rs         |
| TypeScript | typescript_skipper.frs | typescript_skipper.gen.rs| typescript.rs     |
| C#         | csharp_skipper.frs     | csharp_skipper.gen.rs    | csharp.rs         |

**Note**: Shared helpers in `unified.rs` (e.g., `skip_line_comment`, `skip_simple_string`, etc.) are now unused by per-language wrappers but retained for Phase 2b — converting these helpers to Frame too (user chose incremental approach).

### Phase 3: OutlineScanner

Single unified scanner with 5 section states + scope stack tracking. Good candidate — non-trivial FSM that would benefit from Frame's state machine clarity.

### Phase 4: NativeRegionScanner

Single unified scanner with 2 states + context. Good candidate — the core "oceans model" scanner that finds Frame islands in native code.

### Phase 5: ImportScanner

7 language-specific scanners with 5+ states handling quotes, parens, etc. Medium priority — more complex than body closers but same per-language pattern.

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
cd framepiler_test_env/tests && FRAMEC=../../target/release/framec ./run_tests.sh --serial
```

## Test Infrastructure Notes

- **PATH**: `run_tests.sh` auto-detects `$HOME/.cargo/bin` for Rust tests when `cargo` isn't already in PATH (non-login shells, CI).
- **Worktree + submodule**: The `framepiler_test_env/` directory is a git submodule that is NOT checked out in worktrees. Use the main repo's test infrastructure with `FRAMEC=<worktree>/target/release/framec` pointing to the worktree binary.

## What's Next
- **Phase 2b: Shared helpers** — Optionally convert dead helper functions in `unified.rs` to Frame (incremental, per user preference)
- **Phase 3: OutlineScanner** — Convert the outline section scanner to a Frame system
- Additional language backend improvements as needed
- Phase 15 (GraphViz backend) from V4 plan when dogfooding is complete
