# Frame Language AI Guide
*Version 0.57 - Comprehensive Grammar and Code Style Reference for AI Systems*
*Last Updated: January 25, 2025*

## 🎯 Purpose
This guide is specifically designed for AI systems (LLMs, code assistants, etc.) to quickly understand Frame's grammar, generate correct Frame code, and assist developers effectively. Frame is a state machine language that transpiles to Python (and other targets).

## 📚 Core Concepts

### Frame Philosophy
- **State-Centric**: Programs are organized around states and transitions
- **Event-Driven**: States respond to events (messages)
- **Transpiled**: Frame code generates Python, not interpreted directly
- **Modern Syntax**: Python-like with state machine extensions

## 🔤 Grammar Quick Reference

### Module Structure
```frame
# Comments use hash symbols (v0.40+)
import math                           # Python import
import Module from "./module.frm"    # Frame import (v0.57)

# Module-level declarations
var global_counter = 0               # Module variable
enum Status { Active, Inactive }     # Enum declaration

# Functions (standalone)
fn calculate(x, y) {
    return x + y
}

# Systems (state machines)
system TrafficLight {
    # System blocks (optional, in order)
    interface:
        start()
        stop()
    
    machine:
        $Red => $Green => $Yellow => $Red
        
        $Red {
            start() -> $Green
        }
    
    actions:
        log(msg) {
            print(msg)
        }
    
    domain:
        var timer = 0
}

# Classes (v0.45+)
class Point {
    fn init(x, y) {        # Constructor (special name)
        self.x = x
        self.y = y
    }
    
    fn distance() {
        return (self.x ** 2 + self.y ** 2) ** 0.5
    }
}
```

### State Machine Syntax

#### States and Transitions
```frame
machine:
    # State declaration with optional hierarchy
    $Initial => $Active => $Final
    
    # State with event handlers
    $Active {
        # Event handler with transition
        click() -> $Final
        
        # Event with parameters
        update(data) {
            self.process(data)
            -> $Processing
        }
        
        # Enter/Exit events
        $>() {              # Enter state
            print("Entering Active")
        }
        
        <$() {              # Exit state  
            print("Leaving Active")
        }
    }
    
    # Hierarchical states (child => parent)
    $ChildState => $ParentState {
        # Forward event to parent
        unhandled() => $^
    }
```

### Event Handlers

#### Handler Patterns
```frame
$State {
    # Simple handler with immediate transition
    event() -> $NextState
    
    # Handler with code block
    event(param) {
        var result = self.calculate(param)
        if result > 0 {
            -> $Success
        } else {
            -> $Failure
        }
    }
    
    # Handler with return value (interface method)
    query() {
        system.return = self.data
        return
    }
    
    # Async handler (v0.37+)
    async process(id) {
        var result = await fetch_data(id)
        system.return = result
    }
}
```

### Control Flow

#### Conditionals
```frame
# If/elif/else
if x > 0 {
    positive_action()
} elif x < 0 {
    negative_action()
} else {
    zero_action()
}

# Ternary (inline conditional)
var result = condition ? true_value : false_value
```

#### Loops
```frame
# For loop
for item in collection {
    process(item)
}

# For with range
for i in range(10) {
    print(str(i))
}

# While loop
while condition {
    update()
}

# Loop with else clause
for x in items {
    if found(x) {
        break
    }
} else {
    print("Not found")
}
```

#### Pattern Matching (v0.44+)
```frame
match value {
    case 0 { 
        return "zero" 
    }
    case 1 or 2 or 3 { 
        return "small" 
    }
    case [first, *rest] {
        return "list with " + str(first)
    }
    case {"key": val} {
        return "dict with key=" + str(val)
    }
    case x if x > 100 {
        return "large"
    }
    case _ {
        return "other"
    }
}
```

### Data Types and Literals

#### Collections
```frame
# Lists
var list = [1, 2, 3]
var empty_list = []
list.append(4)
var first = list[0]
var last = list[-1]
var slice = list[1:3]

# Dictionaries  
var dict = {"key": "value", "count": 42}
var empty_dict = {}
dict["new_key"] = "new_value"
var value = dict["key"]

# Sets
var set = {1, 2, 3}
var empty_set = {,}    # Special empty set syntax (v0.38+)
set.add(4)

# Tuples
var tuple = (1, 2, 3)
var single = (1,)      # Single element tuple
```

#### Comprehensions (v0.34+)
```frame
# List comprehension
var squares = [x * x for x in range(10)]
var filtered = [x for x in items if x > 0]

# Dict comprehension
var map = {k: v * 2 for k, v in original.items()}

# Set comprehension (v0.41+)
var unique = {x.lower() for x in words}
```

#### String Literals (v0.40+)
```frame
var plain = "Hello"
var fstring = f"Hello {name}"           # F-string interpolation
var raw = r"C:\Users\path"              # Raw string (no escapes)
var bytes = b"binary data"              # Byte string
var multi = """Multi-line
string literal"""                        # Triple-quoted
```

### Operators

#### Arithmetic
```frame
+   # Addition
-   # Subtraction  
*   # Multiplication
/   # Division
//  # Floor division (v0.40+)
%   # Modulo
**  # Exponentiation (v0.38+)
@   # Matrix multiplication (v0.40+)
```

#### Comparison
```frame
==  # Equal
!=  # Not equal
<   # Less than
>   # Greater than
<=  # Less than or equal
>=  # Greater than or equal
```

#### Logical (v0.38+)
```frame
and  # Logical AND (not &&)
or   # Logical OR (not ||)
not  # Logical NOT (not !)
```

#### Bitwise (v0.39+)
```frame
&   # Bitwise AND
|   # Bitwise OR
^   # Bitwise XOR (v0.40+)
~   # Bitwise NOT
<<  # Left shift
>>  # Right shift
```

#### Membership & Identity (v0.38+)
```frame
in      # Membership test
not in  # Not in membership
is      # Identity test
is not  # Not identity
```

#### Assignment
```frame
=    # Assignment
+=   # Add and assign
-=   # Subtract and assign
*=   # Multiply and assign
/=   # Divide and assign
//=  # Floor divide and assign
%=   # Modulo and assign
**=  # Power and assign
&=   # Bitwise AND and assign
|=   # Bitwise OR and assign
^=   # Bitwise XOR and assign
<<=  # Left shift and assign
>>=  # Right shift and assign
```

#### Special
```frame
.    # Attribute access
[]   # Indexing/slicing
()   # Function call
:=   # Walrus operator (v0.56+)
*    # Unpacking operator (v0.34+)
```

### Functions and Methods

#### Function Definitions
```frame
# Basic function
fn add(a, b) {
    return a + b
}

# Async function (v0.35+)
async fn fetch_data(url) {
    var response = await http_get(url)
    return response.json()
}

# Function with default parameters
fn greet(name, greeting = "Hello") {
    print(f"{greeting}, {name}!")
}

# Lambda expressions (v0.38+)
var square = lambda x: x * x
var add = lambda x, y: x + y
```

#### Method Types
```frame
system Example {
    operations:
        # Instance method (default)
        instance_method() {
            self.value = 42
        }
        
        # Static method
        @staticmethod
        static_method() {
            return "no self access"
        }
    
    actions:
        # Action methods (internal helpers)
        helper() {
            return self.calculate()
        }
}
```

### Module System (v0.34+)

#### Module Declaration
```frame
module Utils {
    var counter = 0
    
    fn increment() {
        counter = counter + 1
        return counter
    }
    
    # Nested modules
    module Math {
        fn add(a, b) {
            return a + b
        }
    }
}

# Usage
fn main() {
    var x = Utils.increment()
    var sum = Utils.Math.add(3, 4)
}
```

### Import Statements

#### Python Imports (v0.31+)
```frame
import math
import numpy as np
from datetime import datetime
from typing import *
```

#### Frame Imports (v0.57+)
```frame
# Import Frame modules from other .frm files
import Utils from "./utils.frm"
import Calculator from "./calc.frm" as Calc
import { add, multiply } from "./math.frm"

# Module access uses :: in Frame source
fn example() {
    var result = Utils::add(5, 3)        # Frame source
    var area = Math::Constants::PI * r   # Nested module access
}
# Note: :: transpiles to . in Python output

# Compilation commands:
# Single file (concatenation - default):
framec -m main.frm -l python_3

# Separate Python files (NEW!):
framec -m main.frm -l python_3 -o ./output
# Creates: output/main.py, output/utils.py, output/__init__.py
```

### Async/Await (v0.35+)

```frame
# Async function
async fn process_batch(items) {
    var results = []
    for item in items {
        var result = await process_item(item)
        results.append(result)
    }
    return results
}

# Async in systems
system AsyncProcessor {
    interface:
        async handle(data)
    
    machine:
        $Idle {
            async handle(data) {
                var result = await self.process(data)
                system.return = result
            }
        }
}
```

### Exception Handling (v0.49+)

```frame
try {
    risky_operation()
} except ValueError as e {
    handle_value_error(e)
} except {
    handle_any_error()
} finally {
    cleanup()
}
```

### Type Annotations (v0.43+)

```frame
# Variable annotations
var count: int = 0
var name: str = "Frame"
var items: list[str] = []

# Function annotations
fn calculate(x: float, y: float) -> float {
    return x + y
}

# Type aliases (v0.56+)
type Point = tuple[float, float]
type Matrix = list[list[float]]
```

### Generators (v0.42+)

```frame
# Generator function
fn fibonacci(n) {
    var a = 0
    var b = 1
    for i in range(n) {
        yield a
        var temp = a
        a = b
        b = temp + b
    }
}

# Async generator
async fn fetch_pages(urls) {
    for url in urls {
        var page = await fetch(url)
        yield page
    }
}
```

## 💻 Code Generation Patterns

### State Machine Template
```frame
system StateMachineName {
    interface:
        # Public methods
        start()
        stop()
        process(data)
    
    machine:
        # State declarations and hierarchy
        $Initial => $Running => $Complete
        
        $Initial {
            start() -> $Running
        }
        
        $Running {
            process(data) {
                if self.validate(data) {
                    self.handle(data)
                    -> $Complete
                }
            }
            
            stop() -> $Initial
        }
        
        $Complete {
            $>() {
                self.cleanup()
            }
        }
    
    actions:
        validate(data) {
            return data != None
        }
        
        handle(data) {
            self.result = data
        }
        
        cleanup() {
            self.result = None
        }
    
    domain:
        var result = None
}
```

### Common Patterns

#### Event Loop Pattern
```frame
$Active {
    tick() {
        self.update()
        if self.done {
            -> $Complete
        }
        # Stay in Active (no transition)
    }
}
```

#### Guard Pattern
```frame
$State {
    action(param) {
        # Guard clause
        if not self.is_valid(param) {
            return
        }
        
        # Main logic
        self.process(param)
        -> $NextState
    }
}
```

#### State with Timeout
```frame
$Waiting {
    $>() {
        self.start_timer(5.0)
    }
    
    timeout() -> $Error
    
    response(data) {
        self.cancel_timer()
        -> $Success
    }
    
    <$() {
        self.cancel_timer()
    }
}
```

## 🚫 Common Mistakes to Avoid

### ❌ DON'T Use Old Syntax
```frame
# WRONG - Old v0.11 syntax
^(value)           # Old return
|event|            # Old event syntax
&&, ||, !          # C-style logical operators
null, nil          # Old null keywords
```

### ✅ DO Use Modern Syntax
```frame
# CORRECT - Modern syntax
return value       # Modern return
event()           # Modern event syntax
and, or, not      # Python logical operators
None              # Python null keyword
```

### ❌ DON'T Mix Statement Types
```frame
# WRONG - Can't mix transition with other statements after
$State {
    event() {
        -> $Next
        cleanup()  # ERROR: unreachable
    }
}
```

### ✅ DO Order Correctly
```frame
# CORRECT - Cleanup before transition
$State {
    event() {
        cleanup()
        -> $Next
    }
}
```

### ❌ DON'T Use self in Static Methods
```frame
# WRONG
@staticmethod
fn static_method() {
    self.value = 42  # ERROR: no self in static
}
```

### ✅ DO Use Static Methods Properly
```frame
# CORRECT
@staticmethod
fn static_method() {
    return 42  # No self access
}
```

## 📝 Style Guidelines

### Naming Conventions
- **Systems**: PascalCase (e.g., `TrafficLight`)
- **States**: $PascalCase (e.g., `$Active`)
- **Functions/Methods**: snake_case (e.g., `calculate_total`)
- **Variables**: snake_case (e.g., `user_count`)
- **Constants**: UPPER_SNAKE_CASE (e.g., `MAX_RETRIES`)
- **Enums**: PascalCase (e.g., `Status`)
- **Classes**: PascalCase (e.g., `Point`)

### Indentation
- Use 4 spaces (no tabs)
- Blocks are indented consistently

### Comments
```frame
# Single-line comments (v0.40+)

{-- Multi-line documentation comments
    for detailed explanations
    across multiple lines --}
```

### State Organization
1. Enter handler `$>()`
2. Event handlers (alphabetical or logical order)
3. Exit handler `<$()`

### Block Order in Systems
1. `interface:` - Public API
2. `machine:` - State definitions
3. `actions:` - Internal helpers
4. `operations:` - Additional methods
5. `domain:` - Instance variables

## 🔧 Transpilation Notes

### Python Generation
- Frame transpiles to Python 3.x
- Systems become classes
- States become methods
- Events become method calls
- Domain variables become instance variables

### Generated Structure
```python
# Frame system becomes Python class
class TrafficLight:
    def __init__(self):
        self.__state = self.__Red
        self.timer = 0  # domain variable
    
    def __Red(self, e):
        if e._message == "start":
            self.__transition("Green")
```

## 🎯 Quick Lookup Tables

### State Events
| Event | Symbol | Purpose |
|-------|--------|---------|
| Enter | `$>()` | Called when entering state |
| Exit | `<$()` | Called when leaving state |
| Forward | `=> $^` | Forward to parent state |

### Transition Operators
| Operator | Meaning | Example |
|----------|---------|---------|
| `->` | Transition to | `-> $NextState` |
| `=>` | Hierarchy | `$Child => $Parent` |
| `=> $^` | Forward to parent | `unhandled() => $^` |

### Special Variables
| Variable | Purpose | Scope |
|----------|---------|-------|
| `self` | Instance reference | Instance methods |
| `system.return` | Set interface return value | Event handlers |
| `$@` | Current event | Event handlers |

## 🚀 Advanced Features

### State Parameters (v0.49+)
```frame
$State(x, y) {
    $>() {
        print(f"Entered with {x}, {y}")
    }
}

# Transition with parameters
-> $State(10, 20)
```

### With Statement (v0.37+)
```frame
with open("file.txt") as f {
    var content = f.read()
}

async with session.get(url) as response {
    var data = await response.json()
}
```

### Walrus Operator (v0.56+)
```frame
if (n := len(items)) > 10 {
    print(f"List too long ({n} items)")
}

while (chunk := file.read(1024)) != "" {
    process(chunk)
}
```

### Destructuring (v0.54+)
```frame
# Multiple assignment
var x, y = get_point()
var a, b, c = [1, 2, 3]

# Star expressions
var first, *rest = items
var *head, last = sequence
```

## 📚 Version History Highlights

- **v0.57**: Multi-file module system infrastructure with Frame imports (100% test success)
- **v0.56**: Walrus operator, type aliases
- **v0.54**: Star expressions, collection constructors
- **v0.45**: Class support with OOP
- **v0.44**: Pattern matching with match-case
- **v0.43**: Type annotations
- **v0.42**: Generators and async generators
- **v0.41**: Set comprehensions
- **v0.40**: Python comments (#), string literals, operators
- **v0.39**: Compound assignments, bitwise, identity operators
- **v0.38**: Python logical operators (and/or/not), membership
- **v0.37**: Async event handlers, slicing
- **v0.35**: Async/await foundation
- **v0.34**: Module system, list comprehensions
- **v0.31**: Import statements, self expression

## 🤖 AI Code Generation Tips

1. **Always use modern syntax** - No v0.11 legacy constructs
2. **Check version features** - Ensure features exist in target version
3. **Follow state machine patterns** - States respond to events
4. **Use Python idioms** - Frame follows Python conventions
5. **Validate transitions** - Ensure all states are reachable
6. **Handle edge cases** - Include enter/exit handlers where needed
7. **Comment appropriately** - Use # for single-line comments
8. **Test generated code** - Ensure it transpiles and runs correctly

---

*This guide is optimized for AI systems to quickly parse and understand Frame language constructs. For human-readable documentation, see the main docs/ directory.*