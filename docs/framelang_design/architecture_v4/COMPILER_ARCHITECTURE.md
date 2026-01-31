# Frame v4 Compiler Architecture - Oceans Model

## Executive Summary

Frame v4 implements a **hybrid compiler architecture** that combines classical compiler design principles with Frame's unique "islands in oceans" model. Frame constructs (islands) are embedded within native code (oceans), requiring a unified approach to parsing, validation, and code generation.

## Core Architecture Principles

### 1. The Oceans Model
- **Native code is the ocean** - The bulk of the source is native language code
- **Frame constructs are islands** - Frame systems, states, and statements are embedded islands
- **Unified representation** - Both must be represented in a single, queryable structure
- **Bidirectional validation** - Frame validates Frame semantics; native validates native syntax

### 2. Classical Compiler Phases with Frame Adaptations

```
┌─────────────────┐
│   Lexical       │ → Tokenize mixed Frame/native source
│   Analysis      │   Identify Frame markers (@@, ->, =>, etc.)
└────────┬────────┘
         │
┌────────▼────────┐
│   Syntactic     │ → Parse Frame constructs into Frame AST
│   Analysis      │   Parse native code into Native AST
└────────┬────────┘
         │
┌────────▼────────┐
│   AST Merger    │ → Combine Frame AST + Native AST
│                 │   Build unified Hybrid AST
└────────┬────────┘
         │
┌────────▼────────┐
│   Semantic      │ → Validate Frame semantics (Arcanum)
│   Analysis      │   Validate native semantics (facades)
│                 │   Cross-validate Frame↔Native references
└────────┬────────┘
         │
    ❌ STOP if errors
         │
┌────────▼────────┐
│   Code          │ → Visitor pattern traversal
│   Generation    │   Generate target language code
└─────────────────┘
```

### 3. Phase Separation with Error Propagation

**Critical Rule**: Each phase must complete successfully before the next phase begins. If errors are detected during semantic analysis, compilation MUST halt and not proceed to code generation.

## Data Structures

### 1. Hybrid AST (HybridAst)

The unified representation of both Frame and native code:

```rust
pub enum HybridNode {
    // Frame nodes
    FrameSystem {
        name: String,
        span: Span,
        interface: Vec<InterfaceMethod>,
        machine: Option<MachineNode>,
        actions: Vec<ActionNode>,
        operations: Vec<OperationNode>,
        domain: Vec<DomainVar>,
    },
    FrameState {
        name: String,
        params: Vec<Param>,
        handlers: Vec<HandlerNode>,
        parent: Option<String>,
        span: Span,
    },
    FrameTransition {
        target: String,
        args: Vec<Expr>,
        span: Span,
    },
    
    // Native nodes (language-specific)
    NativeFunction {
        name: String,
        params: Vec<Param>,
        body: Vec<HybridNode>,
        return_type: Option<Type>,
        span: Span,
    },
    NativeStatement {
        kind: StatementKind,
        content: String,
        span: Span,
    },
    NativeExpression {
        kind: ExprKind,
        content: String,
        span: Span,
    },
    
    // Hybrid nodes (Frame-in-native)
    MixedHandler {
        event_name: String,
        params: Vec<Param>,
        native_body: Vec<HybridNode>,
        frame_statements: Vec<FrameStatement>,
        span: Span,
    },
}
```

### 2. Unified Symbol Table (UnifiedSymbolTable)

Combines Frame's Arcanum with native symbol information:

```rust
pub struct UnifiedSymbolTable {
    // Frame symbols (authoritative for Frame constructs)
    pub arcanum: Arcanum,
    
    // Native symbols (advisory for cross-validation)
    pub native_symbols: NativeSymbolTable,
    
    // Cross-reference mappings
    pub frame_to_native: HashMap<FrameSymbol, NativeSymbol>,
    pub native_to_frame: HashMap<NativeSymbol, FrameSymbol>,
}

pub struct Arcanum {
    pub systems: HashMap<String, SystemEntry>,
}

pub struct SystemEntry {
    pub states: HashMap<String, StateEntry>,
    pub interface_methods: HashSet<String>,
    pub actions: HashSet<String>,
    pub operations: HashSet<String>,
    pub domain_vars: HashMap<String, VarType>,
    pub event_handlers: HashMap<String, HandlerEntry>,
}

pub struct NativeSymbolTable {
    pub functions: HashMap<String, FunctionSymbol>,
    pub variables: HashMap<String, VarSymbol>,
    pub types: HashMap<String, TypeSymbol>,
    pub imports: Vec<ImportSymbol>,
}
```

## Compilation Pipeline

### Phase 1: Lexical Analysis (Scanner)

**Purpose**: Tokenize the mixed Frame/native source into a token stream.

**Components**:
- `FrameScanner`: Identifies Frame-specific tokens (`@@`, `->`, `=>`, `$$`, etc.)
- `NativeScanner`: Language-specific tokenization

**Output**: Unified token stream with token types and spans

### Phase 2: Syntactic Analysis (Parser)

**Purpose**: Build separate ASTs for Frame and native constructs.

**Components**:
- `FrameParser`: Parses Frame systems, states, transitions
- `NativeParser`: Language-specific parsing (can be lightweight/partial)

**Key Features**:
- Frame parser is authoritative for Frame constructs
- Native parser can be partial (only what's needed for validation)
- Both parsers preserve source spans for error reporting

**Output**: 
- Frame AST (complete and authoritative)
- Native AST (potentially partial but sufficient for validation)

### Phase 3: AST Merger

**Purpose**: Combine Frame and Native ASTs into a unified Hybrid AST.

**Process**:
1. Start with native code structure as the base
2. Replace Frame construct regions with parsed Frame AST nodes
3. Maintain bidirectional links between Frame and native contexts
4. Preserve all source mapping information

**Output**: `HybridAst` with complete program representation

### Phase 4: Symbol Table Construction

**Purpose**: Build unified symbol tables for semantic analysis.

**Process**:
1. Traverse Frame AST → populate Arcanum
2. Traverse Native AST → populate NativeSymbolTable
3. Build cross-reference mappings
4. Identify potential conflicts or shadowing

**Output**: `UnifiedSymbolTable` with all program symbols

### Phase 5: Semantic Analysis (Validator)

**Purpose**: Validate both Frame and native semantics with cross-validation.

**Frame Validation (via Arcanum)**:
- E402: Unknown state transitions
- E403: Invalid parent forwarding
- E405: State parameter arity mismatch
- E406: Invalid interface method calls
- E407: Incorrect Frame statement context

**Native Validation (via Facades)**:
- Type checking (where possible)
- Import resolution
- Variable scoping
- Basic syntax validation

**Cross-Validation**:
- Interface methods match native implementations
- Domain variables properly initialized
- Event handlers have correct signatures
- Frame calls from native are valid

**Critical**: If ANY errors are found, compilation MUST stop here.

**Output**: 
- `ValidationResult { ok: bool, errors: Vec<Error> }`
- If `ok == false`, return errors and halt
- If `ok == true`, proceed to code generation

### Phase 6: Code Generation (Visitor)

**Purpose**: Generate target language code from the validated Hybrid AST.

**Architecture**: Visitor Pattern
```rust
trait AstVisitor {
    fn visit_system(&mut self, system: &FrameSystem) -> String;
    fn visit_state(&mut self, state: &FrameState) -> String;
    fn visit_handler(&mut self, handler: &MixedHandler) -> String;
    fn visit_transition(&mut self, trans: &FrameTransition) -> String;
    fn visit_native_function(&mut self, func: &NativeFunction) -> String;
    // ... etc
}

struct PythonCodeGenerator;
impl AstVisitor for PythonCodeGenerator { /* ... */ }

struct TypeScriptCodeGenerator;
impl AstVisitor for TypeScriptCodeGenerator { /* ... */ }
```

**Features**:
- Clean separation of traversal from generation
- Language-specific visitors for each target
- Source map generation during traversal
- Deterministic output

**Output**: Generated source code with source maps

## Error Handling Philosophy

### Fail Early, Fail Hard

1. **Parser errors** → Stop immediately, return parse errors
2. **Symbol conflicts** → Stop at symbol table construction
3. **Semantic errors** → Stop before code generation
4. **Never generate invalid code** → No output if validation fails

### Error Quality

Every error must include:
- Error code (E4xx for Frame, N4xx for native)
- Source location (file:line:column)
- Clear description of the problem
- Suggested fix when possible
- Available alternatives (e.g., "Did you mean $Green?")

## Implementation Strategy

### Incremental Approach

1. **Stage 1**: Enhance Arcanum (✅ Complete)
2. **Stage 2**: Build Frame AST parser
3. **Stage 3**: Add lightweight native AST parsing
4. **Stage 4**: Implement AST merger
5. **Stage 5**: Create unified symbol table
6. **Stage 6**: Integrate semantic validation
7. **Stage 7**: Implement visitor-based code generation

### Compatibility

- Maintain v3 backend as fallback during transition
- New v4 pipeline can be feature-flagged
- Gradual migration of languages (Python first, then TypeScript, etc.)

## Benefits of This Architecture

1. **Correctness**: Proper validation before code generation
2. **Error Quality**: Comprehensive error messages with context
3. **Maintainability**: Clean phase separation
4. **Extensibility**: Easy to add new validation rules or targets
5. **Performance**: Single-pass parsing, efficient validation
6. **Debugging**: Complete source mapping throughout

## Comparison with Current Implementation

| Aspect | Current v4 | Proposed Architecture |
|--------|-----------|----------------------|
| Validation | Optional, non-blocking | Required, blocking |
| AST | Separate, not unified | Hybrid AST |
| Symbol Table | Frame-only (Arcanum) | Unified (Frame + Native) |
| Code Generation | Direct text manipulation | Visitor pattern on AST |
| Error Handling | Warnings only | Fail on first error |
| Native Integration | Text-based regions | Parsed native AST |

## Conclusion

This architecture brings Frame v4 in line with classical compiler design while respecting Frame's unique "islands in oceans" model. By building proper data structures (Hybrid AST, Unified Symbol Table) and enforcing phase separation with error propagation, we ensure that only valid programs produce output, and error messages are meaningful in terms of both Frame and native semantics.