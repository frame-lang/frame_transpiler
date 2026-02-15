# Frame v4 Specification

**Version**: 4.0.0  
**Status**: Draft  
**Date**: January 2, 2025

## Executive Summary

Frame v4 represents a fundamental shift from a complex compiler with its own syntax to a **pure preprocessing tool** that generates native code for state machines. This specification defines the complete v4 architecture, implementation requirements, and migration path from v3.

## Core Philosophy

Frame v4 operates as a preprocessor that:
1. **Reads** Frame source files with language-specific extensions
2. **Generates** pure native code in the target language
3. **Exits** - letting native toolchains handle everything else

No Frame runtime. No complex type system. Just state machine structure with native implementation.

## Architecture Overview

### Pipeline Stages

```
Frame Source (.fpy, .frts, .frs, etc.)
    ↓
Stage 1: Target Detection
    ↓
Stage 2: Annotation Collection
    ↓
Stage 3: Frame Structure Parsing
    ↓
Stage 4: Semantic Validation
    ↓
Stage 5: Code Generation
    ↓
Native Source (.py, .ts, .rs, etc.)
```

## Language Specification

### File Structure

```bnf
file            = prolog imports? annotation* system+
prolog          = "@@target" language_identifier
language_identifier = "python" | "typescript" | "rust" | "c" | "cpp" | "java" | "csharp" | "go"
imports         = native_import_statement+
annotation      = native_annotation | frame_annotation
```

### System Definition

```bnf
system          = annotation* "@@system" identifier "{" system_body "}"
system_params   = "(" system_param_list ")"
system_param_list = system_param ("," system_param)*
system_param    = start_state_param | enter_param | domain_param
start_state_param = "$(" parameter_list ")"
enter_param     = "$>(" parameter_list ")"
domain_param    = identifier

system_body     = operations? interface? machine? actions? domain?
```

### Block Order (Canonical)

When present, blocks must appear in this order:
1. `operations:` - Public static and instance methods for direct system access
2. `interface:` - Public API methods (go through state machine)
3. `machine:` - State definitions
4. `actions:` - Private implementation methods
5. `domain:` - Private state variables

### State Machine Definition

```bnf
machine         = "machine:" state+
state           = "$" identifier state_params? "{" handler* "}"
state_params    = "(" parameter_list ")"
handler         = event_handler | enter_handler | exit_handler
event_handler   = identifier "(" params? ")" return_type? "{" native_code_block "}"
enter_handler   = "$>(" params? ")" "{" native_code_block "}"
exit_handler    = "$<(" params? ")" "{" native_code_block "}"
```

### Frame Statements (in Native Code Blocks)

```bnf
frame_statement = transition | forward | stack_push | stack_pop | system_return
transition      = "->" "$" identifier ("(" args? ")")?
forward         = "=>" "$^"
stack_push      = "push$"
stack_pop       = "pop$" | "->" "pop$"
system_return   = "system.return" "=" expression
```

### Annotations

```bnf
frame_annotation = "@@" identifier annotation_args?
annotation_args  = "(" key_value_list ")"

# Native annotations (preserved as opaque strings)
native_annotation = python_annotation | rust_annotation | csharp_annotation | ...
python_annotation = "@" identifier annotation_args?
rust_annotation  = "#[" annotation_content "]"
csharp_annotation = "[" identifier annotation_args? "]"
```

## System Parameters

### Parameter Types and Flow

1. **Start State Parameters**: `$(x, y)`
   - Flow to initial state constructor
   - **Mandatory**: Initial state MUST accept if declared

2. **Enter Parameters**: `$>(a, b)`
   - Flow to initial state's enter handler
   - **Mandatory**: Enter handler MUST accept if declared

3. **Domain Parameters**: Plain identifiers
   - Initialize domain variables by name
   - **Mandatory**: Domain variable MUST exist for each

### Compilation Rules

```frame
@@system Example ($(x, y), $>(init), config) {
    domain:
        config = None  # REQUIRED for domain param
        
    machine:
        $Start(x, y) {  # REQUIRED to match $(x, y)
            $>(init) {  # REQUIRED to match $>(init)
                # Implementation
            }
        }
}
```

**Compiler Errors**:
- Start state doesn't match `$(...)` parameters
- Enter handler doesn't match `$>(...)` parameters  
- Domain parameter has no matching domain variable
- Wrong parameter order (must be `$()`, `$>()`, then plain)

## Annotation System

### Frame Annotations

| Annotation | Purpose | Parameters |
|------------|---------|------------|
| `@@target` | Specifies target language | Language identifier |
| `@@persist` | Enables persistence generation | Optional: format, compression |
| `@@system` | System instantiation tracking | Variable = System(args) |

### Native Annotation Preservation

Native annotations are recognized by pattern and preserved without interpretation:

| Language | Pattern | Example |
|----------|---------|---------|
| Python | `@annotation` | `@dataclass`, `@property` |
| TypeScript | `@annotation` | `@Injectable()` |
| Rust | `#[annotation]` | `#[derive(Serialize)]` |
| C# | `[Annotation]` | `[Serializable]` |
| Java | `@Annotation` | `@Override` |
| C++ | `[[annotation]]` | `[[nodiscard]]` |
| Go | Struct tags | `` `json:"name"` `` |

## Code Generation

### Generation Strategy

1. **Native Annotations**: Emitted first, before system/class declaration
2. **System Structure**: Generated as class/struct per target language
3. **Native Code Blocks**: Inserted unchanged (indentation adjusted)
4. **Frame Statements**: Expanded to minimal native equivalents

### No Runtime Libraries

v4 generates self-contained code with no Frame runtime dependencies:
- State management: Generated inline
- Event dispatch: Generated router method
- Stack operations: Native array/vector
- Persistence: Native serialization

### Target Language Mapping

| Frame Construct | Python | TypeScript | Rust |
|-----------------|--------|------------|------|
| system | class | class | struct + impl |
| interface method | public method | public method | pub fn |
| operations method | @staticmethod or method | static or method | pub fn |
| action | private method | private method | fn (private) |
| domain var | self.var | private var | field |
| $State | state enum + handler | state enum + handler | enum variant |

## Semantic Validation

### Compile-Time Checks

1. **Structure Validation**
   - Block order compliance
   - State reference validity
   - Interface method implementation

2. **Parameter Validation**  
   - System parameters have destinations
   - State parameters match declarations
   - Domain variables exist for domain params

3. **System Tracking** (`@@system`)
   - Method calls match interface
   - No access to private actions
   - Type compatibility (delegated to native)

### What Frame Does NOT Validate

- Native code syntax (left to native compiler)
- Type checking (left to native compiler)
- Native annotation semantics (left to native compiler)

## File Extensions and Targets

### Language-Specific Extensions (Required)

| Extension | Target Language | @@target Pragma |
|-----------|----------------|-----------------|
| `.fpy` | Python | `@@target python` |
| `.frts` | TypeScript | `@@target typescript` |
| `.frs` | Rust | `@@target rust` |
| `.fc` | C | `@@target c` |
| `.fcpp` | C++ | `@@target cpp` |
| `.fjava` | Java | `@@target java` |
| `.frcs` | C# | `@@target csharp` |
| `.fgo` | Go | `@@target go` |

**Note**: `.frm` universal extension is deprecated and should not be used.

## Implementation Requirements

### Parser Implementation

```rust
// framec/src/frame_c/v4/parser.rs
pub struct ParserV4 {
    // No MixedBody
    // No MIR
    // No complex native parsing
}

impl ParserV4 {
    pub fn parse(source: &str) -> Result<SystemAst> {
        // 1. Parse @@target pragma
        // 2. Collect annotations as strings
        // 3. Parse Frame structure only
        // 4. Store native blocks as opaque strings
    }
}
```

### AST Structure

```rust
pub struct SystemAst {
    pub target: TargetLanguage,
    pub annotations: Vec<String>,  // Opaque native annotations
    pub name: String,
    pub params: SystemParams,
    pub operations: Vec<Operation>,
    pub interface: Vec<InterfaceMethod>,
    pub machine: Vec<State>,
    pub actions: Vec<Action>,
    pub domain: Vec<DomainVar>,
}

pub struct State {
    pub name: String,
    pub params: Vec<String>,
    pub handlers: Vec<Handler>,
}

pub struct Handler {
    pub handler_type: HandlerType,
    pub params: Vec<String>,
    pub native_code: String,  // Opaque native code block
}
```

### Code Generator Interface

```rust
pub trait CodeGeneratorV4 {
    fn generate(&self, ast: &SystemAst) -> Result<String>;
    fn emit_annotations(&self, annotations: &[String]) -> String;
    fn emit_system(&self, system: &SystemAst) -> String;
    fn emit_persistence(&self, system: &SystemAst) -> Option<String>;
}
```

## Migration

See [MIGRATION_V3_TO_V4.md](MIGRATION_V3_TO_V4.md) for migration guidance from earlier Frame versions.

## Testing Strategy

### Test Categories

1. **Structural Tests**: Frame parsing and structure
2. **Generation Tests**: Correct native code output
3. **Annotation Tests**: Native annotations preserved
4. **Parameter Tests**: System parameters flow correctly
5. **Compilation Tests**: Generated code compiles with native compiler

### Test Infrastructure

```bash
# Test compilation only
framec compile test.fpy -l python_3 -o test.py

# Test with native compiler
python -m py_compile test.py
typescript --noEmit test.ts
rustc --crate-type lib test.rs
```

## Success Criteria

### Phase 1 (Core Implementation) - COMPLETE
- [x] Frame parser with AST
- [x] Native annotation preservation
- [x] Basic code generation for PRT languages
- [x] System parameter validation

### Phase 2 (Features) - IN PROGRESS
- [ ] Persistence generation (`@@persist`)
- [x] Async handler support
- [x] Full PRT language support (Python, Rust, TypeScript)

### Phase 3 (Polish)
- [ ] Performance optimization
- [ ] Documentation complete
- [ ] Test coverage > 90%

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Native annotation conflicts | Let native compilers handle |
| Complex native expressions | Pass through as opaque strings |

## Async Behavior

### Async Handler Semantics

Frame has **no special async semantics**. The `async` keyword is passed through to native code, and Frame's behavior remains consistent:

1. **State transitions are immediate**: When `-> $State()` executes, the state variable updates immediately
2. **Async handlers are just functions**: They may take time to complete, but don't affect state machine semantics
3. **No limbo states**: The machine always has a definitive current state

#### Example:
```python
$StateA {
    async process() {
        -> $StateB()  # State becomes $StateB immediately
        
        # This code still executes, but machine is already in $StateB
        result = await longOperation()  
        # If another event arrives here, $StateB handles it
    }
}

$StateB {
    async $>() {
        # State is already $StateB when this runs
        # This is just an event handler that happens to be async
        await initialize()
        # No special semantics - just native async behavior
    }
}
```

### Key Points:
- Enter/exit handlers are just event handlers with special names
- State variable updates before any handlers execute
- Async completion doesn't affect state machine state
- Exception handling follows native language patterns

## Module Resolution

Frame v4 uses **pure native module resolution**. Frame does not track or validate imports - all module/import handling is delegated to the native language toolchain.

### Import Handling:
- Native import statements are passed through unchanged
- Frame does not validate imported Frame systems
- Module resolution follows native language rules
- No Frame-specific module system

### Example:
```python
@@target python

# Native imports - Frame doesn't interpret these
import json
from datetime import datetime
from other_module import OtherFrameSystem  # Frame doesn't track this

@@system MySystem {
    # ...
}
```

This approach:
- Simplifies the Frame compiler
- Leverages existing language ecosystems
- Avoids reimplementing module resolution
- Follows the preprocessor philosophy

## Debug Support

Frame v4 will provide comprehensive debugging support through a phased approach, with debugging implementation coming last in the v4 development cycle.

### Phase 1: Source Maps (MVP)
Generate standard source maps to enable basic debugging:
- Set breakpoints in Frame source files (`.fpy`, `.frts`, etc.)
- Step through Frame code while executing native code
- Map generated code locations back to Frame source
- Use native debugger with Frame source visibility

### Phase 2: Frame Debug Adapter
Build Frame-aware debugging through VSCode Debug Adapter Protocol:
- **State Machine Visualization**: Highlight current state
- **Transition Tracking**: Visual feedback on state changes
- **Compartment Inspector**: View state-local variables
- **State Stack Viewer**: Inspect push/pop operations
- **Frame-Specific Stepping**: Step to next transition/state

### Phase 3: Advanced Integration
Full Frame debugging experience:
- **Interactive State Diagram**: Live state machine visualization
- **Time-Travel Debugging**: Step backwards through transitions
- **Event Recording**: Record and replay event sequences
- **Transition History**: Complete state transition log

### Implementation Priority
Debugging comes **last** in v4 implementation:
1. First: Core compiler, code generation
2. Then: Features (persistence, async, etc.)
3. Finally: Debug support (all three phases)

This ensures v4 is functional before adding debug tooling.

## Error Reporting

Frame v4 provides context-appropriate error messages based on the error source:

### Frame Structural Errors
Full context and helpful diagnostics for Frame-specific issues:
```
ERROR: Start state parameters mismatch
  --> robot.fpy:10:5
   |
10 | @@system Robot ($(x, y)) {
   |               ^^^^^^^^ System declares 2 parameters
...
15 |     $Idle {  // ERROR: Must accept (x, y)
   |     ^^^^^ State missing required parameters
```

### Native Code Errors
Delegate to native compiler - Frame doesn't parse or validate native code:
- Syntax errors in native blocks → Native compiler reports
- Type errors → Native compiler reports
- Runtime errors → Native runtime reports

### Error Categories

| Error Type | Frame Handling | Example |
|------------|---------------|---------|
| Missing state | Full Frame diagnostic | "State $Unknown not defined" |
| Parameter mismatch | Detailed Frame error | "Enter handler missing required parameter 'init'" |
| Bad block order | Frame structural error | "interface block must come before machine block" |
| Native syntax | Pass to native compiler | Python SyntaxError |
| Type mismatch | Native compiler | TypeScript type error |
| Import errors | Native toolchain | Module not found |

### Error Message Format
```
[ERROR_CODE] Message
  --> file.fpy:line:column
   |
line | source code
   | ^^^ specific location
   |
   = help: Suggestion for fixing
```

This approach:
- Provides excellent diagnostics for Frame issues
- Doesn't duplicate native compiler work
- Maintains clear separation of concerns
- Helps developers quickly identify error source

## Appendices

### A. Complete Grammar
See [`grammar_v4.md`](grammar_v4.md)

### B. System Parameters  
See [`system_parameters.md`](system_parameters.md)

### C. Examples
See [`examples/`](examples/) directory

### D. Migration Guide
See [`MIGRATION_V3_TO_V4.md`](MIGRATION_V3_TO_V4.md)

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2025-01-02 | Frame Team | Initial draft |

## Approval

| Role | Name | Date | Signature |
|------|------|------|-----------|
| Lead Developer | | | |
| Architecture Review | | | |
| Documentation Review | | | |