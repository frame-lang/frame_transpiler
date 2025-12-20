# Framepiler Architecture V4 — Overview

## Executive Summary

V4 represents a fundamental simplification of the Frame compilation pipeline, embracing a **native-first philosophy** where Frame provides structural organization while all implementation uses native language syntax. This eliminates the complex MixedBody/MIR approach of V3 in favor of a simpler "preserve and pass-through" model for native code and annotations.

## Core Philosophy Changes from V3

| Aspect | V3 Approach | V4 Approach |
|--------|-------------|-------------|
| **Code blocks** | MixedBody (Frame + native) | Native-only syntax |
| **Annotations** | Frame-only | Native + Frame (@@-prefixed) |
| **Parsing strategy** | Deep parsing and expansion | Preserve and pass-through |
| **Language handling** | Complex per-language scanners | Unified annotation patterns |
| **Validation** | Frame validates everything | Native compilers validate native |

## Architecture Principles

1. **Minimal Parsing**: Frame only parses what it needs to understand (structure, transitions, Frame annotations)
2. **Native Delegation**: Let native compilers handle native syntax, types, and semantics
3. **Annotation Preservation**: Recognize native annotation patterns, preserve without interpretation
4. **Simplified Pipeline**: Fewer stages, less language-specific code
5. **Clear Boundaries**: Frame handles state machines, native handles implementation

## Pipeline Stages (Simplified from V3)

### Stage 1: Source Analysis
- **Input**: Frame source files (`.frm`, `.fpy`, `.frts`, etc.)
- **Process**: 
  - Identify `@@target` pragma
  - Collect native imports
  - Identify native annotations (`@decorator`, `#[attribute]`, `[Attribute]`)
- **Output**: Source metadata and annotation positions

### Stage 2: Frame Structure Parsing
- **Input**: Annotated source
- **Process**:
  - Parse Frame structural elements (system, states, blocks)
  - Preserve native annotations as opaque strings
  - Identify Frame annotations (`@@persist`, `@@system`)
  - Extract Frame statements (`->`, `=>`, `$$[+]`, etc.) from native code
- **Output**: Frame AST with attached native annotations and code blocks

### Stage 3: Semantic Analysis
- **Input**: Frame AST
- **Process**:
  - Validate Frame structure (state references, transitions)
  - Process `@@system` declarations for semantic validation
  - Build symbol tables for Frame elements
  - Check block ordering (operations, interface, machine, actions, domain)
- **Output**: Validated AST with symbol information

### Stage 4: Code Generation
- **Input**: Validated AST
- **Process**:
  - Emit native annotations in correct positions
  - Generate Frame runtime scaffolding
  - Insert native code blocks unchanged
  - Generate Frame-specific helpers (state management, persistence)
- **Output**: Native target language code

### Stage 5: Post-Processing (Optional)
- **Input**: Generated code
- **Process**:
  - Source map generation
  - Native formatter application (optional)
  - Debug symbol generation
- **Output**: Final code with tooling support files

## Key Simplifications from V3

### Eliminated Components
1. **MixedBody/MIR**: No longer needed - all blocks are native
2. **Native Region Scanners**: Simplified - just identify annotation patterns
3. **Frame Statement Expansion**: Minimal - Frame statements stay Frame syntax
4. **Body Closers**: Simpler - native blocks are opaque
5. **Native Parse Facades**: Let native compilers validate

### New/Enhanced Components
1. **Native Annotation Parser**: Recognizes patterns (`@`, `#[]`, `[]`) without interpretation
2. **Annotation Preservers**: Maintains annotation positioning through pipeline
3. **Persistence Generator**: Creates save/restore methods when `@@persist` present
4. **System Validator**: Enhanced validation for `@@system` declarations

## Language-Specific Handling

### Annotation Patterns (Unified Recognition)

| Language | Pattern | Example | Regex |
|----------|---------|---------|-------|
| Python, TypeScript, Java | `@annotation` | `@dataclass` | `@\w+(\(.*?\))?` |
| Rust | `#[annotation]` | `#[derive(Serialize)]` | `#\[.*?\]` |
| C# | `[Annotation]` | `[Serializable]` | `\[\w+(\(.*?\))?\]` |
| C++ | `[[annotation]]` | `[[nodiscard]]` | `\[\[.*?\]\]` |
| Go | Struct tags | `` `json:"name"` `` | `` `.*?` `` |

### Simplified Per-Language Requirements

Instead of complex language-specific scanners, v4 only needs:
1. **Annotation pattern recognition** (2-3 patterns total)
2. **Native import preservation** (already handled)
3. **Basic syntax awareness** for code block boundaries

## File Extensions and Targets

The `@@target` pragma remains authoritative. Extensions are conventions:

| Extension | Target | Pragma |
|-----------|--------|--------|
| `.frm` | Universal | `@@target <language>` required |
| `.fpy` | Python | `@@target python` |
| `.frts` | TypeScript | `@@target typescript` |
| `.frs` | Rust | `@@target rust` |
| `.fc` | C | `@@target c` |
| `.fcpp` | C++ | `@@target cpp` |
| `.fjava` | Java | `@@target java` |
| `.frcs` | C# | `@@target csharp` |

## Benefits of V4 Architecture

1. **Simplicity**: Fewer stages, less complexity
2. **Maintainability**: Less language-specific code
3. **Extensibility**: New languages easier to add
4. **Native Power**: Full access to language ecosystems
5. **Better Errors**: Native compilers provide familiar error messages
6. **Future Proof**: New language features automatically supported

## Migration from V3

See [PLAN.md](PLAN.md) for the detailed migration strategy.

## Testing Strategy

### Simplified Testing Approach
1. **Frame Structure Tests**: Validate Frame parsing and generation
2. **Annotation Preservation Tests**: Ensure annotations maintained correctly  
3. **Native Compilation Tests**: Let native compilers validate output
4. **Integration Tests**: End-to-end with native toolchains

### Shared Test Environment
Continue V3's shared test environment strategy but with simpler requirements:
- Docker containers for each language
- Native compiler validation as primary test
- Focus on Frame structure correctness

## Implementation Priority

### Phase 1: Core Changes (PRT Languages)
1. Native annotation recognition
2. Simplified Frame parser
3. Code generation with annotation preservation
4. Python, TypeScript, Rust support

### Phase 2: Extended Language Support
1. Java, C#, C++, C
2. Go (struct tags)
3. Additional languages as needed

### Phase 3: Advanced Features
1. Enhanced persistence with native serialization
2. Debug adapter with native debugging support
3. IDE integration improvements

## Success Criteria

1. **All V3 tests pass** with V4 implementation
2. **Native annotations work** in generated code
3. **Simpler codebase** with fewer lines of code than V3
4. **Better performance** due to simplified parsing
5. **Native ecosystem integration** works seamlessly

## Open Questions

1. **Gradual migration?** Can V3 and V4 coexist during transition?
2. **Feature flags?** Use flags to enable V4 features incrementally?
3. **Backwards compatibility?** Support V3 syntax with warnings?

See [PLAN.md](PLAN.md) for answers and migration approach.