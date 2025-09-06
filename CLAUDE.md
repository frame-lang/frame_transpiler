# Frame Transpiler - Claude Context

⚠️ **IMPORTANT: When starting a new session, ALWAYS read these documents first:**
1. This file (CLAUDE.md) - Project structure and conventions
2. `docs/framelang_design/dev_notes.md` - Latest development status
3. `docs/v0.34_roadmap.md` - Module system and Rust target plans
4. `framec_tests/reports/test_log.md` - Current test results

## Project Overview

Frame is a state machine language that transpiles to multiple target languages. The project has evolved through v0.20 (syntax modernization), v0.30 (multi-entity support), v0.31 (import statements and self expression enhancements), v0.32 (advanced enum features), v0.33 (Frame Standard Library), v0.34 (Complete Module System implementation with qualified names), v0.35 (async/await foundation), v0.36 (event-handlers-as-functions), and v0.37 (async event handlers with runtime infrastructure).

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
# Use release build for FSL features
python3 runner/frame_test_runner.py --all --matrix --json --verbose --framec /Users/marktruluck/projects/frame_transpiler/target/release/framec
```

## Current State

**Branch**: `v0.30`  
**Version**: `v0.37`  
**Status**: ✅ **100% TEST SUCCESS RATE** (215/215 tests passing) - Complete Async Support with Slicing Operations

📋 **For release notes and development status, see**: [`docs/framelang_design/dev_notes.md`](docs/framelang_design/dev_notes.md)
📊 **For v0.30 achievements, see**: [`docs/v0.30_achievements.md`](docs/v0.30_achievements.md)
📊 **For v0.31 achievements, see**: [`docs/v0.31_achievements.md`](docs/v0.31_achievements.md)
📊 **For v0.32 achievements, see**: [`docs/v0.32_achievements.md`](docs/v0.32_achievements.md)
📊 **For v0.33 achievements, see**: [`docs/v0.33_achievements.md`](docs/v0.33_achievements.md)
📊 **For v0.34 achievements, see**: [`docs/v0.34_achievements.md`](docs/v0.34_achievements.md)
📊 **For v0.35 achievements, see**: [`docs/v0.35_achievements.md`](docs/v0.35_achievements.md)
📊 **For v0.36 achievements, see**: [`docs/v0.36_achievements.md`](docs/v0.36_achievements.md)
📊 **For v0.37 achievements, see**: [`docs/v0.37_achievements.md`](docs/v0.37_achievements.md)
📋 **For v0.34 release notes, see**: [`docs/release_notes_v0.34.md`](docs/release_notes_v0.34.md)
📋 **For v0.34 roadmap, see**: [`docs/v0.34_roadmap.md`](docs/v0.34_roadmap.md)
📊 **For latest test results, see**: [`framec_tests/reports/test_log.md`](framec_tests/reports/test_log.md)

## Architecture

```
Frame Source (.frm) 
    ↓
Scanner (Tokenizer) → framec/src/frame_c/scanner.rs
    ↓  
Parser → framec/src/frame_c/parser.rs
    ↓
AST (FrameModule) → framec/src/frame_c/ast.rs
    ↓
Visitor (Code Generation) → framec/src/frame_c/visitors/python_visitor.rs
    ↓
Target Code (Python - 1st Class Language)
```

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
- **Test Coverage**: 208/209 tests passing (99.5% success rate)

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
from fsl import str

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

### v0.34 Frame Standard Library (FSL) - Import Required ✅

The Frame Standard Library provides native built-in operations that work consistently across all target languages. As of v0.34, FSL requires explicit imports to prevent namespace conflicts.

```frame
// v0.34: Must import FSL operations explicitly
from fsl import str, int, float, bool
```

**Critical Fix**: Removed 'add' from FSL registry to prevent conflicts with user-defined functions.

#### Phase 1 - Type Conversions ✅
```frame
var x = 42
var s = str(x)        // "42" - no backticks needed!
var i = int("123")    // 123
var f = float("3.14") // 3.14
var b = bool(0)       // false
```

#### Phase 2 - List Operations ✅
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

// Properties
var len = list.length    // Converts to len(list)
var empty = list.is_empty // Converts to len(list) == 0

// Negative indexing works!
var last_item = list[-1]
```

#### Phase 3 - String Operations ✅
```frame
var text = "  Hello World  "
var upper = text.upper()     // "  HELLO WORLD  "
var lower = text.lower()     // "  hello world  "
var trimmed = text.trim()    // "Hello World" (→ strip())
var replaced = text.replace("World", "Frame")
var parts = text.split(" ")
var len = text.length        // Converts to len(text)

// Pending (need visitor work):
// text.contains("world")    // Will convert to "world" in text
// text.substring(0, 5)      // Will convert to text[0:5]
```

#### Implementation Details
- **Two-Pass Parsing**: FSL operations recognized in semantic analysis pass
- **Parser Fix**: Added BuiltInCallExprT handling in unary_expression
- **Visitor Transformations**: Properties like `.length` converted during code generation
- **Debug Control**: `FRAME_TRANSPILER_DEBUG=1` environment variable for debug output

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
                return = result
            }
            
            // Async enter handler (required due to async chain)
            async $>() {
                print("Processing started")
            }
        }
}
```

### v0.34 Module System (Complete Implementation)

#### Module Declarations (IMPLEMENTED in v0.34)
- **Module keyword**: `module name { ... }` syntax fully supported
- **Nested modules**: Can declare modules within modules with full functionality
- **Symbol table scoping**: Proper scope management for module contents
- **Module functions**: Functions inside modules fully accessible
- **Module variables**: Variables inside modules with proper scoping

#### FSL as Optional Import (IMPLEMENTED in v0.34)
- **Explicit import required**: FSL operations no longer available by default
- **Import tracking**: Parser tracks which FSL operations are imported
- **Namespace protection**: Prevents conflicts with user-defined functions

#### Qualified Names (IMPLEMENTED in v0.34)
- **Function calls**: `module.function()` syntax working
- **Variable access**: `module.variable` syntax working
- **Nested modules**: `module.submodule.function()` syntax working
- **Cross-module access**: Functions in modules accessible from outside

```frame
// Must import FSL operations explicitly
from fsl import str, int, float

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
    var s = str(result)               // FSL with explicit import
}
```

#### All Features Implemented
- **Qualified names**: ✅ `module.function()` syntax working
- **Code generation**: ✅ Module structures generated in target languages
- **Cross-module access**: ✅ Functions in modules accessible from outside
- **100% Test Coverage**: ✅ All 189 tests passing

#### Example Module Usage
```frame
from fsl import str, int, list  // Must explicitly import FSL

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
    var s = str(result)                  // FSL with explicit import
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
                return = "processed_" + data
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
- **✅ Test Coverage**: 207/207 tests passing (100% success rate) with 7 async tests
- **⚠️ Runtime Limitation**: Frame's synchronous event-driven architecture limits complex async state handler patterns

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
- **Interface Return Assignment**: `return = value` (sets return value directly in event handler)
- **System Return Variable**: `system.return = value` (sets interface return value from anywhere)

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
- `^=` - Old return assignment → Use `return = value`
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

### Interface Return Assignment (2025-01-17)
- **Decision**: Replace `^=` with `return = value` syntax
- **Rationale**: More conventional and readable syntax
- **Implementation**: Parser recognizes `return =` as interface return assignment
- **Migration**: `^= expr` → `return = expr`
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

### v0.35 Test Success - All Tests Passing
- **Total Tests**: 207/207 (100% success rate) 🎉
- **Module System Tests**: All passing (preserved from v0.34)
- **FSL Integration Tests**: All passing
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

### Debug Output
- Debug output goes to stderr (eprintln! in Rust)
- Use `FRAME_TRANSPILER_DEBUG=1` environment variable to enable debug output
- Never send debug output to stdout (it contaminates generated code)