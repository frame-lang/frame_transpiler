# AI Planning - Frame Transpiler

Cross-tool communication document for AI assistants working on the Frame transpiler.

## Current State (v0.96.3, build 69)

### What Just Happened
- **Fixed 3 Rust backend codegen bugs**: All `.gen.rs` files now regenerate cleanly from `.frs` specs with zero manual fixes needed.
  1. `parse_simple_expression` extended to handle `::` path expressions (e.g., `Vec::new()`, `String::new()`) — no more spurious `new` field or broken initializers.
  2. `return` now emitted after every `self.__transition(__compartment)` call across all backends (Python, TypeScript, Rust, C).
  3. Domain var fields emit with `Visibility::Public` so generated FSMs can be driven externally.
- **547/547 tests passing**: Python 146/146, TypeScript 128/128, Rust 132/132, C 141/141.
- **Zero compiler warnings**: Clean `cargo build --release`.
- **All 7 body closers dogfooded**: Regenerated from `.frs` specs with no manual fixes.

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
2. **`.gen.rs`** — Generated via `./target/release/framec compile -l rust -o . <name>.frs` (no manual fixes needed)
3. **`.rs`** — Thin glue wrapper: `include!("<name>.gen.rs")` + implements `BodyCloser` trait

### Regeneration Checklist

To regenerate a body closer after modifying its `.frs`:

```bash
# 1. Transpile
./target/release/framec compile -l rust -o /tmp <name>.frs
cp /tmp/<name>.rs framec/src/frame_c/v4/body_closer/<name>.gen.rs

# 2. Build and test (no manual fixes needed)
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

## What's Next
- Additional language backend improvements as needed
- Explore further dogfooding candidates
