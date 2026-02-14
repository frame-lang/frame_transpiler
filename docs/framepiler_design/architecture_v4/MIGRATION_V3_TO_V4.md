# V3 to V4 Migration Guide

This document covers migrating Frame code from V3 syntax to V4 syntax.

## Quick Reference

| V3 Syntax | V4 Syntax |
|-----------|-----------|
| `module Name { }` | `@@system Name { }` |
| `fn name() { }` (standalone) | Native function (outside @@system) |
| `import X from "./file.frm"` | Native import |
| `:> $State` | Not supported - use `->` |
| `#SystemName ... ##` | `@@system SystemName { }` |
| `.frm` extension | `.frm` (or language-specific) |

## Syntax Changes

### System Declaration

**V3:**
```
module Calculator {
    ...
}
```

**V4:**
```
@@system Calculator {
    ...
}
```

### Target Declaration

**V3:**
```
@@target python
```

**V4:**
```
@@target python_3
```

Note: V4 uses `python_3` not `python`.

### Standalone Functions

**V3:**
```
fn helperFunction(x) {
    return x * 2
}

async fn asyncHelper() {
    await doSomething()
}

module MySystem {
    ...
}
```

**V4:**
```
@@target python_3

def helper_function(x):
    return x * 2

async def async_helper():
    await do_something()

@@system MySystem {
    ...
}
```

Standalone `fn` functions become native functions in the preamble.

### Frame Imports

**V3:**
```
import Collections from "./collections.frm"
import Errors from "./errors.frm"

module MySystem {
    machine:
        $Start {
            process() {
                var list = Collections.createList()
            }
        }
}
```

**V4:**

Frame imports are not supported. Options:

1. **Inline the code** - Copy required functionality into your system
2. **Use native imports** - Convert Frame modules to native modules

```
@@target python_3

# Use native Python modules instead
from collections import deque
from dataclasses import dataclass

@@system MySystem {
    machine:
        $Start {
            process() {
                items = deque()
            }
        }
}
```

### Change State Operator

**V3:**
```
:> $NewState
```

**V4:**

The `:>` operator is not supported. Use `->` (transition) or `->>` (change state without exit/enter).

```
-> $NewState     # With exit/enter handlers
->> $NewState    # Without exit/enter handlers
```

### Event Handler Syntax

**V3:**
```
$State {
    |event| {
        // handler
    }
}
```

**V4:**
```
$State {
    event() {
        // handler
    }
}
```

The pipe syntax `|event|` is not supported. Use method-style `event()`.

### Parameters

**V3:**
```
|event|[x, y]| {
    // use x, y
}
```

**V4:**
```
event(x: type, y: type) {
    // use x, y
}
```

## Feature Differences

### Not Supported in V4

| V3 Feature | V4 Alternative |
|------------|----------------|
| Frame modules (`module`) | Use `@@system` |
| Frame imports | Use native imports |
| Standalone `fn` | Native functions |
| `:>` change state | Use `->>` |
| Frame constants | Native constants |
| Frame enums | Native enums |
| Pipe event syntax | Method syntax |

### New in V4

| Feature | Description |
|---------|-------------|
| `@@persist` | Persistence code generation |
| Native type passthrough | Any native type syntax works |
| HSM `=> $^` | Forward to parent state |
| State stack `$$[+]` / `$$[-]` | Push/pop state |

## Migration Process

### Step 1: Update Target

```diff
- @@target python
+ @@target python_3
```

### Step 2: Convert Module to System

```diff
- module Calculator {
+ @@system Calculator {
```

### Step 3: Move Standalone Functions to Preamble

Move `fn` declarations outside `@@system` and convert to native syntax.

### Step 4: Replace Frame Imports

Convert Frame module imports to native imports or inline the functionality.

### Step 5: Update Event Syntax

```diff
  $State {
-     |processData|[items]| {
+     processData(items: list) {
```

### Step 6: Replace :> Operator

```diff
- :> $NewState
+ ->> $NewState
```

### Step 7: Test Compilation

```bash
./target/release/framec compile -l python_3 -o output/ your_file.frm
```

## Example Migration

### V3 Code

```
@@target python

import Errors from "./errors.frm"

fn validate(data) {
    if not data:
        return Errors.createError("Empty data")
    return Errors.createOk(data)
}

module Processor {
    domain:
        var result = None

    interface:
        process(data)
        getResult()

    machine:
        $Idle {
            |process|[data]| {
                var validation = validate(data)
                if Errors.isOk(validation) {
                    :> $Processing
                } else {
                    print("Validation failed")
                }
            }

            |getResult|| {
                return self.result
            }
        }

        $Processing {
            |process|[data]| {
                self.result = data.upper()
                -> $Done
            }
        }

        $Done {
            |getResult|| {
                return self.result
            }
        }
}
```

### V4 Code

```
@@target python_3

def validate(data):
    if not data:
        return {"ok": False, "error": "Empty data"}
    return {"ok": True, "value": data}

@@system Processor {
    domain:
        var result = None

    interface:
        process(data)
        getResult()

    machine:
        $Idle {
            process(data) {
                validation = validate(data)
                if validation["ok"]:
                    ->> $Processing
                else:
                    print("Validation failed")
            }

            getResult() {
                return self.result
            }
        }

        $Processing {
            process(data) {
                self.result = data.upper()
                -> $Done
            }
        }

        $Done {
            getResult() {
                return self.result
            }
        }
}
```

## Test Migration

V3 tests can be migrated by:

1. Updating syntax as described above
2. Converting Frame module tests to native tests
3. Running through V4 compiler

Tests using only `@@system` syntax with native code in handlers should work with minimal changes.
