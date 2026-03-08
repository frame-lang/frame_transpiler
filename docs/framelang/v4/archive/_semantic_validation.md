> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame v4 Semantic Validation

## Overview

Frame v4 introduces compile-time semantic validation to enforce architectural patterns and proper system usage. This validation occurs after parsing but before code generation, working with Frame's universal syntax within blocks.

## System Instance Tracking

When Frame encounters a `@@system` annotation, it registers the variable as a system instance:

```python
@@system light = TrafficLight("red")
# Frame now knows: light is an instance of TrafficLight system
```

## Validation Rules

### 1. Interface Compliance

Only methods declared in a system's `interface:` block can be called externally:

```python
system TrafficLight {
    interface:
        timer()           # Public
        getColor(): str   # Public
    
    actions:
        updateDisplay()   # Private
    
    domain:
        color: str        # Private
}

# In another system:
@@system light = TrafficLight()
light.timer()           # ✓ Valid - in interface
light.updateDisplay()   # ✗ ERROR: Cannot call private action
light.color            # ✗ ERROR: Cannot access private domain
```

### 2. Method Existence

All method calls on system instances are validated:

```python
@@system light = TrafficLight()
light.nonExistent()    # ✗ ERROR: Method 'nonExistent' does not exist on TrafficLight
```

### 3. Access Control

Frame enforces strict encapsulation:

| Component | Internal Access | External Access |
|-----------|----------------|-----------------|
| Interface methods | ✓ Can call | ✓ Can call |
| Actions | ✓ Can call | ✗ Cannot call |
| Operations | ✓ Can call | ✗ Cannot call |
| Domain variables | ✓ Can access | ✗ Cannot access |
| Frame machinery | ✗ Cannot access | ✗ Cannot access |

### 4. State Validation

State names referenced in transitions must exist:

```python
system Example {
    machine:
        $Start {
            go() {
                -> $NextState()  # ✗ ERROR if $NextState doesn't exist
            }
        }
}
```

### 5. Parameter Matching

State transitions and enter handlers must have matching parameters:

```python
machine:
    $Start {
        go() {
            -> $Next(42, "test")  # Pass 2 arguments
        }
    }
    
    $Next {
        $>(value: int, name: str) {  # ✓ Must accept 2 arguments
            print(name + ": " + value)
        }
    }
```

## Implementation Strategy

### Symbol Table

Frame maintains a symbol table during compilation:

```rust
struct SymbolTable {
    // Variable name -> System type
    system_instances: HashMap<String, String>,
    
    // System name -> Interface methods
    interfaces: HashMap<String, HashSet<String>>,
    
    // System name -> All components
    system_metadata: HashMap<String, SystemMetadata>,
}

struct SystemMetadata {
    interface_methods: HashSet<String>,
    actions: HashSet<String>,
    operations: HashSet<String>,
    domain_vars: HashSet<String>,
    states: HashSet<String>,
}
```

### Validation Phases

1. **Parse Phase**: Build AST and identify all systems
2. **Collection Phase**: Gather all system metadata
3. **Resolution Phase**: Resolve cross-references
4. **Validation Phase**: Check all rules

### Error Reporting

Semantic validation errors include:

```
ERROR: Cannot call private action 'updateDisplay' on system instance 'light'
  --> controller.frm:15:17
   |
15 |     light.updateDisplay()
   |           ^^^^^^^^^^^^^^ 'updateDisplay' is not in TrafficLight's interface
   |
   = help: Only methods declared in 'interface:' can be called externally
```

## Context-Aware Validation

Frame tracks the current context to allow internal access:

```python
system TrafficLight {
    machine:
        $Red {
            timer() {
                updateDisplay()      # ✓ OK - internal call
                self.color = "green" # ✓ OK - internal access
            }
        }
    
    actions:
        updateDisplay() {
            self.lastChange = now() # ✓ OK - action accessing domain
        }
}
```

## Cross-File Validation

When systems reference other systems, Frame loads interface metadata:

```python
# traffic.frm
system TrafficLight {
    interface:
        timer()
        getColor(): str
}

# controller.frm
@@system light = TrafficLight()  # Frame loads TrafficLight metadata
light.timer()                     # Validates against loaded interface
```

## Type Inference

Frame tracks system types through assignments:

```python
@@system light1 = TrafficLight()
light2 = light1        # Frame infers: light2 is also TrafficLight
light2.timer()         # ✓ Valid - Frame knows the type
```

## Return Value Validation

Frame validates that interface methods return appropriate values:

```python
interface:
    calculate(x: int): int    # Must return int

machine:
    $Active {
        calculate(x: int) {
            return x * 2      # ✓ Valid - returns int to interface
        }
    }
    
actions:
    helper(x: int) {
        system.return = x * 3  # ✓ Valid - sets interface return
        return "done"          # ✓ Valid - returns string to caller
    }
```

See [Return Semantics](return_semantics.md) for complete return behavior documentation.

## Benefits

1. **Early Error Detection**: Catches architectural violations at compile time
2. **Interface Enforcement**: Ensures proper encapsulation
3. **Refactoring Safety**: Changes to interfaces are validated across all usages
4. **Documentation**: Interface blocks serve as clear API documentation
5. **IDE Support**: Validation data enables auto-completion and inline errors
