# Frame v4 Current Status

**Last Updated**: 2025-01-09

## Executive Summary

Frame v4 is currently in active development with approximately 80% functionality when using the v3 compiler backend. The pure v4 implementation is about 10% complete with fundamental architectural issues that need resolution.

## Implementation Approaches

### 1. V3 Direct Compiler (DEFAULT - 80% Functional)
- **Status**: Production Ready with V3 limitations
- **Activation**: Default or `USE_V3_DIRECT=1`
- **Description**: Uses complete v3 compilation pipeline
- **Pros**: 
  - Most features work
  - Battle-tested code
  - Handles complex Frame programs
- **Cons**:
  - Generates runtime library dependencies
  - Doesn't remove `var` keywords
  - Parameters not properly handled
  - Uses v3 `@target` instead of v4 `@@target`

### 2. V3-Based Compiler (30% Functional)
- **Status**: Experimental
- **Activation**: `USE_V3_PARSERS=1`
- **Description**: Uses v3 system parser with custom v4 code generation
- **Pros**:
  - No runtime library dependencies
  - Cleaner generated code
- **Cons**:
  - Missing parameter handling
  - Broken indentation
  - Limited Frame statement support
  - Only handles basic transitions

### 3. V4 Pure Implementation (10% Functional)
- **Status**: Broken - Do Not Use
- **Activation**: `USE_V4_PURE=1`
- **Description**: Native v4 scanner/parser/codegen
- **Pros**:
  - Clean architecture
  - No v3 dependencies
- **Cons**:
  - MIR assembly broken
  - Native code not properly captured
  - Most features missing

## Feature Compatibility Matrix

| Feature | V3 Direct | V3-Based | V4 Pure | Notes |
|---------|-----------|----------|---------|-------|
| **Basic Transitions** | ✅ | ✅ | ❌ | `-> $State` |
| **Transition Arguments** | ✅ | ❌ | ❌ | `-> $State(args)` |
| **Event Parameters** | ✅ | ❌ | ❌ | `event(a, b)` |
| **System Return** | ⚠️ | ⚠️ | ❌ | `system.return = value` |
| **Forward Events** | ✅ | ❌ | ❌ | `=>` |
| **Parent Forward** | ✅ | ❌ | ❌ | `=> $^` |
| **Stack Operations** | ✅ | ❌ | ❌ | `$$[+]`, `$$[-]` |
| **Enter Handlers** | ✅ | ❌ | ❌ | `$>()` |
| **Exit Handlers** | ✅ | ❌ | ❌ | `<$()` |
| **Interface Methods** | ✅ | ⚠️ | ❌ | Public API |
| **Actions** | ✅ | ❌ | ❌ | Private methods |
| **Operations** | ✅ | ❌ | ❌ | Helper methods |
| **Domain Variables** | ✅ | ❌ | ❌ | State variables |
| **HSM** | ✅ | ❌ | ❌ | Hierarchical states |
| **Persistence** | ✅ | ❌ | ❌ | `@@persist` |
| **Native Imports** | ✅ | ✅ | ⚠️ | Language imports |
| **Type Annotations** | ✅ | ⚠️ | ❌ | `: type` |

Legend:
- ✅ Fully working
- ⚠️ Partially working or has issues
- ❌ Not working

## Known Issues

### Critical Issues
1. **V4 Pure MIR Broken**: Native code not properly captured in MIR
2. **Parameter Handling**: Interface and event parameters not extracted
3. **Variable Keywords**: `var` not removed from Python output
4. **System Return**: `system.return` not consistently transformed

### Major Issues
5. **Indentation**: Handler bodies have incorrect indentation
6. **Domain Variables**: Not initialized in constructor
7. **Actions/Operations**: Only stub implementations
8. **Enter/Exit Handlers**: Not parsed or generated
9. **HSM Parent Forwarding**: Incomplete implementation
10. **Stack Operations**: Basic support only

### Minor Issues
11. **Source Maps**: Not generated
12. **Debug Symbols**: Missing
13. **Error Messages**: Could be more helpful
14. **Performance**: Not optimized

## Language Support Status

| Language | V3 Direct | V3-Based | V4 Pure | Production Ready |
|----------|-----------|----------|---------|------------------|
| Python | ✅ | ⚠️ | ❌ | Yes (V3 Direct) |
| TypeScript | ✅ | ❌ | ❌ | Yes (V3 Direct) |
| Rust | ⚠️ | ❌ | ❌ | Experimental |
| C | ⚠️ | ❌ | ❌ | Experimental |
| C++ | ⚠️ | ❌ | ❌ | Experimental |
| Java | ⚠️ | ❌ | ❌ | Experimental |
| C# | ⚠️ | ❌ | ❌ | Experimental |

## Test Results

### Python Tests (V3 Direct)
- **Total**: 451 tests
- **Passing**: 451 (100%)
- **Status**: Production Ready

### TypeScript Tests (V3 Direct)
- **Total**: 440 tests
- **Passing**: 435 (98.8%)
- **Status**: Production Ready

### Other Languages
- Limited test coverage
- Not recommended for production

## Example Output Comparison

### Input Frame Code
```frame
@@target python

system Calculator {
    interface:
        add(a, b): int
        
    machine:
        $Ready {
            add(a, b): int {
                var result = a + b
                system.return = result
            }
        }
}
```

### V3 Direct Output (Current Default)
```python
from frame_runtime_py import FrameEvent, FrameCompartment

class Calculator:
    def __init__(self):
        # ... runtime initialization ...
    
    def _event_add(self, __e: FrameEvent, compartment: FrameCompartment):
        if compartment.state == "__Calculator_state_Ready":
            var result = a + b  # BUG: var keyword not removed
            self._system_return_stack[-1] = result
    
    def add(self, *args, **kwargs):
        # ... runtime wrapper ...
```

### V3-Based Output (Experimental)
```python
class Calculator:
    def __init__(self):
        self._state = 'Ready'
    
    def _handle_Ready_add(self):
        var result = a + b  # BUG: var keyword, missing params
        self._system_return = result  # GOOD: proper transformation
    
    def add(self):  # BUG: missing parameters
        self._dispatch('add', locals())
```

### V4 Pure Output (Broken)
```python
# DOES NOT COMPILE - MIR issues prevent generation
```

## Recommended Usage

### For Production
Use V3 Direct (default):
```bash
framec -l python_3 myfile.fpy
```

### For Testing V4 Features
Use V3-Based compiler for simple systems:
```bash
USE_V3_PARSERS=1 framec -l python_3 simple.fpy
```

### For Development
Help fix V4 Pure implementation:
```bash
USE_V4_PURE=1 framec -l python_3 test.fpy  # Currently broken
```

## Path to V4 Completion

### Phase 1: Fix Critical Issues (Current)
- [ ] Fix MIR native code capture
- [ ] Implement parameter extraction
- [ ] Handle variable declarations
- [ ] Fix system.return transformation

### Phase 2: Complete Core Features
- [ ] Implement actions parsing
- [ ] Implement operations parsing
- [ ] Add enter/exit handlers
- [ ] Complete HSM support
- [ ] Full domain variable support

### Phase 3: Remove V3 Dependencies
- [ ] Replace v3 module partitioner
- [ ] Implement v4 native scanners
- [ ] Complete v4 expanders
- [ ] Remove runtime libraries

### Phase 4: Polish and Optimize
- [ ] Generate source maps
- [ ] Improve error messages
- [ ] Optimize performance
- [ ] Complete test coverage

## Contributing

To contribute to Frame v4 development:

1. **Understand the Architecture**: Read `PIPELINE_ARCHITECTURE.md`
2. **Check Current Status**: Review this document
3. **Pick an Issue**: Start with "Minor Issues" if new
4. **Write Tests First**: Add tests before implementation
5. **Follow V3 Patterns**: Study v3 implementation for guidance
6. **Document Changes**: Update architecture docs

## Conclusion

Frame v4 is usable today through the v3 backend but needs significant work to achieve its vision of a pure, runtime-free state machine compiler. The architecture is sound but implementation is incomplete. Production users should continue using v3 direct compilation while v4 matures.