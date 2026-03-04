# AI Planning - Frame Transpiler

Cross-tool communication document for AI assistants working on the Frame transpiler.

## Current State (v0.96.2)

### What Just Happened
- **Type pass-through**: Removed `Type::Int/Float/String/Bool` from `frame_ast.rs`. All types are now `Type::Custom(String)` — Frame has no type system, types are opaque strings passed through verbatim. Backend `convert_type()` functions map generic names (e.g., `"int"` → `"i64"` for Rust, `"number"` for TypeScript).
- **Rust domain var init**: Domain vars without explicit initializers now get `Default::default()` in Rust struct literals.
- **C BodyCloser dogfooding**: Replaced hand-written C BodyCloser state machine with a Frame system (`body_closer/c.frs` → `c.gen.rs`).
- **Dead code removal**: Removed ~1,400 lines of dead V3 code from `mod.rs`, `frame_parser.rs`, `system_codegen.rs`, and other files.
- **Zero compiler warnings**: All `cargo build --release` warnings eliminated.
- **547/547 tests passing**: Python 146, TypeScript 128, Rust 132, C 141.

### Active Branch
- `claude/exciting-williamson` (worktree of `v4_pure`)

### Architecture
V4 is a **pure preprocessor** — native code passes through verbatim, `@@system` blocks are expanded to classes.

Pipeline: Segmenter → Lexer → Parser → Arcanum → Validator → Codegen → Backend → Assembler

### Dogfooding Status
The transpiler is beginning to use Frame systems internally:
- ✅ C BodyCloser (`body_closer/c.frs`)
- Candidates for conversion: other unstructured state machines in the codebase

### What's Next
- Continue dogfooding: identify and replace remaining hand-written state machines with Frame systems
- Additional language backend improvements as needed
