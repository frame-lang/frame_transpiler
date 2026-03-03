# Frame Language Grammar (v0.83.0)

**Last Updated**: 2025-01-15  
**Status**: **100% Python test success rate (451/451 tests passing)**, TypeScript 98.8% transpilation success

This document provides the formal grammar specification for the Frame language using BNF notation, along with examples for each language construct.

## Module Structure

```bnf
module: (import_stmt | enum_decl | class_decl | var_decl | function | async_function | system)*
```

**v0.31 Import Support**: Modules can now include native import statements at the top level, supporting Python module imports without requiring backticks.

**v0.30 Multi-Entity Support**: Modules can contain any combination of functions and systems in any order. Each entity (function or system) can have individual attributes.

## Module System (v0.34 + v0.57)

Frame v0.34 introduces a complete module system with named modules, qualified access, and nested module support. Frame v0.57 extends this with multi-file module capabilities, including Frame file imports, dependency resolution, and project-wide compilation.

```bnf
module_decl: 'module' IDENTIFIER '{' module_body '}'
module_body: (import_stmt | enum_decl | var_decl | function | system | module_decl)*
qualified_name: IDENTIFIER ('.' IDENTIFIER)*
```

### Module Features

- **Named Modules**: Declare named modules with `module ModuleName { ... }`
- **Qualified Access**: Access module contents via `module.function()` or `module.variable`
- **Nested Modules**: Support for hierarchical module organization
- **Scope Isolation**: Each module has its own namespace
- **Import Integration**: Python imports work within module contexts

### Module Examples

```frame
// Named module with functions and variables
module Utils {
    var counter = 0
    
    fn increment() {
        counter = counter + 1
        return counter
    }
    
    fn reset() {
        counter = 0
    }
}

// Using module contents
fn main() {
    var val = Utils.increment()
    print("Counter: " + str(val))
    Utils.reset()
}

// Nested modules
module Math {
    module Constants {
        var PI = 3.14159
        var E = 2.71828
    }
    
    fn circleArea(r) {
        return Constants.PI * r * r
    }
}

// Module with Python imports
module DataProcessor {
    from fsl import str, int
    
    fn process(data) {
        var result = str(data)
        return "Processed: " + result
    }
}
```

## Multi-File Module System (v0.57) ✅ COMPLETE

Frame v0.57 delivers a fully functional multi-file module system infrastructure, enabling programs to be split across multiple `.frm` files with proper import/export mechanisms, dependency management, and automatic compilation.

### Frame File Import Syntax

```bnf
frame_import: 'import' IDENTIFIER 'from' STRING
frame_import_aliased: 'import' IDENTIFIER 'from' STRING 'as' IDENTIFIER  
frame_import_selective: 'import' '{' identifier_list '}' 'from' STRING
identifier_list: IDENTIFIER (',' IDENTIFIER)*
```

### Import Examples

```frame
# Standard module import from Frame file
import MathUtils from "./utils.frm"
import DataStore from "../lib/datastore.frm"

# Import with alias
import LongModuleName from "./very_long_module_name.frm" as LMN
import Calculator from "./calc.frm" as Calc

# Selective imports (destructuring)
import { add, subtract, multiply } from "./math_ops.frm"
import { validateEmail, validatePhone } from "./validators.frm"

# Using imported modules
fn main() {
    var sum = MathUtils.add(5, 3)
    var result = Calc.compute(10, 20)
    var valid = validateEmail("user@example.com")
}
```

### Infrastructure Components

**Core System Architecture:**
- **ModuleResolver**: Resolves import paths to file system paths with security validation
- **DependencyGraph**: Manages module dependencies with cycle detection using topological sorting  
- **ModuleCache**: Provides incremental compilation through SHA-256 based change detection
- **ModuleLinker**: Combines compiled modules with optimization strategies
- **MultiFileCompiler**: Orchestrates the complete multi-file compilation workflow

### Multi-File Compilation

Frame provides automatic multi-file compilation with the `-m` flag:

```bash
# Compile multi-file Frame project (concatenation mode - default)
framec -m main.frm -l python_3

# Generate separate Python files (NEW in v0.57!)
framec -m main.frm -l python_3 -o /output/dir

# The compiler automatically:
# 1. Discovers all imported .frm files
# 2. Resolves dependencies with cycle detection
# 3. Compiles in topological order
# 4. Links modules based on strategy:
#    - Default: Concatenates into single output file
#    - With -o flag: Generates separate .py files with proper imports
```

### Module Generation

Modules are transpiled to Python classes with static methods:

```frame
// Frame module with variables and functions
module MathUtils {
    var PI = 3.14159
    var E = 2.71828
    
    fn add(a, b) {
        return a + b
    }
    
    fn circleArea(radius) {
        return PI * radius * radius  // PI automatically qualified as MathUtils.PI
    }
}
```

Generates:

```python
class MathUtils:
    PI = 3.14159
    E = 2.71828
    
    @staticmethod
    def add(a, b):
        return a + b
    
    @staticmethod
    def circleArea(radius):
        return MathUtils.PI * radius * radius  # Properly qualified
```

### Test Runner Support

The Frame test runner automatically detects and handles multi-file tests:

```python
# Test runner detects Frame imports (.frm files)
# and automatically uses -m flag for compilation
if 'import' in line and '.frm' in line:
    use_multifile_compilation()
```

### Example Multi-File Structure (Planned)

```frame
// utils.frm - Utility module
module Utils {
    fn helper(x) {
        return x * 2
    }
    
    var version = "1.0"
}

// main.frm - Main application
import Utils from "./utils.frm"

fn main() {
    var result = Utils.helper(21)
    print("Result: " + str(result))
    print("Utils version: " + Utils.version)
}
```

### Security Features

- **Path Traversal Protection**: Prevents access outside project root
- **Symlink Validation**: Blocks dangerous symlinks to system directories
- **File Extension Validation**: Only `.frm` files can be imported
- **Project Boundary Enforcement**: All imports must be within project scope

### Performance Optimizations

- **Dependency Caching**: Resolution results cached in memory for fast lookup
- **Incremental Compilation**: SHA-256 based change detection for rebuild optimization
- **Topological Sorting**: Efficient compilation ordering with automatic cycle detection
- **Module Linking**: Smart concatenation with future dead-code elimination support

### Current Status (v0.57)

- ✅ **Infrastructure Complete**: All core components implemented and tested
- ✅ **Build Integration**: Successfully integrated with existing Frame compiler
- ✅ **Test Validation**: All 341 existing tests continue to pass (100% compatibility)
- 🔄 **Import Parsing**: Parser extensions for import statements (next phase)
- 🔄 **Cross-File Compilation**: Full multi-file project compilation (next phase)

## Classes (v0.45, Enhanced v0.58)

Frame v0.45 introduces class support for object-oriented programming, providing a familiar syntax for defining classes with methods and variables. v0.58 adds support for class decorators.

```bnf
class_decl: decorator* 'class' IDENTIFIER ('(' IDENTIFIER ')')? '{' class_body '}'  // v0.58: Decorators, v0.76: Python-style inheritance
decorator: '@' IDENTIFIER ('(' arguments? ')')?  // v0.58: Class decorators
class_body: (var_decl | method_decl | static_method_decl | property_method_decl)*

method_decl: 'fn' IDENTIFIER '(' parameters? ')' (':' type)? '{' statements '}'
static_method_decl: '@staticmethod' 'fn' IDENTIFIER '(' parameters? ')' (':' type)? '{' statements '}'
property_method_decl: '@property' 'fn' IDENTIFIER '(' ')' (':' type)? '{' statements '}'  // v0.55
```

### Class Features

- **Constructor Methods**: Methods named `init` become constructors (`__init__` in Python)
- **Instance Methods**: Regular methods with implicit `self` parameter
- **Static Methods**: Methods decorated with `@staticmethod` (no implicit `self`)
- **Class Variables**: Variables declared at class level (shared across instances)
- **Property Methods**: Methods decorated with `@property` for computed attributes (v0.55)
- **Class Decorators**: Python decorator pass-through for classes (v0.58)
- **Instance Variables**: Variables assigned via `self.varname` in methods
- **Method Calls**: Instance methods called via `object.method()`, static via `ClassName.method()`

### Class Decorators (v0.58)

Frame v0.58 adds support for Python class decorators, allowing direct pass-through of decorator syntax:

```frame
# Dataclass decorator
@dataclass
class Person {
    fn init(name, age) {
        self.name = name
        self.age = age
    }
}

# Multiple decorators
@dataclass
@frozen
class ImmutablePoint {
    fn init(x, y) {
        self.x = x
        self.y = y
    }
}

# Decorators with arguments
@decorator_with_args("value", key=42)
class ConfiguredClass {
    fn method() {
        return "configured"
    }
}
```

**Decorator Features**:
- **Python Pass-Through**: Decorators are passed directly to Python output
- **Multiple Decorators**: Stack multiple decorators on a single class
- **Parameterized Decorators**: Support for decorators with arguments
- **Standard Library**: Works with Python's built-in decorators (`@dataclass`, etc.)
- **Custom Decorators**: Compatible with user-defined decorators

### Class Examples

```frame
# Basic class with constructor and methods
class Point {
    # Class/static variable
    var instance_count = 0
    
    # Constructor (implicit init method)
    fn init(x, y) {
        self.x = x  # Instance variables
        self.y = y
        Point.instance_count = Point.instance_count + 1
    }
    
    # Instance method
    fn distance_to(other) {
        var dx = self.x - other.x
        var dy = self.y - other.y
        return ((dx * dx) + (dy * dy)) ** 0.5
    }
    
    # Static method
    @staticmethod
    fn origin() {
        return Point(0, 0)
    }
    
    # Special method (Python __str__)
    fn __str__() {
        return "Point(" + str(self.x) + ", " + str(self.y) + ")"
    }
}

# Class inheritance (v0.76: Python-style syntax)
class Point3D(Point) {
    fn init(x, y, z) {
        super.init(x, y)  # Call parent constructor
        self.z = z
    }
    
    fn distance_to(other) {
        var dx = self.x - other.x
        var dy = self.y - other.y
        var dz = self.z - other.z
        return ((dx * dx) + (dy * dy) + (dz * dz)) ** 0.5
    }
}

# Using classes
fn main() {
    var p1 = Point(3.0, 4.0)
    var p2 = Point(6.0, 8.0)
    var p3 = Point3D(1.0, 2.0, 3.0)
    var origin = Point.origin()  # Static method call
    
    var dist = p1.distance_to(p2)  # Instance method call
    print("Distance: " + str(dist))
    print("Points created: " + str(Point.instance_count))
}
```

### @property Decorator (v0.55)

Frame v0.55 supports the `@property` decorator for creating computed properties that behave like attributes:

```frame
class Rectangle {
    fn init(width, height) {
        self.width = width
        self.height = height
    }
    
    // Computed property using @property decorator
    @property
    fn area() {
        return self.width * self.height
    }
    
    @property
    fn perimeter() {
        return 2 * (self.width + self.height)
    }
}

fn main() {
    var rect = Rectangle(5, 3)
    
    // Access properties like attributes (no parentheses)
    print("Area: " + str(rect.area))        # Calls area() method
    print("Perimeter: " + str(rect.perimeter))  # Calls perimeter() method
}
```

### Class Decorators (v0.58)

Frame v0.58 supports Python decorator pass-through for classes, enabling integration with Python's decorator ecosystem:

```frame
from dataclasses import dataclass

# Simple decorator
@dataclass
class Point {
    var x = 0
    var y = 0
}

# Decorator with arguments
@dataclass(frozen=True)
class ImmutablePoint {
    var x = 0
    var y = 0
}

# Multiple decorators
@dataclass
@total_ordering
class ComparablePoint {
    var x = 0
    var y = 0
    
    fn __eq__(other) {
        return self.x == other.x and self.y == other.y
    }
    
    fn __lt__(other) {
        return self.x < other.x or (self.x == other.x and self.y < other.y)
    }
}
```

**Decorator Features**:
- **Pass-through**: Decorators are passed unchanged to Python output
- **Arguments Support**: Decorators can include parenthesized arguments
- **Multiple Decorators**: Multiple decorators can be stacked on a class
- **Method Decorators Preserved**: `@staticmethod` and `@property` continue to work for methods

### Key Differences from Traditional OOP

- **Implicit `self`**: Method signatures don't include `self` parameter (added automatically)
- **Inheritance**: Frame v0.76 uses Python-style syntax: `class Child(Parent)`
- **No Access Modifiers**: All members are public
- **Python-style Special Methods**: Use `__str__`, `__repr__`, etc. for special behavior
- **Decorator Pass-through**: Python decorators work directly (v0.58)

## Scope Rules (v0.31)

Frame implements LEGB (Local, Enclosing, Global, Built-in) scope resolution with strict isolation between functions and systems:

### Scope Hierarchy
1. **Local**: Current function/system scope, block variables
2. **Enclosing**: Parent scopes up to module level
3. **Global**: Module-level declarations (functions, systems)
4. **Built-in**: Language built-ins (print, str, int, len)

### Scope Isolation Rules
- **Functions cannot access system internals**: Actions and operations are private to their system
- **Systems cannot access other systems' internals**: Each system is fully encapsulated
- **Module-level functions are globally accessible**: Can be called from any function or system
- **Built-ins are universally accessible**: Available in all contexts

### Example
```frame
fn moduleFunc() { return "global" }  // Module-level, accessible everywhere

system SystemA {
    actions:
        privateAction() { }  // Only accessible within SystemA
}

system SystemB {
    machine:
        $Start {
            test() {
                moduleFunc()     // ✅ Can call module function
                // privateAction()  // ❌ Cannot access SystemA's action
            }
        }
}

fn main() {
    moduleFunc()         // ✅ Can call module function
    // privateAction()      // ❌ Cannot access system action
}
```

## Import Statements (v0.31 + v0.57)

Frame v0.31 introduces native Python import statement support. Frame v0.57 extends this with Frame file imports for multi-file projects.

```bnf
import_stmt: python_import | frame_import

# Python imports (v0.31)
python_import: simple_import | aliased_import | from_import
simple_import: 'import' dotted_name
aliased_import: 'import' dotted_name 'as' IDENTIFIER
from_import: 'from' dotted_name 'import' (import_items | '*')

# Frame file imports (v0.57)
frame_import: frame_module_import | frame_aliased_import | frame_selective_import
frame_module_import: 'import' IDENTIFIER 'from' STRING_LITERAL
frame_aliased_import: 'import' IDENTIFIER 'from' STRING_LITERAL 'as' IDENTIFIER
frame_selective_import: 'import' '{' import_items '}' 'from' STRING_LITERAL

dotted_name: IDENTIFIER ('.' IDENTIFIER)*
import_items: IDENTIFIER (',' IDENTIFIER)*
```

### Python Import Examples (v0.31)
```frame
# Simple imports
import math
import json

# Aliased imports
import numpy as np
import os.path as osp

# From imports
from collections import defaultdict, OrderedDict
from typing import List, Dict, Optional

# Wildcard imports
from typing import *

# Using imported modules in functions
fn main() {
    var pi = math.pi
    var root = math.sqrt(16)
    var data = json.dumps({"key": "value"})
}

# Using imported modules in systems
system Calculator {
    operations:
        compute() {
            var result = math.cos(0)
            return result
        }
}

### Frame File Import Examples (v0.57)
```frame
# Import a Frame module from a file
import Utils from "./utils.frm"
import Calculator from "../lib/calculator.frm"

# Import with alias
import DataProcessor from "./processor.frm" as DP
import MathUtils from "./math_utils.frm" as Math

# Selective imports (destructuring)
import { add, subtract, multiply } from "./math_ops.frm"
import { validateEmail, validatePhone } from "./validators.frm"

# Using imported Frame modules
fn main() {
    var result = Utils.process(42)
    var sum = Math.add(10, 20)
    
    # Selective imports used directly
    var product = multiply(5, 6)
    var isValid = validateEmail("user@example.com")
}

# Note: During single-file compilation, Frame imports generate
# placeholder comments. The multi-file compiler will resolve
# and link these imports during the build phase.
```
```

## Backtick Expressions (Deprecated in v0.37)

**Status**: Being phased out in favor of native Frame syntax

Backtick expressions (`\`expression\``) were previously used for embedding target language code directly. As of v0.37, Frame is moving away from backticks:

### Current Limitations Without Backticks
- **Module member access**: `math.pi`, `json.dumps()` not yet supported natively
- **Complex indexing**: Dictionary assignment, chained indexing
- **Method chaining**: Complex method call chains

### Workarounds
```frame
// Instead of: var pi = `math.pi`
import math
var pi = 3.14159  // Use literal value for now

// Instead of: var result = `dict[key]`
// Use simplified approach until native support added
```

### Future Direction
The goal is to support all common patterns natively without backticks. Module member access syntax (`module.member`) is planned for a future release.

## Native Python Functions (v0.31)

Frame v0.31 provides direct access to Python built-in functions without requiring Frame-specific built-ins:

```frame
// Python built-ins are directly accessible
fn main() {
    print("Hello, World!")           // Python's print function
    var x = str(42)                  // Python's str function
    var y = int("10")                // Python's int function
    var z = len([1, 2, 3])          // Python's len function
    var result = max(5, 10)         // Python's max function
}
```

**Note**: Frame no longer maintains its own built-in print function. All function calls that are not declared in Frame scope are passed through to the target language (Python).

## Functions

Frame v0.30 supports multiple functions per module with any names. Functions are peer entities alongside systems within modules.

```bnf
function: attributes? 'fn' IDENTIFIER '(' parameter_list? ')' type? function_body
function_body: '{' stmt* '}'
parameter_list: parameter (',' parameter)*
parameter: IDENTIFIER type?
type: ':' type_expr
type_expr: IDENTIFIER | SUPERSTRING
```

**Note**: Function parameter lists always require parentheses `()`, even when empty. The `parameter_list?` indicates the parameters inside are optional, but the parentheses themselves are mandatory.

**v0.30 Feature**: Multiple functions are fully supported with any function names. Empty parameter lists `()` are fully supported, unlike v0.11 which rejected empty parameter syntax in certain contexts.

### Function-System Integration

Functions can interact with systems through public interfaces:

- **Operations**: Use `SystemName.operationName()` syntax for static method calls
- **Interface Methods**: Use `systemInstance.methodName()` syntax for instance method calls  
- **Actions**: Not accessible from functions (private implementation details)

```bnf
system_operation_call: IDENTIFIER '.' IDENTIFIER '(' argument_list? ')'
system_instance_call: IDENTIFIER '.' IDENTIFIER '(' argument_list? ')'
```

### Function Examples
```frame
// Multiple functions in v0.30
fn main() {
    helper("test")
    var result = calculate(10, 20)
    print(result)
}

fn helper(msg) {
    print("Helper: " + msg)
}

fn calculate(x, y) {
    return x * y + 5
}

// Function calling system operations (static methods)
fn main() {
    var result = Utils.add(5, 3)
    print("5 + 3 = " + str(result))
    
    var category = Utils.categorizeNumber(42)
    print("42 is " + category)
}

// Function calling system interface methods (instance methods)
fn demo() {
    var counter = Counter()
    counter.increment()
    counter.increment()
    print("Count: " + str(counter.getCount()))
}

system Utils {
    operations:
        add(x: int, y: int): int {
            return x + y
        }
        
        categorizeNumber(num: int): string {
            if num < 10 {
                return "single digit"
            } else {
                return "multi digit"
            }
        }
}
```

## Module Variables (v0.31)

Module-level variables can be declared at the top level of a Frame module, making them accessible from all functions and systems in the module.

```bnf
module_var: 'var' IDENTIFIER (',' IDENTIFIER)* type? '=' expr (',' expr)*  // v0.53: Multiple variable declarations
```

### Module Variable Features

- **Global Accessibility**: Module variables are accessible from any function or system in the module
- **Automatic Global Declaration**: The transpiler automatically generates `global` declarations in Python when module variables are modified
- **Shadowing Protection**: Local variables cannot shadow module variables (enforced at transpilation for Python target)
- **Type Annotations**: Optional type annotations for better code clarity

### Module Variable Examples

```frame
// Module-level variables
var counter = 0
var message: string = "Hello"
var data = []

// Multiple variable declarations (v0.53)
var x, y, z = 1, 2, 3
var a, b = 10, 20

fn increment() {
    counter = counter + 1  // Automatic 'global counter' in Python
    return counter
}

## Type Annotations (v0.55)

Frame v0.55 confirms full support for Python-style type annotations, enabling better code generation and type safety across Frame programs.

```bnf
type_annotation: ':' type_expr
type_expr: IDENTIFIER  // Simple types like int, str, bool
         | IDENTIFIER '[' type_list ']'  // Generic types like List[int]
         | type_expr '|' type_expr  // Union types (Python 3.10+)
```

### Type Annotation Features

**Supported Contexts:**
- Function parameters: `fn process(data: str, count: int)`
- Function returns: `fn calculate() : float`
- Variable declarations: `var name: str = "Frame"`
- State parameters: `$Active(min: int, max: int)`
- Interface methods: `processData(input: str) : bool`

```frame
// Function with type annotations
fn calculate(x: float, y: float) : float {
    return x * y / 2.0
}

// Variable with type annotation
var result: float = calculate(3.14, 2.0)

// System with typed interface
system TypedProcessor {
    interface:
        process(data: str) : int
        validate(input: Any) : bool
    
    machine:
        $Ready {
            process(data: str) : int {
                var length: int = len(data)
                @@:return = length
                return
            }
            
            validate(input: Any) : bool {
                @@:return = input != None
                return
            }
        }
}

// State with typed parameters
system ConfigurableTimer {
    machine:
        $Active(duration: float, name: str) {
            $>() {
                print(name + " active for " + str(duration) + " seconds")
            }
        }
}
```

**Type Annotation Benefits:**
- **Better Code Generation**: Type hints are preserved in generated Python code
- **IDE Support**: Enhanced autocompletion and type checking in IDEs
- **Documentation**: Types serve as inline documentation
- **Runtime Safety**: Optional runtime type checking in target languages

## Type Aliases (v0.56)

Frame v0.56 introduces type aliases for creating custom type definitions, following Python 3.12+ syntax.

```bnf
type_alias: 'type' IDENTIFIER '=' type_expr
```

### Type Alias Features

- **Custom Type Names**: Create readable aliases for complex types
- **Generic Types**: Support for parameterized type aliases
- **Improved Readability**: Replace complex type expressions with meaningful names
- **Python 3.12+ Compatible**: Generates modern Python type alias syntax

### Type Alias Examples

```frame
# Simple type alias
type UserID = int
type Username = str

# Complex type aliases
type Point = tuple[float, float]
type Coordinates = list[Point]
type UserDict = dict[UserID, Username]

# Using type aliases in functions
fn processUser(id: UserID, name: Username) : bool {
    return id > 0 and name != ""
}

# Using with generic types
type Optional[T] = T | None
type Result[T, E] = tuple[bool, T | E]

fn divide(a: float, b: float) : Result[float, str] {
    if b == 0 {
        return (false, "Division by zero")
    }
    return (true, a / b)
}
```

**Implementation Note**: The `type` keyword is context-sensitive - it's only recognized as a keyword when starting a type alias declaration. The built-in `type()` function works normally in all other contexts.

## Async/Await Support (v0.35-v0.37)

Frame v0.35-v0.37 introduces comprehensive async/await support for asynchronous programming:

```bnf
async_function: 'async' 'fn' IDENTIFIER '(' parameter_list? ')' type? function_body
async_interface_method: 'async' IDENTIFIER '(' parameter_list? ')'
async_event_handler: 'async' event_handler
await_expr: 'await' expr
```

### Async Features

- **Async Functions**: Functions declared with `async fn` become coroutines
- **Async Interface Methods**: Interface methods can be marked as `async`
- **Async Event Handlers**: State handlers can be explicitly marked as `async`
- **Await Expressions**: Use `await` to wait for async operations
- **Automatic Propagation**: State handlers become async when handling async interface events
- **Runtime Infrastructure**: v0.37 adds runtime nodes for tracking async requirements

### Async Examples

```frame
// Async function
async fn fetchData(url) {
    print("Fetching from " + url)
    var response = await http_get(url)
    return response
}

// System with async interface methods
system DataProcessor {
    interface:
        async processData(data)  // Async interface method
        getStatus()              // Sync interface method
    
    machine:
        $Ready {
            // Handler automatically async due to interface method
            async processData(data) {
                var result = await process_item(data)
                @@:return = result
            }

            getStatus() {
                @@:return = "ready"
            }
        }
}

// Explicit async event handlers (v0.37)
system AsyncMachine {
    machine:
        $Processing {
            // Explicitly marked async enter handler
            async $>() {
                var data = await initialize()
                self.data = data
            }
            
            // Async event handler
            async handleRequest(id) {
                var result = await fetch_item(id)
                @@:return = result
            }
        }
}
```

### Async Chain Validation (v0.37)

Frame v0.37 validates async chains at compile time:
- Handlers using `await` must be marked `async`
- Enter/exit handlers in async transition chains must be async
- Clear error messages explain which handlers need async marking

## Slicing Operations (v0.37, enhanced v0.38)

Frame v0.37 adds full Python-style slicing support for strings and lists:

```bnf
slice_expr: expr '[' slice_notation ']'
slice_notation: slice_component? ':' slice_component? (':' slice_component?)?
slice_component: expr
```

### Slicing Features

- **Basic Slices**: `text[:5]`, `list[2:8]`, `data[7:]`
- **Step Parameter**: `list[::2]`, `data[::-1]`, `nums[1:8:2]`
- **Negative Indices**: `text[-5:]`, `list[:-2]`
- **Expression Support**: Complex expressions in slice positions `text[start+1:end-1]`
- **String Slicing**: Full support for string slicing
- **List Slicing**: Full support for list slicing

### Slicing Examples

```frame
fn demonstrateSlicing() {
    var text = "Hello, World!"
    var nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    
    // Basic slicing
    print(text[:5])         // "Hello"
    print(text[7:])         // "World!"
    print(nums[:5])         // [0, 1, 2, 3, 4]
    print(nums[2:8])        // [2, 3, 4, 5, 6, 7]
    
    // Step parameter
    print(nums[::2])        // [0, 2, 4, 6, 8]
    print(nums[1::2])       // [1, 3, 5, 7, 9]
    print(text[::-1])       // "!dlroW ,olleH" (reversed)
    
    // Negative indices
    print(text[-6:])        // "World!"
    print(nums[-3:])        // [7, 8, 9]
    print(nums[:-5])        // [0, 1, 2, 3, 4]
    
    // Expressions in slices (v0.38)
    var start = 2
    var end = 8
    print(nums[start:end])       // [2, 3, 4, 5, 6, 7]
    print(nums[start+1:end-1])   // [3, 4, 5, 6]
    print(text[len(text)-6:])    // "World!"
}
```

## Native String Operations (v0.38)

Frame fully supports all Python string methods through natural pass-through to the target language. No special imports or syntax required.

### String Search Methods
```frame
var text = "Hello, World! Hello, Frame!"

// Finding substrings
var pos = text.find("World")           // 7 (first occurrence, -1 if not found)
var last = text.rfind("Hello")         // 14 (last occurrence)
var idx = text.index("Frame")          // 21 (raises ValueError if not found)
var ridx = text.rindex("Hello")        // 14 (last occurrence, raises if not found)
var count = text.count("Hello")        // 2 (count occurrences)
```

### String Check Methods
```frame
var text = "Frame Language"

// Prefix/suffix checks
var starts = text.startswith("Frame")      // true
var ends = text.endswith("age")           // true

// Character type checks
"12345".isdigit()                          // true
"abcdef".isalpha()                         // true
"abc123".isalnum()                         // true
"   ".isspace()                            // true
"hello".islower()                          // true
"HELLO".isupper()                          // true
"Hello World".istitle()                    // true
"valid_identifier".isidentifier()          // true
```

### String Transformation Methods
```frame
var text = "  Hello, World!  "

// Case transformations
text.upper()                    // "  HELLO, WORLD!  "
text.lower()                    // "  hello, world!  "
text.title()                    // "  Hello, World!  "
text.capitalize()               // "  hello, world!  "
text.swapcase()                 // "  hELLO, wORLD!  "

// Stripping whitespace
text.strip()                    // "Hello, World!"
text.lstrip()                   // "Hello, World!  "
text.rstrip()                   // "  Hello, World!"

// Replace
text.replace("World", "Frame")  // "  Hello, Frame!  "
```

### String Split and Join
```frame
var text = "apple,banana,cherry"

// Splitting
var parts = text.split(",")              // ["apple", "banana", "cherry"]
var rsplit_parts = text.rsplit(",", 1)   // ["apple,banana", "cherry"]
var lines = "Line 1\nLine 2".splitlines() // ["Line 1", "Line 2"]

// Partitioning
var part = text.partition(",")           // ("apple", ",", "banana,cherry")
var rpart = text.rpartition(",")         // ("apple,banana", ",", "cherry")

// Joining
var separator = " | "
var joined = separator.join(parts)       // "apple | banana | cherry"
```

### String Formatting
```frame
var text = "Frame"

// Alignment and padding
text.center(10, "*")            // "**Frame***"
text.ljust(10, "-")            // "Frame-----"
text.rjust(10, "+")            // "+++++Frame"
"42".zfill(5)                  // "00042"

// Format strings
"Hello, {}!".format("Frame")    // "Hello, Frame!"
```

## Native List Operations (v0.38)

Frame fully supports all Python list methods through natural pass-through to the target language.

### List Creation and Access
```frame
// List creation
var empty = []
var numbers = [1, 2, 3, 4, 5]
var mixed = [1, "hello", 3.14, true, None]
var nested = [[1, 2], [3, 4]]

// Indexing and length
var first = numbers[0]          // 1
var last = numbers[-1]          // 5
var length = len(numbers)       // 5
```

### List Modification Methods
```frame
var list = [1, 2, 3]

// Adding elements
list.append(4)                  // [1, 2, 3, 4]
list.insert(1, 99)             // [1, 99, 2, 3, 4]
list.extend([5, 6])            // [1, 99, 2, 3, 4, 5, 6]

// Removing elements
list.remove(99)                 // Remove first occurrence
var last = list.pop()          // Remove and return last element
var item = list.pop(0)         // Remove and return element at index
list.clear()                   // Remove all elements
```

### List Search and Query
```frame
var list = [10, 20, 30, 20, 40]

// Finding elements
var idx = list.index(20)        // 1 (first occurrence)
var count = list.count(20)      // 2 (count occurrences)

// Membership tests
var has_30 = 30 in list        // true
var no_99 = 99 not in list     // true
```

### List Ordering and Copying
```frame
var list = [3, 1, 4, 1, 5, 9]

// Ordering
list.sort()                     // Sort in place
list.reverse()                  // Reverse in place

// Copying
var copy = list.copy()          // Shallow copy
```

### List Comprehensions
```frame
// Basic comprehension
var squares = [x * x for x in range(5)]

// With condition
var evens = [x for x in range(10) if x % 2 == 0]

// Nested comprehension
var matrix = [[i + j for j in range(3)] for i in range(3)]
```

### List Unpacking Operator
```frame
var list1 = [1, 2, 3]
var list2 = [4, 5, 6]

// Unpacking in list literal
var combined = [*list1, *list2, 7, 8]  // [1, 2, 3, 4, 5, 6, 7, 8]
```

## With Statement Support (v0.37)

Frame v0.37 adds support for context managers via with statements:

```bnf
with_stmt: 'with' expr 'as' IDENTIFIER '{' stmt* '}'
async_with_stmt: 'async' 'with' expr 'as' IDENTIFIER '{' stmt* '}'
```

### With Statement Examples

```frame
// Synchronous with statement
fn readFile(path) {
    with open(path, "r") as file {
        var content = file.read()
        return content
    }
}

// Async with statement
async fn fetchWithSession() {
    async with aiohttp.ClientSession() as session {
        async with session.get("https://api.example.com") as response {
            var data = await response.json()
            return data
        }
    }
}
```

fn getMessage() {
    return message  // Read access doesn't need global declaration
}

system DataCollector {
    interface:
        collect(value)
    
    machine:
        $Start {
            collect(value) {
                data.append(value)  // Automatic 'global data' in Python
                counter = counter + 1  // Automatic 'global counter' in Python
                return
            }
        }
}
```

### Implementation Notes

- **Python Target**: The transpiler performs two-pass analysis to identify module variable modifications and automatically insert `global` declarations where needed
- **Read vs Write**: Only modifications require global declarations; read-only access works without them
- **Conditional Imports**: Import statements like `from enum import Enum` are only generated when actually used

## Enumerations (v0.32)

Frame v0.32 introduces advanced enum support with custom values, string enums, and iteration capabilities.

```bnf
enum_decl: 'enum' IDENTIFIER enum_type? '{' enum_member_list '}'
enum_type: ':' ('int' | 'string')
enum_member_list: enum_member (',' enum_member)* ','?
enum_member: IDENTIFIER ('=' enum_value)?
enum_value: integer_literal | string_literal | negative_integer
negative_integer: '-' integer_literal
```

### Integer Enums

Integer enums support custom values, negative values, and auto-increment:

```frame
// Default auto-increment from 0
enum Status {
    Idle,      // 0
    Running,   // 1
    Complete   // 2
}

// Custom values
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    ServerError = 500
}

// Mixed explicit and auto values
enum Priority {
    Low = -1,    // -1
    Normal,      // 0 (auto continues from -1)
    High = 10,   // 10
    Critical     // 11 (auto continues from 10)
}
```

### String Enums

String enums use the `: string` type annotation:

```frame
// Explicit string values
enum Color : string {
    Red = "red",
    Green = "green",
    Blue = "blue"
}

// Auto string values (uses member name)
enum LogLevel : string {
    Debug,    // "Debug"
    Info,     // "Info"
    Warning,  // "Warning"
    Error     // "Error"
}
```

### Module-Scope Enums

Enums can be declared at module level (outside systems):

```frame
// Module-level enum
enum GlobalStatus {
    Active,
    Inactive
}

fn main() {
    var status = GlobalStatus.Active
    print(status.name)  // "Active"
}

system Monitor {
    machine:
        $Idle {
            check() {
                if state == GlobalStatus.Active {
                    // ...
                }
            }
        }
}
```

### Enum Iteration

Enums support iteration using for-in loops:

```frame
enum MenuOption {
    NewFile,
    OpenFile,
    SaveFile,
    Exit
}

fn displayMenu() {
    for option in MenuOption {
        print(option.name + " = " + option.value)
    }
}
```

### Enum Properties

Enum members have `.name` and `.value` properties:

```frame
var status = HttpStatus.NotFound
print(status.name)   // "NotFound"
print(status.value)  // 404
```

## Systems

```bnf
system: 'system' IDENTIFIER system_params? '{' 
        operations_block?
        interface_block?
        machine_block?
        actions_block?
        domain_block?
        '}'

system_params: '(' system_param_list ')'
system_param_list: system_param (',' system_param)*
system_param: start_state_param | enter_event_param | domain_param
start_state_param: '$(' parameter_list ')'
enter_event_param: '$>(' parameter_list ')'
domain_param: IDENTIFIER type?
```

**Block Order**: System blocks must appear in the specified order when present: `operations:`, `interface:`, `machine:`, `actions:`, `domain:`. Blocks are optional but order is enforced by the parser.

**v0.38 Parser Limitation**: Domain blocks with dictionary literals currently must appear as the last block in a system definition. If a domain block with dictionary initialization appears before other blocks, a parse error will occur.

### System Examples

#### Basic System
```frame
system TrafficLight {
    interface:
        tick()
    
    machine:
        $Red {
            $>() {
                print("Red")
            }
            
            tick() {
                -> $Green
            }
        }
        
        $Green {
            $>() {
                print("Green")
            }
            
            tick() {
                -> $Yellow
            }
        }
        
        $Yellow {
            $>() {
                print("Yellow")
            }
            
            tick() {
                -> $Red
            }
        }
}
```

#### System with Parameters
```frame
// System with start state parameters
system StartStateParameters ($(msg)) {
    machine:
        $Start(msg) {
            $>() {
                print(msg)
                return
            }
        }
}

// System with start state enter event parameters
system StartStateEnterParameters ($>(msg)) {
    machine:
        $Start {
            $>(msg) {
                print(msg)
                return
            }
        }
}

// System with domain parameters
system DomainParameters (msg) {
    domain:
        var msg = None
        
    machine:
        $Start {
            $>() {
                print(msg)
                return
            }
        }
}

// System with all parameter types
system AllParameterTypes ($(A,B), $>(C,D), E,F) {
    domain:
        var E = None
        var F = None
    
    machine:
        $Start(A,B) {
            $>(C,D) {
                print(A + B + C + D + E + F)
                return
            }
        }
}
```

### State Parameters (v0.55)

Frame v0.55 adds full support for state parameters, allowing states to receive and store values when transitioned to, similar to function parameters.

**State Parameter Features:**
- **Parameter Declaration**: States can declare parameters with optional type annotations
- **Transition Arguments**: Pass values when transitioning to parameterized states
- **Parameter Access**: State parameters accessible throughout state handlers
- **State Variables**: Can initialize state variables from state parameters
- **Type Safety**: Optional type annotations for parameters

```frame
system TimerSystem {
    interface:
        configure(min_val, max_val)
        start(duration)
    
    machine:
        $Idle {
            configure(min_val, max_val) {
                // Transition with arguments to parameterized state
                -> $Configured(min_val, max_val)
            }
            
            start(duration) {
                -> $Counting(duration)
            }
        }
        
        // State with parameters
        $Configured(min: int, max: int) {
            // State variable initialized from parameter
            var current = min
            
            $>() {
                print("Configured with range: " + str(min) + " to " + str(max))
            }
            
            increment() {
                current = current + 1
                if current > max {
                    current = min
                }
                print("Current: " + str(current))
            }
        }
        
        // State with single parameter
        $Counting(timeout: int) {
            var remaining = timeout
            
            $>() {
                print("Starting countdown from " + str(timeout))
            }
            
            tick() {
                remaining = remaining - 1
                if remaining <= 0 {
                    -> $Idle
                }
                print("Remaining: " + str(remaining))
            }
        }
}
```

**State Parameter Usage:**
- Parameters are scoped to the state and persist across event handlers
- Parameters can have type annotations for better code generation
- State variables can be initialized using state parameters
- Transitions must provide matching arguments for target state parameters

### System Instantiation

System instantiation uses flattened argument lists:

```frame
fn main() {
    // No parameters
    var sys1 = TrafficLight()
    
    // Start state parameters - flattened list
    var sys2 = StartStateParameters("hello")
    
    // Start state enter event parameters - flattened list
    var sys3 = StartStateEnterParameters("world")
    
    // Domain parameters - flattened list
    var sys4 = DomainParameters("message")
    
    // All parameter types - flattened list
    var sys5 = AllParameterTypes("a", "b", "c", "d", "e", "f")
}
```

## Interface Block

```bnf
interface_block: 'interface:' interface_method*
interface_method: IDENTIFIER '(' parameter_list? ')' (type ('=' expr)?)?
```

**v0.31 Default Return Values**: Interface methods can specify default return values using the syntax `: type = value`. This value is returned unless overridden by event handlers or `@@:return` assignments.

## Machine Block

```bnf
machine_block: 'machine:' state*
state: '$' IDENTIFIER state_params? ('=>' '$' IDENTIFIER)? '{' event_handler* state_var* '}'
state_params: '(' parameter_list ')'  // v0.55: State parameters support
event_handler: event_selector '{' stmt* terminator? '}'
event_selector: IDENTIFIER '(' parameter_list? ')' (type ('=' expr)?)?
               | '$>' '(' parameter_list? ')'  // Enter handler
               | '<$' '(' parameter_list? ')'  // Exit handler
terminator: 'return' expr?
          | '=>'              // Forward/dispatch event  
          | '->' '$' IDENTIFIER ('(' expr_list ')')?  // Transition with optional arguments
stmt: parent_dispatch_stmt | /* other statements */
parent_dispatch_stmt: '=>' '$^'  // Parent dispatch statement
state_var: 'var' IDENTIFIER (',' IDENTIFIER)* type? '=' expr (',' expr)*  // v0.53: Multiple variable declarations
```

## Domain Block

```bnf
domain_block: 'domain:' (domain_var | enum_decl)*
domain_var: 'var' IDENTIFIER (',' IDENTIFIER)* type? '=' expr (',' expr)*  // v0.53: Multiple variable declarations
```

### Domain Variable Access (v0.31)

Domain variables are accessed using the `self.variable` syntax, which clearly distinguishes them from local variables and parameters.

```frame
system Counter {
    domain:
        var count: int = 0
        var message: string = "Count"
    
    interface:
        increment()
        getValue(): int
    
    machine:
        $Start {
            increment() {
                // Using self.variable for domain access
                self.count = self.count + 1
                print(self.message + ": " + str(self.count))
                return
            }
            
            getValue(): int {
                @@:return = self.count  // self.variable in @@:return assignment
            }
        }
}
```

### Self.Variable Features (v0.31)

- **Explicit Domain Access**: `self.` prefix required for all domain variable access
- **Lvalue and Rvalue**: Works in both assignment targets and expressions
- **Nested Expressions**: Fully supported in complex expressions
- **Method Arguments**: Can be passed as arguments to methods
- **All Contexts**: Works in operations, actions, and event handlers

### Self.Variable Examples

```frame
system SelfVariableDemo {
    domain:
        var x: int = 0
        var y: int = 0
        var data = []
    
    operations:
        process() {
            // Lvalue assignment
            self.x = 100
            
            // Rvalue in expression
            var doubled = self.x * 2
            
            // Complex expressions
            self.y = (self.x + 10) * 2
            
            // Method arguments
            self.data.append(self.x)
            print("Value: " + str(self.y))
            
            // Chained assignment
            self.x = self.y = 50
        }
}

## Operations Block

```bnf
operations_block: 'operations:' operation*
operation: attribute* IDENTIFIER '(' parameter_list? ')' (type ('=' expr)?)? '{' stmt* '}'
attribute: '@' IDENTIFIER  // Python-style attributes (e.g., @staticmethod)
```

**v0.31 Operations Restriction**: Operations (non-static) can use `@@:return` but cannot use Frame state-machine syntax (transitions, forwards, stack ops). This is enforced at parse time (E401).

**v0.30 Implementation Note**: Operations and actions are resolved at code generation time through symbol table lookup. Calls to operations generate with `self.` prefix for instance methods, while static operations use `ClassName.method()` syntax. Actions automatically receive the `_do` suffix in generated code.

**v0.31 Self Expression Support**: The `self` keyword can now be used as a standalone expression (e.g., `jsonpickle.encode(self)`), not just in dotted access (`self.variable`). Static operations marked with `@staticmethod` cannot use `self` - this is validated at parse time.

### Operations Examples

#### Instance Operations
```frame
system Calculator {
    operations:
        // Instance operation - includes implicit 'self' parameter
        getResult(): int {
            return self.currentValue  // Must use self. to access domain vars
        }
        
        // Instance operation using self as expression
        serialize(): String {
            return jsonpickle.encode(self)  // self as standalone expression (v0.31)
        }
    
    domain:
        var currentValue: int = 0
}
```

#### Static Operations  
```frame
system MathUtils {
    operations:
        // Static operation - no 'self' parameter, callable without instance
        @staticmethod
        add(a: int, b: int): int {
            return a + b
        }
        
        @staticmethod
        multiply(x: int, y: int): int {
            return x * y
        }
        
        // Static operations CANNOT use self - parse error
        @staticmethod
        invalid(): int {
            // return self.value  // ERROR: Cannot use 'self' in static operation
            return 0
        }
}
```

## Conditional Statements

### If Statement Grammar
```bnf
if_stmt: 'if' expr ':' stmt elif_clause* else_clause?
       | 'if' expr block elif_clause* else_clause?

elif_clause: 'elif' expr ':' stmt
           | 'elif' expr block

else_clause: 'else' ':' stmt  
           | 'else' block

block: '{' stmt* '}'
```

### Design Decisions

1. **Two Syntax Forms**: Python-style with colons for single statements, braced blocks for multiple statements

2. **Python-style for single statements**:
   ```frame
   if x > 5:
       doSomething()
   elif y < 10:
       doOther()
   else:
       doDefault()
   ```

3. **Braced blocks for multiple statements**:
   ```frame
   if x > 5 {
       doSomething()
       doMore()
   } elif y < 10 {
       doOther()
       doAnother()
   } else {
       doDefault()
   }
   ```

4. **Mixed syntax allowed**:
   ```frame
   if simpleCondition:
       singleStatement()
   elif complexCondition {
       firstStatement()
       secondStatement()
   } else:
       fallbackStatement()
   ```

5. **Syntax Enforcement**: 
   - After `:` only single statements are allowed (no `{` blocks)
   - After condition without `:`, braces `{` are required for blocks
   - Invalid: `if x: { stmt }` or `else: { stmt }`
   - Valid: `if x: stmt` or `if x { stmt }`

6. **Clear block boundaries**: Colons mark single statements, braces mark multi-statement blocks

7. **No parentheses required**: Conditions don't need parentheses (but are allowed)

8. **If as Statement**: `if` is a statement, not an expression

### Syntax Errors

The parser enforces strict separation between Python-style and braced syntax:

**Invalid Syntax** (will cause parser errors):
```frame
// ERROR: Block statement after colon
if x: {
    doSomething()
}

// ERROR: Block statement after colon  
elif y: {
    doOther()
}

// ERROR: Block statement after colon
else: {
    doDefault()
}
```

**Valid Syntax**:
```frame
// Correct: Python-style with single statements
if x: doSomething()
elif y: doOther()  
else: doDefault()

// Correct: Braced blocks
if x {
    doSomething()
}
elif y {
    doOther()
}
else {
    doDefault()
}
```

## Actions Block

### Action Grammar
```bnf
actions_block: 'actions:' action*
action: IDENTIFIER '(' parameter_list? ')' (type ('=' expr)?)? action_body
action_body: '{' stmt* '}'
parameter_list: parameter (',' parameter)*
parameter: IDENTIFIER type?
type: ':' IDENTIFIER
```

**v0.31 Default Values**: Actions can specify default return values for their return to the caller (not `@@:return`). Actions can set `@@:return` explicitly.

### Design Decisions

1. **Braces Required**: Action bodies must always use braces `{}`
2. **Statements**: Action bodies can contain any valid statements including if/elif/else with return statements
3. **Parameters**: Optional parameter list with optional types
4. **Return Type**: Optional return type annotation
5. **Return Statements**: Full support for return statements as regular statements (v0.20 improvement)

### Action Method Examples

```frame
actions:
    // Simple action with return
    add(x: int, y: int): int {
        return x + y
    }
    
    // Action with conditional returns
    classify(score: int): string {
        if score >= 90 {
            return "A"
        } elif score >= 80 {
            return "B"
        } elif score >= 70 {
            return "C"
        } else {
            return "F"
        }
    }
    
    // Early return validation pattern
    validate(input: string): bool {
        if input == "" {
            return false  // Early return for invalid input
        }
        
        if checkFormat(input) {
            return true
        }
        
        return false
    }
```

## Transitions and Events

### Transition Grammar
```bnf
transition: ('(' exit_args ')')? '->' ('(' enter_args ')')? label? '$' state_identifier ('(' state_params ')')?
transition: '(' '->' ('(' enter_args ')')? label? '$' state_identifier ('(' state_params ')')? ')'
exit_args: expr_list
enter_args: expr_list
state_params: expr_list
label: STRING
```

### Enter and Exit Events

#### Enter Event Handler
```bnf
enter_handler: '$>' '(' parameter_list? ')' '{' stmt* '}'
```

#### Exit Event Handler  
```bnf
exit_handler: '<$' '(' parameter_list? ')' '{' stmt* '}'
```

### Hierarchical State Machines

Frame v0.20+ supports hierarchical state machines where child states can inherit behavior from parent states and forward events up the hierarchy. This enables code reuse and structured state organization.

```bnf
hierarchy: '$' IDENTIFIER '=>' '$' IDENTIFIER
parent_dispatch: '=>' '$^'
```

**Key Features:**
- **State Inheritance**: Child states automatically inherit event handlers from parent states
- **Parent Dispatch**: Events can be forwarded from child to parent using `=> $^`
- **Compartment Hierarchy**: Runtime maintains proper parent-child compartment relationships
- **Syntax Restrictions**: Only `=> $^` (dispatch) is allowed; `-> $^` (transition to parent) is blocked

#### State Hierarchy Declaration

```frame
machine:
    // Parent state with common event handlers
    $Parent {
        commonEvent() {
            print("Handled in parent state")
            return
        }
        
        sharedLogic() {
            print("Common logic for all child states")
            return
        }
    }
    
    // Child state inherits from parent
    $Child => $Parent {
        specificEvent() {
            print("Handled only in child state")
            return
        }
        
        // Child can override parent handlers
        commonEvent() {
            print("Child-specific handling")
            => $^  // Forward to parent after child processing
        }
    }
    
    // Multiple inheritance levels supported
    $GrandChild => $Child {
        specializedEvent() {
            print("Most specific handling")
            return
        }
    }
```

#### Event Forwarding to Parent States

The `=> $^` statement forwards events from child states to their parent states. This is a **statement** (not terminator), allowing code execution to continue after the parent call:

```frame
$Child => $Parent {
    sharedEvent() {
        print("Child processing first")
        => $^  // Forward event to parent state
        print("This executes after parent unless parent transitions")
        return
    }
    
    complexEvent() {
        if validate_locally() {
            handle_locally()
            return  // Handle entirely in child
        }
        
        print("Forwarding to parent for complex handling")
        => $^  // Let parent handle complex case
        // Parent may transition, so this might not execute
        print("Parent completed processing")
        return
    }
}
```

**Parent Dispatch Behavior:**
- **Child Processing First**: Child state processes event before forwarding
- **Transition Detection**: If parent triggers transition, child code after `=> $^` doesn't execute
- **Multiple Forwards**: Multiple `=> $^` calls allowed in same handler
- **Statement Position**: Can appear anywhere in event handler (beginning, middle, end)
- **Validation**: Parser prevents usage in non-hierarchical states

#### Runtime Compartment Architecture

Frame v0.30 implements proper hierarchical compartment initialization to support parent dispatch:

**Generated Compartment Structure:**
```python
# Non-hierarchical state (normal)
self.__compartment = FrameCompartment('StateName', None, None, None, None)

# Hierarchical child state (enhanced)
self.__compartment = FrameCompartment('Child', None, None, None, 
    FrameCompartment('Parent', None, None, None, None))
```

**Key Implementation Details:**
- **Parent Reference**: Child compartments maintain reference to parent compartment
- **Infinite Recursion Fix**: Proper initialization prevents `parent_compartment=None` issues
- **Router Integration**: Parent dispatch uses existing `__router` infrastructure
- **Transition Safety**: Generated code checks for transitions after parent calls

#### HSM Syntax Restrictions

Frame v0.30 enforces clear separation between transitions and dispatch operations:

**✅ ALLOWED - Parent Dispatch (Statement):**
```frame
$Child => $Parent {
    event() {
        => $^  // Forward event to parent (statement)
        return
    }
}
```

**❌ BLOCKED - Transition to Parent:**
```frame
$Child => $Parent {
    event() {
        -> $^  // SYNTAX ERROR: Transitions to parent not allowed
        return
    }
}
```

**Error Message:** `"Syntax error: '-> $^' is not allowed. Use '=> $^' for parent dispatch instead."`

**Rationale:** 
- Transitions represent state changes with clear target states
- Parent dispatch represents event forwarding within state hierarchy
- Ambiguous `-> $^` syntax could confuse transition vs dispatch semantics
- Clear distinction improves code readability and maintainability

#### Complete HSM Example

```frame
system HSMExample {
    interface:
        start()
        commonEvent()
        childEvent()
        
    machine:
        // Top-level parent state
        $Parent {
            commonEvent() {
                print("Parent handling common event")
                return
            }
            
            fallbackEvent() {
                print("Parent fallback handler")  
                return
            }
        }
        
        // Child state with inheritance
        $Child => $Parent {
            $>() {
                print("Entering child state")
                return
            }
            
            start() {
                print("Starting from child")
                return
            }
            
            childEvent() {
                print("Child-specific event")
                return
            }
            
            // Override with forwarding
            commonEvent() {
                print("Child preprocessing")
                => $^  // Forward to parent
                print("Child postprocessing")
                return
            }
        }
        
        // Deeply nested hierarchy
        $GrandChild => $Child {
            specialEvent() {
                print("Most specific behavior")
                => $^  // Forward up the hierarchy
                return
            }
        }
}
```

**Generated Runtime Behavior:**
1. **Event Reception**: Events first processed by most specific state (e.g., $GrandChild)
2. **Local Processing**: Child state executes its handler logic
3. **Parent Forwarding**: `=> $^` forwards event to immediate parent
4. **Hierarchy Traversal**: Event continues up hierarchy until handled or root reached
5. **Transition Detection**: If any level triggers transition, execution terminates

### Event Handlers with Return Statements

Frame v0.20 supports return statements as regular statements within event handlers, enabling conventional conditional logic:

```frame
machine:
    $ProcessingState {
        validateInput(value: int): string {
            // Early return validation pattern
            if value < 0 {
                return "Invalid: negative"
            } elif value > 100 {
                return "Invalid: out of range"
            }
            
            // Complex conditional logic with returns
            if value >= 90 {
                return "Excellent"
            } elif value >= 70 {
                return "Good"
            } elif value >= 50 {
                return "Average"
            } else {
                return "Below Average"
            }
        }
        
        processRequest(type: string): int {
            // Return statements in nested conditions
            if type == "urgent" {
                if checkPermissions() {
                    return 1  // High priority
                } else {
                    return 2  // Medium priority
                }
            } elif type == "normal" {
                return 3  // Normal priority
            } else {
                return 4  // Low priority
            }
        }
    }
```

### Event Forwarding

1. **Transition forwarding**: Uses `-> =>` syntax to forward events during transitions
2. **Parent forwarding**: Uses `=> $^` to forward events to parent states in HSM
3. **Event dispatch**: Uses `=>` for general event forwarding

### Design Decisions

1. **Enter/Exit Syntax**: Uses `$>()` for enter and `<$()` for exit events
2. **Parameter Passing**: Both enter and exit handlers can accept parameters
3. **Terminator Optional**: Event handlers can optionally end with a terminator (`return`, `=>`, or `->`), or use statements like `=> $^`
4. **HSM Support**: Full hierarchical state machine support with `=>` operator
5. **Event Forwarding**: Multiple forwarding mechanisms for different use cases
6. **Block Terminators**: Transitions (`->`) are block terminators - no statements can follow them

## Examples

### Simple Action
```frame
actions:
    doSomething() {
        if x doY()
    }
```

### Action with Parameters and Return Type
```frame
actions:
    calculate(x: int, y: int): int {
        if x > y {
            return x + y
        } else {
            return x - y
        }
    }
```

### If Statement Examples

#### Simple If
```frame
if temperature > 100 {
    shutDown()
}
```

#### Single Statement (no braces)
```frame
if x doY()
```

#### If-Elif-Else
```frame
if score >= 90 {
    grade = "A"
} elif score >= 80 {
    grade = "B"
} elif score >= 70 {
    grade = "C"
} else {
    grade = "F"
}
```

#### Mixed braces and single statements
```frame
if condition1: doFirst()
elif condition2 {
    doSecond()
    doThird()
} else: doDefault()
```

## Loop Statements

### Loop Grammar
```bnf
// For loops (v0.51: added else clause support)
for_stmt: 'for' (var_decl | identifier) 'in' expr ':' stmt else_clause?
        | 'for' (var_decl | identifier) 'in' expr block else_clause?
        | 'for' var_decl ';' expr ';' expr block else_clause?  // C-style for loop

// While loops (v0.51: added else clause support)
while_stmt: 'while' expr ':' stmt else_clause?
          | 'while' expr block else_clause?

// Loop else clause (v0.51)
else_clause: 'else' ':' stmt
           | 'else' block

// Legacy loop syntax (maintained for backward compatibility)
loop_stmt: 'loop' '{' stmt* '}'
         | 'loop' var_decl ';' expr ';' expr '{' stmt* '}'
         | 'loop' (var_decl | identifier) 'in' expr '{' stmt* '}'
```

### Design Decisions

1. **Python-style keywords**: Use `for` and `while` instead of generic `loop`
2. **Consistent syntax with conditionals**: Support both `:` for single statements and `{}` for blocks
3. **For-in loops**: Primary iteration pattern following Python
4. **C-style for loops**: Support traditional three-part loops with both `for` and `loop` keywords
5. **While loops**: Condition-based loops with clear syntax
6. **Backward compatibility**: Original `loop` syntax still supported

### Loop Examples

#### For-in loops
```frame
// Python-style with colon
for item in items:
    process(item)

// Braced blocks
for item in items {
    process(item)
    doMore()
}

// With variable declaration
for var item in items:
    process(item)
```

#### C-style for loops
```frame
// Traditional index-based iteration using 'for' keyword
for var i = 0; i < len(list); i = i + 1 {
    print("Item " + str(i) + ": " + str(list[i]))
}

// Countdown loop
for var i = 10; i > 0; i = i - 1 {
    print("Countdown: " + str(i))
}

// Note: The 'loop' keyword also supports C-style syntax for backward compatibility
loop var j = 0; j < 5; j = j + 1 {
    print("Loop iteration: " + str(j))
}
```

#### While loops
```frame
// Python-style with colon
while x < 10:
    x = x + 1

// Braced blocks
while x < 10 {
    x = x + 1
    doSomething()
}

// Infinite loop
while true:
    doWork()
    if done: break
```

#### Range-based iteration (future)
```frame
// Simple range (0 to 9)
for i in range(10):
    print(i)

// Range with start and stop
for i in range(5, 10):
    print(i)
```

#### Loop Else Clauses (v0.51)

Frame v0.51 adds support for Python's loop else clause feature. The else block executes when a loop completes normally (without encountering a break statement). This is particularly useful for search patterns and completion detection.

```frame
// For-else: Search pattern
for item in items {
    if item == target {
        print("Found: " + item)
        break
    }
}
else {
    print("Item not found")  // Executes only if break was NOT hit
}

// While-else: Process until condition
var attempts = 0
while attempts < max_attempts {
    if try_operation() {
        print("Success!")
        break
    }
    attempts = attempts + 1
}
else {
    print("Max attempts reached")  // Executes if loop completed without break
}

// Nested loops with else
for i in range(3) {
    for j in range(3) {
        if matrix[i][j] == target {
            print("Found at " + str(i) + "," + str(j))
            break
        }
    }
    else {
        print("Row " + str(i) + " complete")  // Inner loop completed normally
    }
}
else {
    print("Matrix search complete")  // Outer loop completed normally
}

// Continue doesn't affect else
for i in range(10) {
    if i % 2 == 0 {
        continue  // Skip even numbers
    }
    process(i)
}
else {
    print("All odd numbers processed")  // Still executes (no break)
}
```

**Important Notes:**
- The else block executes ONLY when the loop terminates normally (condition becomes false)
- The else block does NOT execute if the loop is exited via `break`
- The else block DOES execute if the loop uses `continue` (as continue doesn't exit the loop)
- The else block also executes if the loop condition is initially false (zero iterations)

#### Multiple Assignment and Tuple Unpacking (v0.52)

Frame v0.52 introduces Python-style multiple assignment and tuple unpacking, enabling more elegant variable assignments and value swapping operations.

```frame
# Basic multiple assignment
x, y = 10, 20                    # Assigns 10 to x, 20 to y

# Variable swapping
a, b = b, a                       # Swaps values without temp variable

# Tuple unpacking
var t = (100, 200, 300)
p, q, r = t                       # Unpacks tuple elements

# Function return unpacking
fn get_coordinates() {
    return (42, 73)
}
lat, lon = get_coordinates()     # Unpacks multiple return values

# List unpacking
var lst = [1]
lst.append(2)
lst.append(3)
x1, y1, z1 = lst                 # Unpacks list elements

# Complex expressions
n1, n2, n3 = n1 + 1, n2 * 2, n3 ** 2  # Multiple expressions on RHS
```

**Grammar Changes:**
- Assignment now supports comma-separated left-hand values: `lvalue_list`
- Right-hand side automatically forms tuple when comma-separated: `rvalue_list`
- Parser distinguishes between tuple literals and multiple assignment based on context
- Only simple `=` operator supported for multiple assignment (not compound operators)

**Important Notes:**
- Multiple variable declarations (`var x, y, z = 1, 2, 3`) now supported as of v0.53
- ~~List literals with commas create nested tuples: `[1, 2, 3]` becomes `[(1, 2, 3)]`~~ **FIXED in v0.53**
- Assignment targets must be valid lvalues (identifiers, indexed expressions, etc.)
- Right-hand values automatically wrapped in tuple when multiple values present (except in collection literals as of v0.53)

### Syntax Enforcement

Similar to if/elif/else statements:
- After `:` only single statements are allowed (no `{` blocks)
- After condition/iterable without `:`, braces `{` are required

## Statements

```bnf
stmt: expr_stmt
    | var_decl
    | assignment
    | assert_stmt
    | del_stmt
    | if_stmt
    | match_stmt
    | for_stmt
    | while_stmt
    | loop_stmt
    | try_stmt
    | raise_stmt
    | return_stmt
    | return_assign_stmt
    | system_return_stmt
    | parent_dispatch_stmt
    | transition_stmt
    | state_stack_op
    | block_stmt
    | break_stmt
    | continue_stmt

expr_stmt: expr
var_decl: 'var' unpacking_pattern type? '=' expr  // v0.54: Star expressions for unpacking
unpacking_pattern: (IDENTIFIER | '*' IDENTIFIER) (',' (IDENTIFIER | '*' IDENTIFIER))*  // v0.54: Star unpacking
assert_stmt: 'assert' expr (',' expr)?
del_stmt: 'del' expr  // v0.50: Delete statement
assignment: lvalue_list assignment_op rvalue_list  // v0.52: Multiple assignment
lvalue_list: lvalue (',' lvalue)*  // v0.52: Comma-separated targets
rvalue_list: expr (',' expr)*      // v0.52: Comma-separated values
lvalue: IDENTIFIER | call_chain | index_expr
assignment_op: '=' | '+=' | '-=' | '*=' | '/=' | '//=' | '%=' | '**=' | '@='  // v0.39-40
             | '&=' | '|=' | '^=' | '<<=' | '>>='  // v0.39-40: Bitwise compound
try_stmt: 'try' block except_clause+ else_clause? finally_clause?
except_clause: 'except' exception_spec? ('as' IDENTIFIER)? block
exception_spec: IDENTIFIER | '(' IDENTIFIER (',' IDENTIFIER)* ')'
else_clause: 'else' block
finally_clause: 'finally' block
raise_stmt: 'raise' expr? ('from' expr)?
return_stmt: 'return' expr?  // v0.38: Fixed to parse lambda expressions
return_assign_stmt: 'return' '=' expr
system_return_stmt: '@@:return' '=' expr
parent_dispatch_stmt: '=>' '$^'
transition_stmt: '->' '$' IDENTIFIER
state_stack_op: '$$[' '+' ']' | '$$[' '-' ']'
block_stmt: '{' stmt* '}'
break_stmt: 'break'
continue_stmt: 'continue'
```

### Pattern Matching (v0.44)

Frame v0.44 introduces comprehensive pattern matching with match-case statements:

```bnf
match_stmt: 'match' expr '{' case_clause+ '}'
case_clause: 'case' pattern guard_clause? '{' stmt* '}'
guard_clause: 'if' expr

pattern: or_pattern ('as' IDENTIFIER)?
or_pattern: pattern_atom ('or' pattern_atom)*
pattern_atom: literal_pattern
            | capture_pattern
            | wildcard_pattern
            | sequence_pattern
            | mapping_pattern
            | class_pattern
            | '(' pattern ')'

literal_pattern: NUMBER | STRING | 'true' | 'false' | 'None'
capture_pattern: IDENTIFIER
wildcard_pattern: '_'
sequence_pattern: '[' pattern_list? ']' | '(' pattern_list? ')'
pattern_list: pattern_element (',' pattern_element)*
pattern_element: pattern | star_pattern
star_pattern: '*' IDENTIFIER
mapping_pattern: '{' mapping_item_list? '}'
mapping_item_list: mapping_item (',' mapping_item)*
mapping_item: (STRING | IDENTIFIER) ':' pattern
class_pattern: '(' STRING ',' pattern_list? ')'  // Tuple workaround for classes
```

**Pattern Types:**
- **Literal Patterns**: Match specific values (numbers, strings, booleans, None)
- **Capture Patterns**: Bind matched values to variables
- **Wildcard Pattern**: Match anything without binding (`_`)
- **Sequence Patterns**: Match lists and tuples with optional star patterns
- **Mapping Patterns**: Match dictionary structures
- **OR Patterns**: Multiple alternatives using `or` keyword (not `|` due to Frame pipe operator)
- **AS Patterns**: Bind entire matched pattern to a variable
- **Star Patterns**: Capture remaining sequence elements (`*rest`)
- **Guard Clauses**: Add conditions to patterns with `if`

**Examples:**
```frame
# Literal and capture patterns
match value {
    case 42 {
        return "the answer"
    }
    case x {
        return "captured: " + str(x)
    }
}

# OR patterns (using 'or' keyword)
match status {
    case 200 or 201 or 204 {
        return "success"
    }
    case 400 or 404 or 403 {
        return "client error"
    }
}

# Star patterns for unpacking
match lst {
    case [first, *rest] {
        return "first: " + str(first) + ", rest: " + str(rest)
    }
    case [first, *middle, last] {
        return "edges with middle"
    }
}

# AS patterns for binding
match data {
    case [x, y] as point {
        return "point: " + str(point)
    }
    case (1 or 2 or 3) as num {
        return "small number: " + str(num)
    }
}

# Guard clauses
match score {
    case x if x >= 90 {
        return "A"
    }
    case x if x >= 80 {
        return "B"
    }
}

# Nested patterns
match response {
    case {"status": 200 or 201, "data": [first, *rest]} {
        return "success with data"
    }
    case {"error": {"code": code, "message": msg}} {
        return "error " + str(code) + ": " + msg
    }
}
```

### Exception Handling

Frame v0.41 (previously documented in v0.32 but now fully tested) supports Python-style exception handling with try-except-else-finally blocks and raise statements:

```frame
// Basic try-except
try {
    risky_operation()
}
except {
    print("Error occurred")
}

// Specific exception types with variable binding
try {
    file_operation()
}
except IOError as e {
    print("IO Error: " + str(e))
}
except (ValueError, TypeError) as err {
    print("Value or Type error: " + str(err))
}

// Else clause - runs if no exception
try {
    validate_input()
}
except ValidationError {
    print("Validation failed")
}
else {
    print("Validation succeeded")
    process_input()
}

// Finally clause - always runs
try {
    allocate_resource()
}
except ResourceError as e {
    handle_error(e)
}
finally {
    cleanup()
}

// Raise exceptions
raise ValueError("Invalid input")

// Re-raise current exception
try {
    operation()
}
except {
    log_error()
    raise  // Re-raise the caught exception
}

// Exception chaining (from clause)
try {
    parse_config()
}
except ParseError as e {
    raise ConfigError("Invalid config") from e
}
```

The exception handling maps to target language idioms:
- **Python**: Direct 1:1 mapping
- **JavaScript**: Uses instanceof checks and success flags for else clause
- **Java/C#**: Multi-catch and success flags
- **Go**: defer/recover pattern or error returns
- **Rust**: Result<T, E> pattern

### Delete Statement (v0.50)

Frame v0.50 introduces the `del` statement for removing variables, list elements, dictionary entries, and object attributes:

```frame
// Delete variables
var x = 42
del x  # x is no longer defined

// Delete list elements
var mylist = [1, 2, 3, 4, 5]
del mylist[2]      # Remove element at index 2
del mylist[-1]     # Remove last element
del mylist[1:3]    # Delete slice

// Delete dictionary entries
var mydict = {"a": 1, "b": 2, "c": 3}
del mydict["b"]    # Remove key "b"

// Delete with variable key
var key = "a"
del mydict[key]

// Delete from nested structures
var data = {
    "users": [
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25}
    ]
}
del data["users"][0]["age"]  # Remove age from first user
del data["users"][1]         # Remove second user entirely

// Delete slices with step
var nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
del nums[::2]      # Delete every other element
del nums[2:7:2]    # Delete elements at indices 2, 4, 6
```

**Note**: After deleting a variable with `del`, attempting to access it will result in a runtime error (NameError in Python). The Frame parser currently does not allow redeclaration of a deleted variable within the same scope.

### Parent Dispatch Statement

Frame v0.20 introduces the `=> $^` statement for forwarding events from child states to their parent states in hierarchical state machines. Unlike the deprecated `@:>` terminator, this is a regular statement that allows code to continue after the parent call (unless the parent triggers a transition):

```frame
machine:
    $Child => $Parent {
        testEvent() {
            print("Child processing first")
            => $^  // Forward to parent state
            print("This executes after parent unless parent transitions")
        }
    }
```

**Key Features:**
- **Statement syntax**: Can appear anywhere in event handler, not just at the end
- **Transition detection**: Code after `=> $^` doesn't execute if parent triggers a transition
- **Validation**: Parser prevents usage in non-hierarchical states
- **Flexibility**: Multiple `=> $^` calls allowed in same handler

### State Stack Operations

Frame v0.20 provides comprehensive state stack operations for implementing history mechanisms and state preservation. These operations are **fully validated** and working correctly:

```frame
// State stack push - saves current state
gotoModal() {
    push$          // Push current state onto stack
    -> $ModalState // Transition to new state
    return
}

// State stack pop - returns to saved state
closeModal() {
    -> pop$        // Pop and transition to previous state
    return
}
```

**State Stack Operators:**
- **`push$`** - Push current state compartment onto stack (preserves variables)
- **`pop$`** - Pop state compartment from stack and use as transition target

**Key Features:**
- **State Preservation**: Variables maintain their values when using stack operations
- **Generic Return**: No need to hardcode which state to return to
- **Compartment Management**: Works with Frame's state compartment system
- **Flexible Usage**: Can be combined with transitions and other statements

**Validation Status**: ✅ **100% Working** - All state stack tests pass including:
- Basic push/pop operations
- Multiple nested push/pop sequences  
- State variable preservation
- Enter/exit event handling with stack operations
- Complex state context preservation

### System Return Assignment (v0.31)

Frame v0.31 introduces the `@@:return` special variable for setting interface return values anywhere within event handlers or action methods.

**IMPORTANT**: Frame supports a special use of the `system` keyword:
1. **`@@:return`**: For setting interface return values within event handlers, actions, and non-static operations
2. **`system.interfaceMethod()`**: For calling interface methods within the system (v0.81.2+)

The general `system.method()` syntax is only supported for interface methods. For other method types, use `self.method()`.

```frame
// Setting interface return values with @@:return
interface:
    validateInput(data: string): bool = false  // Default return value

machine:
    $ProcessingState {
        validateInput(data: string) {  // Can override with : bool = true
            if data == "" {
                @@:return = false  // Set interface return value
                return                 // Exit event handler
            }

            if checkFormat(data) {
                @@:return = true   // Set interface return value
                return                 // Exit event handler
            }

            @@:return = false      // Default case
            return
        }
    }

// Event handler default overrides interface default
machine:
    $Start {
        getStatus(): int = 99 {  // Override interface default
            // Implicit @@:return = 99 on entry
            if someCondition {
                @@:return = 200  // Further override
            }
            return
        }
    }

// Actions can also set @@:return
actions:
    processData(input: string): string {
        if input == "error" {
            @@:return = "failed"   // Set interface return value
            return "internal"   // Return value to caller (action method)
        }

        @@:return = "success"      // Set interface return value
        return input            // Return value to caller (action method)
    }
```

### System Interface Method Calls (v0.81.2)

Frame v0.81.2 introduces the ability to call interface methods from within the system using the `system.interfaceMethod()` syntax. This provides a clear distinction between interface method calls and other method types.

**Syntax**: `system.methodName()` or `system.methodName(arguments)`

**Valid Contexts**:
- Event handlers (within machine states)
- Action methods
- Non-static operations (when operations block appears after interface block)

**Grammar**:
```bnf
system_interface_call: 'system' '.' IDENTIFIER '(' expression_list? ')'
system_interface_ref: 'system' '.' IDENTIFIER  // Method reference without call
```

**Examples**:

```frame
system Calculator {
    interface:
        add(a: int, b: int): int
        getValue(): int
        reset()
        
    machine:
        $Ready {
            // Event handlers can call interface methods
            calculateSum() {
                var result = system.add(5, 3)      // Call with arguments
                var current = system.getValue()    // Call without arguments
                system.reset()                     // Void call
                @@:return = result + current
            }

            add(a: int, b: int): int {
                var sum = a + b
                @@:return = sum
            }

            getValue(): int {
                @@:return = self.stored_value
            }
            
            reset() {
                self.stored_value = 0
            }
        }
    
    actions:
        // Actions can call interface methods
        logCurrentValue() {
            var current = system.getValue()  // Read current value
            print("Current value: " + str(current))
        }
        
        processData(input: int) {
            var doubled = system.add(input, input)  // Use interface method
            print("Doubled: " + str(doubled))
        }
    
    operations:
        // Non-static operations can call interface methods
        formatValue(): str {
            var val = system.getValue()
            return "Value: " + str(val)
        }
    
    domain:
        var stored_value = 0
}
```

**Important Notes**:

1. **Interface Method Validation**: The transpiler validates that the called method exists in the system's interface block. Using `system.nonExistentMethod()` will produce a compile-time error.

2. **Alternative to `self.interfaceMethod`**: Using `self.interfaceMethod()` for interface methods is invalid and produces a helpful error message suggesting the use of `system.interfaceMethod()` instead.

3. **Block Ordering**: Due to parser implementation, operations blocks that use `system.interfaceMethod()` should appear after the interface block in the system definition for proper validation.

4. **Generated Code**: `system.interfaceMethod()` calls are transpiled to `self.interfaceMethod()` in the generated Python code, maintaining the proper object-oriented semantics.

5. **Return Values**: Interface method calls can be used in expressions, assignments, and as standalone statements, just like regular method calls.

**Error Examples**:

```frame
system BadExample {
    interface:
        getValue(): int
        
    machine:
        $Start {
            test() {
                // ❌ This will produce an error
                var val = self.getValue()
                // Error: Interface method 'getValue' should be called using 
                // 'system.getValue' instead of 'self.getValue'
                
                // ✅ Correct usage
                var val = system.getValue()
            }
        }
}
```

## Expressions

```bnf
expr: assignment_expr | binary_expr | unary_expr | primary_expr | call_expr | self_expr | fsl_expr | lambda_expr

assignment_expr: IDENTIFIER ':=' expr  // v0.56: Walrus operator (assignment expression)

binary_expr: expr operator expr
operator: '+' | '-' | '*' | '/' | '//' | '%' | '**' | '@'  // v0.40: Added floor division and matrix multiplication
        | '==' | '!=' | '<' | '>' | '<=' | '>='
        | '&' | '|' | '^' | '<<' | '>>'  // v0.39-40: Bitwise operators
        | 'and' | 'or'  // v0.38: Python logical operators (xor removed in v0.76)
        | 'is' | 'is not' | 'in' | 'not in'  // v0.39: Identity/membership

unary_expr: ('-' | 'not' | '~') expr  // v0.38-39: Python unary operators

primary_expr: IDENTIFIER | NUMBER | string_literal | literal_method_call | SUPERSTRING
            | 'true' | 'false' | 'None'
            | '(' expr ')' | '@'
            | list_literal | dict_literal | set_literal | tuple_literal
            | function_ref  // v0.38: Function reference

// v0.40: Comprehensive string literal support
string_literal: STRING | FSTRING | RAWSTRING | BYTESTRING | TRIPLE_QUOTED

// v0.41: String literal method calls
literal_method_call: string_literal '.' IDENTIFIER '(' arg_list? ')'

function_ref: IDENTIFIER  // Function name without parentheses

// v0.38: Lambda Expressions - can be used in return statements
lambda_expr: 'lambda' lambda_params? ':' expr
lambda_params: IDENTIFIER (',' IDENTIFIER)*

self_expr: 'self' | 'self' '.' IDENTIFIER  // v0.31: self as standalone or dotted access

call_expr: IDENTIFIER '(' arg_list? ')' | '_' IDENTIFIER '(' arg_list? ')'
arg_list: expr (',' expr)*

fsl_expr: fsl_conversion | fsl_property | fsl_method  // v0.33: Native Python Operations
fsl_conversion: ('str' | 'int' | 'float' | 'bool') '(' expr ')'
fsl_property: expr '.' ('length' | 'size' | 'capacity' | 'name' | 'value')
fsl_method: expr '.' ('append' | 'pop' | 'clear' | 'remove') '(' arg_list? ')'

// v0.38: Collection Literals
list_literal: '[' expr_list? ']'
dict_literal: '{' dict_pair_list? '}'
set_literal: '{' expr_list '}'  // Disambiguated from dict by lack of colons
tuple_literal: '(' expr_list? ')'  // Disambiguated by context and trailing comma

dict_pair_list: dict_pair (',' dict_pair)* ','?
dict_pair: expr ':' expr
expr_list: expr (',' expr)* ','?

// v0.53: Collection Literal Parsing Fix
// Collection literals now correctly parse comma-separated elements without
// wrapping them in tuples. The parser uses context-aware parsing to distinguish
// between comma-separated collection elements and tuple expressions.

/* Collection Literal Examples (v0.53 behavior):
   
   Lists:
   var lst = [1, 2, 3]              // → lst = [1, 2, 3] ✅
   var nested = [[1, 2], [3, 4]]    // → nested = [[1, 2], [3, 4]] ✅
   
   Dictionaries:
   var dict = {"a": 1, "b": 2}      // → dict = {"a": 1, "b": 2} ✅
   
   Sets:
   var set = {1, 2, 3}               // → set = {1, 2, 3} ✅
   
   Tuples:
   var tup = (1, 2, 3)               // → tup = (1, 2, 3) ✅
   var single = (42,)                // → single = (42,) ✅
   
   Mixed Collections:
   var data = {"list": [1, 2, 3]}   // → data = {"list": [1, 2, 3]} ✅
*/

// v0.38: Collection Constructors (transformed to literals)
collection_constructor: list_constructor | set_constructor | tuple_constructor | dict_constructor
list_constructor: 'list' '(' arg_list? ')'     // → [args...]
set_constructor: 'set' '(' arg_list? ')'       // → {args...}
tuple_constructor: 'tuple' '(' arg_list? ')'   // → (args...)
dict_constructor: 'dict' '(' ')'               // → dict() (Python-compliant)
```

**Action Call Syntax (v0.66)**: Action calls MUST use explicit `self.actionName()` syntax. The underscore prefix is added automatically during Python code generation. Interface methods called from within the system also require `self.methodName()` prefix.

**Operation Call Syntax (v0.66)**: Operation calls from within the system MUST use explicit `self.operationName()` syntax.

**Self Expression**: The `self` keyword is required for all action, operation, and interface method calls within a system. Static methods marked with `@staticmethod` cannot use `self` in any form.

### Assignment Expressions / Walrus Operator (v0.56)

Frame v0.56 introduces the walrus operator `:=` for assignment expressions, following Python 3.8+ syntax:

```frame
# Walrus operator in if statement
if (n := len(data)) > 10 {
    print("Large dataset with " + str(n) + " items")
}

# In while loops
while (line := readline()) != "" {
    process(line)
}

# In list comprehensions
var results = [y for x in data if (y := transform(x)) > 0]

# Reusing computed values
fn processData(items) {
    if (total := sum(items)) > 100 {
        return total / len(items)  # Can reuse 'total'
    }
    return 0
}

# Variable creation and testing
if (match := search(pattern, text)) {
    print("Found: " + match)
}
```

**Key Features:**
- Creates a variable and returns its value in a single expression
- Variable is created in the current scope (not a new scope)
- Useful for avoiding repeated calculations
- Improves code readability by reducing duplication

### Lambda Expressions (v0.38)

Frame v0.38 supports full Python lambda syntax for creating anonymous functions:

```frame
// Basic lambda
var square = lambda x: x * x
var result = square(5)  // 25

// Multi-parameter lambda
var add = lambda a, b: a + b
var sum = add(3, 4)  // 7

// No-parameter lambda
var get_pi = lambda: 3.14159
var pi = get_pi()  // 3.14159

// Lambda in collections
var ops = {
    "add": lambda a, b: a + b,
    "mul": lambda a, b: a * b,
    "square": lambda x: x * x
}

// Using lambdas
var result = ops["add"](5, 3)  // 8
```

### First-Class Functions (v0.38)

Frame v0.38 supports both named functions and lambdas as first-class values:

```frame
// Functions as values
fn add(a, b) { return a + b }
fn multiply(a, b) { return a * b }

// Assign function to variable
var my_func = add
var result = my_func(3, 4)  // 7

// Pass function as parameter
fn apply_op(op, x, y) {
    return op(x, y)
}
result = apply_op(multiply, 5, 3)  // 15

// Return function from function
fn get_operation(name) {
    if name == "add" {
        return add
    } else {
        return multiply
    }
}

// Store functions in collections
var operations = [add, multiply]
var ops_dict = {"plus": add, "times": multiply}
```

**Function References**: When a function name appears without parentheses, it's treated as a function reference (first-class value) that can be assigned, passed, or returned.

### Collection Syntax (v0.38)

Frame v0.38 introduces comprehensive collection syntax support with both literal and constructor forms for all Python collection types, including the new empty set literal syntax.

#### Collection Literals
```frame
// List literals
var numbers = [1, 2, 3, 4, 5]
var empty_list = []
var nested = [[1, 2], [3, 4]]

// Dictionary literals  
var person = {"name": "Alice", "age": 30}
var empty_dict = {}
var nested_dict = {"user": {"id": 1, "active": true}}

// Dictionary indexing (fully working in v0.38)
var name = person["name"]  // "Alice"
person["age"] = 31  // Assignment works
var user_id = nested_dict["user"]["id"]  // Nested access works

// Set literals
var unique_numbers = {1, 2, 3}
var single_set = {42}
var empty_set = {,}  // v0.38: New empty set literal syntax

// Tuple literals
var coordinates = (10, 20)
var single_tuple = (42,)  // Trailing comma required
var empty_tuple = ()
```

#### Collection Constructors
Collection constructors are automatically transformed into their literal equivalents:

```frame
// List constructor → literal
var l = list(1, 2, 3)      // → [1, 2, 3]

// Set constructor → literal  
var s = set(1, 2, 3)       // → {1, 2, 3}

// Tuple constructor → literal
var t = tuple(10, 20, 30)  // → (10, 20, 30)

// Dict constructor (Python-compliant)
var d = dict()             // → dict() (kept as-is)
```

#### Disambiguation Rules
- **`{}` syntax**: Empty braces default to dictionary for Python compatibility
- **`{,}` syntax**: v0.38 explicit empty set literal (generates `set()` in Python)
- **Dict vs Set**: Presence of colons (`:`) indicates dictionary, otherwise set
- **Tuple vs Parentheses**: Multiple elements or trailing comma indicates tuple
- **Single-element tuples**: Automatically get trailing comma in generated Python

### Logical Operators (v0.38)

**Breaking Change**: C-style logical operators have been completely removed in favor of Python-style keywords.

| Old Syntax (Removed) | New Syntax (Required) | Description |
|---------------------|----------------------|-------------|
| `&&` | `and` | Logical AND |
| `\|\|` | `or` | Logical OR |
| `!` | `not` | Logical NOT |

Examples:
```frame
// v0.38: Python logical operators
if x > 0 and y > 0 {
    print("Both positive")
}

if a or b {
    print("At least one true")
}

if not condition {
    print("Condition is false")
}

// Complex expressions
if (a and b) or (not c and d) {
    print("Complex logic")
}
```

**Call Chain Support**: Multi-node call chains like `sys.methodName()` correctly generate interface method calls on system instances without adding action prefixes.

**Native Python Operations (v0.33)**: Frame supports Python built-in operations directly, eliminating the need for backticks when using common operations like type conversions and collection methods.

## Native Python Functions (v0.38)

Frame v0.38 supports native Python built-in functions **without requiring imports**:

```frame
// Type conversions work natively
var x = 42
var s = str(x)        // "42" - no import needed
var i = int("123")    // 123
var f = float("3.14") // 3.14
var b = bool(0)       // False

// Built-in functions work directly
var items = [1, 2, 3]
var count = len(items)  // 3
print("Hello")          // Direct output

// String methods on string objects
var text = "hello"
var upper = text.upper()  // "HELLO"
var lower = text.lower()  // "hello"

// List methods on list objects
var list = [1, 2, 3]
list.append(4)        // Adds to end
var last = list.pop() // Removes and returns last

// Dictionary methods on dict objects
var dict = {"a": 1}
var val = dict.get("a", 0)  // 1
dict.setdefault("b", [])    // Creates if not exists
```

## Legacy Python Import Syntax - v0.34 (DEPRECATED)

**Note**: As of v0.38, Python functions work directly in Frame without special imports. This legacy import syntax is retained for backward compatibility but is no longer required.

### Legacy Import Requirements (v0.34)

Previously, operations required explicit imports:

```frame
// Legacy import syntax (no longer needed)
from fsl import str, int, float

// Legacy import all (no longer needed)
from fsl import *

// Modern Frame: str/int/float work directly without imports
fn noImport() {
    var s = str(42)  // Calls external str() if available
}
```

### Type Conversion Operations
```frame
from fsl import str, int, float  // Required import

fn example() {
    var x = 42
    var s = str(x)      // Convert to string: "42"
    var i = int("123")  // Convert to integer: 123
    var f = float("3.14") // Convert to float: 3.14
}
```

### Collection Properties (Planned)
```frame
fn listExample() {
    var items = [1, 2, 3]
    var count = items.length  // Get list length
    var size = items.size     // Alternative syntax
}
```

### Collection Methods (Planned)
```frame
fn listOperations() {
    var items = []
    items.append(42)      // Add to end
    var last = items.pop() // Remove and return last
    items.clear()         // Remove all elements
    items.remove(42)      // Remove specific value
}
```

### Enum Properties (v0.32)
```frame
enum Status { Active, Inactive }

fn enumExample() {
    var s = Status.Active
    var name = s.name   // "Active"
    var value = s.value // 0
}
```

**Implementation Status:**
- ✅ Type conversions (str, int, float) - Phase 1 Complete
- 🚧 Boolean conversion - In progress
- 📋 Collection properties - Planned for Phase 2
- 📋 Collection methods - Planned for Phase 2

## Known Limitations (v0.38)

While Frame v0.38 has extensive Python compatibility, the following limitations exist:

### Parser Limitations
1. **Domain Variable Dictionary Initialization**: Cannot initialize domain variables with dictionary literals
   ```frame
   // This causes a parse error:
   domain:
       var settings = {"key": "value"}  // ❌ Not supported
   
   // Workaround - initialize in state enter handler:
   domain:
       var settings
   
   machine:
       $Ready {
           $>() {
               settings = {"key": "value"}  // ✅ Works here
           }
       }
   ```

### Language Features Fully Implemented in v0.38
1. **First-Class Functions**: ✅ Functions can be passed as parameters, assigned to variables, and returned
2. **Lambda Expressions**: ✅ Full Python lambda syntax with closures
3. **Exponent Operator**: ✅ Right-associative `**` operator for powers
4. **Empty Set Literal**: ✅ `{,}` syntax for empty sets (generates `set()` in Python)

### Test Suite Status (v0.38 with UTF-8 Fix)
- **Total Tests**: 290
- **Passing**: 285 (98.3%)
- **Failing**: 5 (1.7%)
- UTF-8 scanner fix resolved Unicode character handling issues
- Remaining failures are parse errors in test files or unimplemented features

## Lexical Structure

### Character Encoding

Frame v0.38 fully supports UTF-8 encoded source files:

- **Full Unicode Support**: All Unicode characters are properly handled in strings and comments
- **UTF-8 Source Files**: Source files can contain any valid UTF-8 characters
- **Character-based Scanning**: Scanner uses character indices, not byte indices
- **Multi-byte Characters**: Properly handles emoji, accented characters, and all Unicode symbols

### Comments (v0.40)

```bnf
LINE_COMMENT: '#' ~[\n]* \n  // v0.40: Python-style single-line comments
MULTI_LINE_COMMENT: '{--' ~* '--}'  // Frame documentation comments
```

**Breaking Change in v0.40**: C-style comments (`//`, `/* */`) have been removed. Use Python-style `#` for single-line comments.

Comments can contain any UTF-8 characters:
```frame
# This is a comment with Unicode: ✓ ✗ 你好 🎉
{-- Multi-line documentation comment
    with Unicode symbols: ○ ● ★ ☆
    and emojis: 😀 🚀 💻 --}
```

### Numeric Literals (v0.56)

Frame v0.56 enhances numeric literal support with underscores for readability, scientific notation, and complex numbers:

```frame
# Underscores for readability
var million = 1_000_000
var binary_mask = 0b1111_0000_1111_0000
var hex_color = 0xFF_FF_FF
var precise = 3.141_592_653_589_793

# Scientific notation
var avogadro = 6.022e23
var planck = 6.626e-34
var electron_mass = 9.109_383_56e-31

# Complex numbers
var z1 = 3.14j
var z2 = 2.5 + 3.7j
var z3 = 1e10j

# All bases support underscores
var binary = 0b1010_1010
var octal = 0o755_644
var hex = 0xDEAD_BEEF
```

**Key Features:**
- **Underscores**: Use `_` as digit separator for improved readability
- **Scientific Notation**: Support for `e` or `E` exponents with optional sign
- **Complex Numbers**: Add `j` or `J` suffix for imaginary numbers
- **All Bases**: Binary, octal, and hexadecimal literals all support underscores

### Tokens

```bnf
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*
NUMBER: [0-9]([0-9_]*)* ('.' [0-9]([0-9_]*)*)?  // Decimal with underscores (v0.56)
      | [0-9]([0-9_]*)* ('.' [0-9]([0-9_]*)*)? ('e'|'E') [+-]? [0-9]+  // Scientific notation (v0.56)
      | [0-9]([0-9_]*)* ('.' [0-9]([0-9_]*)*)? ('j'|'J')  // Complex number (v0.56)
      | '0b' [01]([01_]*)*       // Binary literal with underscores (v0.40/v0.56)
      | '0o' [0-7]([0-7_]*)*     // Octal literal with underscores (v0.40/v0.56)
      | '0x' [0-9a-fA-F]([0-9a-fA-F_]*)*  // Hexadecimal with underscores (v0.40/v0.56)
STRING: '"' (ESC | ~["])* '"'  // Regular string
FSTRING: 'f"' (ESC | EXPR | ~["])* '"'  // f-string with expressions (v0.40)
RAWSTRING: 'r"' (~["])* '"'  // Raw string, no escape processing (v0.40)
BYTESTRING: 'b"' (ESC | ~["])* '"'  // Byte string literal (v0.40)
TRIPLE_QUOTED: '"""' ~* '"""'  // Multi-line string (v0.40)
              | [frb]'"""' ~* '"""'  // Prefixed triple-quoted (v0.40)
SUPERSTRING: '`' ~[`]* '`' | '```' ~* '```'  // Deprecated, use raw strings
```

**String Literals (v0.40)**: Comprehensive Python-style string support:

```frame
// Regular strings - UTF-8 support
var greeting = "Hello, 世界! 🌍"
var message = "Unicode works: ✓ ✗ ★ ☆"

// F-strings - formatted string literals
var name = "Frame"
var version = 0.40
var msg = f"Hello {name} v{version}!"

// Raw strings - no escape processing
var path = r"C:\Users\Frame\Documents"
var pattern = r"\d{3}-\d{4}"

// Byte strings - binary data
var data = b"Binary data"

// Triple-quoted strings - multi-line
var text = """This is a
multi-line string
with preserved formatting"""

// Prefixed triple-quoted
var raw_multi = r"""Raw multi-line
with \n literal"""

// Percent formatting
var formatted = "Name: %s, Version: %.2f" % (name, version)

// v0.41: String literal method calls - call methods directly on string literals
var upper1 = "hello".upper()                    // "HELLO"  
var lower1 = "WORLD".lower()                    // "world"
var stripped = "  spaces  ".strip()            // "spaces"
var upper2 = f"{name}".upper()                  // F-string literal method call
var lower2 = r"FRAME".lower()                   // Raw string literal method call  
var multi_strip = """  multiline  """.strip()  // Triple-quoted literal method call
```

## Keywords

```
system interface machine actions operations domain
fn var return
if elif else for while loop in break continue
true false None
```

**Note on `system` keyword**:
- Reserved keyword that cannot be used as a variable or identifier name
- Used for system declarations: `system MySystem { ... }`
- Used for interface method calls: `system.interfaceMethod()` (v0.81.2+)
- For non-interface methods, use `self.method()` instead
- Using `system` as a variable name will cause a parse error
- Interface return values are set with `@@:return = value` (not via the `system` keyword)

## Null Value (v0.31)

Frame v0.31 uses `None` as the single null value keyword, aligning with Python conventions:

- **Standard**: `None` - The only keyword for null/undefined values
- **Removed**: `null` and `nil` are no longer supported

Example:
```frame
var x = None        // The only way to represent null values

if value == None {  // Standard null comparison
    print("Value is None")
}
```

## Removed Legacy Features (v0.31)

The following v0.11 syntax has been **completely removed** from the language as of v0.31:

1. **System declaration**: 
   - Old: `#SystemName ... ##`
   - New: `system SystemName { ... }`

2. **System parameters**:
   - Old: `#SystemName [$[start], >[enter], #[domain]]`
   - New: `system SystemName ($(start), $>(enter), domain)`

3. **System instantiation**:
   - Old: `SystemName($("a"), >("b"), #("c"))`
   - New: `SystemName("a", "b", "c")` (flattened arguments)

4. **Block markers**: 
   - Old: `-interface-`, `-machine-`, `-actions-`, `-domain-`
   - New: `interface:`, `machine:`, `actions:`, `domain:`

5. **Return operators**: 
   - **REMOVED**: `^` and `^(value)`
   - Use: `return` and `return value`

6. **Return assignment**:
   - **REMOVED**: `^=` and `return = value`
   - Use: `@@:return = value`

7. **Ternary test operators**:
   - **REMOVED**: `?`, `?!`, `?~`, `?#`, `?:`
   - Use: if/elif/else statements

8. **Test terminators**:
   - **REMOVED**: `:|` and `::`
   - No longer needed

9. **Pattern matching**:
   - **REMOVED**: `~/` (string), `#/` (number), `:/` (enum)
   - Use: if/elif/else with comparisons

10. **Parameter lists**: 
   - **REMOVED**: `[param1, param2]`
   - Use: `(param1, param2)`

11. **Event selectors**: 
   - **REMOVED**: `|eventName|`
   - Use: `eventName()`

12. **Enter/Exit events**:
   - **REMOVED**: `|>|` and `|<|`
   - Use: `$>()` and `<$()`

13. **Attributes**:
   - **REMOVED**: `#[static]` (Rust-style)
   - Use: `@staticmethod` (Python-style)

14. **Current event reference**:
   - Old: `@` for current event
   - Use: `$@` for current event (single `@` now reserved for attributes)

### Compilation Behavior
- Using any removed syntax causes immediate **compilation errors**
- Clear error messages guide users to modern syntax
- No backward compatibility mode available
- All code must be migrated to v0.31 syntax

### System Parameter Migration Guide

| v0.11 Syntax | v0.20 Syntax | Description |
|--------------|--------------|-------------|
| `$[params]` | `$(params)` | Start state parameters |
| `>[params]` | `$>(params)` | Start state enter event parameters |
| `#[params]` | `params` | Domain parameters (no special syntax) |
| `$(<args>)` | `args` | Start state arguments (flattened) |
| `>(<args>)` | `args` | Enter event arguments (flattened) |
| `#(<args>)` | `args` | Domain arguments (flattened) |

## Special Event Handlers

Frame systems support special built-in event handlers:

```frame
$StateName {
    // Enter event - called when transitioning into this state
    $>() {
        print("Entering state")
        return
    }
    
    // Exit event - called when transitioning out of this state
    <$() {
        print("Exiting state") 
        return
    }
    
    // Regular event handlers
    eventName() {
        // handle event
        return
    }
}
```

## Special Symbols

- `$` - State prefix and enter event symbol
- `<$` - Exit event symbol  
- `->` - Transition operator
- `=>` - Dispatch/hierarchy operator
- `=> $^` - Forward event to parent state (v0.20)
- `$@` - Current event reference
- `#` - System type prefix (v0.11 legacy)
- `##` - System terminator (v0.11 legacy)

## Array Indexing with Function Calls (v0.38)

Frame v0.38 adds support for calling functions stored in arrays or dictionaries directly after indexing:

```frame
// Store functions in collections
var operations = [add, multiply, subtract]
var ops_dict = {"add": add, "mul": multiply}

// Call indexed functions directly
var result1 = operations[0](10, 5)    // Calls add(10, 5)
var result2 = operations[1](10, 5)    // Calls multiply(10, 5)
var result3 = ops_dict["add"](3, 4)   // Calls add(3, 4)

// Works with any expression that returns a callable
var matrix = [[fn1, fn2], [fn3, fn4]]
var value = matrix[0][1](x, y)        // Calls fn2(x, y)
```

This pattern is particularly useful for:
- Dispatch tables and strategy patterns
- Function arrays for event handling
- Dynamic function selection based on runtime conditions
- Implementing callback mechanisms

The parser automatically detects when a function call follows an array/dictionary index operation and generates the appropriate code without requiring intermediate variables.



## Version History

### v0.54 (2025-09-12) - Star Expressions and Collection Constructors

#### Star Expressions for Unpacking
- **Python-style unpacking**: Support for `*rest` syntax in variable declarations
- **Multiple positions**: Star can appear at beginning, middle, or end
- **Works with tuples and lists**: `var x, *rest = (1, 2, 3, 4)`
- **Edge cases handled**: Empty unpacking, single element unpacking

#### Collection Constructor Arguments (Validated)
- **list()**: Works with iterables, strings, ranges
- **dict()**: Works with pairs, empty constructor
- **set()**: Works with iterables, strings  
- **tuple()**: Works with iterables, ranges
- **Type conversions**: str(), int(), float(), bool() all working

### v0.53 (2025-09-11) - Collection Literal Parsing Fix

#### Bug Fixes
- **Collection Literal Comma Parsing**: Fixed issue where comma-separated values in collection literals were incorrectly wrapped in tuples
  - Lists: `[1, 2, 3]` now correctly generates `[1, 2, 3]` instead of `[(1, 2, 3)]`
  - Dictionaries, sets, and tuples also correctly handle comma-separated elements
  - Parser now uses context-aware parsing to distinguish collection elements from tuple expressions

#### Technical Implementation
- Added `is_parsing_collection` flag to parser for context tracking
- Modified collection parsing functions to set/restore context flag
- Updated `assignment_or_lambda()` to check context before tuple wrapping

#### Known Limitations (Updated in v0.53)
- ~~Multiple variable declarations (`var x, y = 10, 20`) remain partially implemented~~ **FIXED in v0.53**

### v0.52 (2025-01-29) - Multiple Assignment and Tuple Unpacking

#### New Features
- **Multiple Assignment**: Assign values to multiple variables in a single statement (`x, y = 10, 20`)
- **Tuple Unpacking**: Unpack tuples and lists into individual variables (`p, q, r = tuple_value`)
- **Variable Swapping**: Elegant value swapping without temporary variables (`a, b = b, a`)
- **Function Return Unpacking**: Unpack multiple return values (`lat, lon = get_coordinates()`)

#### Grammar Changes
- Assignment syntax extended to support comma-separated targets and values
- Parser handles comma-separated expressions as tuples when appropriate
- Automatic tuple wrapping for multiple RHS values

#### Known Limitations (Resolved in v0.53)
- ~~Multiple variable declarations (`var x, y = 10, 20`) require separate declarations~~ **FIXED in v0.53**
- List literals with commas create nested tuples (workaround available)

### v0.51 (2025-01-28) - Loop Else Clauses

#### New Features
- **Loop Else Clauses**: Python-style else blocks for for and while loops
- **Break Detection**: Else block executes only when loop completes without break
- **Search Patterns**: Ideal for implementing search-and-not-found patterns

### v0.50 (2025-01-28) - Delete Statement

#### New Features
- **Delete Statement**: `del` keyword for removing variables, list elements, dictionary entries, and slices
- **Comprehensive Deletion Support**: Works with all indexable types and nested structures
- **Slice Deletion**: Full support for deleting slices including with step parameters

#### Examples
```frame
# Delete variables and elements
del x                      # Delete variable
del mylist[2]             # Delete list element
del mydict["key"]         # Delete dict entry
del data[0:5]            # Delete slice
del nested["a"][0]["b"]   # Delete from nested structures
```

### v0.40 (2025-09-09) - Python Alignment Complete

#### Breaking Changes
- **Comment Syntax**: Removed C-style comments (`//`, `/* */`), now use Python-style `#` comments
- **Logical Operators**: Removed C-style `&&`, `||`, `!` operators (use `and`, `or`, `not`)

#### New Features

##### Bitwise XOR Operator
- **Operator**: `^` for bitwise XOR operations
- **Compound Assignment**: `^=` for XOR with assignment
- **Precedence**: Between bitwise AND and OR operations

##### Matrix Multiplication Operator
- **Operator**: `@` for matrix multiplication (requires NumPy or similar)
- **Compound Assignment**: `@=` for matrix multiplication with assignment
- **Precedence**: Same as multiplication and division
- **Use Case**: Scientific computing with NumPy arrays

##### Floor Division
- **Operator**: `//` for integer division (enabled by comment syntax change)
- **Compound Assignment**: `//=` for floor division with assignment

##### Numeric Literals
- **Binary**: `0b1010` notation for binary numbers
- **Octal**: `0o755` notation for octal numbers
- **Hexadecimal**: `0x1A2B` notation for hex numbers

#### Examples
```frame
# Python-style comments (v0.40)
fn test_new_operators() {
    # Bitwise XOR
    var flags = 0b1010
    flags ^= 0b0011  # Toggle bits
    
    # Matrix multiplication (requires NumPy)
    import numpy as np
    var a = np.array([[1, 2], [3, 4]])
    var b = np.array([[5, 6], [7, 8]])
    var result = a @ b  # Matrix multiplication
    a @= b              # In-place matrix multiplication
    
    # Floor division
    var result = 10 // 3  # Result: 3
    result //= 2          # Result: 1
    
    # All compound assignments
    var x = 10
    x += 5   # 15
    x -= 3   # 12
    x *= 2   # 24
    x /= 4   # 6.0
    x //= 2  # 3
    x %= 2   # 1
    x **= 3  # 1
    x |= 2   # 3
    x &= 2   # 2
    x ^= 3   # 1
    x <<= 2  # 4
    x >>= 1  # 2
}
```

### v0.39 (2025-09-08) - Python Operators

#### New Operators
- **Compound Assignments**: `+=`, `-=`, `*=`, `/=`, `%=`, `**=`, `&=`, `|=`, `<<=`, `>>=`
- **Bitwise Operators**: `&` (AND), `|` (OR), `~` (NOT), `<<` (left shift), `>>` (right shift)
- **Identity Operators**: `is`, `is not` for object identity comparison
- **Membership Operators**: `in`, `not in` for container membership testing

### v0.38 (2025-09-07) - First-Class Functions & Collections

#### New Features
- **First-Class Functions**: Functions as values, pass as arguments, return from functions
- **Lambda Expressions**: `lambda x: x * 2` anonymous function syntax
- **Exponent Operator**: `**` for exponentiation with right-associativity
- **Empty Set Literal**: `{,}` to distinguish from empty dict `{}`
- **Python Logical Operators**: `and`, `or`, `not` keywords (removed `&&`, `||`, `!`)


## Complete Feature Summary (v0.55)

Frame v0.55 represents a feature-complete state machine language with modern Python-aligned syntax and 100% test coverage.

### Language Features by Category

#### State Machines
- ✅ States with enter/exit handlers
- ✅ Hierarchical state machines
- ✅ State parameters and arguments
- ✅ Event forwarding and transitions
- ✅ Interface/machine/actions/operations/domain blocks

#### Type System
- ✅ Type annotations for parameters and returns
- ✅ Classes with methods and variables
- ✅ Enums with custom values
- ✅ Static methods with @staticmethod
- ✅ Property decorators with @property

#### Modern Syntax
- ✅ Async/await functions and handlers
- ✅ Lambda expressions with closures
- ✅ First-class functions
- ✅ Pattern matching (match-case)
- ✅ List/dict/set comprehensions
- ✅ Generator expressions

#### Operators (Python-aligned)
- ✅ Logical: and, or, not
- ✅ Bitwise: &, |, ^, ~, <<, >>
- ✅ Compound: +=, -=, *=, /=, %=, **=, &=, |=, ^=, <<=, >>=, //=, @=
- ✅ Identity: is, is not
- ✅ Membership: in, not in
- ✅ Matrix: @, @=
- ✅ Floor division: //, //=
- ✅ Exponentiation: **

#### Collections
- ✅ Lists with all methods
- ✅ Dictionaries with all methods
- ✅ Sets and empty set literal {,}
- ✅ Tuples and unpacking
- ✅ Slicing with step support
- ✅ Star expressions for unpacking
- ✅ Function references in collections

#### Strings
- ✅ F-strings with expressions
- ✅ Raw strings (r-prefix)
- ✅ Byte strings (b-prefix)
- ✅ Triple-quoted multiline
- ✅ Percent formatting
- ✅ All string methods

#### Control Flow
- ✅ If/elif/else statements
- ✅ For/while loops with else clauses
- ✅ Try/except/finally blocks
- ✅ With/async with statements
- ✅ Assert statements
- ✅ Del statements

#### Module System
- ✅ Named modules with nesting
- ✅ Qualified access (module.function)
- ✅ Import statements (Python modules)
- ✅ Module-level variables
- ✅ Global keyword support

### Test Coverage: 100% (339/339 tests passing)

Frame v0.55 achieves complete test coverage with all features validated and working correctly.

## Debugging Support (v0.59)

Frame v0.59 delivers comprehensive debugging support with **100% AST node line tracking coverage**, enabling IDEs and debuggers to provide native Frame debugging experiences through source map generation.

### Source Map Generation ✅ COMPLETE

The Frame transpiler now supports full source map generation through the `--debug-output` flag:

```bash
framec -l python_3 --debug-output input.frm
```

This outputs JSON containing the transpiled code, complete source mappings, and metadata:

```json
{
  "python": "<generated Python code>",
  "sourceMap": {
    "version": "1.0",
    "sourceFile": "input.frm",
    "targetFile": "input.py",
    "mappings": [
      {"frameLine": 4, "pythonLine": 20},
      {"frameLine": 5, "pythonLine": 21},
      {"frameLine": 7, "pythonLine": 23}
    ]
  },
  "metadata": {
    "frameVersion": "0.30.0",
    "generatedAt": "2025-09-17T13:35:58Z",
    "checksum": "sha256:d69cc30c06..."
  }
}
```

### Complete AST Coverage (v0.59)

**All 122 AST nodes** now have line tracking fields, providing comprehensive debugging coverage:

#### High-Priority Nodes (36 nodes)
- Control flow: `IfStmtNode`, `ForStmtNode`, `WhileStmtNode`, `MatchStmtNode`
- Functions: `FunctionNode`, `AsyncFunctionNode`, `LambdaExprNode`
- Systems: `SystemNode`, `StateNode`, `EventHandlerNode`, `TransitionNode`
- Statements: `AssignmentStmtNode`, `ReturnStmtNode`, `CallExprNode`

#### Expression Nodes (30+ nodes)
- Binary operations: `BinaryExprNode` with all operators
- Unary operations: `UnaryExprNode` with all operators
- Literals: `StringLiteralNode`, `NumberLiteralNode`, `BoolLiteralNode`
- Collections: `ListNode`, `DictLiteralNode`, `SetLiteralNode`, `TupleLiteralNode`

#### Comprehension Nodes (6 nodes)
- `ListComprehensionNode`, `DictComprehensionNode`, `SetComprehensionNode`
- `GeneratorExprNode`, `SliceNode`, `StarExprNode`

#### Advanced Features (20+ nodes)
- Pattern matching: `CaseClause`, `GuardClause`, `PatternNode` variants
- Async/await: `AsyncStmtNode`, `AwaitExprNode`
- Classes: `ClassNode`, `MethodNode`, `PropertyNode`
- Type system: `TypeAnnotationNode`, `TypeAliasNode`

### Debugging Capabilities

With complete source maps, debuggers can now:
- ✅ Set breakpoints on **any** Frame source line
- ✅ Display Frame source when stopped at breakpoints
- ✅ Step through Frame code line-by-line
- ✅ Show accurate Frame source locations in call stacks
- ✅ Inspect Frame variables during execution
- ✅ Track execution flow through state transitions
- ✅ Debug complex expressions and comprehensions

### Debug Adapter Protocol (DAP) Support

The v0.59 implementation provides everything needed for VSCode extension DAP integration:
- Source file preservation with original `.frm` names
- Precise line-to-line mappings for all language constructs
- Metadata for validation and version checking
- JSON format ready for IDE consumption

### Implementation Achievements

- **100% Coverage**: All 122 AST nodes tracked (up from 11.5%)
- **Zero Performance Impact**: Line tracking adds negligible overhead

## Semantic Resolution (v0.62) 🆕

Frame v0.62 introduces semantic call resolution during parsing, moving complex call chain analysis from the visitor layer to the parser where it semantically belongs. This architectural improvement simplifies code generation and improves maintainability.

### Semantic Analysis Architecture

```
First Pass (Symbol Table Building)
    ↓
Second Pass (Semantic Analysis)
    ├── SemanticAnalyzer resolves call types
    ├── Stores resolution in AST nodes
    └── Visitor uses pre-resolved types
```

### ResolvedCallType Enum

```rust
pub enum ResolvedCallType {
    Action(String),           // Internal action method
    Operation(String),        // System operation
    SystemOperation {         // Static system operation
        system: String,
        operation: String,
        is_static: bool,
    },
    ClassMethod {            // Class method (static or instance)
        class: String,
        method: String,
        is_static: bool,
    },
    ModuleFunction {         // Module function
        module: String,
        function: String,
    },
    External(String),        // External function/builtin
}
```

## Method Call Resolution Policy (v0.81.3)

Frame v0.81.3 implements a comprehensive method call resolution policy that eliminates ambiguity and provides clear semantic distinctions between different method types.

### Resolution Rules

**Method calls are resolved using explicit prefix syntax:**

1. **`system.interfaceMethod()`** → Interface method calls
2. **`SystemName.staticOperation()`** → Static operation calls  
3. **`self.methodName()`** → Action or instance operation calls

### Conflict Detection and Validation

**For `self.methodName()` calls, the transpiler checks in this order:**

1. **Action/Operation Conflict**: If both an action and operation exist with the same name → **Compile Error**
2. **Valid Resolution**: If only an action OR only an operation exists → **Allow**
3. **Interface Only**: If only an interface method exists → **Error** (suggest `system.methodName()`)
4. **Not Found**: If method doesn't exist in any block → **Error** (undefined method)

### Policy Benefits

- **No Ambiguity**: Each prefix has a distinct, unambiguous meaning
- **Clear Intent**: Code explicitly shows what type of method is being called
- **Conflict Prevention**: Naming conflicts between actions and operations are caught at compile time
- **Type Safety**: Interface methods cannot be accidentally called with wrong syntax

### Examples

```frame
system Calculator {
    interface:
        compute()      // Interface method
        
    actions:
        doCalc() {     // Action method
            // Valid: calling action
            self.helper()      // ✅ Calls action 'helper' 
            
            // Valid: calling interface method
            system.compute()   // ✅ Calls interface 'compute'
            
            // Invalid: interface method with wrong syntax
            self.compute()     // ❌ Error: Use 'system.compute()' instead
        }
        
        helper() {     // Another action
            print("Helper action")
        }
        
    operations:
        multiply(a, b) {       // Operation method
            // Valid: calling action
            self.doCalc()      // ✅ Calls action 'doCalc'
            
            // Valid: calling interface method  
            system.compute()   // ✅ Calls interface 'compute'
            
            return a * b
        }
        
        @staticmethod
        add(a, b) {           // Static operation
            return a + b
        }
        
    machine:
        $Start {
            compute() {
                // Valid calls from event handler
                self.doCalc()           // ✅ Calls action
                self.multiply(2, 3)     // ✅ Calls operation
                system.compute()        // ✅ Calls interface (recursive)
                Calculator.add(1, 2)    // ✅ Calls static operation
            }
        }
}

// Conflict detection example
system ConflictExample {
    actions:
        process() { }     // Action named 'process'
        
    operations: 
        process() { }     // ❌ ERROR: Conflict with action 'process'
}
```

## Semantic Call Resolution (v0.66)

Frame v0.66 makes semantic call resolution an integral part of the parser, removing the previous feature flag and ensuring all calls are properly resolved during parsing. The parser performs two-pass analysis:

1. **First Pass**: Builds the symbol table with all definitions
2. **Second Pass**: Resolves all call expressions to their semantic types

### ResolvedCallType Enum

The parser resolves all call expressions to one of these semantic types:

```rust
pub enum ResolvedCallType {
    Action(String),              // Internal action call (adds _ prefix in Python)
    Operation(String),           // Internal operation call  
    SystemInterface {            // Interface method call from within system (v0.66)
        system: String,
        method: String,
    },
    SystemOperation {            // Qualified system operation call
        system: String,
        operation: String,
        is_static: bool,
    },
    ClassMethod {                // Class method call
        class: String,
        method: String,
        is_static: bool,
    },
    ModuleFunction {             // Module function call
        module: String,
        function: String,
    },
    External(String),            // External function call (built-ins, imports)
}
```

### Call Resolution Examples (v0.66)

```frame
system Calculator {
    interface:
        compute()
        reset()
        
    actions:
        doCalc() {
            // v0.66: MUST use explicit self prefix
            self.doCalc()           // Resolved as Action("doCalc")
        }
    
    operations:
        @staticmethod
        add(a, b) {
            return a + b
        }
        
        multiply(a, b) {
            // Instance operation can call actions
            self.doCalc()           // Resolved as Action("doCalc")
            return a * b
        }
        
    machine:
        $Start {
            compute() {
                // All internal calls require self prefix
                self.multiply(2, 3)  // Resolved as Operation("multiply")
                self.doCalc()       // Resolved as Action("doCalc")
                self.reset()        // Resolved as SystemInterface("Calculator", "reset")
                Calculator.add(1,2) // Resolved as SystemOperation (static)
            }
    
    machine:
        $Idle {
            calculate() {
                // Resolved as SystemOperation
                var sum = Calculator.add(5, 3)
                
                // Resolved as External (Python builtin)
                print("Sum: " + str(sum))
                
                // Resolved as ClassMethod
                var p = Point.origin()
            }
        }
}
```

### Feature Flag Control

The semantic resolution feature can be enabled via environment variable:

```bash
# Enable semantic resolution during compilation
FRAME_SEMANTIC_RESOLUTION=1 framec -l python_3 file.frm

# With debug output to see resolution results
FRAME_TRANSPILER_DEBUG=1 FRAME_SEMANTIC_RESOLUTION=1 framec -l python_3 file.frm
```

### AST Enhancement

Each `CallExprNode` now includes:
- `context`: CallContextType (SelfCall, StaticCall, ExternalCall)
- `resolved_type`: Option<ResolvedCallType> for semantic resolution

### Benefits

1. **Simplified Visitor Logic**: Visitors no longer need complex call chain analysis
2. **Better Error Messages**: Semantic errors caught during parsing with context
3. **Cleaner Architecture**: Semantic analysis in the appropriate layer
4. **Easier Maintenance**: Single location for call resolution logic
5. **Future-Proof**: Foundation for advanced type checking and IDE support
- **Full Backward Compatibility**: All existing tests pass (374/374)
- **Production Ready**: Comprehensive testing and validation complete
