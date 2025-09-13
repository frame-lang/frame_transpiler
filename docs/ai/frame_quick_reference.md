# Frame Language Quick Reference Card
*AI-Optimized Cheat Sheet for Frame v0.57*

## 🎯 One-Line Summary
Frame is a Python-like state machine language where systems contain states ($Name) that respond to events with transitions (->).

## ⚡ Instant Examples

### Minimal System
```frame
system Light {
    machine:
        $Off {
            turnOn() -> $On
        }
        $On {
            turnOff() -> $Off
        }
}
```

### Function
```frame
fn add(x, y) {
    return x + y
}
```

### Class
```frame
class Point {
    fn init(x, y) {
        self.x = x
        self.y = y
    }
}
```

## 🔑 Essential Syntax

### Declarations
```frame
var x = 42                  # Variable
const PI = 3.14            # Constant  
fn name() { }              # Function
system Name { }            # State machine
class Name { }             # Class
enum Color { Red, Blue }   # Enum
module Utils { }           # Module
```

### State Machine Core
```frame
$State {                   # State block
    event() -> $Next       # Event handler with transition
    $>() { }              # Enter state
    <$() { }              # Exit state
    => $^                 # Forward to parent
}
```

### Control Flow
```frame
if x > 0 { } elif { } else { }
for x in items { }
while condition { }
match value { case 1 { } }
return value
break / continue
```

### Operators Priority
```frame
# Must use Python style:
and, or, not              # NOT: &&, ||, !
None                      # NOT: null, nil
# Comments                # NOT: // Comments
```

### Collections
```frame
[1, 2, 3]                # List
{"k": "v"}               # Dict
{1, 2, 3}                # Set
{,}                      # Empty set
(1, 2, 3)                # Tuple
```

### Imports
```frame
import math                           # Python
import Utils from "./utils.frm"      # Frame file
```

## 🚨 Critical Rules

1. **State names start with $**: `$Active` not `Active`
2. **Events use ()**: `click()` not `|click|`
3. **Transitions use ->**: `-> $Next` not `=> $Next`
4. **Python operators only**: `and` not `&&`
5. **Hash comments only**: `#` not `//`
6. **None not null**: Use `None` not `null`/`nil`
7. **Constructor is init**: Method named `init` not `__init__`

## 📏 Block Order (Must Follow)
```frame
system Name {
    interface:      # 1. Public API (optional)
    machine:        # 2. States (optional)
    actions:        # 3. Helpers (optional)
    operations:     # 4. Methods (optional)
    domain:         # 5. Variables (optional)
}
```

## 🎨 Code Templates

### Timer Pattern
```frame
system Timer {
    interface:
        start()
        stop()
    
    machine:
        $Idle {
            start() {
                self.reset()
                -> $Running
            }
        }
        
        $Running {
            tick() {
                self.count = self.count + 1
                if self.count >= self.limit {
                    -> $Complete
                }
            }
            
            stop() -> $Idle
        }
        
        $Complete {
            $>() {
                print("Timer complete!")
            }
        }
    
    actions:
        reset() {
            self.count = 0
        }
    
    domain:
        var count = 0
        var limit = 10
}
```

### Async Pattern
```frame
async fn fetch_data(url) {
    var response = await http_get(url)
    return response.json()
}

system AsyncSystem {
    interface:
        async process(id)
    
    machine:
        $Ready {
            async process(id) {
                var data = await fetch_data(id)
                system.return = data
            }
        }
}
```

### Error Handling Pattern
```frame
$Processing {
    process(data) {
        try {
            var result = self.validate(data)
            -> $Success
        } except ValueError {
            -> $Error
        }
    }
}
```

## 🔍 Type Reference

### Built-in Types
- `int`, `float`, `str`, `bool`
- `list`, `dict`, `set`, `tuple`
- `None`

### Type Annotations
```frame
var x: int = 42
fn add(a: float, b: float) -> float { }
type Point = tuple[float, float]
```

## 📊 Version Feature Map

| Version | Key Features |
|---------|-------------|
| v0.57 | Multi-file imports |
| v0.56 | Walrus operator := |
| v0.45 | Classes |
| v0.44 | Pattern matching |
| v0.43 | Type annotations |
| v0.42 | Generators |
| v0.40 | # comments, f-strings |
| v0.38 | and/or/not operators |
| v0.35 | async/await |
| v0.34 | Modules, comprehensions |
| v0.31 | Import statements |

## 🔴 Never Use (Deprecated)

```frame
# ALL REMOVED - Will cause errors:
^(value)          # Old return
|event|           # Old event  
&&, ||, !         # C-style logical
null, nil         # Old null
// comment        # C-style comment
-interface-       # Old block syntax
#[attribute]      # Old attributes
```

## ✅ Validation Checklist

- [ ] All states start with $
- [ ] All events have ()
- [ ] Using Python operators (and/or/not)
- [ ] Using # for comments
- [ ] Using None not null
- [ ] Blocks in correct order
- [ ] No unreachable code after ->
- [ ] Constructor named init
- [ ] Static methods marked @staticmethod

## 🎯 Generation Strategy

1. Start with `system Name { }`
2. Add `machine:` block with states
3. Define state transitions with events
4. Add `interface:` for public API
5. Add `domain:` for variables
6. Add `actions:` for helpers
7. Use modern Python syntax throughout

---
*Optimized for AI parsing. Each section stands alone for quick context injection.*