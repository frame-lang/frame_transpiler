# Frame Language Syntax Contexts Design Document

## Overview

Frame v0.30 supports multiple entity types within modules, including functions, operations, and actions. This document defines the expected syntax behavior and parsing rules for each context to ensure consistent language semantics.

## Entity Types and Contexts

### 1. Functions (`fn main() { ... }`)

**Purpose**: Standalone utility functions that operate outside the state machine context.

**Characteristics**:
- **Scope**: Global module scope, accessible from anywhere in the module
- **Self Reference**: No `self` parameter or reference
- **State Access**: Cannot access state machine state, compartments, or domain variables
- **Transitions**: **Prohibited** - Functions cannot contain state transitions
- **Method Calls**: All method calls are external (no `self.` prefix)
- **Return Handling**: Direct return values, no return stack manipulation

**Example**:
```frame
fn utility() {
    print("Hello from utility function")
    var result = calculate(10, 20)
    return result
}
```

### 2. Operations (`operations: { ... }`)

**Purpose**: Reusable business logic methods within a system that can be called from event handlers or other operations.

**Characteristics**:
- **Scope**: System scope, accessible via `self.` prefix from within the system
- **Self Reference**: Always have implicit `self` parameter when called
- **State Access**: Can access domain variables via `self.`
- **Transitions**: **Prohibited** - Operations cannot contain state transitions
- **Method Calls**: Internal calls use `self.` prefix, external calls don't
- **Return Handling**: Can return values directly
- **Static Support**: Can be marked with `@staticmethod` attribute

#### Static vs Non-Static Operations

**Non-Static Operations**:
```frame
operations:
    calculateTotal(items) {
        var total = 0
        for item in items {
            total += item.price
        }
        return total
    }
```
- **Generated**: `def calculateTotal(self, items):`
- **Called as**: `self.calculateTotal(items)`
- **Access**: Can access domain variables and other instance methods

**Static Operations**:
```frame
operations:
    @staticmethod
    validateInput(data) {
        print("Validating input data")
        return data.length > 0
    }
```
- **Generated**: `@staticmethod def validateInput(data):`
- **Called as**: `SystemName.validateInput(data)` (external) or `validateInput(data)` (internal)
- **Access**: Cannot access domain variables or instance methods

### 3. Actions (`actions: { ... }`)

**Purpose**: State machine behavior implementations that handle specific state transitions and side effects.

**Characteristics**:
- **Scope**: System scope, typically called from event handlers
- **Self Reference**: Always have implicit `self` parameter
- **State Access**: Can access domain variables via `self.`
- **Transitions**: **Prohibited** - Actions cannot contain state transitions (handled by event handlers)
- **Method Calls**: Internal calls use `self.` prefix, external calls don't
- **Return Handling**: No return values (actions are side effects)
- **Naming Convention**: Generated with `_do` suffix (e.g., `start_do()`)

**Example**:
```frame
actions:
    initialize() {
        print("System initializing")
        self.setupDefaults()
        self.logStartup()
    }
```

## Syntax Consistency Requirements

### Function Calls

Function calls must parse identically across all contexts:
- `print("hello")` → Always parsed as function call
- `obj.method()` → Always parsed as method call chain
- `self.operation()` → Always parsed as method call chain (in operations/actions)

### Variable Declarations

Consistent across all contexts:
```frame
var name = "value"
const MAX_SIZE = 100
```

### Control Flow

Consistent across all contexts:
```frame
if condition {
    // statements
} elif other_condition {
    // statements  
} else {
    // statements
}

for item in collection {
    // statements
}
```

### Method Resolution Rules

#### Internal Method Calls (Operations/Actions)

1. **Action Calls**: `actionName()` → `self.actionName_do()`
2. **Operation Calls**: `operationName()` → `self.operationName()`
3. **Static Operation Calls**: `staticOperation()` → `ClassName.staticOperation()`

#### External Method Calls (All Contexts)

1. **Object Methods**: `obj.method()` → `obj.method()`
2. **System Constructor**: `SystemName()` → `SystemName()`
3. **Built-in Functions**: `print()` → `print()`

## Implementation Requirements

### Parser Context Flags

The parser maintains context flags to enforce syntax rules:

```rust
is_function_scope: bool      // Functions context
operation_scope_depth: i32   // Operations nesting depth
is_action_scope: bool        // Actions context
```

### Expression Parsing Consistency

The expression parsing must maintain consistency across all contexts:
- **Function calls** must always parse as call expressions
- **Expression lists** should only represent actual parameter lists or tuples
- **Call chains** must maintain consistent node structure

### Code Generation Rules

#### Function Generation
```python
def functionName(param1, param2):
    # Direct implementation
    return result
```

#### Operation Generation
```python
def operationName(self, param1, param2):
    # Can access self.domain_vars
    return result

@staticmethod  
def staticOperationName(param1, param2):
    # No self parameter
    return result
```

#### Action Generation
```python
def actionName_do(self, param1, param2):
    # Side effects only
    # No return value
```

## Error Prevention

### Prohibited Constructs

1. **Transitions in Functions/Operations/Actions**: State transitions (`-> $State`) are only allowed in event handlers
2. **Self Access in Functions**: Functions cannot access `self` or domain variables
3. **Return Values in Actions**: Actions perform side effects and should not return values
4. **State Access in Static Operations**: Static operations cannot access instance state

### Validation Rules

1. **Context Validation**: Parser must validate that constructs are used in appropriate contexts
2. **Scope Validation**: Symbol resolution must respect scope boundaries
3. **Type Validation**: Method calls must resolve to valid targets based on context

## Migration and Compatibility

### v0.20 to v0.30 Changes

- **Multiple Functions**: v0.30 supports multiple functions per module (not just `main()`)
- **Peer Entities**: Functions and systems are now peer entities within modules
- **Scope Isolation**: Clearer separation between function, operation, and action scopes

### Backward Compatibility

- All v0.20 syntax remains valid in v0.30
- Single-function modules continue to work
- Existing system definitions maintain compatibility

## Testing Strategy

### Parser Testing
- Verify function calls parse identically across all contexts
- Test static vs non-static operations
- Validate scope-based error reporting

### Code Generation Testing  
- Ensure consistent Python output across contexts
- Verify method resolution correctness
- Test static decorator application

### Integration Testing
- End-to-end validation of generated code execution
- Cross-context method calling
- Error handling and validation

---

**Version**: Frame v0.30
**Last Updated**: 2025-01-28