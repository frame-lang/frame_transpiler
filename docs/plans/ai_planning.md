# AI Planning - Frame Transpiler

Cross-tool communication document for AI assistants working on the Frame transpiler.

## Current State (v0.96.2, build 68)

### What Just Happened
- **Type pass-through**: Removed `Type::Int/Float/String/Bool` from `frame_ast.rs`. All types are now `Type::Custom(String)` — Frame has no type system, types are opaque strings passed through verbatim. Backend `convert_type()` functions map generic names (e.g., `"int"` → `"i64"` for Rust, `"number"` for TypeScript).
- **Rust domain var init**: Domain vars without explicit initializers now get `Default::default()` in Rust struct literals.
- **All 7 body closers dogfooded**: Replaced all hand-written body closer state machines with Frame systems.
- **Dead code removal**: Removed ~1,400 lines of dead V3 code from `mod.rs`, `frame_parser.rs`, `system_codegen.rs`, and other files.
- **Zero compiler warnings**: All `cargo build --release` warnings eliminated.
- **545/547 tests passing**: Python 144/146, TypeScript 128/128, Rust 132/132, C 141/141. (2 pre-existing Python failures: `38_context_data`, `test_python_island_mega_syntax`.)

### Active Branch
- `claude/exciting-williamson` (worktree of `v4_pure`)

### Architecture
V4 is a **pure preprocessor** — native code passes through verbatim, `@@system` blocks are expanded to classes.

Pipeline: Segmenter → Lexer → Parser → Arcanum → Validator → Codegen → Backend → Assembler

## Dogfooding Status

All 7 body closers are now Frame-generated state machines:

| Language   | .frs spec          | .gen.rs (generated) | .rs (glue wrapper) | Extra states                                          |
|------------|--------------------|--------------------|-------------------|-------------------------------------------------------|
| C          | c.frs              | c.gen.rs           | c.rs              | InString, InCharLiteral, InLineComment, InBlockComment |
| Java       | java.frs           | java.gen.rs        | java.rs           | InString, InCharLiteral, InLineComment, InBlockComment |
| C++        | cpp.frs            | cpp.gen.rs         | cpp.rs            | + InRawString (R"delim(...)delim")                     |
| Rust       | rust_lang.frs      | rust_lang.gen.rs   | rust.rs           | + InRawString (r#"..."#), nested block comments        |
| Python     | python.frs         | python.gen.rs      | python.rs         | InString, InTripleString, InLineComment, string prefixes |
| TypeScript | typescript.frs     | typescript.gen.rs  | typescript.rs     | + InTemplate (backtick `${}` nesting), Frame V4 stmt detection |
| C#         | csharp.frs         | csharp.gen.rs      | csharp.rs         | + InVerbatimString, InRawString, InPreprocessor, InCharLiteral |

### Dogfooding Pattern (3 files per language)

1. **`.frs`** — Frame specification using `@@target rust` and `@@system <Name>BodyCloserFsm { ... }`
2. **`.gen.rs`** — Generated via `./target/release/framec <name>.frs -l rust > <name>.gen.rs` then manual fixes (see below)
3. **`.rs`** — Thin glue wrapper: `include!("<name>.gen.rs")` + implements `BodyCloser` trait

### Known Rust Backend Issues (Manual Fixes Required After Generation)

When generating `.gen.rs` files from `.frs` specs via `framec --target rust`, these manual fixes are ALWAYS needed:

1. **Spurious `new` field**: Domain vars with `Vec::new()` or `String::new()` initializers cause the backend to emit a phantom `new: Box<dyn std::any::Any>` field. **Fix**: Remove ALL `new: Box<dyn std::any::Any>` lines from the struct definition, and remove ALL `new: Default::default()` lines from the constructor.

2. **Broken initializers**: `Vec::new()` becomes `Vec` (bare type, not a value) and `String::new()` becomes `String`. **Fix**: Change `bytes: Vec,` → `bytes: Vec::new(),` and `error_msg: String,` → `error_msg: String::new(),` in the constructor. Same for any other `Vec` or `String` domain vars.

3. **Missing `pub` on domain vars**: The generated struct fields lack `pub` visibility. **Fix**: Add `pub` to all domain var fields (bytes, pos, depth, result_pos, error_kind, error_msg, and any language-specific fields).

4. **Missing `return` after transitions**: When `self.__transition(__compartment)` appears inside a loop body, execution continues past it. **Fix**: Replace ALL `self.__transition(__compartment)` with `self.__transition(__compartment); return;`. This is safe even at function-end (harmless redundant return). Clean up any `; return;;` double-semicolons that result from lines that already had a semicolon.

### Regeneration Checklist

To regenerate a body closer after modifying its `.frs`:

```bash
# 1. Transpile
./target/release/framec framec/src/frame_c/v4/body_closer/<name>.frs -l rust > framec/src/frame_c/v4/body_closer/<name>.gen.rs

# 2. Apply manual fixes (in order):
#    a. Remove `new: Box<dyn std::any::Any>` from struct (all occurrences)
#    b. Remove `new: Default::default()` from constructor (all occurrences)
#    c. Fix `bytes: Vec,` → `bytes: Vec::new(),` (and similar for String, other Vecs)
#    d. Add `pub` to all domain var fields in the struct
#    e. Replace `self.__transition(__compartment)` → `self.__transition(__compartment); return;`
#    f. Clean up `; return;;` → `; return;`

# 3. Build and test
cargo build --release
cd framepiler_test_env/tests && FRAMEC=../../target/release/framec ./run_tests.sh --serial
```

### Future Dogfooding Candidates

Other state machines in the codebase that could be converted to Frame systems:
- Segmenter state machines (if any exist as hand-written FSMs)
- Any other scanning/parsing state machines

## Test Infrastructure Notes

- **PATH issue**: The test runner needs `cargo` in PATH for Rust tests. If running from a tool that doesn't inherit the user's shell profile, set `export PATH="$HOME/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin:/usr/local/bin:$PATH"` before invoking `run_tests.sh`.
- **Worktree + submodule**: The `framepiler_test_env/` directory is a git submodule that is NOT checked out in worktrees. Use the main repo's test infrastructure with `FRAMEC=<worktree>/target/release/framec` pointing to the worktree binary.
- **Pre-existing failures**: `38_context_data` (Python) and `test_python_island_mega_syntax` (Python) are known pre-existing failures.

## What's Next
- Fix the 2 remaining Rust backend codegen bugs (so `.gen.rs` files need fewer manual fixes)
- Additional language backend improvements as needed
- Investigate pre-existing Python test failures
