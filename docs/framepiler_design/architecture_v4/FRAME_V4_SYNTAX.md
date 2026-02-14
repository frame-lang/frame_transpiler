# Frame V4 Syntax Reference

**Version**: 4.0
**Status**: Authoritative

## Overview

Frame V4 is a preprocessor for state machine code generation. It parses Frame syntax, validates Frame semantics, and generates native code while preserving user-written native code exactly.

## File Structure

```
@@target <language>

<native preamble - imports, functions, etc.>

@@system SystemName {
    <system body>
}

<native code - test harness, etc.>
```

## Directives

### @@target (Required)

Specifies the target language. Must be first non-comment line.

```
@@target python_3
@@target typescript
@@target rust
```

### @@system (Required)

Defines a Frame state machine system.

```
@@system SystemName {
    interface:
    machine:
    actions:
    operations:
    domain:
}
```

### @@persist (Optional)

Enables persistence code generation.

```
@@persist
@@system PersistentSystem {
    ...
}
```

## System Sections

Sections must appear in this order when present:

| Section | Purpose | Required |
|---------|---------|----------|
| `interface:` | Public API methods | No |
| `machine:` | State definitions | Yes |
| `actions:` | Private implementation methods | No |
| `operations:` | Utility methods | No |
| `domain:` | Instance variables | No |

### interface:

Declares public methods that dispatch events to the state machine.

```
interface:
    methodName(param: type): returnType
    anotherMethod()
```

### machine:

Defines states and their event handlers.

```
machine:
    $StateName {
        eventName(params) {
            // native code + Frame statements
        }

        $>() {
            // enter handler
        }

        $<() {
            // exit handler
        }
    }
```

### actions:

Private methods callable from handlers.

```
actions:
    doSomething() {
        // native code
    }
```

### operations:

Utility methods (not routed through state machine).

```
operations:
    helper(): type {
        // native code
    }
```

### domain:

Instance variables with optional initialization.

```
domain:
    var count: int = 0
    var name: string = "default"
    var items: list = []
```

## State Syntax

### State Declaration

```
$StateName {
    // handlers
}
```

### Hierarchical State (HSM)

```
$Child => $Parent {
    // Child inherits unhandled events from Parent
}
```

### Event Handler

```
eventName(param1: type1, param2: type2): returnType {
    // handler body
}
```

### Enter Handler

Called when entering this state.

```
$>() {
    // initialization code
}

$>(param: type) {
    // with parameters from transition
}
```

### Exit Handler

Called when leaving this state.

```
$<() {
    // cleanup code
}
```

## Frame Statements

Frame statements appear within handler bodies alongside native code.

### Transition

Change state with exit/enter handler calls.

```
-> $TargetState
-> $TargetState(enterArg1, enterArg2)
```

### Change State

Change state without calling exit/enter handlers.

```
->> $TargetState
```

### Forward to Parent (HSM)

Delegate event to parent state handler.

```
=> $^
```

### State Stack Push

Push current state onto stack before transition.

```
$$[+]
-> $OtherState
```

### State Stack Pop

Pop and transition to stacked state.

```
$$[-]
```

## Native Code

Native code passes through the Frame compiler unchanged.

### In Preamble

```
@@target python_3

import json
from typing import List, Dict

def helper_function(x):
    return x * 2

@@system MySystem {
    ...
}
```

### In Handler Bodies

```
$Ready {
    process(data: dict) {
        # This is native Python - preserved exactly
        result = json.loads(data)
        for item in result:
            print(f"Processing: {item}")

        # Frame statement
        -> $Done
    }
}
```

### After System

```
@@system MySystem {
    ...
}

# Native test harness
if __name__ == '__main__':
    s = MySystem()
    s.run()
```

## Types

Types in Frame are opaque native strings. The Frame compiler does not interpret them.

```
interface:
    method(x: int, y: string[]): Map<string, number>
```

Frame passes through: `int`, `string[]`, `Map<string, number>` - all are valid.

## Complete Example

```
@@target python_3

from typing import List

@@system TrafficLight {
    interface:
        next()
        get_state(): str

    domain:
        var transitions: List[str] = []

    machine:
        $Red {
            $>() {
                self.transitions.append("-> Red")
                print("STOP")
            }

            next() {
                -> $Green
            }

            get_state(): str {
                return "Red"
            }
        }

        $Green {
            $>() {
                self.transitions.append("-> Green")
                print("GO")
            }

            next() {
                -> $Yellow
            }

            get_state(): str {
                return "Green"
            }
        }

        $Yellow {
            $>() {
                self.transitions.append("-> Yellow")
                print("CAUTION")
            }

            next() {
                -> $Red
            }

            get_state(): str {
                return "Yellow"
            }
        }
}

if __name__ == '__main__':
    light = TrafficLight()
    for _ in range(4):
        print(f"State: {light.get_state()}")
        light.next()
```

## Supported Target Languages

| Language | @@target value |
|----------|---------------|
| Python 3 | `python_3` |
| TypeScript | `typescript` |
| Rust | `rust` |
| C | `c` |
| C++ | `cpp` |
| Java | `java` |
| C# | `csharp` |
| Go | `go` |

**Priority (PRT)**: Python, Rust, TypeScript - fully tested and supported.

## Error Codes

| Code | Description |
|------|-------------|
| E001 | Parse error |
| E402 | Unknown state reference |
| E403 | Duplicate state definition |
| E405 | Parameter count mismatch |
