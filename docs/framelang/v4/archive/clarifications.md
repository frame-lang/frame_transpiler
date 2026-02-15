# Frame v4 Architecture Clarifications

## Key Decision: Native Code in Blocks

Frame v4 uses **native language syntax within code blocks**. Frame provides the structural framework for state machines while allowing developers to write implementation code in their target language.

## What Uses Native Syntax

### Within System Blocks
- **Variables**: Language-specific (`let`/`const` for JS/TS, plain assignment for Python, etc.)
- **Functions**: Native syntax in operations/actions blocks
- **Control Flow**: Native language constructs
- **Operators**: Native language operators
- **State Transitions**: Frame-specific: `-> $State()`, `=> $^`, `$$[+]`, `$$[-]`

### Example
```python
system Calculator {
    operations:
        add(x, y) {
            # Native Python code
            result = x + y
            if result > 100:
                return 100
            return result
        }
}
```

This allows developers to use familiar language features and idioms.

## What Uses Frame Syntax

### Frame-Specific Constructs
- **System Structure**: `system Name { ... }`
- **State Definitions**: `$StateName { ... }`
- **State Transitions**: `-> $State()`, `=> $^`
- **Stack Operations**: `$$[+]`, `$$[-]`
- **Section Headers**: `interface:`, `machine:`, `actions:`, `operations:`, `domain:`

### Example
```python
@@target python

# Native Python imports
import json
from datetime import datetime
import numpy as np

system DataProcessor {
    operations:
        process() {
            # Native Python code
            data = [1, 2, 3]
            return data
        }
}
```

## Why This Approach?

### Benefits
1. **Language Familiarity**: Developers write in their native language
2. **Native Integration**: Full access to language features and libraries
3. **Clear Structure**: Frame provides consistent state machine organization
4. **Debugging**: Frame Debug Adapter provides state-aware debugging

### Trade-offs Accepted
- Code is not portable between languages (must rewrite for each target)
- Different languages may have different capabilities
- Frame must understand multiple language syntaxes

## Implementation Notes

### Transpilation Process
1. **Scanner**: Identifies Frame constructs within blocks
2. **Parser**: Builds AST using Frame grammar
3. **Validator**: Ensures Frame syntax correctness
4. **Code Generator**: Converts Frame syntax to target language

### Frame Structure Elements

| Frame Element | Description |
|--------------|-------------|
| `system Name { }` | Defines a Frame system |
| `$StateName { }` | Defines a state |
| `-> $Target()` | State transition |
| `=> $^` | Event forwarding |
| `$$[+]` | Stack push |
| `$$[-]` | Stack pop |
| `interface:` | Public API section |
| `machine:` | State machine section |
| `actions:` | Private methods section |
| `operations:` | Internal helpers section |
| `domain:` | State variables section |

## Future Considerations

### Potential Enhancements
- **Gradual Native Support**: Allow opt-in native syntax in specific contexts
- **Language Extensions**: Frame-specific constructs that enhance state machines
- **Type System**: Optional Frame type annotations for better validation

### Debugging Strategy
The Frame Debug Adapter Protocol will:
1. Map Frame source positions to generated code
2. Provide Frame-aware breakpoints and stepping
3. Show state machine visualization during execution
4. Handle Frame-to-native variable mapping

## Return Statement Behavior

Frame uses **contextual return semantics** that do the expected thing:

### Event Handlers
```python
machine:
    $Active {
        process(data) {
            # This returns to the interface
            return processedData
        }
    }
```

### Actions and Operations
```python
actions:
    helper(x) {
        # This returns to the direct caller
        return x * 2
    }
```

### system.return
```python
actions:
    validateAndProcess(data) {
        # Set interface return value
        system.return = {"status": "success"}
        # Return different value to caller
        return true
    }
```

## Summary

Frame v4 **is** a native-first approach. It uses native language syntax within blocks while Frame provides the state machine structural framework. This provides:

- **Familiarity**: Developers use their native language
- **Integration**: Full native library and feature support
- **Structure**: Consistent state machine organization via Frame
- **Validation**: Semantic checking of system interactions
- **Intuitive Returns**: Contextual return behavior that does what's expected