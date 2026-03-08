> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame V4 Compiler Architecture

This document describes the Frame V4 compiler implementation.

---

## Overview

Frame V4 is a **preprocessor** for state machine code. It:

1. **Parses** Frame syntax (`@@system`, states, transitions)
2. **Validates** Frame semantics (state exists, parameters match)
3. **Generates** target language code (Python, Rust, TypeScript)
4. **Preserves** native code exactly as written

Frame does NOT parse or validate native code. That's the target compiler's job.

---

## The Oceans Model

Frame uses the "oceans model" for mixed Frame/native code:

- **Native code is the ocean** — Preserved exactly as written
- **Frame constructs are islands** — Identified, validated, and expanded

```
Handler Body:
┌─────────────────────────────────────────────┐
│ x = compute_value()        ← Ocean (native) │
│ if x > threshold:          ← Ocean (native) │
│     -> $Exceeded           ← Island (Frame) │
│ else:                      ← Ocean (native) │
│     -> $Normal             ← Island (Frame) │
└─────────────────────────────────────────────┘
```

The Native Region Scanner finds islands. The Splicer replaces them with generated code. Everything else passes through unchanged.

---

## Two-Pass Validation Model

Frame V4 uses a two-pass validation architecture:

| Pass | When | What | Who |
|------|------|------|-----|
| **Pass 1** | Transpile-time | Frame semantics | Frame compiler |
| **Pass 2** | Compile/Run-time | Native semantics | Target compiler |

**Frame validates what only Frame knows:**
- State exists (`-> $Unknown` → E402)
- Parent exists for forward (`=> $^` without parent → E403)
- Parameter arity matches (`-> $State(a,b)` when State takes 1 param → E405)
- Terminal statements are last (code after `->` → E400)
- Sections are ordered correctly (E113)

**Native compiler validates the rest:**
- Variables exist
- Types are compatible
- Imports resolve
- Syntax is correct

---

## Compilation Pipeline

```
Source (.frm)
     │
     ▼
┌─────────────┐
│ Frame Parser │ ──→ Frame AST (systems, states, handlers)
└─────────────┘      Native code stored as spans, not parsed
     │
     ▼
┌─────────────┐
│   Arcanum   │ ──→ Symbol table (states, events, domain vars)
└─────────────┘
     │
     ▼
┌─────────────┐
│  Validator  │ ──→ Frame semantic errors (E4xx)
└─────────────┘
     │
     ▼
┌─────────────┐
│   Codegen   │ ──→ CodegenNode (language-agnostic IR)
└─────────────┘
     │
     ▼
┌─────────────┐
│   Backend   │ ──→ Target code (Python/Rust/TypeScript)
└─────────────┘
     │
     ▼
Target (.py/.rs/.ts)
```

---

## Core Components

### 1. Frame Parser (`frame_parser.rs`)

Parses Frame constructs into AST. Does NOT parse native code.

```rust
pub struct FrameParser {
    source: Vec<u8>,
    cursor: usize,
    target: TargetLanguage,
}
```

The parser:
- Identifies Frame constructs (systems, states, handlers)
- Stores Frame statements (transitions, forwards, etc.)
- Records spans for native regions (doesn't parse native code)

### 2. Arcanum — Symbol Table (`arcanum.rs`)

Tracks Frame-declared symbols for validation.

```rust
pub struct Arcanum {
    pub systems: HashMap<String, SystemEntry>,
}

pub struct SystemEntry {
    pub interface_methods: HashSet<String>,
    pub actions: HashSet<String>,
    pub operations: HashSet<String>,
    pub domain_vars: HashMap<String, FrameSymbol>,
    pub machines: HashMap<String, MachineEntry>,
}

pub struct StateEntry {
    pub name: String,
    pub params: Vec<FrameSymbol>,
    pub parent: Option<String>,
    pub handlers: HashMap<String, HandlerEntry>,
}
```

**Scope Hierarchy:**
```
System Scope
├── domain variables
├── interface methods
├── actions / operations
└── State Scope (per state)
    ├── state parameters
    └── Handler Scope (per handler)
        └── handler parameters
```

### 3. Frame Validator (`frame_validator.rs`)

Validates Frame semantics using Arcanum.

```rust
pub struct FrameValidator;

impl FrameValidator {
    pub fn validate(&self, ast: &FrameAst, arcanum: &Arcanum)
        -> Result<(), Vec<ValidationError>>;
}
```

### 4. Native Region Scanner (`native_region_scanner.rs`)

Identifies Frame "islands" within native code "oceans".

```rust
pub enum Region {
    NativeText { span: RegionSpan },     // Preserve
    FrameSegment { span: RegionSpan, kind: FrameSegmentKind },  // Expand
}

pub enum FrameSegmentKind {
    Transition,   // -> $State
    Forward,      // => $^
    StackPush,    // push$
    StackPop,     // pop$ or -> pop$
}
```

### 5. Splicer (`splice.rs`)

Combines native code with generated Frame expansions.

```rust
impl Splicer {
    pub fn splice(
        source: &[u8],
        regions: &[Region],
        expansions: &[String],
    ) -> SplicedBody;
}
```

**Algorithm:**
1. Scan handler body for Frame segments
2. Generate expansion code for each Frame segment
3. Build output: native regions verbatim, Frame segments replaced

### 6. Language Backends (`codegen/backends/`)

Emit target language code from CodegenNode.

```rust
pub trait LanguageBackend {
    fn emit(&self, node: &CodegenNode, ctx: &mut EmitContext) -> String;
    fn runtime_imports(&self) -> Vec<String>;
}
```

---

## Validation Error Codes

### Structure Errors (E1xx)

| Code | Description |
|------|-------------|
| E113 | Section ordering violation |
| E114 | Duplicate section |

### Frame Statement Errors (E4xx)

| Code | Description |
|------|-------------|
| E400 | Terminal statement must be last in handler |
| E401 | Frame statement not allowed in actions/operations |
| E402 | Unknown state in transition |
| E403 | Forward requires parent state |
| E404 | Duplicate state definition |
| E405 | Parameter arity mismatch |
| E406 | Invalid interface method call |

### Error Message Format

```
[ERROR_CODE] Message
  --> file.frm:line:column
   |
line | source code
   | ^^^ specific location
   |
   = help: Suggestion for fixing
```

---

## Code Generation

### Frame Statement Expansion

| Frame Statement | Python Expansion |
|-----------------|------------------|
| `-> $Green` | `self._transition("Green")` |
| `=> $^` | `self._forward_to_parent()` |
| `push$` | `self._state_stack.append(self._state)` |
| `pop$` | `self._state = self._state_stack.pop()` |
| `-> pop$` | `self._transition(self._state_stack.pop())` |

### Generated System Structure

**Python:**
```python
class SystemName:
    def __init__(self):
        self._state = "InitialState"
        self._state_stack = []
        # domain variables

    def _transition(self, target_state, ...):
        self._exit()
        self._state = target_state
        self._enter()

    # Interface methods dispatch to handlers
    # Handler methods contain spliced code
```

**TypeScript:**
```typescript
class SystemName {
    private _state: string;
    private _state_stack: string[];

    constructor() {
        this._state = "InitialState";
        this._state_stack = [];
    }

    private _transition(targetState: string, ...): void {
        this._exit();
        this._state = targetState;
        this._enter();
    }
}
```

---

## Native Build Integration

Frame integrates into native build toolchains as a preprocessor:

### Python
```toml
# pyproject.toml
[tool.frame]
source = "src/**/*.frm"
output = "src/generated"
```

### TypeScript
```json
{
  "scripts": {
    "prebuild": "framec compile src/**/*.frm --out src/generated",
    "build": "tsc"
  }
}
```

### Rust
```rust
// build.rs
use frame_compiler::compile_frame_files;

fn main() {
    compile_frame_files("src/**/*.frm", "src/generated");
}
```

---

## Source Maps

Frame tracks source spans throughout the pipeline, enabling source maps:

- Map generated code positions → Frame source positions
- Enable debugging Frame source in IDE
- Map native compiler errors back to Frame

Source maps are span-based (bookkeeping), not AST-based.

---

## Target Languages

### Priority (PRT)

| Language | Status | Backend |
|----------|--------|---------|
| Python 3 | Active | `backends/python.rs` |
| TypeScript | Active | `backends/typescript.rs` |
| Rust | Active | `backends/rust.rs` |

### Other Languages

| Language | Status |
|----------|--------|
| C# | Partial |
| Java | Partial |
| C | Experimental |
| C++ | Experimental |

---

## Key Design Principles

1. **Frame validates Frame.** Native compilers validate native code.
2. **Preserve native code exactly.** No reformatting, no reordering.
3. **Single pipeline.** No fallbacks, no multiple approaches.
4. **PRT first.** Python, Rust, TypeScript are priority languages.
5. **Source maps via spans.** Track positions, don't parse native.
