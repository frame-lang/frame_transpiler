# Frame Language Privacy Model

## Overview

Frame v0.30 follows object-oriented privacy principles where system internals (actions and operations) are private implementation details that should not be accessed directly from external contexts like standalone functions.

## Frame Privacy Semantics

### Public Interface
- **Interface Methods**: Public API of a system, accessible from external contexts
- **Access Pattern**: `system_instance.interface_method()`
- **Purpose**: Define the contract between systems and external code

### Private Implementation  
- **Actions**: Private behavior implementations, only callable from within the system
- **Operations**: Private utility methods, only callable from within the system  
- **Access Pattern**: `self.action_do()`, `self.operation()` (internal only)
- **Purpose**: Internal implementation details, not part of public API

## Comparison with Python Privacy

| Frame Context | Python Equivalent | Access Pattern | Visibility |
|---------------|-------------------|----------------|------------|
| Interface Methods | Public methods | `obj.method()` | External ✅ |
| Actions | Private methods `__method` | `self.method()` | Internal only ❌ |
| Operations | Protected methods `_method` | `self.method()` | Internal only ❌ |

## Design Principle: Encapsulation

### ✅ **Correct Usage**
```frame
fn main() {
    // Functions can only access public interfaces
    var calculator = Calculator()
    var result = calculator.compute(5, 3)  // Interface method
    print("Result: " + str(result))
}

system Calculator {
    interface:
        compute(a: int, b: int): int
        
    machine:
        $Ready {
            compute(a, b) {
                // System can access its own internals
                self.add_action(a, b)  // Action call
                var temp = self.helper_operation(a)  // Operation call
                return temp + b
            }
        }
        
    actions:
        add_action(x: int, y: int) {
            print("Adding " + str(x) + " + " + str(y))
        }
        
    operations:
        helper_operation(value: int): int {
            return value * 2
        }
}
```

### ❌ **Incorrect Usage (Should Be Prevented)**
```frame
fn main() {
    // Functions should NOT be able to access private system internals
    var result = add_action(5, 3)  // ❌ Action not accessible
    var temp = helper_operation(10)  // ❌ Operation not accessible
}
```

## Implementation Strategy

### Current Issue in Frame v0.30
The test `test_functions_simple.frm` violates encapsulation by expecting functions to directly call system actions:

```frame
fn main() {
    var result = add(5, 3)  // ❌ 'add' is a system action, should not be accessible
}

system Utils {
    actions:
        add(x: int, y: int): int {  // Private action
            return x + y
        }
}
```

**Error Generated**: `NameError: name 'add' is not defined`
**Root Cause**: Functions cannot access private system methods (correct behavior)

### Proposed Solutions

#### Option 1: Fix Test Semantics (Recommended)
Make the test follow proper Frame encapsulation:

```frame
fn main() {
    var utils = Utils()
    var result = utils.add_numbers(5, 3)  // Call through interface
}

system Utils {
    interface:
        add_numbers(x: int, y: int): int  // Public interface
        
    machine:
        $Ready {
            add_numbers(x, y) {
                self.add_action(x, y)  // Internal call to private action
            }
        }
        
    actions:
        add_action(x: int, y: int): int {  // Private implementation
            return x + y
        }
}
```

#### Option 2: Utility Function Pattern
Move shared logic to standalone functions:

```frame
fn main() {
    var result = add(5, 3)  // Call standalone utility function
}

fn add(x: int, y: int): int {  // Standalone utility function
    return x + y
}
```

## Transpiler Behavior

### Current (Correct) Behavior
- Functions cannot access system actions/operations
- Generates `NameError` when attempting direct access
- Enforces proper encapsulation boundaries

### Should NOT Change
The transpiler correctly prevents functions from accessing private system methods. This enforces good object-oriented design principles and prevents violations of encapsulation.

## Error Messages

When functions attempt to access private system methods, the transpiler should:

1. **Continue generating NameError** (current behavior is correct)
2. **Consider adding helpful error message** during transpilation:
   ```
   Error: Function 'main' cannot access private action 'add' from system 'Utils'.
   Use system interface methods instead: utils_instance.public_method()
   ```

## Design Benefits

### Encapsulation
- System internals are protected from external modification
- Clear separation between public interface and private implementation
- Prevents tight coupling between functions and system internals

### Maintainability  
- System internals can be changed without affecting external code
- Public interfaces provide stable contracts
- Easier to reason about system dependencies

### Consistency with OOP Principles
- Follows established object-oriented design patterns
- Consistent with Python's privacy conventions
- Makes Frame code more predictable for developers

## Conclusion

The current Frame v0.30 transpiler behavior is **correct** in preventing functions from accessing private system actions/operations. Tests that expect this behavior should be **fixed to follow proper encapsulation**, not the transpiler.

**Recommendation**: Update failing tests to use proper Frame semantics rather than changing the transpiler to allow encapsulation violations.

---

**Version**: Frame v0.30  
**Last Updated**: 2025-01-28  
**Status**: Design Decision - Maintain Current Privacy Model