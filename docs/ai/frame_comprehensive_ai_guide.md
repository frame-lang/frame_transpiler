# Frame Language Comprehensive AI Guide
*The Authoritative Reference for AI Systems Working with Frame v0.78.14*

## 🎯 Executive Summary

Frame is a state machine language that transpiles to Python (primary target) and other languages. It combines Python-like syntax with state machine primitives to create event-driven, state-centric programs. This guide provides everything an AI system needs to understand, generate, and debug Frame code.

**Key Points:**
- Frame transpiles to Python - it's not interpreted
- State machines are first-class citizens with special syntax
- Python-like syntax with state machine extensions (`$State`, `->`, events)
- 99.5% test success rate as of v0.78.14 with 374/376 tests passing
- Cross-system static method calls now work correctly

## 📋 Table of Contents

1. [Language Philosophy](#language-philosophy)
2. [Complete Grammar Reference](#complete-grammar-reference)
3. [Code Generation Rules](#code-generation-rules)
4. [Common Patterns](#common-patterns)
5. [Error Prevention](#error-prevention)
6. [Testing and Validation](#testing-and-validation)
7. [Version History](#version-history)

## 🌟 Language Philosophy

### Core Principles
1. **State-Centric Design**: Programs are organized around states and state transitions
2. **Event-Driven Architecture**: States respond to events (messages) with actions and transitions
3. **Python Alignment**: Syntax and operators closely match Python for familiarity
4. **Transpilation Model**: Frame generates clean, readable Python code
5. **Explicit Over Implicit**: Clear state transitions and event handling

### Mental Model
Think of Frame programs as:
- **Systems**: Containers for state machines (like classes)
- **States**: Named contexts that handle events differently
- **Events**: Messages that trigger state behavior
- **Transitions**: Explicit moves between states (`->`)
- **Actions**: Private implementation methods
- **Operations**: Public static/instance methods

## 📖 Complete Grammar Reference

### Module Structure
```frame
# Module can contain (in any order):
# - Imports (Python and Frame)
# - Module-level variables
# - Enums
# - Functions
# - Systems
# - Classes
# - Modules (nested)

# Python imports (v0.31+)
import math
import numpy as np
from typing import List, Dict
from collections import *

# Frame file imports (v0.57 infrastructure)
import Utils from "./utils.frm"
import Calculator from "./calc.frm" as Calc
import { add, multiply } from "./math.frm"

# Module-level variables
var global_counter = 0
const MAX_RETRIES = 3

# Enums (v0.32+)
enum Status {
    Active
    Inactive
    Pending
}

enum HttpCode : int {
    Ok = 200
    NotFound = 404
    ServerError = 500
}

enum Environment : string {
    Development = "dev"
    Staging = "staging"
    Production  # Auto: "Production"
}

# Functions
fn calculate(x: int, y: int) : int {
    return x + y
}

# Async functions (v0.35+)
async fn fetch_data(url: str) {
    var result = await http_get(url)
    return result
}

# Classes (v0.45+)
class Point {
    var instance_count = 0  # Class variable
    
    fn init(x: float, y: float) {  # Constructor
        self.x = x  # Instance variables
        self.y = y
        Point.instance_count = Point.instance_count + 1
    }
    
    fn distance_to(other: Point) : float {
        var dx = self.x - other.x
        var dy = self.y - other.y
        return (dx ** 2 + dy ** 2) ** 0.5
    }
    
    @staticmethod
    fn origin() : Point {
        return Point(0.0, 0.0)
    }
    
    @property
    fn magnitude() : float {
        return (self.x ** 2 + self.y ** 2) ** 0.5
    }
}

# Modules (v0.34+)
module MathUtils {
    var PI = 3.14159
    
    fn circle_area(radius: float) : float {
        return PI * radius * radius
    }
}

# Systems (state machines)
system StateMachine {
    # Blocks appear in this order (all optional):
    operations:   # Public helper methods
    interface:    # Public methods
    machine:      # State definitions
    actions:      # Private methods
    domain:       # Instance variables
}
```

### System Components

#### Interface Block
```frame
interface:
    # Public methods callable from outside
    start()                           # No params, no return
    configure(settings: Dict)         # With parameters
    get_status() : str                # With return type
    process(data) : bool = false      # With default return
    async fetch() : str                # Async method (v0.35+)
```

#### Machine Block (State Definitions)
```frame
machine:
    # State declarations and hierarchy
    $Initial => $Active => $Final     # Linear chain
    $Child => $Parent                  # Hierarchical
    
    # State with event handlers
    $Active {
        # Basic event handler
        click() {
            print("Clicked")
        }
        
        # Event with transition
        submit() -> $Processing
        
        # Event with code and transition
        update(data) {
            self._validate(data)       # Call action
            -> $Updated
        }
        
        # Event with conditional transition
        check(value) {
            if value > 0 {
                -> $Positive
            } else {
                -> $Negative
            }
        }
        
        # Special events
        $>() {                        # Enter state
            print("Entering Active")
            self._setup()
        }
        
        <$() {                        # Exit state
            print("Leaving Active")
            self._cleanup()
        }
        
        # Event forwarding to parent (hierarchical)
        unhandled() => $^
        
        # Setting interface return value
        get_data() {
            @@:return = self.data
            return
        }
        
        # Async event handler (v0.37+)
        async process() {
            var result = await self._fetch_data()
            -> $Complete
        }
    }
    
    # State with parameters (v0.55+)
    $Configured(min: int, max: int) {
        var range = max - min
        
        validate(value: int) {
            if value >= min and value <= max {
                -> $Valid
            } else {
                -> $Invalid(value)
            }
        }
    }
    
    # State transitions with arguments
    $Ready {
        configure(min: int, max: int) {
            -> $Configured(min, max)
        }
    }
```

#### Actions Block (Private Methods)
```frame
actions:
    # Private implementation methods (start with _)
    _validate(data) {
        if not data {
            self._log_error("Invalid data")
            return false
        }
        return true
    }
    
    _log_error(msg: str) {
        print(f"ERROR: {msg}")
    }
    
    # Actions can set @@:return
    _process() : int = -1 {
        if self.data {
            @@:return = 42
            return 1  # Return to caller
        }
        return 0
    }
```

#### Operations Block (Public Methods)
```frame
operations:
    # Instance methods (default)
    calculate(x: int, y: int) : int {
        return x + y
    }
    
    # Static methods
    @staticmethod
    parse(text: str) : Dict {
        # Cannot use self in static methods
        return json.loads(text)
    }
```

#### Domain Block (Instance Variables)
```frame
domain:
    var counter = 0
    var data = None
    var config = {"debug": false}
    var items = []
```

### Complete Operator Reference

#### Arithmetic Operators
```frame
x + y      # Addition
x - y      # Subtraction
x * y      # Multiplication
x / y      # Division
x // y     # Floor division (v0.40+)
x % y      # Modulo
x ** y     # Exponentiation (v0.39+)
x @ y      # Matrix multiplication (v0.40+)
-x         # Unary negation
+x         # Unary plus
```

#### Comparison Operators
```frame
x == y     # Equal
x != y     # Not equal
x < y      # Less than
x <= y     # Less than or equal
x > y      # Greater than
x >= y     # Greater than or equal
```

#### Logical Operators (Python-style only, v0.38+)
```frame
x and y    # Logical AND
x or y     # Logical OR
not x      # Logical NOT
```

#### Bitwise Operators (v0.39+)
```frame
x & y      # Bitwise AND
x | y      # Bitwise OR
x ^ y      # Bitwise XOR (v0.40+)
~x         # Bitwise NOT
x << y     # Left shift
x >> y     # Right shift
```

#### Assignment Operators
```frame
x = y      # Assignment
x += y     # Add and assign (v0.39+)
x -= y     # Subtract and assign
x *= y     # Multiply and assign
x /= y     # Divide and assign
x //= y    # Floor divide and assign
x %= y     # Modulo and assign
x **= y    # Power and assign
x &= y     # Bitwise AND and assign
x |= y     # Bitwise OR and assign
x ^= y     # Bitwise XOR and assign
x <<= y    # Left shift and assign
x >>= y    # Right shift and assign
x @= y     # Matrix multiply and assign
```

#### Membership Operators (v0.38+)
```frame
x in y     # Membership test
x not in y # Negative membership test
```

#### Identity Operators (v0.39+)
```frame
x is y     # Identity test
x is not y # Negative identity test
```

#### Special Operators
```frame
# Walrus operator (v0.56+)
if (match := search(pattern, text)) {
    print(f"Found: {match}")
}

# Unpacking (v0.34+)
var combined = [*list1, *list2]
var merged = {**dict1, **dict2}

# Star expressions (v0.54+)
var first, *rest = [1, 2, 3, 4]
var start, *middle, end = items
```

### Control Flow

#### Conditionals
```frame
# If statement
if condition {
    action()
}

# If-else
if x > 0 {
    positive()
} else {
    non_positive()
}

# If-elif-else
if x > 0 {
    positive()
} elif x < 0 {
    negative()
} else {
    zero()
}

# Inline conditions in expressions
var result = value if condition else default
```

#### Loops
```frame
# For loop
for item in collection {
    process(item)
}

# For with else (v0.51+)
for item in items {
    if item == target {
        print("Found!")
        break
    }
} else {
    print("Not found")  # Executes if no break
}

# While loop
while condition {
    action()
    if done {
        break
    }
    if skip {
        continue
    }
}

# While with else (v0.51+)
while attempts < max_attempts {
    if try_operation() {
        break
    }
    attempts = attempts + 1
} else {
    print("Max attempts reached")
}

# Enum iteration (v0.32+)
for status in StatusEnum {
    print(f"{status.name}: {status.value}")
}
```

#### Pattern Matching (v0.44+)
```frame
match value {
    case 0 {
        print("Zero")
    }
    case 1 or 2 or 3 {
        print("Small")
    }
    case [x, y] {
        print(f"Pair: {x}, {y}")
    }
    case [first, *rest] {
        print(f"List with {len(rest) + 1} items")
    }
    case {"type": "error", "code": code} {
        handle_error(code)
    }
    case x if x > 100 {
        print("Large number")
    }
    case _ {
        print("Other")
    }
}
```

### Collection Types

#### Lists
```frame
# List literals
var empty = []
var numbers = [1, 2, 3]
var mixed = [1, "two", 3.0, None]

# List operations
numbers.append(4)
numbers.insert(0, 0)
numbers.remove(2)
var last = numbers.pop()
numbers.extend([5, 6])
numbers.clear()

# List indexing and slicing
var first = numbers[0]
var last = numbers[-1]
var slice = numbers[1:3]
var reversed = numbers[::-1]

# List comprehensions (v0.34+)
var squares = [x ** 2 for x in range(10)]
var evens = [x for x in numbers if x % 2 == 0]
var flattened = [item for sublist in lists for item in sublist]
```

#### Dictionaries
```frame
# Dictionary literals
var empty = {}
var config = {"debug": true, "port": 8080}
var nested = {"user": {"name": "Alice", "age": 30}}

# Dictionary operations
config["timeout"] = 30
del config["debug"]
var port = config.get("port", 3000)
var keys = list(config.keys())
var values = list(config.values())

# Dictionary comprehensions (v0.41+)
var squared = {x: x ** 2 for x in range(5)}
var filtered = {k: v for k, v in data.items() if v > 0}
```

#### Sets
```frame
# Set literals
var empty_set = {,}  # Empty set (v0.38+)
var numbers = {1, 2, 3}
var unique = {1, 2, 2, 3, 3}  # {1, 2, 3}

# Set operations
numbers.add(4)
numbers.remove(2)
numbers.discard(5)  # No error if not present
var union = set1 | set2
var intersection = set1 & set2

# Set comprehensions (v0.41+)
var squares = {x ** 2 for x in range(10)}
```

#### Tuples
```frame
# Tuple literals
var empty = ()
var single = (1,)  # Note the comma
var pair = (1, 2)
var mixed = (1, "two", 3.0)

# Tuple unpacking (v0.52+)
var x, y = (10, 20)
var a, b = b, a  # Swap values
var first, *rest = (1, 2, 3, 4)
```

### String Features

#### String Literals (v0.40+)
```frame
# Regular strings
var text = "Hello, World!"
var quoted = 'Single quotes work too'

# F-strings (formatted strings)
var name = "Frame"
var version = 0.57
var msg = f"Welcome to {name} v{version}"
var calc = f"2 + 2 = {2 + 2}"

# Raw strings (no escape processing)
var path = r"C:\Users\Frame\Documents"
var regex = r"\d{3}-\d{4}"

# Byte strings
var data = b"Binary data"

# Triple-quoted strings (multi-line)
var doc = """This is a
multi-line string
with preserved formatting"""

# String methods (all Python methods work)
var upper = text.upper()
var lower = text.lower()
var stripped = text.strip()
var replaced = text.replace("Hello", "Hi")
var parts = text.split(", ")

# Method calls on literals (v0.41+)
var clean = "  text  ".strip()
var loud = "hello".upper()
```

### Type System

#### Type Annotations (v0.43+, confirmed working v0.55+)
```frame
# Function parameters and returns
fn process(data: str, count: int) : bool {
    return len(data) > count
}

# Variable declarations
var name: str = "Frame"
var age: int = 0
var scores: List[float] = [98.5, 87.2, 91.0]

# Class members
class Person {
    fn init(name: str, age: int) {
        self.name: str = name
        self.age: int = age
    }
}

# Type aliases (v0.56+)
type UserID = int
type Coordinate = tuple[float, float]
type Optional[T] = T | None
type Result[T, E] = tuple[bool, T | E]
```

### Error Handling (v0.49+)

```frame
# Try-except blocks
try {
    risky_operation()
} except ValueError as e {
    print(f"Value error: {e}")
} except (TypeError, KeyError) {
    print("Type or key error")
} except {
    print("Unknown error")
}

# Try-finally (cleanup without handling)
try {
    resource = acquire_resource()
    use_resource(resource)
} finally {
    release_resource(resource)
}

# Try-except-finally
try {
    process_data()
} except ProcessError as e {
    log_error(e)
    raise  # Re-raise exception
} finally {
    cleanup()
}

# Raise exceptions
if error_condition {
    raise ValueError("Invalid input")
}
```

### Context Managers (v0.47+)

```frame
# With statement
with open("file.txt") as f {
    content = f.read()
    process(content)
}

# Async with (v0.37+)
async with aiohttp.ClientSession() as session {
    var response = await session.get(url)
    var data = await response.json()
}
```

### Memory Management

#### Delete Statement (v0.50+)
```frame
# Delete variables
var x = 42
del x

# Delete list elements
var items = [1, 2, 3, 4, 5]
del items[2]      # Remove at index
del items[-1]     # Remove last
del items[1:3]    # Remove slice

# Delete dictionary entries
var config = {"a": 1, "b": 2}
del config["b"]

# Delete attributes
del obj.attribute
```

### Multiple Assignment (v0.52+)

```frame
# Basic multiple assignment
var x, y = 10, 20
var a, b, c = 1, 2, 3

# Tuple unpacking
var coords = (3, 4)
var x, y = coords

# Swapping values
var a, b = b, a

# With different types
var name, age, active = "Alice", 30, true

# Multiple variable declarations (v0.53+)
var x, y, z = 1, 2, 3  # All three variables declared
```

## 🎨 Code Generation Rules

### Naming Conventions

```frame
# Follow Python PEP 8 conventions
SystemName         # PascalCase for systems and classes
function_name      # snake_case for functions
variable_name      # snake_case for variables
CONSTANT_NAME      # UPPER_SNAKE_CASE for constants
$StateName         # Dollar prefix for states
_private_action    # Underscore prefix for private methods
```

### State Machine Best Practices

1. **State Names**: Always prefix with `$` and use PascalCase
2. **Event Handlers**: Use descriptive names, lowercase with underscores
3. **Transitions**: Place `->` on its own line or at end of handler
4. **Enter/Exit**: Use `$>()` and `<$()` for state lifecycle
5. **Parent Dispatch**: Use `=> $^` for hierarchical forwarding

### System Design Patterns

#### Basic State Machine
```frame
system Toggle {
    interface:
        toggle()
        is_on() : bool
    
    machine:
        $Off {
            toggle() -> $On
            is_on() {
                @@:return = false
                return
            }
        }
        
        $On {
            toggle() -> $Off
            is_on() {
                @@:return = true
                return
            }
        }
}
```

#### Hierarchical State Machine
```frame
system Device {
    machine:
        $PoweredOff {
            power_on() -> $PoweredOn.$Idle
        }
        
        $PoweredOn {
            power_off() -> $PoweredOff
            
            $Idle => $PoweredOn {
                start_work() -> $Working
            }
            
            $Working => $PoweredOn {
                complete() -> $Idle
                error() -> $Error
            }
            
            $Error => $PoweredOn {
                reset() -> $Idle
            }
        }
}
```

#### State with Parameters
```frame
system Counter {
    machine:
        $Counting(limit: int) {
            var count = 0
            
            increment() {
                count = count + 1
                if count >= limit {
                    -> $Complete(count)
                }
            }
        }
        
        $Complete(final: int) {
            get_result() {
                @@:return = final
                return
            }
        }
    
    interface:
        start(limit: int) {
            -> $Counting(limit)
        }
}
```

## 🚫 Error Prevention

### Common Mistakes to Avoid

#### ❌ Wrong: Missing Dollar Sign on States
```frame
# WRONG
machine:
    Initial {  # Error: Not a state
        start() -> Active
    }

# CORRECT
machine:
    $Initial {
        start() -> $Active
    }
```

#### ❌ Wrong: Using Old Operators
```frame
# WRONG
if x && y || !z {  # Error: Use Python operators
    action()
}

# CORRECT
if x and y or not z {
    action()
}
```

#### ❌ Wrong: Incorrect Return Syntax
```frame
# WRONG
$State {
    get() {
        ^(42)  # Error: Old syntax
    }
}

# CORRECT
$State {
    get() {
        @@:return = 42
        return
    }
}
```

#### ❌ Wrong: Block Order
```frame
# WRONG
system Wrong {
    domain:      # Error: Wrong order
        var x = 0
    machine:
        $State { }
}

# CORRECT
system Correct {
    machine:
        $State { }
    domain:      # Domain must be last
        var x = 0
}
```

#### ❌ Wrong: Using self in Static Methods
```frame
# WRONG
operations:
    @staticmethod
    calculate(x) {
        return self.helper(x)  # Error: No self in static
    }

# CORRECT
operations:
    @staticmethod
    calculate(x) {
        return x * 2  # No self reference
    }
```

### Parser Limitations

1. **Domain Block Position**: Must be the last block in a system
2. **Block Order**: operations → interface → machine → actions → domain
3. **Method Indexing**: `method()[0]` not supported (use intermediate variable)
4. **Type Keyword**: Reserved for type aliases, cannot call `type()` function
5. **Deleted Variables**: Cannot redeclare in same scope after `del`

## 🧪 Testing and Validation

### Test File Location
```bash
# Frame test files go here:
/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/

# Generated Python files go in same directory:
/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/
```

### Transpilation Command
```bash
# Build the transpiler
cargo build --release

# Transpile a Frame file to Python
./target/release/framec -l python_3 input.frm > output.py

# With debug output
FRAME_TRANSPILER_DEBUG=1 ./target/release/framec -l python_3 input.frm
```

### Test Runner
```bash
cd framec_tests
python3 runner/frame_test_runner.py --all --matrix --json --verbose
```

### Validation Steps
1. **Generate**: Transpile .frm to .py
2. **Execute**: Run the generated Python
3. **Verify**: Check output matches expected
4. **Report**: Document what was validated

## 📚 Version History

### Current: v0.57 (2025-09-13)
- Multi-file module system infrastructure
- Frame file import syntax (parsing phase)
- Dependency management with cycle detection
- Incremental compilation with SHA-256 caching
- 100% backward compatibility maintained

### Recent Releases

#### v0.56 (2025-01-24)
- Walrus operator (`:=`) for assignment expressions
- Type aliases with Python 3.12+ syntax
- Enhanced numeric literals (underscores, complex numbers)
- Context-sensitive `type` keyword

#### v0.55 (2025-09-12)
- State parameters fixed and working
- Type annotations confirmed functional
- @property decorator support
- 100% test success rate achieved

#### v0.54 (2025-09-12)
- Star expressions for unpacking (`*rest`)
- Collection constructor validation
- Multiple variable declarations

#### v0.53 (2025-09-12)
- Fixed collection literal parsing
- Context-aware tuple detection
- Multiple variable declarations

#### v0.52 (2025-01-29)
- Multiple assignment and tuple unpacking
- Variable swapping without temporaries

#### v0.51 (2025-09-11)
- Loop else clauses (for/while...else)

#### v0.50 (2025-09-11)
- Delete statement support

### Historical Progression
- v0.30: Multi-entity support
- v0.31: Import statements, self expression
- v0.32: Advanced enums
- v0.33: Frame Standard Library (FSL) [historical; removed in favor of FID + native modules]
- v0.34: Module system, list comprehensions
- v0.35: Async/await foundation
- v0.36: Event-handlers-as-functions
- v0.37: Async event handlers
- v0.38: Python operator alignment
- v0.39: Complete Python operators
- v0.40: Python comments, XOR, matrix mult
- v0.41: String literal method calls
- v0.42: Generators (planned)
- v0.43: Type annotations
- v0.44: Pattern matching (match-case)
- v0.45: Basic class support
- v0.46: Complete class features
- v0.47: With statement
- v0.48: Access modifiers (planned)
- v0.49: Complete error handling

## 🎯 Quick Decision Tree for AI

When generating Frame code:

1. **Is it a state machine?** → Use `system` with `machine:` block
2. **Is it a simple function?** → Use `fn name() { }`
3. **Is it a class/object?** → Use `class Name { }`
4. **Need to organize code?** → Use `module Name { }`
5. **Need state transitions?** → Use `-> $StateName`
6. **Need to return from interface?** → Use `@@:return = value`
7. **Need async operations?** → Use `async fn` and `await`
8. **Need pattern matching?** → Use `match/case` statements
9. **Need error handling?** → Use `try/except/finally`
10. **Need resource cleanup?** → Use `with` statements

## 🔧 Transpiler Behavior Notes

### Python Generation
- Frame generates Python 3.x compatible code
- Uses classes for systems with `FrameEvent` for events
- Compartments track state and variables
- Router handles event dispatch
- State methods become class methods
- Interface methods are public
- Actions have underscore prefix (private)
- Operations can be static or instance methods

### Important Implementation Details
1. **Two-Pass Parsing**: Symbol table built first, then code parsed
2. **Greedy Token Matching**: `@@:return` matched as single token
3. **Context Tracking**: Parser tracks function vs system context
4. **Scope Isolation**: Functions cannot access system internals
5. **Auto-Global Generation**: Module variables get `global` declarations

## 📝 Final Checklist for AI Systems

When working with Frame code:

✅ Use `$` prefix for all state names
✅ Use Python operators (`and`, `or`, `not`) not C-style
✅ Use `@@:return =` for interface returns
✅ Use `->` for state transitions
✅ Use `#` for comments, not `//`
✅ Put domain block last in systems
✅ Use proper block order in systems
✅ Don't use `self` in static methods
✅ Remember Frame transpiles to Python
✅ Test with actual transpilation, not just syntax

## 🚀 Getting Started Template

```frame
# Minimal working Frame program
system HelloWorld {
    interface:
        greet(name: str)
    
    machine:
        $Ready {
            greet(name: str) {
                print(f"Hello, {name}!")
            }
        }
}

fn main() {
    var hw = HelloWorld()
    hw.greet("World")
}
```

---

*This guide represents the complete, authoritative reference for Frame v0.57. When in doubt, test with the actual transpiler.*
