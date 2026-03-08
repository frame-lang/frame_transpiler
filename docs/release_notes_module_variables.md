# Module Variables with Automatic Global Declaration Generation

## Release: v0.31 Enhancement
**Date**: January 2025  
**Impact**: Major improvement for Python code generation

## Overview

Frame v0.31 introduces comprehensive support for module-level variables with automatic scope management. The transpiler now intelligently generates `global` declarations in Python when module variables are modified within functions or system states, eliminating a common source of runtime errors.

## Features

### 1. Automatic Global Declaration Generation

When a module variable is modified in a function or system state, the transpiler automatically generates the necessary `global` declaration:

```frame
// Frame source
var counter = 0

fn increment() {
    counter = counter + 1  // Modification detected
}
```

```python
# Generated Python
counter = 0

def increment():
    global counter  # Auto-generated!
    counter = counter + 1
```

### 2. System State Support

The same automatic generation works for system state methods:

```frame
system Monitor {
    machine:
        $Active {
            update() {
                counter = counter + 10  // System modifying module var
            }
        }
}
```

```python
class Monitor:
    def __monitor_state_Active(self, __e, compartment):
        global counter  # Auto-generated for systems too!
        if __e._message == "update":
            counter = counter + 10
```

### 3. Intelligent Detection

The transpiler uses a two-pass approach:
1. First pass: Identifies all local variable declarations
2. Second pass: Detects module variable modifications (excluding shadowed locals)

This handles complex cases including:
- CallChainExprT assignments (e.g., `counter = counter + 1`)
- Multiple modifications in a single function
- Nested function calls

### 4. Conditional Import Generation

Import statements are only generated when actually used:
- `from enum import Enum` only appears if enums are defined
- Reduces unnecessary imports in generated code

### 5. Shadowing Protection

For the Python target, local variables cannot shadow module variables:
- Detected at transpilation time
- Provides clear error messages
- Prevents Python's UnboundLocalError

## Technical Implementation

### Detection Algorithm

The implementation in `python_visitor.rs` includes:

```rust
// Collect global assignments from statements
fn collect_global_assignments(&mut self, statements: &Vec<DeclOrStmtType>) {
    // Two-pass approach
    let mut local_vars = HashSet::<String>::new();
    
    // First pass: Find local declarations
    for stmt in statements {
        if let DeclOrStmtType::VarDeclT { var_decl_t_rcref } = stmt {
            local_vars.insert(var_decl_t_rcref.borrow().name.clone());
        }
    }
    
    // Second pass: Find module variable modifications
    for stmt in statements {
        self.collect_global_assignments_from_stmt(stmt);
    }
    
    // Remove shadowed variables
    for local_var in &local_vars {
        self.global_vars_in_function.remove(local_var);
    }
}
```

### Support for v0.30 Syntax

The implementation properly handles Frame v0.30's CallChainExprT for assignments:

```rust
if let ExprType::CallChainExprT { call_chain } = &**lhs {
    if let Some(first_node) = call_chain.first() {
        if let CallChainNodeType::UndeclaredIdentifierNodeT { name, .. } = &first_node.call_chain_node_type {
            // Check if it's a module variable
            if self.is_module_variable(name) {
                self.global_vars_in_function.insert(name.clone());
            }
        }
    }
}
```

## Testing

Comprehensive test coverage includes:
- `test_module_scope_variables.frm` - Basic module variable access
- `test_module_scope_comprehensive.frm` - Full feature validation
- Test suite improvement: 97.6% pass rate (161/165 tests)

## Migration Guide

### For Existing Code

No changes required! The transpiler automatically handles global declarations:

**Before** (manual global declarations):
```python
def my_function():
    global counter  # Had to add manually
    counter = counter + 1
```

**After** (automatic generation):
```frame
fn my_function() {
    counter = counter + 1  // Just write Frame naturally!
}
```

### Known Limitations

1. **Python Shadowing**: Due to Python's scoping rules, true variable shadowing is not supported. This code will error:
```frame
var module_var = "global"

fn test() {
    print(module_var)      // Read module variable
    var module_var = "local"  // ERROR: Cannot shadow
}
```

2. **Supported Targets**: Currently only implemented for Python target

## Benefits

1. **Eliminates Runtime Errors**: No more UnboundLocalError in Python
2. **Cleaner Frame Code**: No need to think about Python's global keyword
3. **Consistent Behavior**: Works identically for functions and systems
4. **Better Developer Experience**: Write natural Frame code, get correct Python

## Future Enhancements

- Extend to other target languages (C++, JavaScript, etc.)
- Support for `nonlocal` keyword for nested functions
- Enhanced shadowing detection with better error messages

## Acknowledgments

This feature was implemented to address common issues when transpiling Frame's module-level variables to Python, ensuring generated code runs correctly without manual intervention.