# Frame V4 Architecture

**Status:** Active Development
**Model:** Preprocessor with Frame-only Validation

---

## What is Frame V4?

Frame V4 is a **preprocessor** for state machine code. It:

1. **Parses** Frame syntax (@@system, states, transitions)
2. **Validates** Frame semantics (state exists, parameters match)
3. **Generates** target language code (Python, Rust, TypeScript)
4. **Preserves** native code exactly as written

Frame does NOT parse or validate native code. That's the target compiler's job.

---

## The Preprocessor Model

```
┌─────────────────────────────────────────────────────────────────┐
│                     Frame Source (.frm)                          │
│                                                                  │
│   Native code (imports, functions)     ← Preserved verbatim     │
│   @@system TrafficLight {              ← Parsed by Frame        │
│       machine:                                                   │
│           $Red {                                                 │
│               tick() {                                           │
│                   remaining -= 1       ← Native (preserved)     │
│                   if remaining <= 0:   ← Native (preserved)     │
│                       -> $Green        ← Frame (expanded)       │
│               }                                                  │
│           }                                                      │
│   }                                                              │
│   Native code (main, tests)            ← Preserved verbatim     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Target Code (.py/.rs/.ts)                    │
│                                                                  │
│   Native code (imports, functions)     ← Unchanged              │
│   class TrafficLight:                  ← Generated              │
│       def _s_Red_tick(self):                                    │
│           remaining -= 1               ← Native (unchanged)     │
│           if remaining <= 0:           ← Native (unchanged)     │
│               self._transition("Green") ← Generated            │
│   Native code (main, tests)            ← Unchanged              │
└─────────────────────────────────────────────────────────────────┘
```

---

## Two-Pass Validation

| Pass | When | What | Who |
|------|------|------|-----|
| **Pass 1** | Transpile-time | Frame semantics | Frame compiler |
| **Pass 2** | Compile/Run-time | Native semantics | Target compiler |

**Frame validates what only Frame knows:**
- State exists (`-> $Unknown` → E402)
- Parent exists for forward (`>>` without parent → E442)
- Parameter arity matches (`-> $State(a,b)` when State takes 1 param → E405)
- Terminal statements are last (code after `->` → E431)
- Sections are ordered correctly

**Native compiler validates the rest:**
- Variables exist
- Types are compatible
- Imports resolve
- Syntax is correct

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

// Parses Frame syntax only
// Native code is captured by span, not parsed
```

### 2. Arcanum - Symbol Table (`arcanum.rs`)

Tracks Frame-declared symbols for validation.

```rust
pub struct Arcanum {
    pub systems: HashMap<String, SystemInfo>,
}

pub struct SystemInfo {
    pub states: HashMap<String, StateInfo>,
    pub interface_methods: Vec<String>,
    pub actions: Vec<String>,
    pub operations: Vec<String>,
    pub domain_vars: HashMap<String, VarInfo>,
}
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
pub enum RegionV3 {
    NativeText { span: RegionSpan },     // Preserve
    FrameSegment { span: RegionSpan, kind: FrameSegmentKind },  // Expand
}
```

### 5. Splicer (`splice.rs`)

Combines native code with generated Frame expansions.

```rust
impl Splicer {
    pub fn splice(
        source: &[u8],
        regions: &[RegionV3],
        expansions: &[String],
    ) -> SplicedBody;
}
```

### 6. Language Backends (`codegen/backends/`)

Emit target language code from CodegenNode.

```rust
pub trait LanguageBackend {
    fn emit(&self, node: &CodegenNode, ctx: &mut EmitContext) -> String;
    fn runtime_imports(&self) -> Vec<String>;
}
```

---

## Pipeline

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

## What Frame Does NOT Do

Frame is a preprocessor. It does NOT:

- Parse native code (Python/Rust/TypeScript syntax)
- Validate native code (variable existence, types)
- Build native symbol tables
- Do cross-language type checking
- Understand native imports

These are all delegated to the target language compiler.

**Future (V5):** Optional native code analysis for enhanced IDE support. See `docs/architecture_v5/PLAN.md`.

---

## The Oceans Model

Native code is the "ocean". Frame constructs are "islands".

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

The NativeRegionScanner finds islands. The Splicer replaces them with generated code. Everything else passes through unchanged.

---

## Target Languages

### PRT Languages (Priority)

| Language | Status | Backend |
|----------|--------|---------|
| Python 3 | Active | `backends/python.rs` |
| Rust | Active | `backends/rust.rs` |
| TypeScript | Active | `backends/typescript.rs` |

### Other Languages (Deferred)

| Language | Status |
|----------|--------|
| C# | Partial (not actively maintained) |
| Java | Partial (not actively maintained) |
| C | Experimental |
| C++ | Experimental |

---

## Source Maps

Frame tracks source spans throughout the pipeline, enabling source maps:

- Map generated code positions → Frame source positions
- Enable debugging Frame source in IDE
- Map native compiler errors back to Frame

Source maps are span-based (bookkeeping), not AST-based. The preprocessor model makes this straightforward.

---

## Related Documents

- **[PREPROCESSING_ARCHITECTURE.md](PREPROCESSING_ARCHITECTURE.md)** - Integration with native build systems
- **[TWO_PASS_ARCHITECTURE.md](TWO_PASS_ARCHITECTURE.md)** - Detailed two-pass validation model
- **[COMPILER_ARCHITECTURE.md](COMPILER_ARCHITECTURE.md)** - Compilation pipeline details
- **[VALIDATION_ARCHITECTURE.md](VALIDATION_ARCHITECTURE.md)** - Error codes and validation rules
- **[../../../plans/VALIDATION_EXPANSION_PLAN.md](../../../plans/VALIDATION_EXPANSION_PLAN.md)** - Planned validation improvements
- **[../../architecture_v5/PLAN.md](../../../architecture_v5/PLAN.md)** - Future native compiler integration

---

## Key Principles

1. **Frame validates Frame.** Native compilers validate native code.

2. **Preserve native code exactly.** No reformatting, no reordering.

3. **Single pipeline.** No V3 fallback, no multiple approaches.

4. **PRT first.** Python, Rust, TypeScript are priority languages.

5. **Source maps via spans.** Track positions, don't parse native.

---

*Last updated: February 2026*
