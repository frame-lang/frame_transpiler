# Frame Transpiler Architecture v0.56

## Overview

The Frame transpiler converts Frame language source files (.frm) to target languages, with Python as the primary target. The system processes single-file Frame programs through a scanner → parser → AST → visitor pipeline.

**Current Version**: v0.56  
**Test Success Rate**: 100% (341/341 tests passing)  
**Primary Target**: Python 3

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
Symbol Table → framec/src/frame_c/symbol_table.rs
    ↓
Visitor (Code Generation) → framec/src/frame_c/visitors/python_visitor.rs
    ↓
Target Code (Python)
```

## Core Components

### Scanner (`scanner.rs`)
**Purpose**: Tokenizes Frame source code into a stream of tokens.

**Key Features**:
- Single-pass tokenization with lookahead
- Python-style comment support (`#` for single-line)
- Frame documentation comments (`{-- --}`)
- Complex literal support (strings, f-strings, numbers with underscores)
- Operator tokenization including walrus (`:=`), floor division (`//`), matrix multiplication (`@`)

**Token Types** (341 total):
- Keywords: `system`, `interface`, `machine`, `fn`, `var`, `return`, `if`, `elif`, `else`, etc.
- Operators: Arithmetic, logical (`and`, `or`, `not`), bitwise, comparison
- Literals: Numbers (with scientific notation, complex), strings (including f-strings, raw, byte)
- Special: `$` (state prefix), `::` (future module separator), walrus operator

### Parser (`parser.rs`)
**Purpose**: Transforms token stream into Abstract Syntax Tree (AST).

**Architecture**:
- Recursive descent parser (4000+ lines)
- Two-pass parsing for symbol table building
- Context-aware parsing with `ScopeContext` tracking

**Key Methods**:
- `parse()` - Entry point, manages two-pass parsing
- `module()` - Parses top-level module structure
- `system_scope()` - Parses system definitions
- `function_scope_async()` - Parses functions (sync/async)
- `expression()` - Expression parsing with precedence
- `statement()` - Statement parsing with context validation

**Parsing Contexts**:
- Global scope (functions, systems, imports)
- Function scope (statements, local variables)
- System scope (operations, interface, machine, actions, domain)
- Block scopes (loops, conditionals)

### AST (`ast.rs`)
**Purpose**: Defines the syntax tree structure for Frame programs.

**Top-Level Structure**:
```rust
pub struct FrameModule {
    pub module: Module,           // Metadata
    pub imports: Vec<ImportNode>, // Import statements
    pub enums: Vec<EnumDeclNode>, // Module-level enums
    pub type_aliases: Vec<TypeAliasNode>, // Type aliases
    pub functions: Vec<Rc<RefCell<FunctionNode>>>, // Functions
    pub systems: Vec<SystemNode>, // Systems
    pub variables: Vec<Rc<RefCell<VariableDeclNode>>>, // Module vars
}
```

**Key Node Types**:
- **SystemNode**: Complete system definition with operations, interface, machine, actions, domain
- **FunctionNode**: Function with parameters, body, async flag
- **StateNode**: State with event handlers, enter/exit events, state variables
- **ExprNode**: Expression types (binary, unary, call, literal, etc.)
- **ImportNode**: Import statements with various forms
- **ClassNode**: Class definitions with methods and variables

### Symbol Table (`symbol_table.rs`)
**Purpose**: Manages symbol resolution and scope hierarchy.

**Architecture - LEGB Model**:
```rust
pub struct Arcanum {
    pub builtin_symtab: Rc<RefCell<SymbolTable>>,  // Built-in functions
    pub global_symtab: Rc<RefCell<SymbolTable>>,   // Cross-module global
    pub module_symtab: Rc<RefCell<SymbolTable>>,   // Module-level scope
    pub current_symtab: Rc<RefCell<SymbolTable>>,  // Current scope
}
```

**Scope Types**:
- `ParseScopeType::Builtin` - Built-in functions
- `ParseScopeType::Global` - Global scope
- `ParseScopeType::Module` - Module (file) level
- `ParseScopeType::NamedModule` - Named modules within files
- `ParseScopeType::Class` - Class scope
- `ParseScopeType::Function` - Function local scope
- `ParseScopeType::System` - System scope
- Various block scopes (State, EventHandler, Loop, etc.)

**Symbol Types**:
- Functions, systems, classes, enums
- Variables (module, state, local, domain)
- Parameters (function, state, event handler)
- Interface methods, actions, operations

### Python Visitor (`python_visitor.rs`)
**Purpose**: Generates Python code from the AST.

**Key Features**:
- Event handlers as functions architecture (v0.36)
- Async/await support with runtime detection
- State machine implementation with compartments
- Hierarchical state machine support
- Type annotation passthrough

**Code Generation Strategy**:
1. Module-level setup (imports, globals)
2. Class generation for systems
3. State dispatcher methods
4. Event handler functions
5. Interface method implementations
6. Runtime infrastructure (FrameEvent, FrameCompartment)

## Language Features

### Core Constructs
- **Systems**: State machines with operations, interface, machine, actions, domain
- **Functions**: Standalone functions with parameters, return types
- **Classes**: Basic OOP with methods and variables
- **States**: Named states with event handlers, enter/exit events
- **Transitions**: `-> $State` with optional exit/enter arguments

### Type System
- **Basic Types**: Inferred from usage, optional annotations
- **Type Aliases**: `type Name = Type` (Python 3.12+ style)
- **Type Annotations**: Pass-through to Python
- **Enums**: Integer and string enums with custom values

### Control Flow
- **Conditionals**: `if`/`elif`/`else` with boolean expressions
- **Loops**: `for`/`while`/`loop` with `break`/`continue`
- **Pattern Matching**: `match`/`case` with guards
- **Loop Else**: Python-style else clauses on loops

### Expressions
- **Operators**: Full Python operator set (arithmetic, logical, bitwise, comparison)
- **Walrus Operator**: `:=` for assignment expressions
- **Lambda**: `lambda params: expr`
- **Comprehensions**: List, dict, set comprehensions
- **String Formatting**: F-strings, percent formatting

### Module System (Single-File)
- **Named Modules**: `module Name { ... }` within files
- **Qualified Access**: `module.function()` syntax
- **Module Variables**: Module-level variable declarations
- **Nested Modules**: Hierarchical organization

### Import System
- **Python Imports**: `import module`, `from module import item`
- **Aliases**: `import module as alias`
- **No Frame imports**: Cannot import other .frm files (yet)

## Configuration System

### Current Structure (`config.rs`)
- 794 lines supporting 7 language backends
- Complex attribute override system
- YAML-based configuration files

### Actually Used
- `event_handlers_as_functions` - Architecture flag
- `public_state_info` - Rarely used
- `public_compartment` - Rarely used

## Test Infrastructure

### Test Organization
```
framec_tests/
├── python/
│   └── src/         # Single-file Frame tests
├── runner/
│   └── frame_test_runner.py  # Test execution
└── reports/
    ├── test_log.md          # Test status
    └── test_matrix_v0.31.md # Detailed matrix
```

### Test Execution
```bash
cd framec_tests
python3 runner/frame_test_runner.py --all --matrix --json --verbose
```

## Build System

### Compilation
```bash
# Direct compilation
framec -l python_3 input.frm > output.py

# With config
framec --config config.yaml -l python_3 input.frm
```

### Available Targets
- `python_3` - Primary target, fully supported
- Others exist but are unmaintained

## Current Limitations

### Single-File Only
- Cannot import other Frame files
- No module distribution mechanism
- All code must be in one file

### Parser Monolith
- 4000+ lines in single file
- Complex state management
- Difficult to extend

### Config Overhead
- 500+ lines of unused configuration
- Legacy multi-language support
- Complex attribute system

## Performance Characteristics

### Compilation Speed
- Single-file parsing: ~50ms for 1000 lines
- Symbol table construction: O(n) with n symbols
- Code generation: Linear with AST size
- Total: <1 second for typical programs

### Memory Usage
- AST retained in memory during compilation
- Symbol tables use Rc/RefCell for shared ownership
- No incremental compilation support

---

## Changelog

### v0.56 (January 2025)
- Added walrus operator (`:=`) support
- Enhanced numeric literals (underscores, scientific notation, complex)
- Added type aliases (`type Name = Type`)
- Context-sensitive `type` keyword handling

### v0.55 (January 2025)
- Fixed state parameters and type annotations
- Achieved 100% test success rate

### v0.54
- Added star expressions for unpacking
- Collection constructor validation

### v0.53
- Fixed collection literals
- Multiple variable declarations

### v0.52
- Multiple assignment and tuple unpacking

### v0.51
- Loop else clause support

### v0.50
- Del statement support

### v0.45-v0.49
- Class support, error handling, assert statements

### v0.40-v0.44
- Pattern matching, Python operator alignment, string features

### v0.35-v0.39
- Async/await, event handlers as functions, logical operators

### v0.34
- Complete module system (single-file)
- List comprehensions
- Qualified names

### v0.31
- Import statements
- Self expression enhancement
- Static method validation

### v0.30
- Multi-entity support
- System-scoped resolution