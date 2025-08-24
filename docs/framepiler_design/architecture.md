# Frame Transpiler Architecture

## Overview

The Frame transpiler converts Frame language source files (.frm) to target languages (Python, C#, JavaScript, etc.).

## Compilation Pipeline

```
Frame Source (.frm) 
    ↓
Scanner (Tokenizer) → framec/src/frame_c/scanner.rs
    ↓  
Parser → framec/src/frame_c/parser.rs
    ↓
AST (FrameModule) → framec/src/frame_c/ast.rs
    ↓
Visitors (Code Generation) → framec/src/frame_c/visitors/
    ↓
Target Code (Python, C#, etc.)
```

## v0.30 Modular AST Structure

```
FrameModule (Top-Level)
├── Module (metadata/attributes)
├── Functions[] (peer entities)
└── Systems[] (peer entities)
    └── SystemNode
        ├── Module (system-specific metadata)
        ├── Interface Block
        ├── Machine Block  
        ├── Actions Block
        ├── Operations Block
        └── Domain Block
```

## Key Components

### Scanner (scanner.rs)
- Token recognition in `scan_token()` method
- New tokens added to `TokenType` enum
- Use `peek()` and `peek_next()` for lookahead

### Parser (parser.rs)
- Event handler parsing in `event_handler()` method
- Terminator parsing handles `return`, `=>`, `@:>`
- Use `TerminatorType` enum for different terminators
- **v0.30**: Multi-entity parsing with smart fallback to syntactic mode

### AST (ast.rs)
- All syntax tree node definitions
- `TerminatorType` enum defines terminator semantics
- **v0.30**: FrameModule container with peer Functions[] and Systems[]

### Symbol Table (symbol_table.rs)
- **v0.30**: System-scoped state resolution
- Arcanum provides system.get_state() pattern
- Proper isolation between multiple systems

### Visitors (visitors/)
- Each target language has its own visitor
- All visitors must handle new `TerminatorType::DispatchToParentState`
- Python visitor is primary reference implementation
- **v0.30**: Fixed FrameCompartment generation bug

## v0.30 Multi-Entity Features

### Smart Parsing Fallback
When semantic parsing fails on complex multi-entity files, the transpiler automatically falls back to syntactic parsing mode, allowing code generation to continue.

### System-Scoped State Resolution
Multiple systems in the same file maintain proper isolation through system-scoped symbol table operations.

### FrameCompartment Generation
Fixed bug where Python visitor generated system-specific compartment classes instead of the standard FrameCompartment class.

### Call Chain Scope Processing (v0.30 Critical Fix)
Resolved critical bug in Python visitor where external object method calls (`obj.method()`) incorrectly generated `obj.self.method()` while internal operation calls (`self.method()`) lost required `self.` prefixes. The fix implements conditional flag setting in call chain processing to properly distinguish between external and internal call contexts.

## Build System

### Main Build
```bash
cargo build
```

### Test Build (Disabled)
The framec_tests crate is temporarily disabled in Cargo.toml to allow main transpiler builds to succeed.

## Testing

Test files are located in:
- **Frame source (.frm)**: `framec_tests/python/src/`
- **Generated Python**: Next to source files in `src/`

### Validation Protocol
1. **Generate**: Run framec to generate code
2. **Execute**: Run the generated target code
3. **Validate**: Verify output matches expected behavior
4. **Report**: Document verified functionality

## Architecture Improvements (v0.30)

### System-Scoped API Restructuring
Moved from singleton pattern to proper system-scoped operations:
- `system_symbol.get_state()` instead of `arcanum.get_state_in_system()`
- Eliminated infinite loops and parser hangs
- Proper encapsulation of system-specific operations

### Modular Architecture
Replaced SystemNode-centric design with proper FrameModule container:
- Functions and systems are peer entities within modules
- No artificial parent-child relationships
- Clean separation between module structure and entity content