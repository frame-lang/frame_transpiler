# Frame V4 Two-Pass Semantic Architecture

## Document Purpose

This document describes the planned architecture for Frame V4's semantic validation system. It serves as the authoritative design reference for implementing the two-pass model.

**Related Documents:**
- `COMPILER_ARCHITECTURE.md` - Overall compiler pipeline
- `VALIDATION_ARCHITECTURE.md` - Validation error codes and phases
- Plan file: `~/.claude/plans/smooth-scribbling-whisper.md`

---

## Executive Summary

Frame V4 adopts a **two-pass semantic validation model**:

| Pass | When | What | Tool |
|------|------|------|------|
| **Pass 1** | Transpile-time | Frame semantics | Frame compiler |
| **Pass 2** | Compile/Run-time | Native semantics | Native compiler (pyc/tsc/rustc) |

**Core Principle**: Frame validates what only Frame knows; native compilers validate the rest.

---

## Architecture Overview

```
                    ┌─────────────────────────────────────┐
                    │         PASS 1: TRANSPILE           │
                    │         Frame Validation            │
                    └─────────────────────────────────────┘
                                    │
    ┌───────────────────────────────┼───────────────────────────────┐
    │                               │                               │
    ▼                               ▼                               ▼
┌─────────┐                   ┌───────────┐                   ┌───────────┐
│ PARSER  │                   │  ARCANUM  │                   │ VALIDATOR │
│         │ ──── AST ────▶   │  (Symbol  │ ◀── lookups ───  │           │
│         │                   │   Table)  │                   │           │
└─────────┘                   └───────────┘                   └───────────┘
    │                               │                               │
    │                               │                               │
    └───────────────────────────────┼───────────────────────────────┘
                                    │
                                    ▼
                            ┌───────────────┐
                            │    CODEGEN    │
                            │   (Splicer)   │
                            └───────┬───────┘
                                    │
                                    ▼
                            ┌───────────────┐
                            │   Generated   │
                            │     Code      │
                            └───────┬───────┘
                                    │
                    ┌───────────────────────────────────┐
                    │         PASS 2: COMPILE           │
                    │       Native Validation           │
                    └───────────────────────────────────┘
                                    │
                                    ▼
                            ┌───────────────┐
                            │    Native     │
                            │   Compiler    │
                            │ (pyc/tsc/etc) │
                            └───────────────┘
```

---

## Data Structures

### 1. Frame AST (`frame_ast.rs`)

The Frame AST represents **Frame constructs only**. Native code is referenced by span, not parsed or stored.

```rust
/// Root AST node
pub enum FrameAst {
    System(SystemAst),
    Module(ModuleAst),
}

/// System definition
pub struct SystemAst {
    pub name: String,
    pub interface: Vec<InterfaceMethod>,
    pub machine: Option<MachineAst>,
    pub actions: Vec<ActionAst>,
    pub operations: Vec<OperationAst>,
    pub domain: Vec<DomainVar>,
    pub span: Span,
}

/// State definition
pub struct StateAst {
    pub name: String,
    pub params: Vec<StateParam>,
    pub parent: Option<String>,
    pub handlers: Vec<HandlerAst>,
    pub enter: Option<EnterHandler>,
    pub exit: Option<ExitHandler>,
    pub span: Span,
}

/// Handler body - Frame statements only
pub struct HandlerBody {
    pub statements: Vec<Statement>,  // Frame statements only
    pub span: Span,                  // Span into source (for splicer)
}

/// Frame statements that require validation
pub enum Statement {
    Transition(TransitionAst),       // -> $State
    Forward(ForwardAst),             // -> $$
    StackPush(StackPushAst),         // $$[+]
    StackPop(StackPopAst),           // -> $$[-]
    Return(ReturnAst),
    Continue(ContinueAst),
    // NO NativeBlock - native code handled by splicer
}

/// State parameter with type
pub struct StateParam {
    pub name: String,
    pub param_type: Option<Type>,
    pub span: Span,
}

/// Event/handler parameter with type
pub struct EventParam {
    pub name: String,
    pub param_type: Option<Type>,
    pub span: Span,
}
```

**Key Design Decision**: Native code is NOT stored in the AST. The `HandlerBody.span` points to the source bytes, and the splicer extracts native code during codegen.

### 2. Arcanum with Frame Scopes (`arcanum.rs`)

The Arcanum is Frame's symbol table, enhanced with scope tracking for Frame-declared symbols.

```rust
/// Frame's symbol table
pub struct Arcanum {
    pub systems: HashMap<String, SystemEntry>,
}

/// System-level symbols
pub struct SystemEntry {
    pub interface_methods: HashSet<String>,
    pub actions: HashSet<String>,
    pub operations: HashSet<String>,
    pub domain_vars: HashMap<String, FrameSymbol>,
    pub machines: HashMap<String, MachineEntry>,
}

/// Machine containing states
pub struct MachineEntry {
    pub states: HashMap<String, StateEntry>,
}

/// State-level symbols
pub struct StateEntry {
    pub name: String,
    pub params: Vec<FrameSymbol>,              // State parameters
    pub parent: Option<String>,
    pub handlers: HashMap<String, HandlerEntry>,
    pub span: Span,
}

/// Handler-level symbols
pub struct HandlerEntry {
    pub event: String,
    pub params: Vec<FrameSymbol>,              // Handler parameters
    pub span: Span,
}

/// A Frame-declared symbol
pub struct FrameSymbol {
    pub name: String,
    pub kind: FrameSymbolKind,
    pub declared_at: Span,
    pub symbol_type: Option<Type>,
}

pub enum FrameSymbolKind {
    StateParam,
    HandlerParam,
    DomainVar,
}
```

**Scope Hierarchy**:
```
System Scope
├── domain variables
├── interface methods (names only)
├── actions (names only)
├── operations (names only)
└── State Scope (per state)
    ├── state parameters
    └── Handler Scope (per handler)
        └── handler parameters
```

**Scope Resolution API**:
```rust
impl Arcanum {
    /// Resolve a symbol in Frame scope chain
    /// Returns None if not a Frame symbol (might be native)
    pub fn resolve_frame_symbol(
        &self,
        system: &str,
        state: Option<&str>,
        handler: Option<&str>,
        name: &str,
    ) -> Option<&FrameSymbol>;

    /// Check if name is a Frame-declared symbol
    pub fn is_frame_symbol(&self, ...) -> bool;

    /// Validate transition target exists
    pub fn validate_transition(&self, system: &str, target: &str) -> Result<(), String>;

    /// Get state parameter count for arity checking
    pub fn get_state_param_count(&self, system: &str, state: &str) -> Option<usize>;
}
```

### 3. Native Region Scanner (`native_region_scanner/`)

Identifies Frame "islands" within native code "oceans".

```rust
/// Result of scanning a handler body
pub struct ScanResultV3 {
    pub close_byte: usize,
    pub regions: Vec<RegionV3>,
}

/// A region within a handler body
pub enum RegionV3 {
    /// Native code - preserved verbatim
    NativeText { span: RegionSpan },

    /// Frame statement - needs expansion
    FrameSegment {
        span: RegionSpan,
        kind: FrameSegmentKindV3,
        indent: usize,
    },
}

/// Types of Frame segments
pub enum FrameSegmentKindV3 {
    Transition,   // -> $State
    Forward,      // -> $$
    StackPush,    // $$[+]
    StackPop,     // -> $$[-]
}
```

### 4. Splicer (`splice.rs`)

Combines native code with generated Frame expansions.

```rust
/// Result of splicing
pub struct SplicedBodyV3 {
    pub text: String,
    pub splice_map: Vec<(RegionSpan, OriginV3)>,
}

/// Origin of each region in spliced output
pub enum OriginV3 {
    Native { source: RegionSpan },
    Frame { source: RegionSpan },
}

impl SplicerV3 {
    /// Splice native regions with Frame expansions
    pub fn splice(
        &self,
        bytes: &[u8],
        regions: &[RegionV3],
        expansions: &[String],
    ) -> SplicedBodyV3;
}
```

---

## Validation Rules

### Frame Semantic Validation (Pass 1)

| Code | Rule | Arcanum Query |
|------|------|---------------|
| E113 | Section ordering | AST structure check |
| E114 | No duplicate sections | AST structure check |
| E400 | Terminal statement must be last | Statement position check |
| E401 | No Frame statements in actions/operations | Body scan |
| E402 | Transition target state must exist | `arcanum.validate_transition()` |
| E403 | Forward requires parent state | `arcanum.has_parent()` |
| E405 | Transition argument count must match state params | `arcanum.get_state_param_count()` |
| E406 | Interface method must exist for system.method calls | `arcanum.is_interface_method()` |

### Native Validation (Pass 2)

Delegated to native compilers:
- Variable existence
- Type compatibility
- Import resolution
- Function signatures
- Control flow

---

## Codegen: Splicer Model

The splicer preserves native code exactly while replacing Frame segments:

```
Handler Body Source:
┌─────────────────────────────────────────────┐
│ {                                           │
│     x = compute_something()                 │  ← Native (preserved)
│     if x > threshold:                       │  ← Native (preserved)
│         -> $Exceeded(x)                     │  ← Frame (replaced)
│     else:                                   │  ← Native (preserved)
│         -> $Normal                          │  ← Frame (replaced)
│ }                                           │
└─────────────────────────────────────────────┘
                    │
                    ▼ Scanner + Splicer
                    │
┌─────────────────────────────────────────────┐
│     x = compute_something()                 │
│     if x > threshold:                       │
│         self._transition(self._s_Exceeded, x)│
│     else:                                   │
│         self._transition(self._s_Normal)   │
└─────────────────────────────────────────────┘
```

**Splicer Algorithm**:
1. Scan handler body for Frame segments (using native_region_scanner)
2. Generate expansion code for each Frame segment
3. Build output: NativeText regions copied verbatim, FrameSegment regions replaced with expansions
4. Preserve indentation from `indent` field

---

## Implementation Phases

### Phase 1: Enhanced Arcanum ← NEXT

**Goal**: Add scope hierarchy for Frame symbol tracking

**Files**:
- `framec/src/frame_c/v4/arcanum.rs`
- `framec/src/frame_c/v4/arcanum_tests.rs`

**Tasks**:
1. Add `FrameSymbol`, `HandlerEntry` structs
2. Enhance `StateEntry` with params as `Vec<FrameSymbol>`
3. Implement `resolve_frame_symbol()` with scope chain lookup
4. Update `build_arcanum_from_frame_ast()` to populate all scopes
5. Add comprehensive tests

### Phase 2: Complete Frame Validator

**Goal**: Implement all Frame semantic validations

**Files**:
- `framec/src/frame_c/v4/frame_validator.rs`

**Tasks**:
1. Wire validator to use enhanced Arcanum
2. Implement E402 with helpful "did you mean" suggestions
3. Implement E403 parent validation
4. Implement E405 arity checking
5. Implement E406 interface method validation

### Phase 3: Splicer Integration

**Goal**: Ensure codegen uses splicer for all handler bodies

**Files**:
- `framec/src/frame_c/v4/codegen/system_codegen.rs`

**Tasks**:
1. Verify splicer handles all Frame segment types
2. Test with nested native code (if/while/for)
3. Verify indentation preservation
4. Test all target languages

### Phase 4: Source Maps

**Goal**: Map generated code back to Frame source

**Files**:
- `framec/src/frame_c/v4/codegen/source_map.rs` (new)

**Tasks**:
1. Design source map format
2. Generate mappings during splicer operation
3. Emit source map files
4. Test IDE integration

---

## Benefits

1. **Simplicity**: No multi-language type checker needed
2. **Correctness**: All errors caught (Frame at transpile, native at compile)
3. **Maintainability**: Native compilers maintained by their communities
4. **Performance**: No native parsing overhead
5. **Reliability**: Leverage battle-tested native compilers

---

## Comparison: Full Native Parsing vs Two-Pass

| Aspect | Full Native Parsing | Two-Pass Model |
|--------|---------------------|----------------|
| Implementation | 12-18 weeks | 2-4 weeks |
| Native parsers | Need 7 (Py, TS, Rust, C, C++, C#, Java) | None |
| Type checker | Build multi-language | None |
| Error timing | All at transpile | Frame at transpile, native at compile |
| Error location | All in .frm | Frame in .frm, native in generated |
| Maintenance | Track all language specs | Zero |
| Correctness | Same | Same |

---

## Open Questions

1. **Source map format**: Use standard format (like JS source maps) or custom?
2. **Error aggregation**: How many errors to show before stopping?
3. **IDE integration**: LSP support for real-time validation?
4. **Cross-system validation**: How to handle multi-file projects?

---

## References

- [V4 Compiler Architecture](COMPILER_ARCHITECTURE.md)
- [Validation Architecture](VALIDATION_ARCHITECTURE.md)
- [Frame Grammar](../grammar.md)
