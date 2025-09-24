# Frame Transpiler - Claude Context

⚠️ **CRITICAL RULE: NO WORKAROUNDS WITHOUT EXPLICIT APPROVAL**
**NEVER create workarounds for parser/transpiler issues. Always fix the actual problem in the codebase unless explicitly told to create a workaround.**

⚠️ **IMPORTANT: When starting a new session, ALWAYS read these documents first:**
1. This file (CLAUDE.md) - Project structure and conventions
2. `docs/framelang_design/dev_notes.md` - Latest development status
3. `docs/v0.34_roadmap.md` - Module system and Rust target plans
4. `framec_tests/reports/test_log.md` - Current test results

## Project Overview

Frame is a state machine language that transpiles to multiple target languages. The project has evolved through v0.20 (syntax modernization), v0.30 (multi-entity support), v0.31 (import statements and self expression enhancements), v0.32 (advanced enum features), v0.33 (Frame Standard Library), v0.34 (Complete Module System implementation with qualified names), v0.35 (async/await foundation), v0.36 (event-handlers-as-functions), v0.37 (async event handlers with runtime infrastructure), v0.38 (Python logical operators alignment), v0.39 (Python operators complete), v0.40 (Python comment syntax, bitwise XOR, and matrix multiplication), v0.41 (set comprehensions), v0.42 (generators), v0.43 (type annotations), v0.44 (comprehensive pattern matching with match-case), v0.45 (class support with OOP features), v0.46 (assert statement support), v0.47 (with statement support), v0.48 (Python-style access modifiers), v0.49 (complete error handling), v0.50 (del statement support), v0.51 (loop else clauses), v0.52 (multiple assignment), v0.53 (critical bug fixes for collections and multiple variable declarations), v0.54 (star expressions for unpacking), v0.55 (state parameters fixed, type annotations and @property confirmed working), v0.56 (Python enhancement features including walrus operator, type aliases, and enhanced numerics), v0.57 (multi-file module system infrastructure with Frame file imports), v0.58 (class decorators with Python pass-through and GraphViz multi-system support), v0.59 (source map generation for debugging support), v0.60 (critical double-call bug fix and complete AST dump feature), v0.61 (comprehensive call chain analysis and refactoring planning), v0.62 (semantic call resolution infrastructure), v0.63 (accurate semantic call resolution with Actions, Operations, and External calls correctly identified), v0.64 (visitor simplification using resolved types), v0.65 (complete visitor call unification), v0.66 (explicit self/system call syntax with semantic resolution always enabled), v0.74 (source map architecture documentation and marker file linter), v0.75 (CodeBuilder architecture with automatic line tracking and PythonVisitorV2 as default), and v0.76 (Python syntax alignment - removed 'extends' keyword for class inheritance and 'xor' keyword, now using Python-style syntax throughout).

## File Locations

### Test Files
- **CORRECT location for Frame test files**: `/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/`
- **DO NOT** create test files in the project root directory
- All `.frm` test files must go in the framec_tests/python/src/ directory

## Test Validation

⚠️ **CRITICAL: ALWAYS USE THE OFFICIAL TEST RUNNER - NEVER CREATE ONE-OFF VALIDATION SCRIPTS** ⚠️

**Use the existing test runner at:** `framec_tests/runner/frame_test_runner.py`
- DO NOT create custom validation scripts like `validate_all_tests.sh`
- DO NOT write one-off test scripts
- ALWAYS use the official runner for ALL test validation needs

**Standard test validation command:**
```bash
cd framec_tests
# Use release build for all features
python3 runner/frame_test_runner.py --all --matrix --json --verbose --framec /Users/marktruluck/projects/frame_transpiler/target/release/framec
```

## Current State

**Branch**: `v0.30`  
**Version**: `v0.76.0`  
**Status**: ✅ **100% TEST SUCCESS RATE** - CodeBuilder Architecture Complete with PythonVisitorV2 as Default

📋 **For release notes and development status, see**: [`docs/framelang_design/dev_notes.md`](docs/framelang_design/dev_notes.md)
📊 **For v0.30 achievements, see**: [`docs/v0.30_achievements.md`](docs/v0.30_achievements.md)
📊 **For v0.31 achievements, see**: [`docs/v0.31_achievements.md`](docs/v0.31_achievements.md)
📊 **For v0.32 achievements, see**: [`docs/v0.32_achievements.md`](docs/v0.32_achievements.md)
📊 **For v0.33 achievements, see**: [`docs/v0.33_achievements.md`](docs/v0.33_achievements.md)
📊 **For v0.34 achievements, see**: [`docs/v0.34_achievements.md`](docs/v0.34_achievements.md)
📊 **For v0.35 achievements, see**: [`docs/v0.35_achievements.md`](docs/v0.35_achievements.md)
📊 **For v0.36 achievements, see**: [`docs/v0.36_achievements.md`](docs/v0.36_achievements.md)
📊 **For v0.37 achievements, see**: [`docs/v0.37_achievements.md`](docs/v0.37_achievements.md)
📊 **For v0.38 achievements, see**: [`docs/v0.38_achievements.md`](docs/v0.38_achievements.md)
📊 **For v0.39 achievements, see**: [`docs/v0.39_achievements.md`](docs/v0.39_achievements.md)
📊 **For v0.40 achievements, see**: [`docs/v0.40_achievements.md`](docs/v0.40_achievements.md)
📊 **For v0.41 achievements, see**: [`docs/v0.41_achievements.md`](docs/v0.41_achievements.md)
📊 **For v0.42 achievements, see**: [`docs/v0.42_achievements.md`](docs/v0.42_achievements.md)
📊 **For v0.43 achievements, see**: [`docs/v0.43_achievements.md`](docs/v0.43_achievements.md)
📊 **For v0.44 achievements, see**: [`docs/v0.44_achievements.md`](docs/v0.44_achievements.md)
📊 **For v0.45 achievements, see**: [`docs/v0.45_achievements.md`](docs/v0.45_achievements.md)
📊 **For v0.46 achievements, see**: [`docs/v0.46_achievements.md`](docs/v0.46_achievements.md)
📊 **For v0.47 achievements, see**: [`docs/v0.47_achievements.md`](docs/v0.47_achievements.md)
📊 **For v0.48 achievements, see**: [`docs/v0.48_achievements.md`](docs/v0.48_achievements.md)
📊 **For v0.49 achievements, see**: [`docs/v0.49_achievements.md`](docs/v0.49_achievements.md)
📊 **For v0.50 achievements, see**: [`docs/v0.50_achievements.md`](docs/v0.50_achievements.md)
📊 **For v0.51 achievements, see**: [`docs/v0.51_achievements.md`](docs/v0.51_achievements.md)
📊 **For v0.52 achievements, see**: [`docs/v0.52_achievements.md`](docs/v0.52_achievements.md)
📊 **For v0.53 achievements, see**: [`docs/v0.53_achievements.md`](docs/v0.53_achievements.md)
📊 **For v0.54 achievements, see**: [`docs/v0.54_achievements.md`](docs/v0.54_achievements.md)
📊 **For v0.55 achievements, see**: [`docs/v0.55_achievements.md`](docs/v0.55_achievements.md)
📊 **For v0.56 achievements, see**: [`docs/v0.56_achievements.md`](docs/v0.56_achievements.md)
📊 **For v0.57 achievements, see**: [`docs/v0.57_achievements.md`](docs/v0.57_achievements.md)
📊 **For v0.58 achievements, see**: [`docs/v0.58_achievements.md`](docs/v0.58_achievements.md)
📊 **For v0.59 achievements, see**: [`docs/v0.59_achievements.md`](docs/v0.59_achievements.md)
📊 **For v0.60 achievements, see**: [`docs/v0.60_achievements.md`](docs/v0.60_achievements.md)
📊 **For v0.61 achievements, see**: [`docs/v0.61_achievements.md`](docs/v0.61_achievements.md)
📊 **For v0.62 achievements, see**: [`docs/v0.62_achievements.md`](docs/v0.62_achievements.md)
📊 **For v0.63 achievements, see**: [`docs/v0.63_achievements.md`](docs/v0.63_achievements.md)
📊 **For v0.64 achievements, see**: [`docs/v0.64_achievements.md`](docs/v0.64_achievements.md)
📊 **For v0.65 achievements, see**: [`docs/v0.65_achievements.md`](docs/v0.65_achievements.md)
📊 **For v0.66 achievements, see**: [`docs/v0.66_achievements.md`](docs/v0.66_achievements.md)
📊 **For v0.74 achievements, see**: [`docs/v0.74_achievements.md`](docs/v0.74_achievements.md)
📊 **For v0.75 achievements, see**: [`docs/v0.75_achievements.md`](docs/v0.75_achievements.md)
📋 **For v0.34 release notes, see**: [`docs/release_notes_v0.34.md`](docs/release_notes_v0.34.md)
📋 **For v0.34 roadmap, see**: [`docs/v0.34_roadmap.md`](docs/v0.34_roadmap.md)
📊 **For latest test results, see**: [`framec_tests/reports/test_log.md`](framec_tests/reports/test_log.md)

## Architecture

```
Frame Source (.frm) 
    ↓
Scanner (Tokenizer) → framec/src/frame_c/scanner.rs
    ↓  
Parser (First Pass: Symbol Table) → framec/src/frame_c/parser.rs
    ↓
Parser (Second Pass: Semantic Analysis) → framec/src/frame_c/semantic_analyzer.rs
    ↓
AST (FrameModule with Resolved Types) → framec/src/frame_c/ast.rs
    ↓
Visitor (Code Generation) → framec/src/frame_c/visitors/python_visitor_v2.rs (default)
                          → framec/src/frame_c/visitors/python_visitor.rs (legacy via USE_PYTHON_V1)
    ↓
Target Code (Python - 1st Class Language)
```

## v0.75 CodeBuilder Architecture

Frame v0.75 introduces the CodeBuilder architecture for robust, automatic line tracking:

### Key Components

1. **CodeBuilder Module** (`framec/src/frame_c/code_builder.rs`)
   - Automatic character-level tracking
   - Line/column position management
   - Deferred mapping with `map_next()`
   - Child builders for complex generation

2. **PythonVisitorV2** (`framec/src/frame_c/visitors/python_visitor_v2.rs`)
   - Default Python code generator
   - Uses CodeBuilder for all output
   - No manual line tracking needed
   - Perfect source mappings

3. **Legacy Support**
   - Original visitor available via `USE_PYTHON_V1=1` environment variable
   - Produces identical output for backward compatibility

### Benefits
- No more manual line counting or offset calculations
- Changes to code generation don't break source mappings  
- Character-level precision for all tracking
- Clean, maintainable architecture

## Language Support Classification

### 1st Class Language (Full Visitor Implementation)
- **Python**: Complete visitor implementation with all Frame features

### 2nd Class Languages (Design Guides, No Visitor)
Languages with generation guides and considered in Frame's design:
- **C/C++**: Procedural/OOP mapping guides
- **JavaScript**: Prototype-based OOP mapping
- **C#**: Object-oriented with interfaces
- **Java**: Class-based OOP mapping
- **Go**: Struct and interface composition
- **Rust**: Ownership-aware state machines

### 3rd Class Languages (LLM-Generated)
Other languages may be supported via LLM code generation but without guarantees

### v0.36 Event-Handlers-as-Functions Architecture - NEW ✅

Frame v0.36 introduces a fundamental architectural improvement in code generation:

**Key Features**:
- **Individual Handler Functions**: Each event handler generated as a separate function
- **State Dispatchers**: State methods become lightweight routers to handlers
- **Special Event Naming**: `$>` → `_enter`, `<$` → `_exit` for valid Python identifiers
- **Async Detection**: Individual handlers automatically detect and generate `async def` when needed
- **Configuration Flag**: `event_handlers_as_functions` in PythonConfig enables the new architecture
- **Test Coverage**: 100% success rate with all features working

### v0.35 Async/Await Support - COMPLETE ✅

Frame v0.35 adds comprehensive async/await support:

**Async Features**:
- **Async Functions**: `async fn name() { ... }` syntax
- **Async Interface Methods**: `async methodName()` in interfaces
- **Await Expressions**: `await expr` syntax and code generation
- **Async Propagation**: State handlers automatically become async when needed

### v0.34 Features - COMPLETE ✅

Frame v0.34 introduces a complete module system with list comprehensions and unpacking operator support.

#### Module System
```frame
// Named modules with functions and variables
module Utils {
    var counter = 0
    
    fn increment() {
        counter = counter + 1
        return counter
    }
}

// Using module contents
fn main() {
    var val = Utils.increment()
    print("Counter: " + str(val))
}
```

**Module Features**:
- **Named Modules**: `module ModuleName { ... }` syntax
- **Qualified Access**: `module.function()` and `module.variable`
- **Nested Modules**: Full hierarchical organization support
- **Symbol Table**: ModuleSymbol type for proper scope resolution
- **Two-Pass Parsing**: Modules enter scope in both passes

#### List Comprehensions (NEW in v0.34)
```frame
fn examples() {
    // Basic comprehension
    var squares = [x * x for x in range(10)]
    
    // With conditional filtering
    var evens = [x for x in numbers if x % 2 == 0]
    
    // Nested comprehensions
    var matrix = [[i * j for j in range(3)] for i in range(3)]
    
    // Complex expressions
    var processed = [str(x).upper() for x in items if x > 0]
}
```

#### Unpacking Operator (NEW in v0.34)
```frame
fn unpacking() {
    var list1 = [1, 2, 3]
    var list2 = [4, 5, 6]
    
    // Unpacking in list literals
    var combined = [*list1, *list2, 7, 8]
    // Result: [1, 2, 3, 4, 5, 6, 7, 8]
    
    // Multiple unpacking
    var result = [0, *list1, *list2, 99]
}
```

### Native Python Operations Support ✅

Frame v0.38 supports native Python operations directly, without requiring special imports:

#### Type Conversions
```frame
var x = 42
var s = str(x)        // "42" - works natively
var i = int("123")    // 123
var f = float("3.14") // 3.14
var b = bool(0)       // False
```

#### List Operations
```frame
// All list methods work natively
var list = [1, 2, 3]
list.append(4)           // Add to end
list.insert(1, 99)       // Insert at index
list.remove(99)          // Remove first occurrence
var last = list.pop()    // Remove and return last
list.extend([5, 6])      // Add all from another list
list.reverse()           // Reverse in place
list.sort()              // Sort in place
list.clear()            // Remove all

// Query operations
var idx = list.index(3)  // Find index
var cnt = list.count(2)  // Count occurrences
var copy = list.copy()   // Shallow copy

// Properties work directly
var len = len(list)      // Length function

// Negative indexing works!
var last_item = list[-1]

// ALL Python list methods work! Examples:
list.sort()             // Sort in place
list.index(value)       // Find index of value
list.extend([4, 5])     // Add multiple items
list.insert(0, "first") // Insert at position
// ...and many more - see grammar.md for full list
```

#### String Operations
```frame
var text = "  Hello World  "
var upper = text.upper()     // "  HELLO WORLD  "
var lower = text.lower()     // "  hello world  "
var trimmed = text.strip()   // "Hello World"
var replaced = text.replace("World", "Frame")
var parts = text.split(" ")
var len = len(text)         // Length function

// Direct Python syntax (v0.38)
var contains = "world" in text   // Membership operator
var substring = text[0:5]        // Slicing

// ALL Python string methods work! Examples:
text.find("World")           // Search for substring
text.startswith("Hello")     // Check prefix
text.endswith("!")          // Check suffix
text.count("l")             // Count occurrences
text.center(20, "*")        // Padding/alignment
// ...and many more - see grammar.md for full list
```

### v0.38 Complete Features (Session 3 - 2025-09-08)

#### Membership Operators (NEW)
```frame
// Check if item in collection
if "banana" in fruits {
    print("Found banana!")
}

// Check if key in dictionary
if "debug" in config {
    enableDebug()
}

// Not in operator
if "error" not in messages {
    messages.append("No errors")
}
```

#### Nested Dictionary Indexing (FIXED)
```frame
// Deep nesting now works!
config["database"]["host"] = "localhost"
config["database"]["port"] = 3306
var user = config["database"]["credentials"]["username"]

// Variable keys work too
var section = "database"
var field = "host"
var value = config[section][field]
```

#### Lambda Expressions in Collections (WORKING)
```frame
// Dictionary with lambda functions
var operations = {
    "add": lambda x, y: x + y,
    "multiply": lambda x, y: x * y
}
var result = operations["add"](5, 3)  // 8

// List with lambda functions
var transforms = [
    lambda n: n + 1,
    lambda n: n * 2,
    lambda n: n ** 2
]
var squared = transforms[2](5)  // 25

// Complex nested structures
var validators = {
    "rules": [
        lambda x: x > 0,
        lambda x: x < 100
    ]
}
```

### v0.32 Features

#### Advanced Enum Support (NEW in v0.32)
- **Custom Integer Values**: `enum Status { Ok = 200, NotFound = 404 }`
- **String Enums**: `enum Color : string { Red = "red", Blue = "blue" }`
- **Auto String Values**: `enum LogLevel : string { Debug, Info }` → Debug="Debug", Info="Info"
- **Mixed Values**: Explicit values with auto-increment continuation
- **Negative Values**: `enum Priority { Low = -1, High = 10 }`
- **Module-Scope Enums**: Enums can be declared at module level (outside systems)
- **Enum Iteration**: `for status in StatusEnum { ... }`
- **Property Access**: `.name` and `.value` properties on enum members

### v0.31 Features

#### Import Statements (NEW in v0.31)
- **Simple imports**: `import math`
- **Aliased imports**: `import numpy as np`
- **From imports**: `from collections import defaultdict`
- **Wildcard imports**: `from typing import *`

#### Self Expression (NEW in v0.31)
- **Standalone self**: Can use `self` as complete expression
- **Example**: `jsonpickle.encode(self)` works without backticks

#### Static Methods (FIXED in v0.31)
- **Operations default**: Instance methods by default
- **Static declaration**: Use `@staticmethod` for static operations
- **Validation**: Parser validates no `self` in static operations

#### Null Value Standardization (v0.31)
- **Standard**: `None` is the only keyword for null values
- **Removed**: `null` and `nil` are no longer supported (breaking change)

#### Module Variables with Auto-Global Generation (v0.31)
- **Declaration**: `var counter = 0` at module level
- **Auto-Global**: Transpiler automatically generates `global` declarations for modified module variables
- **Functions**: Global declarations added when module variables are modified in functions
- **Systems**: Global declarations also generated for system state methods
- **Shadowing Protection**: Local variables cannot shadow module variables (Python target)
- **Conditional Imports**: Only generates imports (e.g., `from enum import Enum`) when actually used

### v0.37 Async Event Handlers, Runtime Infrastructure & Slicing (NEW)

#### Async Event Handlers
- **Explicit async marking**: `async $>()`, `async eventName()`, `async <$()`
- **Await support**: Full await expression support in async handlers
- **State-level async**: Entire state function becomes async when any handler is async

#### Runtime Infrastructure Nodes
- **RuntimeInfo**: Container for runtime metadata
- **KernelNode**: Tracks kernel async requirements
- **RouterNode**: Tracks router async requirements  
- **TransitionNode**: Records async transitions
- **StateDispatcherNode**: Identifies async state dispatchers

#### Async Chain Validation
- **Compile-time validation**: Ensures all handlers in async chains are properly marked
- **Clear error messages**: Explains which handlers need async and why
- **Transition tracking**: Validates enter/exit handlers in async transition chains

#### With Statement Support
- **Context managers**: `with expr as var { ... }`
- **Async context managers**: `async with expr as var { ... }`
- **Resource management**: Proper cleanup with context managers

#### Slicing Operations (Added 2025-01-22)
- **Full Python-style slicing**: Strings and lists support all slice notations
- **Basic slices**: `text[:5]`, `list[2:8]`, `data[7:]`
- **Step parameter**: `list[::2]`, `data[::-1]`, `nums[1:8:2]`
- **AST integration**: SliceNode with start, end, step expressions
- **Parser enhancement**: Detects colon-based slice notation in brackets

Slicing Example:
```frame
fn demonstrateSlicing() {
    var text = "Hello, World!"
    var nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    
    // Basic slicing
    print(text[:5])         // "Hello"
    print(text[7:])         // "World!"
    print(nums[:5])         // [0, 1, 2, 3, 4]
    
    // With step parameter
    print(nums[::2])        // [0, 2, 4, 6, 8]
    print(nums[::-1])       // [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
}
```

Async Example:
```frame
system AsyncPipeline {
    machine:
        $Processing {
            // Explicit async handler
            async processBatch(id) {
                var result = await process_item(self.data[id])
                system.return = result
            }
            
            // Async enter handler (required due to async chain)
            async $>() {
                print("Processing started")
            }
        }
}
```

### v0.38 Complete Feature Set

#### Breaking Changes
- **Removed**: `&&`, `||`, `!` operators no longer supported
- **Required**: Use Python-style `and`, `or`, `not` keywords exclusively
- **Error Messages**: Scanner provides clear migration guidance

#### Python Logical Operators
```frame
fn examples() {
    // Boolean AND
    if a and b {
        print("Both are true")
    }
    
    // Boolean OR
    if x or y {
        print("At least one is true")
    }
    
    // Boolean NOT
    if not condition {
        print("Condition is false")
    }
    
    // Complex expressions
    if (a and b) or (not c) {
        print("Complex logic")
    }
    
    // In state machines
    machine:
        $State {
            check(x, y) {
                if x > 0 and y > 0 {
                    -> $Valid
                } elif not x or not y {
                    -> $Invalid
                }
            }
        }
}
```

### v0.38 Additional Features Completed

#### First-Class Functions (COMPLETE) ✅
```frame
fn add(a, b) { return a + b }
fn multiply(a, b) { return a * b }

fn examples() {
    // Functions as values
    var op = add
    var result = op(5, 3)  // 8
    
    // Pass functions as parameters
    fn apply(func, x, y) {
        return func(x, y)
    }
    result = apply(multiply, 4, 5)  // 20
    
    // Return functions from functions
    fn get_operation(name) {
        if name == "add" {
            return add
        } else {
            return multiply
        }
    }
}
```

#### Lambda Expressions (COMPLETE) ✅
```frame
fn examples() {
    // Simple lambda
    var square = lambda x: x * x
    print(str(square(5)))  // 25
    
    // Multi-parameter lambda
    var add = lambda x, y: x + y
    print(str(add(3, 4)))  // 7
    
    // Lambda with closures
    var multiplier = 10
    var scale = lambda x: x * multiplier
    print(str(scale(5)))  // 50
}
```

#### Exponent Operator (COMPLETE) ✅
```frame
fn examples() {
    // Basic exponentiation
    var result = 2 ** 3  // 8
    
    // Right associativity
    var tower = 2 ** 3 ** 2  // 512 (2 ** 9)
    
    // Precedence (higher than multiplication)
    var expr = 2 * 3 ** 2  // 18 (2 * 9)
}
```

#### Empty Set Literal (COMPLETE) ✅
```frame
fn examples() {
    // Empty dictionary (unchanged)
    var empty_dict = {}
    
    // Empty set (new syntax)
    var empty_set = {,}
    
    // Non-empty set (unchanged)
    var numbers = {1, 2, 3}
    
    // Operations
    empty_set.add(42)
    var has_42 = 42 in empty_set  // true
}
```


```frame
fn examples() {
    // Boolean AND
    if a and b {
        print("Both are true")
    }
    
    // Boolean OR
    if x or y {
        print("At least one is true")
    }
    
    // Boolean NOT
    if not condition {
        print("Condition is false")
    }
    
    // Complex expressions
    if (a and b) or (not c) {
        print("Complex logic")
    }
    
    // In state machines
    machine:
        $State {
            check(x, y) {
                if x > 0 and y > 0 {
                    -> $Valid
                } elif not x or not y {
                    -> $Invalid
                }
            }
        }
}
```

### v0.39 Python Operator Alignment (COMPLETE) ✅

Frame v0.39 completes the Python operator alignment with comprehensive support for compound assignments, bitwise operators, and identity operators.

#### Compound Assignment Operators ✅
```frame
// Arithmetic compound assignments
var x = 10
x += 5       // x = 15
x -= 3       // x = 12  
x *= 2       // x = 24
x /= 4       // x = 6
x %= 4       // x = 2
x **= 3      // x = 8

// Bitwise compound assignments
var flags = 0b1000
flags |= 0b0001     // Set bit
flags &= 0b1110     // Clear bit
flags <<= 2         // Shift left
flags >>= 1         // Shift right

// Works with collections
var list = [1, 2, 3]
list += [4, 5]      // [1, 2, 3, 4, 5]
```

#### Bitwise Operators ✅
```frame
// Bitwise NOT
var x = 7
var y = ~x          // -8 (two's complement)

// Bitwise AND/OR
var a = 5 & 3       // 1 (0101 & 0011 = 0001)
var b = 5 | 3       // 7 (0101 | 0011 = 0111)

// Bit shifting
var c = 8 << 2      // 32 (shift left by 2)
var d = 32 >> 2     // 8 (shift right by 2)
```

#### Identity Operators ✅
```frame
// Identity checking
var x = None
if x is None {
    print("x is None")
}

if y is not None {
    print("y has a value")
}

// Identity vs Equality
var list1 = [1, 2, 3]
var list2 = [1, 2, 3]
var list3 = list1

// Equality compares values
if list1 == list2 { }  // True - same values

// Identity checks same object
if list1 is list3 { }  // True - same reference
if list1 is not list2 { }  // True - different objects
```

### v0.40 Python Comments, Bitwise XOR, and Matrix Multiplication (COMPLETE) ✅

Frame v0.40 completes Python operator alignment with bitwise XOR, matrix multiplication, and transitions to Python-style comments.

#### Breaking Change: Python-Style Comments
```frame
# Python-style single-line comments (v0.40)
fn example() {
    var x = 42  # Inline comments use hash symbol
    
    {-- Frame documentation comments
        still use special syntax for multiline --}
}
```

**Migration Required**:
- `//` now means floor division operator
- `/* */` C-style comments removed
- Use `#` for all single-line comments

#### Bitwise XOR Operator ✅
```frame
fn xor_operations() {
    # Basic XOR
    var a = 5 ^ 3        # Result: 6
    
    # Compound assignment
    var flags = 0b1010
    flags ^= 0b0011      # Toggle specific bits
    
    # XOR encryption pattern
    var data = 42
    var key = 17
    var encrypted = data ^ key
    var decrypted = encrypted ^ key  # Returns to 42
}
```

#### Matrix Multiplication Operator ✅
```frame
import numpy as np

fn matrix_operations() {
    # Matrix multiplication with NumPy
    var a = np.array([[1, 2], [3, 4]])
    var b = np.array([[5, 6], [7, 8]])
    
    # Matrix multiplication
    var result = a @ b      # [[19, 22], [43, 50]]
    
    # In-place matrix multiplication
    a @= b
    
    # Dot product with vectors
    var v1 = np.array([1, 2, 3])
    var v2 = np.array([4, 5, 6])
    var dot = v1 @ v2      # 32
}
```

**Note**: The `@` operator requires objects with `__matmul__` method (like NumPy arrays).

#### Floor Division ✅
```frame
fn division() {
    var regular = 10 / 3    # 3.333...
    var floor = 10 // 3     # 3
    
    # Compound floor division
    var x = 25
    x //= 4                 # x = 6
}
```

#### Python Numeric Literals ✅
```frame
fn literals() {
    var binary = 0b1010     # Binary notation
    var octal = 0o755       # Octal notation
    var hex = 0x1A2B        # Hexadecimal notation
}
```

#### Python String Literals (NEW in v0.40) ✅
Frame v0.40 adds comprehensive support for Python's string literal features:

```frame
fn string_features() {
    var name = "Frame"
    var version = 0.40
    
    # F-strings (formatted string literals)
    var msg = f"Hello {name} v{version}!"
    var calc = f"Sum: {2 + 3}"
    
    # Raw strings (no escape processing)
    var path = r"C:\Users\Frame\Documents"
    var regex = r"\d{3}-\d{4}"
    
    # Byte strings (binary data)
    var data = b"Binary data"
    
    # Triple-quoted strings (multi-line)
    var text = """This is a
    multi-line string
    with preserved formatting"""
    
    # Prefixed triple-quoted
    var raw_multi = r"""Raw multi-line
    with \n literal"""
    
    # Percent formatting (classic Python)
    var fmt1 = "Hello %s" % name
    var fmt2 = "%s v%.2f" % (name, version)
    var fmt3 = "%(lang)s v%(ver).1f" % {"lang": name, "ver": version}
}
```

**String Feature Support**:
- **F-strings**: `f"text {expr}"` - Formatted string literals with embedded expressions
- **Raw strings**: `r"text"` - No escape sequence processing
- **Byte strings**: `b"text"` - Binary data representation
- **Triple-quoted**: `"""text"""` - Multi-line strings with preserved formatting
- **Prefixed triple-quoted**: `r"""text"""`, `f"""text"""` - Combined features
- **Percent formatting**: `"format" % values` - Classic Python string formatting

### v0.44 Pattern Matching (COMPLETE) ✅

Frame v0.44 introduces comprehensive pattern matching with match-case statements, bringing Python 3.10+ structural pattern matching to Frame.

#### Match-Case Statement
```frame
match expression {
    case pattern {
        # statements
    }
    case pattern if guard {
        # statements with guard condition
    }
    case _ {
        # default case
    }
}
```

#### Supported Pattern Types

**Literal Patterns** ✅
```frame
match value {
    case 42 { return "answer" }
    case "hello" { return "greeting" }
    case true { return "boolean" }
    case None { return "null" }
}
```

**Capture Patterns** ✅
```frame
match value {
    case 0 { return "zero" }
    case x { return "captured: " + str(x) }
}
```

**OR Patterns** ✅ (using `or` keyword)
```frame
match status {
    case 200 or 201 or 204 {
        return "success"
    }
    case 400 or 404 or 403 {
        return "client error"
    }
}
```

**Star Patterns** ✅
```frame
match lst {
    case [first, *rest] {
        return "first: " + str(first) + ", rest: " + str(rest)
    }
    case [first, *middle, last] {
        return "edges with middle"
    }
}
```

**AS Patterns** ✅
```frame
match data {
    case [x, y] as point {
        return "point: " + str(point)
    }
    case (1 or 2 or 3) as num {
        return "small: " + str(num)
    }
}
```

**Sequence Patterns** ✅
```frame
match lst {
    case [] { return "empty" }
    case [x] { return "single" }
    case [x, y] { return "pair" }
    case [x, y, z] { return "triple" }
}
```

**Mapping Patterns** ✅
```frame
match response {
    case {"status": 200, "data": data} {
        return process(data)
    }
    case {"error": {"code": code, "message": msg}} {
        return "Error " + str(code) + ": " + msg
    }
}
```

**Guard Clauses** ✅
```frame
match score {
    case x if x >= 90 { return "A" }
    case x if x >= 80 { return "B" }
    case x if x >= 70 { return "C" }
    case _ { return "F" }
}
```

### v0.45 Class Support (COMPLETE) ✅

Frame v0.45 introduces basic class support for object-oriented programming, enabling familiar OOP patterns alongside Frame's state machine paradigm.

#### Class Declaration Syntax
```frame
# Basic class
class ClassName {

# Class with inheritance (v0.76: Python-style syntax)
class Child(Parent) {
    # Class variables
    var shared_var = 0
    
    # Constructor (method named 'init')
    fn init(param1, param2) {
        self.instance_var1 = param1
        self.instance_var2 = param2
    }
    
    # Instance method
    fn method_name(args) {
        # Access instance vars via self
        return self.instance_var1
    }
    
    # Static method
    @staticmethod
    fn static_method(args) {
        # No self parameter
        return ClassName(args)
    }
}
```

#### Key Features
- **Constructor Methods**: Methods named `init` automatically become constructors
- **Implicit Self**: Method signatures don't include `self` (added automatically)
- **Instance Variables**: Created via `self.varname = value` assignments
- **Class Variables**: Declared at class level with `var name = value`
- **Static Methods**: Use `@staticmethod` decorator for non-instance methods
- **Method Calls**: Instance methods via `obj.method()`, static via `Class.method()`

#### Example Usage
```frame
class Point {
    var instance_count = 0
    
    fn init(x, y) {
        self.x = x
        self.y = y
        Point.instance_count = Point.instance_count + 1
    }
    
    fn distance_to(other) {
        var dx = self.x - other.x
        var dy = self.y - other.y
        return ((dx * dx) + (dy * dy)) ** 0.5
    }
    
    @staticmethod
    fn origin() {
        return Point(0, 0)
    }
}

# Class with inheritance
class Point3D(Point) {
    fn init(x, y, z) {
        super.init(x, y)  # Call parent constructor
        self.z = z
    }
}

fn main() {
    var p1 = Point(3, 4)
    var p2 = Point.origin()
    var p3 = Point3D(1, 2, 3)
    var dist = p1.distance_to(p2)
    print("Distance: " + str(dist))
    print("Points created: " + str(Point.instance_count))
}
```

#### Implementation Status
- ✅ Class declarations with methods and variables
- ✅ Constructor methods (init)
- ✅ Instance methods with implicit self
- ✅ Static methods with @staticmethod
- ✅ Class and instance variables
- ✅ Proper variable scoping in methods
- ✅ Inheritance with Python-style syntax: `class Child(Parent)` (v0.76)
- ❌ Access modifiers (all members public)

### v0.56 Python Enhancement Features (COMPLETE) ✅

Frame v0.56 introduces modern Python 3.8+ features including assignment expressions (walrus operator), type aliases, and enhanced numeric literals, achieving 100% test success rate with all 341 tests passing.

#### Walrus Operator / Assignment Expressions
```frame
# Conditional with assignment
if (match := search(pattern, text)) {
    print("Found: " + match)
}

# While loop with assignment  
while (line := readline()) != "" {
    process(line)
}

# List comprehension with filtering
var results = [y for x in data if (y := transform(x)) > 0]
```

#### Type Aliases (Python 3.12+ Style)
```frame
# Simple type aliases
type UserID = int
type Coordinate = tuple[float, float]

# Generic type aliases
type Optional[T] = T | None
type Result[T, E] = tuple[bool, T | E]

# Using type aliases in functions
fn getUser(id: UserID) : Optional[User] {
    # implementation
}
```

#### Enhanced Numeric Literals
```frame
# Digit separators for readability
var billion = 1_000_000_000
var hex_mask = 0xFF_FF_00_00
var binary = 0b1111_0000_1111_0000

# Scientific notation
var avogadro = 6.022e23
var planck = 6.626_070_15e-34

# Complex numbers
var z = 3 + 4j
var pure_imaginary = 2.5j
```

#### Known Issue: Type Keyword Conflict
- **Issue**: `type` is now a reserved keyword for type aliases
- **Impact**: Cannot call Python's built-in `type()` function directly
- **Workaround**: Comment out type() calls or use `__class__` attribute
- **Future**: Consider context-sensitive keyword recognition

### v0.60 Critical Bug Fixes and Debugging Infrastructure (COMPLETE) ✅

Frame v0.60 represents a significant quality and reliability milestone, achieving 100% test success and resolving critical runtime bugs that affected action call assignments.

#### Double-Call Bug Fix ✅
- **Issue Fixed**: Action calls in variable assignments generated incorrect double parameters
- **Before**: `var result = self.myAction(42)` → `result = self._myAction(42)(42)` ❌
- **After**: `var result = self.myAction(42)` → `result = self._myAction(42)` ✅
- **Root Cause**: Duplicate parameter processing in `visit_call_expression_node_to_string`
- **Impact**: Resolves runtime errors and incorrect behavior in action call assignments

#### AST Serialization Infrastructure ✅
- **New Module**: `framec/src/frame_c/ast_serialize.rs`
- **Purpose**: JSON serialization of Frame AST for debugging and testing
- **Features**:
  - Complete AST structure serialization
  - Expression-level debugging with `debug_expression()`
  - Integration with compiler debug output
  - Future-ready for automated testing and validation
- **Usage**: Enable with `FRAME_TRANSPILER_DEBUG=1` environment variable

#### Test Suite Improvements ✅
- **Fixed**: `test_class_simple.frm` syntax and logic errors
- **Corrected**: Frame class method syntax (removed explicit `self` parameters)
- **Updated**: Class constructor call patterns to match Frame language specification
- **Result**: 100% test success rate (378/378 tests passing)

### v0.57 Multi-File Module System (COMPLETE) ✅

Frame v0.57 delivers a fully functional multi-file module system enabling Frame projects to be organized across multiple `.frm` files with automatic compilation, dependency management, and 100% test success.

#### Frame File Import Syntax
```frame
# Standard module import from Frame file
import Utils from "./utils.frm"
import DataStore from "../lib/datastore.frm"

# Import with alias
import LongModuleName from "./very_long_module_name.frm" as LMN
import Calculator from "./calc.frm" as Calc

# Selective imports (destructuring)
import { add, subtract, multiply } from "./math_ops.frm"
import { validateEmail, validatePhone } from "./validators.frm"
```

#### Module System Infrastructure
- **ModuleResolver**: Resolves file paths with security validation
- **DependencyGraph**: Manages dependencies with cycle detection
- **ModuleCache**: Incremental compilation with SHA-256 hashing
- **ModuleLinker**: Combines modules into final output
- **MultiFileCompiler**: Orchestrates the entire pipeline

#### Implementation Status
- ✅ **Phase 1**: Core infrastructure components
- ✅ **Phase 2**: Import statement parsing
- ✅ **Phase 3**: Path resolution and module discovery
- ✅ **Phase 4**: Module compilation pipeline
- ✅ Module generation as Python classes with static methods
- ✅ CLI integration with -m/--multifile flag
- ✅ Test runner automatic multifile detection
- ✅ 100% test success rate (344/344 tests passing)

### v0.50 Delete Statement (COMPLETE) ✅

Frame v0.50 introduces the `del` statement for removing variables, list elements, dictionary entries, and object attributes, providing complete memory management control.

#### Delete Statement Syntax
```frame
# Delete variables
var x = 42
del x  # Variable no longer accessible

# Delete list elements
var mylist = [1, 2, 3, 4, 5]
del mylist[2]      # Remove element at index 2
del mylist[-1]     # Remove last element

# Delete slices
var nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
del nums[2:5]      # Delete elements 2-4
del nums[::2]      # Delete every other element

# Delete dictionary entries
var mydict = {"a": 1, "b": 2, "c": 3}
del mydict["b"]    # Remove key "b"

# Delete from nested structures
var data = {
    "users": [{"name": "Alice", "age": 30}]
}
del data["users"][0]["age"]  # Remove nested field
```

#### Key Features
- **Variable Deletion**: Remove variables from scope
- **List Element Deletion**: Delete by index (positive or negative)
- **Slice Deletion**: Delete ranges with optional step
- **Dictionary Entry Deletion**: Remove key-value pairs
- **Nested Deletion**: Delete from complex nested structures
- **Expression Support**: Any valid expression can be deletion target

#### Implementation Status
- ✅ Scanner recognizes `del` keyword
- ✅ Parser handles del statements
- ✅ AST node for delete operations
- ✅ Python code generation
- ✅ Comprehensive test coverage
- ⚠️ Parser limitation: Cannot redeclare deleted variables in same scope

### v0.51 Loop Else Clauses (COMPLETE) ✅

Frame v0.51 adds support for Python's loop else clause feature, where else blocks execute when loops complete normally without encountering a break statement.

#### Loop Else Syntax
```frame
# For-else: Search pattern
for item in items {
    if item == target {
        print("Found!")
        break
    }
}
else {
    print("Not found")  # Executes only if no break
}

# While-else: Process with limit
var attempts = 0
while attempts < max_attempts {
    if try_operation() {
        break
    }
    attempts = attempts + 1
}
else {
    print("Max attempts reached")  # Executes if loop completed normally
}
```

#### Key Behavior
- **Executes on normal completion**: Else block runs when loop condition becomes false
- **Skipped on break**: Else block does NOT execute if loop exits via `break`
- **Executes with continue**: Continue doesn't prevent else execution
- **Executes on empty loops**: Else runs even if loop body never executes (condition initially false)

#### Use Cases
- **Search patterns**: Detect when item not found in collection
- **Retry logic**: Handle max attempts reached scenarios
- **Completion detection**: Know when loop finished all iterations
- **Validation loops**: Confirm all items processed successfully

#### Implementation Status
- ✅ AST nodes support optional else blocks
- ✅ Parser handles else clauses for both for and while loops
- ✅ Python code generation with proper indentation
- ✅ Backward compatibility maintained
- ✅ Comprehensive test coverage

### v0.34 Module System (Complete Implementation)

#### Module Declarations (IMPLEMENTED in v0.34)
- **Module keyword**: `module name { ... }` syntax fully supported
- **Nested modules**: Can declare modules within modules with full functionality
- **Symbol table scoping**: Proper scope management for module contents
- **Module functions**: Functions inside modules fully accessible
- **Module variables**: Variables inside modules with proper scoping

#### Native Python Support (v0.38)
- **No special imports needed**: Python built-in functions work directly
- **Type conversions**: `str()`, `int()`, `float()`, `bool()` work natively
- **Collection operations**: All Python list, dict, set methods work directly

#### Qualified Names (IMPLEMENTED in v0.34)
- **Function calls**: `module.function()` syntax working
- **Variable access**: `module.variable` syntax working
- **Nested modules**: `module.submodule.function()` syntax working
- **Cross-module access**: Functions in modules accessible from outside

```frame
// Define modules with functions and variables
module calculator {
    fn add(a, b) {
        return a + b
    }
    
    var version = "1.0"
}

// Use modules with qualified names
fn main() {
    var result = calculator.add(5, 3)  // Qualified function call
    var ver = calculator.version       // Qualified variable access
    var s = str(result)               // Native Python str() function
}
```

#### All Features Implemented
- **Qualified names**: ✅ `module.function()` syntax working
- **Code generation**: ✅ Module structures generated in target languages
- **Cross-module access**: ✅ Functions in modules accessible from outside
- **100% Test Coverage**: ✅ All 189 tests passing

#### Example Module Usage
```frame
// Define modules with functions and variables
module utils {
    fn helper() {
        return 42
    }
    
    var count = 0
    
    fn increment() {
        count = count + 1
        return count
    }
}

// Nested modules
module math {
    module basic {
        fn multiply(a, b) {
            return a * b
        }
    }
}

// Use modules with qualified names
fn main() {
    var result = utils.helper()          // Call module function
    var num = utils.increment()          // Call function that modifies module variable
    var product = math.basic.multiply(3, 4)  // Call nested module function
    var s = str(result)                  // Native Python str() function
    print("Count: " + str(utils.count))  // Access module variable
}
```

### v0.35 Async/Await Support (Complete Implementation)

#### Async Functions (IMPLEMENTED in v0.35)
- **Async keyword**: `async fn name() { ... }` syntax fully supported
- **Await expressions**: `await expr` syntax parsing and code generation
- **Python generation**: Proper `async def` function generation
- **Module integration**: Async functions work seamlessly with module system

#### Async Interface Methods (IMPLEMENTED in v0.35)
- **Async interface methods**: `async methodName()` syntax in system interfaces
- **Python generation**: Generate `async def` methods for async interface declarations
- **Mixed interfaces**: Support both async and sync methods in same interface
- **State handler propagation**: State handlers for async interface events become async

#### Async Implementation Examples
```frame
// Async functions work at module level
async fn fetchRemote(endpoint) {
    print("Fetching from " + endpoint)
    return "data from " + endpoint
}

// Systems with mixed async/sync interface methods
system DataProcessor {
    interface:
        async processData(data)    // → async def processData(self, data)
        normalMethod(x)           // → def normalMethod(self, x)
    
    machine:
        $Ready {
            processData(data) {
                // State handler automatically async (handles async interface method)
                print("Processing: " + data)
                system.return = "processed_" + data
            }
            
            normalMethod(x) {
                // Normal sync state handler
                return x * 2
            }
        }
}
```

#### Current Async Status
- **✅ Parser Integration**: Async keyword recognition and AST support complete
- **✅ Code Generation**: Python async/await generation working  
- **✅ Test Coverage**: All 13 async tests passing (100% success rate)
- **✅ Recent Fixes**: Fixed async handlers missing `async` marking, simplified class-based tests

#### Recent Test Improvements (2025-09-06)
- **Import Tests**: Fixed all 5 import test failures (100% passing)
- **Enum Tests**: Fixed all 3 enum test failures (100% passing)
- **Async Tests**: Fixed remaining async test issues (100% passing)
- **Backtick Removal**: Updated all tests to avoid backticks
- **Overall Progress**: 93.7% → 97.3% success rate

### v0.30 Modular AST Structure

```
FrameModule (Top-Level)
├── Module (metadata/attributes)
├── Functions[] (peer entities)
├── Systems[] (peer entities)
├── Enums[] (module-level enums) - v0.32
└── Variables[] (module variables)
    └── SystemNode
        ├── Module (system-specific metadata)
        ├── Interface Block
        ├── Machine Block  
        ├── Actions Block
        ├── Operations Block
        └── Domain Block (can contain system-scoped enums)
```

## Frame Syntax (Current v0.35)

### Core Language Features

#### System Declaration
- **Syntax**: `system SystemName { ... }`

#### Block Keywords
- `interface:` - Interface methods
- `machine:` - State machine definition  
- `actions:` - Action implementations
- `operations:` - Helper operations
- `domain:` - Domain variables

#### Parameters
- **Syntax**: `(param1, param2)`

#### Event Handlers
- **Syntax**: `eventName()`
- **Enter Event**: `$>()`
- **Exit Event**: `<$()`

#### Return Statements
- **Simple**: `return`
- **With Value**: `return value`
- **System Return Variable**: `system.return = value` (sets interface return value from anywhere in event handlers or actions)

#### Event Forwarding
- **To Parent State**: `=> $^` (statement - can appear anywhere in event handler)
- **Current Event**: `$@`

#### Attributes
- **Syntax**: `@staticmethod` (Python-style)

#### System Parameters
- **Declaration**: `system System ($(start), $>(enter), domain)`
- **Instantiation**: `System("a", "b", "c")` (flattened arguments)

### v0.30 Enhancements

#### Multi-Entity Support
- **Multiple Functions**: Support for multiple functions with any names
- **Multiple Systems**: Support for multiple system definitions per file
- **Module Architecture**: Foundation for comprehensive module system

### v0.31 Breaking Changes

#### Completely Removed Legacy Syntax
The following v0.11 syntax has been **completely removed** and will cause compilation errors:

##### Removed Tokens
- `^` and `^(value)` - Old return syntax → Use `return` or `return value`
- `^=` - Old return assignment → Use `system.return = value`
- `#SystemName ... ##` - Old system declaration → Use `system Name { }`
- `?`, `?!`, `?~`, `?#`, `?:` - Ternary operators → Use if/elif/else
- `:|` and `::` - Test terminators → No longer needed
- `~/`, `#/`, `:/` - Pattern matching → Use if/elif/else with comparisons
- `#[attr]` - Rust-style attributes → Use `@attr`
- `[params]` - Bracket parameters → Use `(params)`
- `|event|` - Pipe event handlers → Use `event()`
- `-block-` - Dash block markers → Use `block:`

##### Migration Required
All code using old syntax must be migrated to modern syntax before compilation.

### v0.31 Enhancements

#### System Return Variable (NEW in v0.31)
- **Syntax**: `system.return = value`
- **Purpose**: Sets the interface method return value from anywhere in event handlers or actions
- **Scope**: Can be used in machine states and action methods
- **Important**: This is the ONLY valid use of the `system` keyword
- **Invalid**: `system.method()` calls are NOT supported - use `self.method()` for interface calls
- **Implementation**: Scanner greedily matches "system.return" as a single token for efficiency

#### Module Variables (NEW in v0.31)
- **Module-level Variables**: Declare variables at module scope accessible from all functions/systems
- **Automatic Global Generation**: Transpiler detects modifications and generates `global` declarations for Python
- **Shadowing Protection**: Local variables cannot shadow module variables (enforced at transpilation)
- **Conditional Imports**: Only generates imports when actually used (e.g., `from enum import Enum`)
- **Two-Pass Analysis**: First identifies locals, then detects module variable modifications
- **System Support**: Works in both functions and system state methods

## Build & Test

### Build
```bash
cargo build
```

### Test Transpiler

**IMPORTANT: GENERATION LOCATION**  
⚠️ **Generate Python files in the SAME directory as the source .frm file for easy location.**
- When transpiling `framec_tests/python/src/test.frm`, generate to `framec_tests/python/src/test.py`
- DO NOT use the `generated/` directory - generate right next to the source file

**CRITICAL: PROPER TEST VALIDATION PROTOCOL**

When claiming tests are "passing" or "working", you MUST follow this 4-step validation process:

1. **Generate**: Run framec to generate code IN THE SAME DIRECTORY as the source
2. **Execute**: Run the generated Python/target code 
3. **Validate**: Verify the output matches expected behavior
4. **Report**: State specifically what functionality was verified

**DO NOT claim "passing", "working", "100% success", or "complete validation" unless all 4 steps are completed.**

#### Example Proper Test Validation:
```bash
# Step 1: Generate (to same directory as source)
./target/debug/framec -l python_3 framec_tests/python/src/test.frm > framec_tests/python/src/test.py

# Step 2: Execute  
python3 framec_tests/python/src/test.py

# Step 3: Validate output
# Expected: "NoParameters started"
# Actual: "NoParameters started" ✅

# Step 4: Report
# VERIFIED: System initialization, enter event handling, print statement execution
```

#### Quick Generation Only (for syntax checking):
```bash
# Available languages: python_3, graphviz
./target/debug/framec -l python_3 file.frm
```

**Note**: Generation-only checks are useful for syntax validation but cannot be called "passing tests".

## Test Infrastructure

📚 **READ THE COMPLETE TESTING GUIDE**: [`framec_tests/docs/test_runner_guide.md`](framec_tests/docs/test_runner_guide.md)

### Test Organization
```
framec_tests/
├── runner/                    # Test execution infrastructure
│   ├── frame_test_runner.py   # Main test runner script
│   └── configs/               # Test configuration files
│       ├── all_tests.json    # Complete test suite
│       ├── hsm_tests.json    # Hierarchical state machines
│       ├── multi_entity_tests.json  # Multi-system/function tests
│       └── scope_tests.json  # Scope resolution tests
├── reports/                   # Test results and matrices
│   ├── test_matrix_v031.md   # Latest detailed test matrix
│   ├── test_results_v031.json # Machine-readable results
│   └── test_log.md           # Standard test status report
├── docs/                      # Test documentation
│   └── test_runner_guide.md  # Complete testing guide
└── python/
    ├── src/                   # Frame test files (.frm)
    ├── models/                # Expected output models
    └── scripts/               # Legacy helper scripts
```

**Key Directories:**
- **`runner/`**: Contains the official test runner and all configuration files for different test suites
- **`reports/`**: Stores all test results, matrices, and status reports - critical for tracking project health  
- **`docs/`**: Complete documentation including the comprehensive test runner guide

### Running Tests

⚠️ **ALWAYS READ THE TESTING GUIDE FIRST**: See [`framec_tests/docs/test_runner_guide.md`](framec_tests/docs/test_runner_guide.md) for complete details on usage, configuration options, and output formats.

#### Standard Test Validation Process
```bash
cd framec_tests
# Run all tests with matrix generation and JSON output
python3 runner/frame_test_runner.py --all --matrix --json --verbose

# After running, always check:
# 1. Test matrix saved to: reports/test_matrix_v0.31.md
# 2. JSON results saved to: reports/test_results_v0.31.json
```

#### Test Reporting Requirements
After EVERY test run, you MUST:
1. **Run the test suite** with `--matrix --json` flags
2. **Update the standard test log** at `reports/test_log.md` with:
   - Last run date/time
   - Total tests, passed, failed, success rate
   - Summary of passing categories
   - Table of failed tests with issue type
   - Any recent fixes applied
3. **Keep these files updated**:
   - `reports/test_log.md` - Main test status report (always overwrite)
   - `reports/test_matrix_v0.31.md` - Detailed test matrix (auto-generated)
   - `reports/test_results_v0.31.json` - JSON results (auto-generated)
4. **Categorize failures** as:
   - Environment issues (missing dependencies)
   - Test design issues (infinite loops, etc.)
   - Actual transpiler bugs
   - Expected failures (error validation tests)

### Test Files Location
**ALWAYS PUT TEST FILES HERE:**
- `/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/` - ALL Frame test files (.frm) go here
- **Generated Python files (.py)**: Generated next to source files in `/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src/`
- NEVER put test files in the main project directory

## Code Conventions

### Scanner (scanner.rs)
- Token recognition in `scan_token()` method
- New tokens added to `TokenType` enum
- Use `peek()` and `peek_next()` for lookahead

### Parser (parser.rs)
- Event handler parsing in `event_handler()` method
- Terminator parsing handles `return`, `=>`, `@:>`
- Use `TerminatorType` enum for different terminators

### AST (ast.rs)
- All syntax tree node definitions
- `TerminatorType` enum defines terminator semantics

### Visitors
- Each target language has its own visitor
- All visitors must handle new `TerminatorType::DispatchToParentState`
- Python visitor is primary reference implementation

## Important Notes

### System Block Structure
- System blocks are optional but must appear in specified order:
  1. `operations:`
  2. `interface:`
  3. `machine:`
  4. `actions:`
  5. `domain:`
- Blocks can be omitted if not needed
- Order is enforced by parser

### Event Handler Terminators
- All event handlers MUST end with a terminator (`return`, `@:>`, `=>`)
- `@:>` forwards events to parent states in hierarchical state machines
- `@:>` is a block terminator - no statements can follow it
- Code generators must emit implicit return after `@:>` dispatch

### Hierarchical State Machines
- Use `$Child => $Parent` syntax for hierarchy
- `@:>` operator forwards events from child to parent
- Child processes event first, then forwards to parent
- Parent state handles forwarded event

### Parameter Validation
- Interface method parameters must exactly match event handler parameters
- Names and types must be identical
- System parameter order: start state, enter event, domain (flattened)

## Git Workflow

### Branches
- `main` - stable v0.11 syntax
- `v0.20` - active development branch

### Commit Style
- Use conventional commits
- Reference specific syntax changes
- Include rationale for design decisions

## Common Tasks

### Adding New Syntax
1. Update scanner.rs with new token recognition
2. Add token to TokenType enum
3. Update parser.rs to handle new syntax
4. Update AST if needed (new node types)
5. Update all visitors in visitors/ directory
6. Update grammar.md documentation
7. Create test cases in test5 project

### Testing Changes
1. Build with `cargo build`
2. Test with sample .frm files
3. Verify generated code compiles/runs
4. Check all visitors handle new syntax

## Design Decisions Log

### `=> $^` Parent Dispatch (2025-01-20)
- **Decision**: Statement syntax (not terminator) replacing deprecated `@:>`
- **Rationale**: More flexible - can appear anywhere in event handler with statements after
- **Implementation**: Parser validates hierarchical context, AST tracks parent state, visitor generates parent call
- **Transition Safety**: Generated code checks for transitions after parent call and returns early if needed
- **Validation**: Parser error if used in non-hierarchical state

### Router-Based Parent Dispatch Architecture (2025-01-20)
- **Decision**: Use existing `__router` infrastructure for all parent dispatch instead of hardcoded method names
- **Rationale**: Maintains architectural consistency, eliminates code duplication, easier maintenance
- **Implementation**: 
  - Modified router signature: `__router(self, __e, compartment=None)`
  - Parent dispatch: `self.__router(__e, compartment.parent_compartment)`
  - Fallback dispatch also uses router for consistency
- **Benefits**: Dynamic state resolution, no hardcoded names, single point of routing logic
- **Compatibility**: Preserves all existing functionality while improving code quality

### v0.20 System Parameters
- **Decision**: Flattened argument lists for instantiation
- **Rationale**: Simpler, more conventional syntax
- **Migration**: `System($(a), $>(b), c)` → `System(a, b, c)`

### Interface Return Assignment (2025-01-17 - Updated 2025-09-07)
- **Decision**: Originally replaced `^=` with `return = value`, now only `system.return = value`
- **Rationale**: Clearer distinction between regular returns and interface return values
- **Implementation**: Parser rejects `return = ` with helpful error message
- **Migration**: `^= expr` → `system.return = expr`
- **Codegen**: Generates assignment to return stack/field in target language

## Files to Never Edit

- Test files in main transpiler project (use test5 instead)
- Legacy v0.11 documentation (keep for reference)
- Generated code files

## Helpful Commands

```bash
# Check for old syntax in docs
grep -r ":>" docs/
grep -r "\^" docs/
grep -r "\|.*\|" docs/

# Find Frame files for testing
find . -name "*.frm"

# Build and test in one command
cargo build && ./target/debug/framec -l python_3 test_file.frm
```

## v0.35 Status - Async/Await Foundation Complete

### Module System + Async/Await Fully Implemented
- **Module-level variables**: ✅ Full support for variables at module scope
- **Module-level functions**: ✅ Functions inside modules accessible with qualified names
- **Nested modules**: ✅ Full support for nested module declarations
- **Cross-module access**: ✅ Proper scoping and qualified name resolution
- **Async functions**: ✅ Complete `async fn` declarations and Python generation
- **Async interface methods**: ✅ Mixed async/sync interfaces with proper code generation
- **Await expressions**: ✅ Full parsing and Python `await` generation
- **Async propagation**: ✅ State handlers automatically async for async interface events

### v0.60 Test Success - All Tests Passing
- **Total Tests**: 378/378 (100% success rate) 🎉
- **Module System Tests**: All passing (preserved from v0.34)
- **Native Python Operation Tests**: All passing
- **Async Function Tests**: All 7 async tests passing
- **Async Interface Tests**: All passing
- **Mixed Async/Sync Tests**: All passing

### v0.35 Async Implementation Details
- **Parser Integration**: `async` keyword in scanner and parser
- **AST Support**: AsyncExprNode and `is_async` flags in interface methods
- **Visitor Logic**: Async detection and Python async code generation
- **Architecture**: Compatible with Frame's event-driven design for simple patterns

### Future Enhancements (Beyond v0.35)
- Full async state machine runtime for complex async workflows
- Multi-file module imports from other .frm files
- Advanced module features (access control, aliasing)
- Build system integration and packaging

## v0.66 Explicit Self/System Syntax (REQUIRED)

Frame v0.66 enforces explicit `self.` prefix for all internal method calls within systems:

### Required Syntax
- **Action calls**: `self.actionName()` (underscore added during Python generation)
- **Operation calls**: `self.operationName()` 
- **Interface method calls from within system**: `self.methodName()`
- **Static operation calls**: `SystemName.operationName()` (with @staticmethod)

### Examples
```frame
system Example {
    interface:
        process()
        
    operations:
        calculate(x) {
            self.doWork()  // v0.66: Required explicit self
            return x * 2
        }
        
    machine:
        $Start {
            process() {
                self.calculate(5)    // Operation call
                self.handleData()    // Action call
                self.process()       // Interface method call (recursive)
            }
        }
        
    actions:
        handleData() {
            print("Handling data")
        }
}
```

### Semantic Resolution
- **Always enabled**: No feature flag needed (removed in v0.66)
- **Two-pass parsing**: First builds symbol table, second resolves all calls
- **ResolvedCallType**: Every call is resolved to its semantic type
- **SystemInterface**: New variant for interface methods called within system

## Important Notes for Development

### Code Style
- Always indent the code in the frame blocks (operations: interface: machine: etc)
- Do not add attribution to claude on the commit messages
- DO NOT add comments to generated code unless explicitly requested

### Testing Requirements
- **Generation != Validation**: Generating code is not the same as validating it works
- **Full validation** means: 
  1. Generate the Python code from .frm file
  2. RUN the generated Python code 
  3. Verify output matches expected behavior
  4. Report specific functionality verified
- Use the test runner for comprehensive testing
- Put transient documents in `docs/tmp/`

### Debug Output and AST Dump Feature (v0.60)
- Debug output goes to stderr (eprintln! in Rust)
- Use `FRAME_TRANSPILER_DEBUG=1` environment variable to enable debug output
- Never send debug output to stdout (it contaminates generated code)

**AST Dump Feature (NEW in v0.60):**
- **Environment Variables:**
  - `FRAME_TRANSPILER_DEBUG=1` - Enables AST serialization, summary, and line mapping
  - `FRAME_AST_OUTPUT=filename.json` - Saves complete AST to specified JSON file
  - `FRAME_TRANSPILER_DEBUG_VERBOSE=1` - Prints full AST JSON to stderr
- **Features:**
  - **AST Summary:** `Systems (1): TestSystem (2 states), Functions (1): hello, Enums (0)`
  - **Line Map:** Hierarchical listing with line numbers for all AST elements
  - **JSON Export:** Complete structured AST data for external analysis
  - **Expression Debugging:** Individual expression serialization support
- **Usage Example:**
  ```bash
  FRAME_TRANSPILER_DEBUG=1 FRAME_AST_OUTPUT=/tmp/ast.json ./framec -l python_3 test.frm
  ```
- **Output:** JSON file with complete AST structure for debugging and validation