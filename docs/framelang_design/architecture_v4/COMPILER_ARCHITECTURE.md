# Frame V4 Compiler Architecture - Two-Pass Semantic Model

## Executive Summary

Frame V4 implements a **two-pass semantic validation architecture** that leverages the strengths of both Frame and native language compilers:

- **Pass 1 (Transpile-time)**: Frame validates Frame-specific semantics
- **Pass 2 (Compile-time)**: Native compiler validates native code semantics

This approach is cleaner, simpler, and more maintainable than attempting to build a full multi-language type checker within Frame.

## Core Design Principle

**Frame validates what only Frame knows; native compilers validate the rest.**

Frame constructs have semantics that only the Frame compiler understands:
- State machine topology (which states exist, parent relationships)
- Transition validity (target state exists, parameter arity matches)
- Handler structure (terminal statements must be last)
- Interface/action/operation declarations

Native code semantics (variable types, import resolution, function signatures) are validated by the target language's compiler - which is already battle-tested and maintained.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    PASS 1: Frame Validation                     │
│                     (At Transpile Time)                         │
└─────────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────────────┐
│   Parser    │ ──▶ │ Frame AST + │ ──▶ │  Frame Semantic     │
│             │     │   Arcanum   │     │     Validator       │
└─────────────┘     └─────────────┘     └──────────┬──────────┘
                                                   │
         Validates:                                │
         ✓ E402: State exists                      │
         ✓ E403: Parent exists for forward         │
         ✓ E405: State parameter arity             │
         ✓ E400: Terminal statement last           │
         ✓ E406: Interface method exists           │
         ✓ E401: No Frame in actions/operations    │
         ✓ E113: Section ordering                  │
         ✓ E114: No duplicate sections             │
                                                   │
                                        ┌──────────▼──────────┐
                                        │      Codegen        │
                                        │   (Splicer Model)   │
                                        └──────────┬──────────┘
                                                   │
                                                   ▼
                                        ┌─────────────────────┐
                                        │   Generated Code    │
                                        │  (Python/TS/etc.)   │
                                        └──────────┬──────────┘
                                                   │
┌─────────────────────────────────────────────────────────────────┐
│                    PASS 2: Native Validation                    │
│                     (At Compile/Run Time)                       │
└─────────────────────────────────────────────────────────────────┘
                                                   │
                                                   ▼
                                        ┌─────────────────────┐
                                        │   Native Compiler   │
                                        │  (pyc/tsc/rustc)    │
                                        └──────────┬──────────┘
                                                   │
         Validates:                                │
         ✓ Variable exists in scope                │
         ✓ Type compatibility                      │
         ✓ Import resolution                       │
         ✓ Native syntax correctness               │
         ✓ Flow analysis                           │
                                                   │
                                                   ▼
                                              [Executable]
```

## The Oceans Model

Frame uses the "oceans model" for mixed Frame/native code:

- **Native code is the ocean** - Preserved exactly as written
- **Frame constructs are islands** - Identified, validated, and replaced with generated code
- **Splicer combines them** - Native regions + Frame expansions → final output

```frame
$Processing(timeout: int) {
    tick(delta: int) {
        # Native Python (ocean) - Frame doesn't parse this
        remaining = timeout - delta
        if remaining <= 0:
            # Frame statement (island) - Frame validates this
            -> $Expired(remaining)
    }
}
```

Frame validates:
- `$Expired` state exists
- `$Expired` takes 1 parameter
- Transition is a valid terminal statement

Native compiler validates:
- `remaining` is defined
- `timeout` and `delta` exist (they're handler params)
- Subtraction is valid

## Data Structures

### Frame AST (frame_ast.rs)

The Frame AST represents Frame constructs only. Native code is referenced by span, not stored.

```rust
pub struct SystemAst {
    pub name: String,
    pub interface: Vec<InterfaceMethod>,
    pub machine: Option<MachineAst>,
    pub actions: Vec<ActionAst>,
    pub operations: Vec<OperationAst>,
    pub domain: Vec<DomainVar>,
    pub span: Span,
}

pub struct StateAst {
    pub name: String,
    pub params: Vec<StateParam>,
    pub parent: Option<String>,
    pub handlers: Vec<HandlerAst>,
    pub span: Span,
}

pub struct HandlerBody {
    pub statements: Vec<Statement>,  // Frame statements only
    pub span: Span,                  // Points to source bytes (for splicer)
}

/// Frame statements - things Frame needs to validate
pub enum Statement {
    Transition(TransitionAst),
    Forward(ForwardAst),
    StackPush(StackPushAst),
    StackPop(StackPopAst),
    Return(ReturnAst),
    // Note: No NativeBlock - native code is handled by splicer
}
```

### Arcanum with Frame Scopes (arcanum.rs)

The Arcanum is Frame's symbol table, enhanced with scope tracking for Frame-declared symbols.

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

pub struct MachineEntry {
    pub states: HashMap<String, StateEntry>,
}

pub struct StateEntry {
    pub name: String,
    pub params: Vec<FrameSymbol>,      // State parameters
    pub parent: Option<String>,
    pub handlers: HashMap<String, HandlerEntry>,
}

pub struct HandlerEntry {
    pub event: String,
    pub params: Vec<FrameSymbol>,      // Handler parameters
}

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

### Scope Resolution

Frame scope resolution follows a simple hierarchy:

```
System Scope
├── domain variables
├── interface methods
├── actions
├── operations
└── State Scope (per state)
    ├── state parameters
    └── Handler Scope (per handler)
        └── handler parameters
```

When validating a Frame reference (e.g., transition argument):
1. Check handler scope (handler params)
2. Check state scope (state params)
3. Check system scope (domain vars)
4. If not found → **don't error** (might be native variable, let native compiler check)

This is the key insight: Frame validates Frame symbols, and **trusts** that undefined symbols are native (the native compiler will catch actual errors).

## Compilation Pipeline

### Phase 1: Parsing

```rust
let mut parser = FrameParser::new(source, target_language);
let ast = parser.parse_module()?;  // Returns FrameAst
```

The parser:
- Identifies Frame constructs (systems, states, handlers)
- Stores Frame statements (transitions, forwards, etc.)
- Records spans for native regions (doesn't parse native code)

### Phase 2: Symbol Table Construction

```rust
let arcanum = build_arcanum_from_frame_ast(&ast);
```

Builds the Arcanum by traversing the Frame AST:
- Collects all state declarations with parameters
- Collects all handler declarations with parameters
- Collects interface methods, actions, operations
- Collects domain variables

### Phase 3: Frame Semantic Validation

```rust
let mut validator = FrameValidator::new();
validator.validate(&ast, &arcanum)?;
```

Validates Frame-specific semantics:

| Error | Description | Validation |
|-------|-------------|------------|
| E402 | Unknown state in transition | Check Arcanum for target state |
| E403 | Forward without parent | Check state has parent in Arcanum |
| E405 | Parameter arity mismatch | Compare arg count to state param count |
| E400 | Terminal not last | Check statement position in handler |
| E401 | Frame in actions/ops | Scan action/operation bodies |
| E406 | Unknown interface method | Check Arcanum interface_methods |
| E113 | Section ordering | Check AST section order |
| E114 | Duplicate sections | Check AST section counts |

### Phase 4: Code Generation (Splicer Model)

```rust
let backend = get_backend(target_language);
let output = backend.emit(&ast, source_bytes, &arcanum);
```

Code generation uses the splicer pattern:
1. Scan handler body for Frame segments (using native_region_scanner)
2. Generate code for each Frame segment
3. Splice: preserve native regions, replace Frame segments with generated code

```rust
fn generate_handler_body(body: &HandlerBody, source: &[u8], lang: TargetLanguage) -> String {
    let body_bytes = &source[body.span.start..body.span.end];

    // Scan for Frame segments
    let mut scanner = get_native_scanner(lang);
    let scan_result = scanner.scan(body_bytes, 0)?;

    // Generate expansions for Frame segments
    let mut expansions = Vec::new();
    for region in &scan_result.regions {
        if let RegionV3::FrameSegment { span, kind, indent } = region {
            let expansion = generate_frame_expansion(kind, span, indent, lang);
            expansions.push(expansion);
        }
    }

    // Splice native + Frame
    let splicer = SplicerV3;
    splicer.splice(body_bytes, &scan_result.regions, &expansions).text
}
```

### Phase 5: Native Compilation (External)

The generated code is compiled/interpreted by the target language's toolchain:

```bash
# Python
python3 generated_code.py

# TypeScript
tsc generated_code.ts && node generated_code.js

# Rust
rustc generated_code.rs && ./generated_code
```

Native errors point to the generated code. Source maps can map these back to the original Frame source.

## What Frame Validates vs. What Native Validates

| Semantic Check | Validated By | When |
|---------------|--------------|------|
| State exists | Frame | Transpile |
| Parent exists | Frame | Transpile |
| Parameter arity | Frame | Transpile |
| Terminal last | Frame | Transpile |
| Section order | Frame | Transpile |
| Variable exists | Native | Compile/Run |
| Type compatibility | Native | Compile/Run |
| Import resolution | Native | Compile/Run |
| Function signatures | Native | Compile/Run |

## Benefits of This Architecture

### 1. Simplicity
- No need to build type checkers for 7 languages
- No need to track native language spec changes
- Clear separation of concerns

### 2. Correctness
- Frame semantics are fully validated
- Native semantics are validated by proven compilers
- All errors are caught (just at different times)

### 3. Maintainability
- Frame compiler focuses on Frame concerns
- Native compilers maintained by their communities
- Less code to write and maintain

### 4. Performance
- No native parsing overhead at transpile time
- Single-pass Frame parsing
- Fast splicer-based code generation

## Comparison with Full Native Parsing

| Aspect | Full Native Parsing | Two-Pass Model |
|--------|--------------------|-----------------|
| Implementation effort | 12-18 weeks | 2-4 weeks |
| Native parsers needed | All 7 languages | None |
| Type checker needed | Yes, multi-language | No |
| Maintenance burden | High | Low |
| Error timing | All at transpile | Frame at transpile, native at compile |
| Error location | All in .frm | Frame in .frm, native in generated |
| Correctness | Same | Same |

## Future Enhancements

### Source Maps

Generate source maps to translate native compiler errors back to Frame source locations:
- Map generated code positions to Frame source spans
- IDE integration can show native errors in Frame source
- Debugger can step through Frame source

### Optional Native Declaration Extraction

For IDE support (autocomplete, hover), optionally parse native code for declarations:
- Extract variable assignments
- Extract function definitions
- Don't validate, just inform

This would be opt-in and non-blocking (errors in extraction don't fail compilation).

## Conclusion

The two-pass semantic model provides:
- **Complete validation** of Frame semantics at transpile time
- **Leverage** of native compilers for native validation
- **Simplicity** in implementation and maintenance
- **Correctness** equivalent to full parsing (errors caught, just at different times)

Frame focuses on what makes Frame unique - state machine semantics - and trusts native compilers to do what they do best.
