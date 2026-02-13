# Frame Language & V4 Transpiler Technical Overview

**Version:** 0.87.1
**Date:** February 2026
**Status:** V4 Pipeline Active Development

---

## Table of Contents

1. [What is Frame?](#1-what-is-frame)
2. [The V4 Vision](#2-the-v4-vision)
3. [Transpiler Architecture](#3-transpiler-architecture)
4. [Code Generation Pipeline](#4-code-generation-pipeline)
5. [Target Language Support](#5-target-language-support)
6. [Current Status & Roadmap](#6-current-status--roadmap)

---

## 1. What is Frame?

### 1.1 Overview

Frame is a **domain-specific language (DSL) for state machines** that transpiles to multiple target languages. It provides a clean, declarative syntax for defining:

- **Hierarchical State Machines (HSM)** with parent-child state relationships
- **Event-driven behavior** with typed event handlers
- **State lifecycle management** (enter/exit handlers)
- **Native code integration** - Frame code coexists with target language code

### 1.2 Why Frame?

State machines are fundamental to robust software but are tedious and error-prone to implement manually. Frame solves this by:

1. **Single Source of Truth** - One `.frm` file generates consistent state machine code
2. **Multi-Language Output** - Same specification generates Python, Rust, TypeScript, etc.
3. **Native Code Preservation** - Embed target language code directly in handlers
4. **Validation** - Catch state machine errors at transpile time

### 1.3 Language Syntax (V4)

```frame
@@target python_3

@@system TrafficLight {
    interface:
        start()
        stop()

    machine:
        $Red {
            $>() {                    // Enter handler
                print("Light is RED")
            }

            start() {
                timer.start(30)
                -> $Green             // Transition to Green
            }
        }

        $Green {
            $>() {
                print("Light is GREEN")
            }

            timeout() {
                -> $Yellow
            }
        }

        $Yellow {
            stop() {
                -> $Red
            }
        }

    actions:
        logChange() {
            print(f"State changed to {self._state}")
        }

    operations:
        getStatus(): str {
            return self._state
        }

    domain:
        var timer = Timer()
        var cycle_count = 0
}
```

### 1.4 Key Language Concepts

| Concept | Syntax | Description |
|---------|--------|-------------|
| System | `@@system Name { }` | Top-level state machine container |
| State | `$StateName { }` | A discrete state in the machine |
| Transition | `-> $TargetState` | Move to another state |
| Enter Handler | `$>() { }` | Code executed when entering a state |
| Exit Handler | `$<() { }` | Code executed when leaving a state |
| Interface | `interface:` | Public methods exposed to callers |
| Actions | `actions:` | Private helper methods |
| Operations | `operations:` | Methods with return values |
| Domain | `domain:` | Instance variables |
| Forward | `>>` | Forward event to parent state (HSM) |
| Stack Push | `$$[+]` | Push current state to stack |
| Stack Pop | `$$[-]` | Pop and return to previous state |

### 1.5 Generated Code Structure

For any target language, Frame generates a class with:

```
class SystemName:
    # State tracking
    _state: string              # Current state name
    _state_stack: list          # For push/pop operations
    _state_context: dict        # State-local variables

    # Core machinery (generated)
    constructor()               # Initialize to start state
    _transition(target, ...)    # Handle state transitions
    _change_state(target)       # Direct state change (no handlers)
    _dispatch_event(event)      # Route events to handlers
    _enter()                    # Call current state's enter handler
    _exit()                     # Call current state's exit handler

    # Interface methods (from interface:)
    start()                     # Calls _dispatch_event("start")
    stop()                      # Calls _dispatch_event("stop")

    # Handler methods (generated from states)
    _s_Red_start()              # $Red.start() handler
    _s_Green_timeout()          # $Green.timeout() handler
    _s_Red_enter()              # $Red.$>() enter handler

    # Actions & Operations (from actions:/operations:)
    logChange()
    getStatus(): str
```

---

## 2. The V4 Vision

### 2.1 Frame V4 is a Preprocessor

Frame V4 is a **preprocessor** that integrates into native build toolchains. Like TypeScript→JavaScript or Sass→CSS, Frame:

1. **Reads** Frame source files (`.frm`, `.fpy`, `.frts`, `.frs`)
2. **Generates** native source files (`.py`, `.ts`, `.rs`)
3. **Exits** - letting native toolchains handle compilation and validation

Frame does NOT parse or validate native code. Native code semantics (variable types, import resolution, function signatures) are validated by the target language's compiler.

### 2.2 Two-Pass Validation Model

| Pass | When | What | Who |
|------|------|------|-----|
| **Pass 1** | Transpile-time | Frame semantics | Frame compiler |
| **Pass 2** | Compile/Run-time | Native semantics | Target compiler (pyc/tsc/rustc) |

**Frame validates what only Frame knows:**
- State machine topology (states exist, parent relationships)
- Transition validity (target state exists, parameter arity)
- Handler structure (terminal statements must be last)
- Interface/action/operation declarations

**Native compilers validate the rest:**
- Variable existence
- Type compatibility
- Import resolution
- Native syntax correctness

### 2.3 V4 Design Principles

1. **Preprocessor Model** - Frame is a code generator, not a compiler. Native compilers do the heavy lifting.

2. **Oceans Model** - Native code is the "ocean", Frame statements are "islands"
   - Native code passes through unchanged
   - Only Frame constructs (transitions, forwards, etc.) are transformed
   - Preserves exact indentation, comments, formatting

3. **Frame-Only Validation**
   - Arcanum (symbol table) tracks Frame-declared symbols
   - Validator checks Frame semantics
   - Unknown symbols are assumed to be native (let native compiler check)

4. **Splicer-Based Codegen**
   - NativeRegionScanner finds Frame islands in native ocean
   - Replace Frame segments with generated target code
   - Preserve surrounding native code exactly

5. **Language-Agnostic Core**
   - `CodegenNode` is the universal intermediate representation
   - Backends translate CodegenNode to target syntax
   - Same semantic tree, different surface syntax

### 2.4 Future: V5 Native Integration

V5 (future) will add **optional** native code analysis:
- Extract symbols from native code using language parsers
- Cross-reference Frame and native symbols
- "Did you mean?" suggestions for typos

This is opt-in and non-blocking. V4 must be complete first. See `docs/architecture_v5/PLAN.md`.

### 2.3 V4 Syntax Enhancements

V4 introduces new module-level constructs:

```frame
@@target python_3           // Target language directive
@@module MyModule {         // Multi-system module
    @@system A { ... }
    @@system B { ... }
}
@@persist redis             // Future: persistence backend
@@expect: E402              // Test: expect specific error
```

---

## 3. Transpiler Architecture

### 3.1 High-Level Pipeline

```
┌─────────────────────────────────────────────────────────────────┐
│                     Frame Source (.frm)                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    1. FRAME PARSER                               │
│  • Tokenizes Frame syntax (@@system, $State, ->, etc.)          │
│  • Builds Frame AST (SystemAst, StateAst, HandlerAst, etc.)     │
│  • Preserves source spans for native code extraction            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    2. ARCANUM (Symbol Table)                     │
│  • Catalogs all systems, states, events, variables              │
│  • Resolves references (state names, event names)               │
│  • Tracks HSM parent-child relationships                        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    3. FRAME VALIDATOR                            │
│  • E001: Parse errors                                           │
│  • E402: Unknown state references                               │
│  • E403: Duplicate state definitions                            │
│  • E405: Parameter mismatches                                   │
│  • Validates against Arcanum symbol table                       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    4. CODEGEN (AST Transform)                    │
│  • Transforms Frame AST → CodegenNode (language-agnostic IR)    │
│  • Generates state machine infrastructure                       │
│  • Extracts native code using source spans                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    5. LANGUAGE BACKEND                           │
│  • PythonBackend, RustBackend, TypeScriptBackend, etc.          │
│  • Emits target-specific syntax from CodegenNode                │
│  • Handles language idioms (self vs this, types, etc.)          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Target Code (.py, .rs, .ts)                  │
└─────────────────────────────────────────────────────────────────┘
```

### 3.2 Key Components

#### 3.2.1 Frame Parser (`frame_parser.rs`)

```rust
pub struct FrameParser {
    source: Vec<u8>,           // Raw source bytes
    cursor: usize,             // Current position
    target: TargetLanguage,    // Target for native code handling
}

impl FrameParser {
    pub fn parse_module(&mut self) -> Result<FrameAst, ParseError>;
    fn parse_system(&mut self) -> Result<SystemAst, ParseError>;
    fn parse_state(&mut self) -> Result<StateAst, ParseError>;
    fn parse_handler(&mut self) -> Result<HandlerAst, ParseError>;
    fn parse_statement(&mut self) -> Result<Statement, ParseError>;
}
```

The parser:
- Skips native code preamble (imports, functions before `@@system`)
- Parses Frame constructs into typed AST nodes
- Records source spans for later native code extraction
- Handles nested structures (states within states for HSM)

#### 3.2.2 Frame AST (`frame_ast.rs`)

```rust
pub enum FrameAst {
    System(SystemAst),
    Module(ModuleAst),
}

pub struct SystemAst {
    pub name: String,
    pub interface: Vec<InterfaceMethodAst>,
    pub machine: MachineAst,
    pub actions: Vec<ActionAst>,
    pub operations: Vec<OperationAst>,
    pub domain: Vec<DomainVarAst>,
}

pub struct StateAst {
    pub name: String,
    pub parent: Option<String>,      // HSM parent state
    pub handlers: Vec<HandlerAst>,
    pub enter_handler: Option<HandlerAst>,
    pub exit_handler: Option<HandlerAst>,
}

pub struct HandlerAst {
    pub event: String,
    pub params: Vec<ParamAst>,
    pub body: BodyAst,               // Contains statements + native code
}

pub enum Statement {
    Transition(TransitionStmt),      // -> $State
    Forward(ForwardStmt),            // >> or >>^
    StackPush(StackPushStmt),        // $$[+]
    StackPop(StackPopStmt),          // $$[-]
    ChangeState(ChangeStateStmt),    // ->> $State
    // Native code is NOT a statement - it's preserved via spans
}
```

#### 3.2.3 Arcanum - Symbol Table (`arcanum.rs`)

```rust
pub struct Arcanum {
    pub systems: HashMap<String, SystemInfo>,
}

pub struct SystemInfo {
    pub name: String,
    pub states: HashMap<String, StateInfo>,
    pub events: HashSet<String>,
    pub interface_methods: Vec<String>,
    pub actions: Vec<String>,
    pub operations: Vec<String>,
    pub domain_vars: HashMap<String, VarInfo>,
}

pub struct StateInfo {
    pub name: String,
    pub parent: Option<String>,
    pub handlers: Vec<String>,       // Event names this state handles
    pub has_enter: bool,
    pub has_exit: bool,
}
```

The Arcanum enables:
- Validation of state references (`-> $Unknown` → E402)
- HSM parent chain resolution
- Event handler discovery for dispatch

#### 3.2.4 Codegen Node (`codegen/ast.rs`)

```rust
pub enum CodegenNode {
    // Structural
    Module { imports: Vec<CodegenNode>, items: Vec<CodegenNode> },
    Class { name: String, fields: Vec<Field>, methods: Vec<CodegenNode>, ... },
    Enum { name: String, variants: Vec<EnumVariant> },

    // Methods
    Method { name: String, params: Vec<Param>, body: Vec<CodegenNode>, ... },
    Constructor { params: Vec<Param>, body: Vec<CodegenNode>, ... },

    // Statements
    VarDecl { name: String, type_annotation: Option<String>, init: Option<Box<CodegenNode>> },
    Assignment { target: Box<CodegenNode>, value: Box<CodegenNode> },
    Return { value: Option<Box<CodegenNode>> },
    If { condition: Box<CodegenNode>, then_block: Vec<CodegenNode>, else_block: Option<Vec<CodegenNode>> },

    // Frame-specific (expanded by backends)
    Transition { target_state: String, exit_args: Vec<CodegenNode>, enter_args: Vec<CodegenNode> },
    Forward { to_parent: bool },
    StackPush { },
    StackPop { },

    // Native code preservation
    NativeBlock { code: String, span: Option<Span> },

    // Expressions
    Literal(Literal),
    Ident(String),
    MethodCall { receiver: Box<CodegenNode>, method: String, args: Vec<CodegenNode> },
    // ... more expression types
}
```

#### 3.2.5 Language Backend Trait (`codegen/backend.rs`)

```rust
pub trait LanguageBackend {
    /// Emit CodegenNode as target language code
    fn emit(&self, node: &CodegenNode, ctx: &mut EmitContext) -> String;

    /// Runtime imports needed (e.g., "from typing import Any")
    fn runtime_imports(&self) -> Vec<String>;

    /// Language-specific class/struct syntax
    fn class_syntax(&self) -> ClassSyntax;

    /// Target language identifier
    fn target_language(&self) -> TargetLanguage;

    /// Keyword mappings
    fn true_keyword(&self) -> &'static str;   // "True" vs "true"
    fn false_keyword(&self) -> &'static str;  // "False" vs "false"
    fn null_keyword(&self) -> &'static str;   // "None" vs "null" vs "nil"
}
```

### 3.3 Native Code Handling (Oceans Model)

The "Oceans Model" is central to Frame's design:

```
┌────────────────────────────────────────────────────────────┐
│  # Native code (ocean)                                      │
│  import math                                                │
│  def helper(): pass                                         │
│                                                             │
│  @@system Calculator {                                      │
│      machine:                                               │
│          $Ready {                                           │
│              calculate(x) {                                 │
│                  ┌─────────────────────────────────────┐   │
│                  │ result = math.sqrt(x)  ← Native     │   │
│                  │ print(f"Result: {result}")          │   │
│                  │ -> $Done               ← Frame      │   │
│                  │ log_complete()         ← Native     │   │
│                  └─────────────────────────────────────┘   │
│              }                                              │
│          }                                                  │
│  }                                                          │
│                                                             │
│  if __name__ == '__main__':                                │
│      calc = Calculator()                                    │
│      calc.calculate(16)                                     │
└────────────────────────────────────────────────────────────┘
```

**Native Code Preservation Process:**

1. **Parser** records `body.span` for each handler (byte offsets in source)
2. **Codegen** extracts raw bytes from source using spans
3. **NativeRegionScanner** identifies Frame segments within native code
4. **Splicer** replaces Frame segments with generated code
5. **Backend** re-indents native code to match target context

### 3.4 Native Region Scanner (`native_region_scanner.rs`)

Scans handler bodies for Frame segments:

```rust
pub enum RegionV3 {
    Native(RegionSpan),           // Pass-through native code
    Transition(RegionSpan),       // -> $State
    Forward(RegionSpan),          // >> or >>^
    StackPush(RegionSpan),        // $$[+]
    StackPop(RegionSpan),         // $$[-]
    ChangeState(RegionSpan),      // ->> $State
}

pub trait NativeRegionScannerV3 {
    fn scan(&mut self, source: &[u8]) -> Vec<RegionV3>;
}
```

Language-specific scanners handle:
- String literals (don't scan inside strings)
- Comments (don't scan inside comments)
- Raw strings (Python `r""`, Rust `r#""#`)

---

## 4. Code Generation Pipeline

### 4.1 System Codegen (`system_codegen.rs`)

Transforms `SystemAst` → `CodegenNode::Class`:

```rust
pub fn generate_system(
    system: &SystemAst,
    arcanum: &Arcanum,
    lang: TargetLanguage,
    source: &[u8]
) -> CodegenNode {
    // 1. Generate state fields
    let fields = vec![
        Field::new("_state", "String"),
        Field::new("_state_stack", "Vec"),
        Field::new("_state_context", "Dict"),
    ];

    // 2. Generate infrastructure methods
    let mut methods = vec![
        generate_constructor(system, lang),
        generate_transition_method(lang),
        generate_change_state_method(lang),
        generate_dispatch_event_method(system, lang),
        generate_enter_method(system, arcanum),
        generate_exit_method(system, arcanum),
    ];

    // 3. Generate interface methods
    for iface in &system.interface {
        methods.push(generate_interface_method(iface));
    }

    // 4. Generate handler methods (extract native code from source)
    for state in &system.machine.states {
        for handler in &state.handlers {
            methods.push(generate_handler_method(state, handler, source, lang));
        }
    }

    // 5. Generate actions and operations
    for action in &system.actions {
        methods.push(generate_action(action, source));
    }

    CodegenNode::Class { name: system.name.clone(), fields, methods, ... }
}
```

### 4.2 Handler Body Processing

```rust
fn generate_handler_method(
    state: &StateAst,
    handler: &HandlerAst,
    source: &[u8],
    lang: TargetLanguage
) -> CodegenNode {
    // 1. Extract raw body from source using span
    let body_bytes = &source[handler.body.span.start..handler.body.span.end];

    // 2. Scan for Frame segments
    let mut scanner = get_native_scanner(lang);
    let regions = scanner.scan(body_bytes);

    // 3. Build spliced body: native + expanded Frame
    let mut body_nodes = vec![];
    for region in regions {
        match region {
            RegionV3::Native(span) => {
                body_nodes.push(CodegenNode::NativeBlock {
                    code: extract_text(body_bytes, span),
                    span: Some(span),
                });
            }
            RegionV3::Transition(span) => {
                let target = parse_transition_target(body_bytes, span);
                body_nodes.push(CodegenNode::Transition {
                    target_state: target,
                    exit_args: vec![],
                    enter_args: vec![],
                });
            }
            // ... handle other Frame segments
        }
    }

    CodegenNode::Method {
        name: format!("_s_{}_{}", state.name, handler.event),
        params: handler.params.clone(),
        body: body_nodes,
        visibility: Visibility::Private,
        ...
    }
}
```

### 4.3 Backend Emission

Each backend implements `emit()` for all `CodegenNode` variants:

```rust
// Python Backend
impl LanguageBackend for PythonBackend {
    fn emit(&self, node: &CodegenNode, ctx: &mut EmitContext) -> String {
        match node {
            CodegenNode::Transition { target_state, .. } => {
                format!("{}self._transition(\"{}\")", ctx.get_indent(), target_state)
            }

            CodegenNode::Method { name, params, body, .. } => {
                let mut result = format!("{}def {}(self, {}):\n",
                    ctx.get_indent(), name, self.emit_params(params));
                ctx.push_indent();
                for stmt in body {
                    result.push_str(&self.emit(stmt, ctx));
                    result.push('\n');
                }
                ctx.pop_indent();
                result
            }

            CodegenNode::NativeBlock { code, .. } => {
                // Re-indent native code to current context
                self.reindent_native(code, ctx)
            }

            // ... 50+ other node types
        }
    }
}
```

---

## 5. Target Language Support

### 5.1 PRT Languages (Priority)

| Language | Status | Backend File |
|----------|--------|--------------|
| Python 3 | Production | `backends/python.rs` |
| Rust | Active Development | `backends/rust.rs` |
| TypeScript | Active Development | `backends/typescript.rs` |

### 5.2 Other Languages

| Language | Status | Notes |
|----------|--------|-------|
| C# | Partial | Basic generation works |
| Java | Partial | Basic generation works |
| C | Experimental | Manual memory management |
| C++ | Experimental | |

### 5.3 Language-Specific Considerations

**Python:**
- Uses `self` for instance access
- Dynamic dispatch via `getattr()`
- `None` for null, `True`/`False` for booleans

**Rust:**
- Uses `self`/`&mut self` for instance access
- Requires explicit `String::from()` for string literals
- Match-based dispatch (no dynamic method lookup)
- Ownership/borrowing affects method signatures

**TypeScript:**
- Uses `this` for instance access
- Dynamic dispatch via `(this as any)[handler]()`
- `null`/`undefined` handling
- Interface generation for type safety

---

## 6. Current Status & Roadmap

### 6.1 What Works (V4)

- Frame parser for V4 syntax (`@@system`, `@@module`, `@@target`)
- Arcanum symbol table construction
- Basic validation (E001, E402, E403, E405)
- Python codegen - 60% of V3 tests passing
- Rust codegen - syntax generation works, test harness issues
- Native code preservation with proper indentation
- HSM parent forwarding
- State stack push/pop

### 6.2 Known Issues

1. **Test Harness** - Docker test runner expects executable main(), V4 generates libraries
2. **TypeScript** - Missing `tsc` in some environments
3. **Rust Dispatch** - Dynamic dispatch pattern needs work
4. **Error Messages** - Need source location info

### 6.3 Roadmap

**Phase 1: PRT Completion** (Current)
- Fix remaining Python codegen issues
- Complete Rust backend (proper dispatch)
- Fix TypeScript backend
- Achieve 90%+ test pass rate for all PRT languages

**Phase 2: V3 Sunset**
- Remove V3 pipeline code
- Migrate all tests to V4
- Single unified codebase

**Phase 3: New Features**
- `@@persist` - State persistence backends
- `@@async` - Async/await support
- Enhanced HSM features
- IDE integration (LSP)

---

## Appendix A: File Structure

```
framec/src/frame_c/
├── v4/
│   ├── frame_parser.rs       # V4 Frame syntax parser
│   ├── frame_ast.rs          # Frame AST definitions
│   ├── frame_validator.rs    # Semantic validation
│   ├── arcanum.rs            # Symbol table
│   ├── native_region_scanner.rs  # Native code scanning
│   ├── codegen/
│   │   ├── mod.rs            # Codegen entry point
│   │   ├── ast.rs            # CodegenNode definitions
│   │   ├── backend.rs        # LanguageBackend trait
│   │   ├── system_codegen.rs # System → CodegenNode transform
│   │   └── backends/
│   │       ├── python.rs
│   │       ├── rust.rs
│   │       ├── typescript.rs
│   │       └── ...
│   └── pipeline/
│       ├── config.rs         # Pipeline configuration
│       ├── compiler.rs       # Main compilation logic
│       └── traits.rs         # Pipeline traits
└── cli.rs                    # Command-line interface
```

---

## Appendix B: Example Transpilation

**Input (`counter.frm`):**
```frame
@@target python_3

@@system Counter {
    interface:
        increment()
        getCount(): int

    machine:
        $Counting {
            increment() {
                self.count += 1
                if self.count >= 10:
                    -> $Full
            }
        }

        $Full {
            $>() {
                print("Counter is full!")
            }
        }

    domain:
        var count = 0
}
```

**Output (`counter.py`):**
```python
from typing import Any, Optional, List, Dict, Callable

class Counter:
    def __init__(self):
        self._state_stack = []
        self._state_context = {}
        self.count = 0
        self._state = "Counting"
        self._enter()

    def _transition(self, target_state, exit_args=None, enter_args=None):
        self._exit()
        self._state = target_state
        self._enter()

    def _change_state(self, target_state):
        self._state = target_state

    def _dispatch_event(self, event, *args):
        handler_name = f"_s_{self._state}_{event}"
        handler = getattr(self, handler_name, None)
        if handler:
            return handler(*args)

    def _enter(self):
        handler_name = f"_s_{self._state}_enter"
        handler = getattr(self, handler_name, None)
        if handler:
            handler()

    def _exit(self):
        handler_name = f"_s_{self._state}_exit"
        handler = getattr(self, handler_name, None)
        if handler:
            handler()

    def increment(self):
        self._dispatch_event("increment")

    def getCount(self) -> int:
        return self.count

    def _s_Counting_increment(self):
        self.count += 1
        if self.count >= 10:
            self._transition("Full")

    def _s_Full_enter(self):
        print("Counter is full!")
```

---

*Document generated for Frame V4 technical assessment.*
