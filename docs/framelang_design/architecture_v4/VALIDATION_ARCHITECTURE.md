# Frame v4 Validation Architecture

## Executive Summary

Frame v4 adopts a **pragmatic hybrid validation approach** that validates Frame semantics authoritatively while delegating native language validation to native compilers. This architecture provides intelligible error messages without the complexity of full heterogeneous AST parsing.

## Core Principles

1. **Separation of Concerns**: Frame validates Frame semantics; native compilers validate native code
2. **No False Promises**: We don't claim to validate what we can't reliably validate
3. **User Intelligibility**: Errors are reported in Frame terms when possible, with clear mapping to native errors
4. **Performance**: Validation doesn't significantly impact compilation time
5. **Maintainability**: No dependency on native language parser versions

## Architecture Components

### 1. Arcanum (Frame Symbol Table) - Authoritative

The Arcanum is Frame's authoritative symbol table for all Frame constructs:

```rust
pub struct Arcanum {
    pub systems: HashMap<String, SystemEntry>,
}

pub struct SystemEntry {
    pub states: HashMap<String, StateEntry>,
    pub interface_methods: HashSet<String>,
    pub actions: HashSet<String>,
    pub operations: HashSet<String>,
    pub domain_vars: HashMap<String, VarType>,
}
```

**Validates:**
- State existence (E402)
- Transition targets are valid
- Parent forwarding availability (E403)
- Interface method compliance
- State parameter arity (E405)
- Frame statement correctness

### 2. Source Map Generation - Required

Every generated line includes source mapping information:

```python
# Generated Python with source maps
class TrafficLight:
    def _handle_Red_tick(self):  # [FSM:5:12-8:14]
        self.log_action()         # [FSM:6:8-6:24]
        # Frame: -> $Green
        self._frame_transition("Green")  # [FSM:7:8-7:17]
```

**Source Map Format:**
```json
{
  "version": 1,
  "sources": ["traffic_light.frm"],
  "mappings": [
    {
      "generated": {"line": 12, "column": 8},
      "source": {"line": 5, "column": 12, "file": "traffic_light.frm"},
      "name": "_handle_Red_tick"
    }
  ]
}
```

### 3. Error Translation Service - Optional

Post-processes native compiler errors to provide Frame context:

```bash
# Native compiler error
AttributeError: 'TrafficLight' object has no attribute 'log_action'
  File "traffic_light.py", line 13, in _handle_Red_tick

# Translated by Frame
Frame Error Context:
  In system TrafficLight, state $Red, handler tick()
  At traffic_light.frm:6:8
  
  The method 'log_action' is not defined. Did you mean:
    - 'log' (defined as action)
    - 'logTransition' (defined as operation)
```

### 4. Native Validation Facade - CI/CD Only

Optional deeper validation for continuous integration:

```bash
framec compile --validate-native system.frm
```

This mode:
- Runs native language linters
- Performs basic type checking
- Validates import statements exist
- Still doesn't parse full native ASTs

## Validation Phases

### Phase 1: Frame Syntax Validation (Parser)
- Frame syntax correctness
- Block structure validity
- Frame statement parsing

### Phase 2: Frame Semantic Validation (Arcanum)
- State existence
- Transition validity
- Interface compliance
- Parameter matching

### Phase 3: Code Generation with Source Maps
- Generate native code
- Embed source mappings
- Create translation metadata

### Phase 4: Native Compilation (Delegated)
- Native compiler validates generated code
- Errors reported with native context

### Phase 5: Error Translation (Optional)
- Map native errors to Frame source
- Provide Frame-contextual help
- Suggest corrections

## Error Codes

### Frame-Specific Errors (E400-E499)

| Code | Description | Example |
|------|-------------|---------|
| E400 | Frame syntax error | `-> $State(` (unclosed paren) |
| E401 | Frame statement not at SOL | Indented `-> $State` |
| E402 | Unknown state | `-> $NonExistent` |
| E403 | No parent for forward | `=> $^` without parent |
| E404 | Duplicate state | Two `$Red` states |
| E405 | Parameter arity mismatch | `-> $State(1,2)` but state expects 3 |
| E406 | Invalid interface call | Calling private action externally |
| E407 | Frame statement in wrong context | `-> $State` in action body |

### Native Errors (Delegated)

All other errors come from native compilers with Frame context added via source maps.

## Implementation Plan

### Stage 1: Enhance Arcanum (Week 1-2)

**Goal**: Complete Frame symbol table with full validation

**Tasks:**
1. Extend Arcanum to track all Frame constructs
2. Add interface method tracking
3. Add action/operation tracking
4. Add domain variable tracking
5. Implement state parameter validation

**Testing:**
- Unit tests for each validation rule
- Integration tests with known error cases
- No temporary file testing

### Stage 2: Source Map Generation (Week 2-3)

**Goal**: Every generated line maps to Frame source

**Tasks:**
1. Create SourceMap data structure
2. Track mappings during code generation
3. Embed mappings as comments
4. Generate separate `.map` files
5. Add source map to debug output

**Testing:**
- Verify each generated construct has mapping
- Test complex nested structures
- Validate map file format

### Stage 3: Frame Validation Suite (Week 3-4)

**Goal**: Comprehensive Frame semantic validation

**Tasks:**
1. Implement E402 (unknown state) validation
2. Implement E403 (parent forward) validation
3. Implement E404 (duplicate state) validation
4. Implement E405 (parameter arity) validation
5. Implement E406 (interface compliance) validation
6. Implement E407 (context validation)

**Testing:**
- Test suite with expected errors
- Multi-error aggregation tests
- Cross-file validation tests

### Stage 4: Error Translation (Week 4-5)

**Goal**: Translate native errors to Frame context

**Tasks:**
1. Create error pattern matchers per language
2. Build Frame context from source maps
3. Implement suggestion engine
4. Create translation database
5. Add CLI translation command

**Testing:**
- Mock native errors and verify translation
- Test each supported language
- Edge case handling

### Stage 5: Testing Infrastructure (Week 5-6)

**Goal**: Robust testing without temporary files

**Tasks:**
1. Create test fixture framework
2. Build expected error assertion system
3. Add validation test runner
4. Create error catalog tests
5. Add regression test suite

**Testing Framework:**
```rust
#[test]
fn test_unknown_state_validation() {
    let fixture = TestFixture::new("
        system Example {
            machine:
                $Start {
                    go() { -> $NonExistent }
                }
        }
    ");
    
    let result = fixture.compile();
    assert_validation_error!(result, E402, 
        line: 4, 
        column: 28,
        state: "NonExistent"
    );
}
```

### Stage 6: Native Facade (Week 6-7) - Optional

**Goal**: Basic native validation for CI/CD

**Tasks:**
1. Integrate language linters
2. Add import validation
3. Add basic type checking
4. Create CI/CD mode flag
5. Add validation report generation

**Testing:**
- CI/CD integration tests
- Performance benchmarks
- False positive detection

## Testing Strategy

### Test Infrastructure Requirements

1. **No Temporary Files**: All tests use in-memory fixtures
2. **Deterministic**: Tests produce same results every run
3. **Isolated**: Tests don't depend on external state
4. **Fast**: Validation tests run in milliseconds

### Test Categories

#### 1. Unit Tests
```rust
// framec/src/frame_c/v4/validator_tests.rs
mod validator_tests {
    use super::*;
    
    #[test]
    fn validates_state_exists() {
        let arcanum = build_test_arcanum();
        let validator = Validator::new(arcanum);
        let mir = MirItem::Transition { 
            target: "NonExistent".into(),
            span: test_span(10, 20)
        };
        
        let errors = validator.validate(&mir);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, "E402");
    }
}
```

#### 2. Integration Tests
```rust
// framec/tests/validation_integration.rs
#[test]
fn full_system_validation() {
    let source = include_str!("fixtures/invalid_transition.frm");
    let result = compile_with_validation(source);
    
    assert!(result.has_errors());
    assert_error_at!(result, E402, line: 15, column: 20);
}
```

#### 3. Error Message Tests
```rust
#[test]
fn error_message_clarity() {
    let source = "system S { machine: $A { go() { -> $B } } }";
    let result = validate(source);
    
    let error = &result.errors[0];
    assert!(error.message.contains("state '$B' does not exist"));
    assert!(error.help.contains("Available states: $A"));
}
```

#### 4. Source Map Tests
```rust
#[test]
fn source_map_accuracy() {
    let fixture = TestFixture::from_file("fixtures/complex_system.frm");
    let generated = fixture.compile();
    let map = generated.source_map();
    
    // Verify every Frame construct has mapping
    for line in generated.lines() {
        if line.contains("_frame_transition") {
            assert!(map.has_mapping_for_line(line.number));
        }
    }
}
```

### Test Fixtures Organization

```
framec_tests/
├── fixtures/
│   ├── validation/
│   │   ├── e402_unknown_state/
│   │   │   ├── simple.frm
│   │   │   ├── expected.json
│   │   │   └── test.rs
│   │   ├── e403_parent_forward/
│   │   └── e404_duplicate_state/
│   ├── source_maps/
│   │   ├── basic_mapping/
│   │   ├── nested_states/
│   │   └── complex_expressions/
│   └── error_translation/
│       ├── python_errors/
│       ├── typescript_errors/
│       └── rust_errors/
```

### Validation Test DSL

Create a DSL for expressing validation tests:

```rust
validation_test! {
    name: unknown_state_transition,
    frame_code: r#"
        system Light {
            machine:
                $Red { tick() { -> $Purple } }  // Error here
                $Green { }
        }
    "#,
    expect_errors: [
        E402 at line 4, column 33: "Unknown state '$Purple'"
    ],
    expect_help: "Available states: $Red, $Green"
}
```

## Migration Path

### From Current V3/V4 Code

1. **Preserve Existing Validation**: Keep current E402/E403 validation
2. **Extend Arcanum**: Add missing symbol tracking
3. **Add Source Maps**: Parallel to existing output
4. **Gradual Error Enhancement**: Add context to errors over time
5. **Deprecate Heterogeneous AST**: Remove native parsing code

### Compatibility

- Source maps are additive (don't break existing code)
- Validation errors maintain same codes
- Error translation is optional
- Native validation is opt-in

## Success Metrics

1. **Validation Coverage**: 100% of Frame semantic rules validated
2. **Source Map Accuracy**: Every generated line has mapping
3. **Error Clarity**: 90% of users understand error without consulting docs
4. **Performance**: <10ms validation overhead for typical systems
5. **Maintainability**: No dependency on native parser versions

## Future Enhancements

1. **IDE Integration**: LSP server using validation infrastructure
2. **Incremental Validation**: Only revalidate changed systems
3. **Cross-File Validation**: Validate imports and multi-file projects
4. **Custom Validation Rules**: User-defined validation plugins
5. **Error Recovery**: Continue validation after errors for better reporting

## Conclusion

This pragmatic approach provides:
- **Reliable validation** of what Frame uniquely knows
- **Clear error messages** through source mapping
- **Native compiler integration** for complete validation
- **Maintainable architecture** without parser dependencies
- **User-friendly experience** with Frame-contextual errors

The key insight: Frame doesn't need to understand everything to provide value. By focusing on Frame semantics and providing excellent bridges to native tooling, we deliver a better developer experience than attempting imperfect heterogeneous validation.