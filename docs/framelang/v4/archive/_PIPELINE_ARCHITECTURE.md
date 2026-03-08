> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame v4 Pipeline Architecture

## Overview

Frame v4 implements a multi-stage compilation pipeline that transforms Frame source files (`.fpy`, `.frts`, `.frs`, etc.) into target language code. The pipeline operates in linear time O(n) with deterministic, single-pass stages.

## Current Status

**As of 2025-01-09:**
- V4 is approximately 80% functional when using the v3 direct compiler
- The pure v4 implementation is ~10% complete with fundamental MIR issues
- Production code currently defaults to v3's complete pipeline

## Pipeline Stages

### Stage 1: Module Partitioning

**Component**: `ModulePartitionerV4` (currently using `ModulePartitionerV3`)

**Purpose**: Segments the source file into distinct regions for processing.

**Input**: Raw Frame source bytes with `@@target` pragma

**Output**: `ModulePartitions` containing:
- `prolog`: Target language declaration (`@@target python`)
- `imports`: Native language import statements
- `bodies`: Code bodies with their types (handler/action/operation)

**Implementation Details**:
- Uses DPDA (Deterministic Pushdown Automaton) body closers per language
- Operates on byte offsets for precision
- Single-pass, linear time complexity
- Body closers are language-specific to handle:
  - Python: Indentation-based blocks
  - TypeScript/JavaScript: Brace-based with template literals
  - Rust: Brace-based with lifetime annotations
  - C/C++: Brace-based with preprocessor directives
  - Java: Brace-based with annotations
  - C#: Brace-based with verbatim/interpolated strings

### Stage 2: Native Region Scanning

**Component**: `NativeRegionScannerV4` per language

**Purpose**: Identifies Frame statements within native code bodies.

**Input**: Body byte slices from Stage 1

**Output**: `[RegionV4::NativeText | RegionV4::FrameSegment]` with spans

**Frame Statement Detection Rules**:
- Must appear at start-of-line (SOL) after optional whitespace
- Recognized patterns:
  - `-> $State(args)` - State transition
  - `=> $^` - Parent forwarding in HSM
  - `=> eventName(args)` - Event forwarding
  - `$$[+]` - Stack push
  - `$$[-]` - Stack pop
  - `system.return = expr` - System return assignment
- Ignored inside strings, comments, and template literals

**Inline Termination Rules**:
- Python: Ends at `;` or `#` comment
- TypeScript/JavaScript: Ends at `;` or `//` comment
- C-family languages: Ends at `;` or `//` comment
- Rust: Ends at `;` or `//` comment

### Stage 3: Frame Segment Parsing

**Component**: `FrameStatementParserV4`

**Purpose**: Parses identified Frame segments into structured representations.

**Input**: Frame segment strings from Stage 2

**Output**: Parsed Frame statements with:
- Statement type (transition, forward, stack op, return)
- Target state (for transitions)
- Arguments (balanced parentheses, string-aware)
- Source spans for error reporting

**Parsing Features**:
- Balanced parenthesis tracking
- String-aware argument splitting
- Nested expression support
- Whitespace tolerance

### Stage 4: MIR (Mixed Intermediate Representation) Assembly

**Component**: `MirAssemblerV4`

**Purpose**: Builds intermediate representation preserving both Frame and native regions.

**Input**: Parsed Frame statements and native text regions

**Output**: `MixedBody` containing:
- Ordered sequence of MIR items
- Native text segments with spans
- Frame statements with expansion points
- Origin tracking for source mapping

**MIR Item Types**:
```rust
enum MirItemV4 {
    Transition {
        target: String,
        exit_args: Vec<String>,
        enter_args: Vec<String>,
        state_args: Vec<String>,
        span: RegionSpan,
    },
    Forward {
        event: Option<String>,  // None for => $^
        args: Vec<String>,
        span: RegionSpan,
    },
    StackPush {
        span: RegionSpan,
    },
    StackPop {
        span: RegionSpan,
    },
    SystemReturn {
        expression: String,
        span: RegionSpan,
    },
}
```

### Stage 5: Frame Statement Expansion

**Component**: Language-specific expanders (`PythonExpanderV4`, `TypeScriptExpanderV4`, etc.)

**Purpose**: Generates target language code for Frame statements.

**Input**: MIR items with indentation context

**Output**: Expanded native code strings

**Expansion Examples**:

Python:
- `-> $Green()` → `self._transition_to_Green()`
- `=> $^` → `self._forward_to_parent()`
- `$$[+]` → `self._stack_push()`
- `system.return = value` → `self._system_return = value`

TypeScript:
- `-> $Green()` → `this._transition_to_Green()`
- `=> $^` → `this._forward_to_parent()`
- `$$[+]` → `this._stack_push()`
- `system.return = value` → `this._system_return = value`

### Stage 6: Splice and Mapping

**Component**: `SplicerV4`

**Purpose**: Replaces Frame statements with expanded code while preserving source mapping.

**Input**: 
- Original source with Frame statements
- Expanded code strings
- Region spans

**Output**: `SplicedBody` containing:
- Final generated code
- Source map for debugging
- Bidirectional mapping (source ↔ generated)

### Stage 6.5: Structural Validation (Early)

**Component**: `ValidatorV4`

**Purpose**: Enforces Frame semantic rules before final code generation.

**Validation Rules**:
- Transitions must be terminal (last statement in handler)
- No Frame statements in actions/operations
- State headers must have `{` on same line in machine block
- Interface methods must be properly declared
- State names must be unique within a system
- Forward targets must exist

### Stage 7: Native Parse Facade (Optional)

**Component**: `NativeParseFacadeV4` per language

**Purpose**: Validates generated native code syntax.

**Features**:
- Language-specific syntax validation
- Indentation checking (Python)
- Brace matching (C-family)
- Type checking preparation
- Error mapping back to Frame source

**Current Status**: 
- Wrapper validation implemented
- Full native parsing optional behind `--validate-native` flag

### Stage 8: Code Generation and Source Maps

**Component**: `CodeGeneratorV4`

**Purpose**: Produces final output with debugging support.

**Output**:
- Target language source file
- Source map (when requested)
- Debug symbols
- Frame metadata

### Stage 9: Final Validation

**Component**: `ValidatorV4` (second pass)

**Purpose**: Comprehensive validation of generated code.

**Checks**:
- Terminal statement enforcement
- No Frame statements in wrong contexts
- Native code policy compliance
- Import resolution
- Symbol availability

### Stage 10: AST and Symbol Integration

**Component**: `Arcanum` symbol table

**Purpose**: Tracks all Frame and native symbols for validation and code generation.

**Features**:
- System discovery and metadata
- State enumeration
- Interface method signatures
- Domain variable tracking
- Cross-reference validation

### Stage 11: Runtime Generation

**Component**: Runtime library generators per language

**Purpose**: Generates runtime support code for Frame systems.

**Python Runtime**:
```python
class FrameCompartment:
    def __init__(self, state, enter_args=None, state_args=None):
        self.state = state
        self.enter_args = enter_args or []
        self.state_args = state_args or {}

class FrameEvent:
    def __init__(self, message, params=None):
        self._message = message
        self._params = params or []
```

**TypeScript Runtime**:
```typescript
export class FrameCompartment {
    constructor(
        public state: string,
        public enterArgs: any[] = [],
        public stateArgs: Record<string, any> = {}
    ) {}
}

export class FrameEvent {
    constructor(
        public _message: string,
        public _params: any[] = []
    ) {}
}
```

## Implementation Status by Component

| Stage | Component | V3 Status | V4 Status | Notes |
|-------|-----------|-----------|-----------|-------|
| 1 | Module Partitioner | ✅ Complete | 🔄 Using V3 | Pragma change: `@target` → `@@target` |
| 2 | Native Region Scanner | ✅ Complete | ⚠️ Partial | Basic scanning works, missing some features |
| 3 | Frame Parser | ✅ Complete | ⚠️ Partial | Transitions work, missing system.return |
| 4 | MIR Assembly | ✅ Complete | ❌ Broken | Fundamental issues with native code capture |
| 5 | Expanders | ✅ Complete | ⚠️ Partial | Only transitions implemented |
| 6 | Splicer | ✅ Complete | 🔄 Using V3 | Works but needs v4 adaptation |
| 7 | Native Facade | ✅ Complete | ❌ Not Started | Low priority |
| 8 | Code Generation | ✅ Complete | ⚠️ Partial | Basic generation works |
| 9 | Validation | ✅ Complete | ⚠️ Partial | Basic validation only |
| 10 | Symbol Table | ✅ Complete | 🔄 Using V3 | Arcanum works |
| 11 | Runtime | ✅ Complete | ❌ Different | V4 targets no runtime deps |

Legend:
- ✅ Complete and working
- 🔄 Using V3 implementation
- ⚠️ Partially implemented
- ❌ Not implemented or broken

## Language-Specific Considerations

### Python
- Indentation-based blocks require special handling
- `var` keyword must be removed (native Python doesn't use it)
- f-strings and format strings must be preserved
- `system.return` → `self._system_return`

### TypeScript
- Template literals with `${}` must not be confused with Frame's `$State`
- Arrow functions need special consideration
- Type annotations must be preserved
- `system.return` → `this._system_return`

### Rust
- Lifetime annotations must be preserved
- Macro invocations must not interfere with Frame parsing
- Ownership and borrowing must be maintained
- `system.return` → `self._system_return`

### C/C++
- Preprocessor directives must be handled
- Pointer syntax must not interfere with Frame
- Templates (C++) need special handling

### Java
- Annotations must be preserved
- Generic syntax must not interfere
- Lambda expressions need consideration

### C#
- Verbatim strings `@"..."` must be handled
- Interpolated strings `$"..."` must not conflict with Frame
- LINQ expressions need consideration
- Preprocessor directives must be handled

## Performance Characteristics

All stages maintain O(n) time complexity:
- Single pass through source
- No backtracking in parsers
- Deterministic state machines
- Bounded lookahead

Memory usage is O(n) with source size:
- No exponential intermediate representations
- Streaming where possible
- Efficient span-based tracking

## Testing Strategy

1. **Unit Tests**: Each stage has isolated tests
2. **Integration Tests**: Full pipeline tests per language
3. **Regression Tests**: Known bug fixes stay fixed
4. **Performance Tests**: Ensure O(n) complexity
5. **Error Recovery Tests**: Graceful handling of malformed input

## Current Limitations

1. **Parameter Handling**: Interface and handler parameters not properly captured
2. **Variable Declarations**: `var` keyword not removed in native blocks
3. **System Returns**: `system.return` not consistently transformed
4. **Domain Variables**: Not properly parsed and initialized
5. **Actions/Operations**: Stub implementations only
6. **Enter/Exit Handlers**: Not implemented
7. **HSM Features**: Parent forwarding incomplete
8. **Stack Operations**: Basic implementation only

## Migration Path from V3

Currently, v4 uses three approaches:
1. **V3 Direct**: Uses complete v3 pipeline (default, ~80% functional)
2. **V3 Adapter**: Uses v3 parsers with v4 generation (~30% functional)  
3. **V4 Pure**: Native v4 implementation (~10% functional)

Environment variables control selection:
- `USE_V3_DIRECT=1` - Use complete v3 pipeline
- `USE_V3_PARSERS=1` - Use v3 parsers with v4 generation
- `USE_V3_ADAPTER=1` - Use v3 adapter approach
- (none) - Defaults to v3 direct

## Next Steps for Full V4 Implementation

1. Fix MIR assembly to properly capture native code
2. Implement proper parameter extraction for interfaces/handlers
3. Remove `var` keyword transformation
4. Complete system.return handling
5. Implement actions and operations parsing
6. Add enter/exit handler support
7. Complete HSM parent forwarding
8. Full domain variable support
9. Remove runtime library dependencies
10. Achieve 100% test coverage
