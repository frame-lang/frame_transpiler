# Framepiler Architecture (V4) — Authoritative Overview

## Purpose

Provide a single, up-to-date overview of the V4 architecture. V4 is a **pure preprocessor** for `@@system` blocks - native code passes through verbatim ("oceans model").

## Core Principles

- **Oceans Model**: Native code is the "ocean" that passes through unchanged. `@@system` blocks are "islands" that get expanded to classes.
- **Target-Aware**: Each file specifies `@@target <lang>` to determine output format.
- **Native Domain**: Domain variables use native language syntax, not Frame syntax.
- **Persistence Built-in**: `@@persist` annotation generates serialization methods directly in the class.

## V4 Pipeline

```
Source file with @@system blocks
    ↓
FrameParser (frame_parser.rs) - Parse @@system into FrameAst
    ↓
Arcanum (arcanum.rs) - Build symbol table from AST
    ↓
FrameValidator (frame_validator.rs) - Validate transitions, states
    ↓
SystemCodegen (system_codegen.rs) - Generate CodegenNode AST
    ↓
Language Backend (backends/*.rs) - Emit target language code
    ↓
Output: Native prolog + Generated class + Native epilog
```

## Key V4 Files

| File | Purpose |
|------|---------|
| `framec/src/frame_c/v4/frame_parser.rs` | Parse `@@system` blocks |
| `framec/src/frame_c/v4/arcanum.rs` | Symbol table |
| `framec/src/frame_c/v4/frame_validator.rs` | Validation |
| `framec/src/frame_c/v4/codegen/system_codegen.rs` | Generate CodegenNode |
| `framec/src/frame_c/v4/codegen/backends/python.rs` | Python backend |
| `framec/src/frame_c/v4/codegen/backends/typescript.rs` | TypeScript backend |
| `framec/src/frame_c/v4/codegen/backends/rust.rs` | Rust backend |
| `framec/src/frame_c/v4/codegen/backends/c.rs` | C backend |

## Supported Languages

| Language | Target | File Extension | Status |
|----------|--------|----------------|--------|
| Python | `python_3` | `.fpy` | 100% (144/144 tests) |
| TypeScript | `typescript` | `.fts` | 100% (126/126 tests) |
| Rust | `rust` | `.frs` | 100% (130/130 tests) |
| C | `c` | `.fc` | 100% (139/139 tests) |

## Features

### Persistence (`@@persist`)

When `@@persist` is present, generates:
- **Python**: `save_state()` / `restore_state()` using pickle
- **TypeScript**: `saveState()` / `restoreState()` using plain objects
- **Rust**: `save_state()` / `restore_state()` using serde_json

### Context Stack

V4 uses a context stack for proper reentrancy:
- `FrameEvent` - routing object (message + parameters)
- `FrameContext` - call context with event reference and return slot
- `_context_stack` - stack of contexts for nested calls

## Documentation

- **Language Reference**: [`docs/framelang/v4/frame_v4_lang_reference.md`](../framelang/v4/frame_v4_lang_reference.md)
- **Architecture**: [`docs/framelang/v4/frame_v4_architecture.md`](../framelang/v4/frame_v4_architecture.md)
- **Runtime**: [`docs/framelang/v4/frame_v4_runtime.md`](../framelang/v4/frame_v4_runtime.md)
- **Codegen Spec**: [`docs/framelang/v4/frame_v4_codegen_spec.md`](../framelang/v4/frame_v4_codegen_spec.md)

## V3 Architecture (DEPRECATED)

> **⚠️ V3 is deprecated.** Documentation archived with `_` prefix:
> - `docs/framepiler_design/_architecture_v3/`
> - `docs/framepiler_design/_architecture_v3.md`
