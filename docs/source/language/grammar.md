# Frame Language Grammar (v0.49)

**Last Updated**: 2025-09-11  
**Status**: v0.49 - Complete error handling support with try/finally fix. **100% test success rate with comprehensive error handling**.

This document provides the formal grammar specification for the Frame language using BNF notation, along with examples for each language construct.

## Module Structure

```bnf
module: (import_stmt | module_decl | enum_decl | var_decl | function | class | system)*

module_decl: 'module' IDENTIFIER '{' module_content '}'
module_content: (module_decl | enum_decl | var_decl | function | class | system)*
```

**v0.34 Module System (FULLY IMPLEMENTED)**: 
- ✅ Module keyword and nested module declarations working
- ✅ FSL requires explicit import (`from fsl import str, int, float`)
- ✅ Symbol table supports nested module scopes
- ✅ FSL imports filtered from generated code (Python target)
- ✅ Qualified name resolution (`module.function()`) fully implemented
- ✅ Cross-module function and variable access working
- ✅ Module code generation creates proper Python structures
- ✅ 100% test success rate with all module features

**v0.31 Import Support**: Modules can now include native import statements at the top level, supporting Python module imports without requiring backticks.

**v0.30 Multi-Entity Support**: Modules can contain any combination of functions and systems in any order. Each entity (function or system) can have individual attributes.

### Module Declaration Examples (v0.34)
```frame
// Empty module
module utils {
}

// Module with functions
module math_utils {
    fn add(a, b) {
        return a + b
    }
    
    fn multiply(a, b) {
        return a * b
    }
}

// Module with variables (fully implemented)
module config {
    var debug = true
    var maxRetries = 3
    
    fn getConfig() {
        return "Debug: " + str(debug) + ", Max: " + str(maxRetries)
    }
}

// Nested modules with full functionality
module lib {
    module helpers {
        fn format(s) {
            return str(s)
        }
        
        var helperVersion = "1.0"
    }
    
    module validators {
        fn isValid(x) {
            return x > 0
        }
    }
}

// Using modules with qualified names
fn main() {
    var sum = math_utils.add(5, 3)
    var product = math_utils.multiply(sum, 2)
    var formatted = lib.helpers.format(product)
    var version = lib.helpers.helperVersion
    var configInfo = config.getConfig()
    
    print("Sum: " + str(sum))
    print("Product: " + str(product)) 
    print("Formatted: " + formatted)
    print("Version: " + version)
    print(configInfo)
}
```

**All Features Working**:
- Module syntax parsing and code generation
- Nested module declarations with full functionality
- FSL import requirement and filtering
- Namespace conflict prevention
- Qualified name access (`module.function()`, `module.variable`)
- Cross-module function calls
- Module variables accessible with proper scoping
- Complete Python code generation for module structures

**No Current Limitations**: All planned v0.34 module features fully implemented and tested.

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

## Import Statements (v0.31/v0.34)

Frame v0.31 introduces native import statement support. v0.34 adds FSL imports.

```bnf
import_stmt: simple_import | aliased_import | from_import

simple_import: 'import' dotted_name
aliased_import: 'import' dotted_name 'as' IDENTIFIER
from_import: 'from' dotted_name 'import' (import_items | '*')

dotted_name: IDENTIFIER ('.' IDENTIFIER)*
import_items: IDENTIFIER (',' IDENTIFIER)*
```

### Import Examples
```frame
// v0.34: FSL imports (required for FSL operations)
from fsl import str, int, float, bool
from fsl import list, map, set
from fsl import *  // Import all FSL operations

// Simple imports
import math
import json

// Aliased imports
import numpy as np
import os.path as osp

// From imports
from collections import defaultdict, OrderedDict
from typing import List, Dict, Optional

// Wildcard imports
from typing import *

// Using imported modules in functions
fn main() {
    var pi = math.pi
    var root = math.sqrt(16)
    var data = json.dumps({"key": "value"})
}

// Using imported modules in systems
system Calculator {
    operations:
        compute() {
            var result = math.cos(0)
            return result
        }
}
```

**Note**: For languages other than Python, backticks can still be used for language-specific import syntax.

## Module System (v0.34 - IMPLEMENTED)

Frame has implemented a comprehensive module system with explicit nested modules within files, qualified name resolution, and cross-module access.

### File as Module
```bnf
// Each .frm file automatically creates a module named after the file
// File: math_utils.frm creates module 'math_utils'
```

### Explicit Nested Modules
```bnf
module_decl: 'module' IDENTIFIER '{' module_body '}'
module_body: (import_stmt | module_decl | function | system | var_decl | enum_decl)*
```

### Module Examples (Current Implementation)
```frame
// Single file with explicit modules - fully working
from fsl import str, int

// Module with functions and variables
module math_utils {
    fn add(a, b) {
        return a + b
    }
    
    var version = "1.0"
}

// Nested modules with full functionality
module advanced {
    module algorithms {
        fn factorial(n) {
            if n <= 1 { return 1 }
            return n * factorial(n - 1)
        }
        
        var algorithmCount = 1
    }
}

// Using modules with qualified names (fully implemented)
fn main() {
    var sum = math_utils.add(5, 3)                    // Qualified function call
    var ver = math_utils.version                      // Qualified variable access
    var fact = advanced.algorithms.factorial(5)      // Nested module access
    var count = advanced.algorithms.algorithmCount   // Nested variable access
    
    print("Sum: " + str(sum))
    print("Version: " + ver)
    print("Factorial: " + str(fact))
    print("Algorithm count: " + str(count))
}
```

### Future: Multi-File Module Support
```frame
// Planned for future versions - multi-file imports
// File: math_utils.frm
module math_utils {
    fn add(a, b) { return a + b }
}

// File: main.frm  
import math_utils                    // Import from another file
fn main() {
    var sum = math_utils.add(5, 3)   // Cross-file qualified access
}
```

### FSL as Optional Import (Implemented)
```frame
// FSL must be explicitly imported - prevents namespace conflicts
from fsl import str, int, list

// FSL works with modules
module utilities {
    fn convertAndFormat(value) {
        return "Value: " + str(value)  // Uses imported FSL str()
    }
}

fn main() {
    var s = str(42)                        // Direct FSL usage
    var formatted = utilities.convertAndFormat(123)  // Module function using FSL
}

// Without FSL import, users can define their own versions
fn customExample() {
    // Define custom str function (no conflict with FSL)
    fn str(x) {
        return "Custom: " + x
    }
    
    var result = str(42)  // Uses local custom function
}
```

### Symbol Resolution Rules (Implemented)
1. **Module qualification**: `module.function()` and `module.variable` work correctly
2. **Nested modules**: `module.submodule.function()` fully supported
3. **FSL integration**: FSL operations work seamlessly with module system
4. **Namespace protection**: FSL imports prevent conflicts with user functions
5. **Scope resolution**: Proper LEGB scoping with module boundaries
6. **No silent conflicts**: Clear error messages for naming conflicts

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
function: attributes? ('async')? 'fn' IDENTIFIER '(' parameter_list? ')' type? function_body
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

## Classes (v0.46)

Frame v0.46 introduces object-oriented programming with class support, enabling familiar OOP patterns alongside Frame's state machine paradigm.

```bnf
class: 'class' IDENTIFIER ('extends' IDENTIFIER)? '{' class_body '}'
class_body: (class_var_decl | method_decl)*
class_var_decl: 'var' IDENTIFIER '=' expr
method_decl: decorator* 'fn' IDENTIFIER '(' parameter_list? ')' type? method_body
decorator: '@' decorator_name
decorator_name: 'property' | 'classmethod' | 'staticmethod' 
              | IDENTIFIER ('.' ('setter' | 'deleter'))?
method_body: '{' stmt* '}'
```

### Basic Class Declaration

```frame
class Point {
    # Class variables
    var origin_x = 0
    var origin_y = 0
    
    # Constructor (method named 'init')
    fn init(x, y) {
        self.x = x
        self.y = y
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
    
    # Class method
    @classmethod
    fn from_tuple(cls, coords) {
        return cls(coords[0], coords[1])
    }
}
```

### Class Inheritance

Classes can inherit from parent classes using the `extends` keyword:

```frame
class Animal {
    var species = "Unknown"
    
    fn init(name) {
        self.name = name
    }
    
    fn speak() {
        return "Some sound from " + self.name
    }
    
    fn move() {
        return self.name + " is moving"
    }
}

class Dog extends Animal {
    fn init(name, breed) {
        # Call parent constructor
        super().__init__(name)
        self.breed = breed
        self.species = "Canis familiaris"
    }
    
    # Override parent method
    fn speak() {
        return "Woof! My name is " + self.name
    }
    
    # Add new method
    fn fetch() {
        return self.name + " is fetching!"
    }
}
```

### Properties

Properties provide controlled access to instance variables:

```frame
class Temperature {
    fn init(celsius) {
        self._celsius = celsius
    }
    
    @property
    fn celsius() {
        return self._celsius
    }
    
    @celsius.setter
    fn celsius(value) {
        if value < -273.15 {
            print("Error: Temperature below absolute zero")
            return
        }
        self._celsius = value
    }
    
    @property
    fn fahrenheit() {
        return self._celsius * 9.0 / 5.0 + 32.0
    }
    
    @fahrenheit.setter
    fn fahrenheit(value) {
        self.celsius = (value - 32.0) * 5.0 / 9.0
    }
}
```

### Special Methods

Frame supports Python special methods (dunder methods) for operator overloading and built-in behavior:

```frame
class Vector {
    fn __init__(x, y) {
        self.x = x
        self.y = y
    }
    
    fn __str__() {
        return "Vector(" + str(self.x) + ", " + str(self.y) + ")"
    }
    
    fn __add__(other) {
        return Vector(self.x + other.x, self.y + other.y)
    }
    
    fn __eq__(other) {
        return self.x == other.x and self.y == other.y
    }
    
    fn __len__() {
        return int((self.x * self.x + self.y * self.y) ** 0.5)
    }
}

# Usage
fn main() {
    var v1 = Vector(3, 4)
    var v2 = Vector(1, 2)
    
    print(str(v1))              # "Vector(3, 4)"
    var v3 = v1 + v2            # Calls __add__
    print(v1 == v2)             # Calls __eq__
    print(str(len(v1)))         # Calls __len__
}
```

### Access Modifiers (v0.48)

Frame follows Python access modifier conventions using naming patterns:

```frame
class BankAccount {
    var account_count = 0  # Public class variable
    
    fn init(owner, balance) {
        self.owner = owner           # Public instance variable
        self._balance = balance      # Protected instance variable
        self.__pin = 1234           # Private instance variable
    }
    
    # Public method
    fn get_owner() {
        return self.owner
    }
    
    # Protected method
    fn _check_balance() {
        return self._balance
    }
    
    # Private method
    fn __validate_pin(pin) {
        return pin == self.__pin
    }
    
    fn withdraw(amount, pin) {
        if not self.__validate_pin(pin) {
            return "Invalid PIN"
        }
        
        if amount > self._balance {
            return "Insufficient funds"
        }
        
        self._balance = self._balance - amount
        return "Withdrew: " + str(amount)
    }
}
```

**Access Modifier Conventions:**
- **Public**: `name` - Normal identifiers, accessible everywhere
- **Protected**: `_name` - Single underscore prefix, intended for internal use and inheritance
- **Private**: `__name` - Double underscore prefix, Python name mangling applied

### Key Features

- **Constructor Methods**: Methods named `init` or `__init__` become constructors
- **Implicit Self**: Method signatures don't include `self` (added automatically)
- **Instance Variables**: Created via `self.varname = value` assignments
- **Class Variables**: Declared at class level with `var name = value`
- **Static Methods**: Use `@staticmethod` decorator for non-instance methods
- **Class Methods**: Use `@classmethod` decorator, receive `cls` parameter
- **Method Calls**: Instance methods via `obj.method()`, static via `Class.method()`
- **Inheritance**: `extends` keyword for single inheritance
- **Super Calls**: `super().__init__()` to call parent methods
- **Properties**: `@property`, `@name.setter`, `@name.deleter` decorators
- **Special Methods**: All Python dunder methods supported through passthrough
- **Access Modifiers**: Python-style naming conventions for public, protected, and private members

### Factory Pattern Example

```frame
class User {
    var user_count = 0
    
    fn init(name, email) {
        self.name = name
        self.email = email
        User.user_count = User.user_count + 1
    }
    
    @classmethod
    fn from_string(cls, user_string) {
        parts = user_string.split(":")
        if len(parts) == 2 {
            return cls(parts[0], parts[1])
        }
        return None
    }
    
    @staticmethod
    fn validate_email(email) {
        return "@" in email and "." in email
    }
    
    fn __str__() {
        return "User(" + self.name + ", " + self.email + ")"
    }
}

# Usage
fn main() {
    var user1 = User("Alice", "alice@example.com")
    var user2 = User.from_string("Bob:bob@example.com")
    
    if User.validate_email("test@example.com") {
        print("Valid email")
    }
    
    print("Total users: " + str(User.user_count))
}
```

## Async/Await Support (v0.35)

Frame v0.35 introduces async/await support for asynchronous programming patterns, enabling integration with async libraries and frameworks.

```bnf
async_function: 'async' 'fn' IDENTIFIER '(' parameter_list? ')' type? function_body
async_interface_method: 'async' IDENTIFIER '(' parameter_list? ')' type?
await_expr: 'await' expr
```

### Async Functions

Async functions are declared with the `async` keyword and can contain await expressions:

```frame
// Async functions at module level
async fn fetchData(url) {
    print("Fetching from " + url)
    return "data from " + url
}

async fn processData(data) {
    print("Processing: " + data)
    var result = await fetchData("api.example.com/process")
    return "processed " + data + " with " + result
}

// Regular function calling async functions
fn main() {
    // Note: Cannot await in non-async function
    // Use async_main() for async coordination
    print("Starting application")
}

async fn async_main() {
    var data = await fetchData("api.example.com/data")
    var processed = await processData(data)
    print("Final result: " + processed)
}
```

### Async Interface Methods

Systems can declare async interface methods that generate async Python methods:

```frame
system AsyncService {
    interface:
        async getData(id)          // Generates: async def getData(self, id)
        async processItem(item)    // Generates: async def processItem(self, item)
        normalMethod(x)            // Generates: def normalMethod(self, x)
    
    machine:
        $Ready {
            getData(id) {
                print("Getting data for id: " + str(id))
                var result = "data_" + str(id)
                return = result
            }
            
            processItem(item) {
                print("Processing: " + item)
                return = "processed_" + item
            }
            
            normalMethod(x) {
                return x * 2
            }
        }
}
```

### Async Event Handlers (v0.37)

Event handlers can be explicitly marked as `async` to support await expressions within state machines:

```frame
system AsyncDataPipeline {
    interface:
        async fetchBatch(urls)
        async processBatch(id)
        
    machine:
        $Idle {
            fetchBatch(urls) {
                self.urls = urls
                -> $Downloading
            }
        }
        
        $Downloading {
            // Explicitly async enter handler
            async $>() {
                var data = await download_parallel(self.urls)
                self.batch_data = data
                -> $Processing
            }
        }
        
        $Processing {
            // Async handler with await
            async processBatch(id) {
                var result = await process_item(self.batch_data[id])
                return = result
            }
            
            // Must be async - entered from async state
            async $>() {
                print("Processing " + str(len(self.batch_data)) + " items")
                -> $Complete
            }
        }
        
        $Complete {
            // Must be async - part of async transition chain
            async $>() {
                print("Pipeline complete")
            }
        }
}
```

**Async Chain Validation**: Frame v0.37 validates that all handlers in an async transition chain are properly marked:
- If an async handler transitions to another state, that state's enter handler must be async
- Exit handlers in states with async transitions must be async (if they exist)
- Provides clear compile-time errors explaining which handlers need async marking

### Current Implementation Status

**✅ Implemented Features (v0.37)**:
- Async function declarations (`async fn name() { }`)
- Async interface method declarations (`async methodName()`)
- Async event handler declarations (`async $>() { }`, `async eventName() { }`)
- Await expression parsing (`await expression`)
- Async function code generation (Python `async def`)
- Async interface method code generation
- Async event handler code generation with full state async propagation
- Runtime infrastructure nodes (RuntimeInfo, KernelNode, RouterNode, etc.)
- Comprehensive async chain validation during semantic analysis
- Clear error messages for missing async markings in transition chains
- `with` and `async with` statement support for context managers

**✅ Runtime Architecture**: 
Frame v0.37 introduces runtime infrastructure nodes that track async requirements throughout the system. The semantic analyzer computes which states, handlers, and runtime components need to be async, enabling proper async/await code generation for the entire state machine runtime.

**Example Working Pattern**:
```frame
system SimpleAsync {
    interface:
        async getData(id)
    
    machine:
        $Ready {
            getData(id) {
                // Works: Simple state logic without await
                self.result = "data_" + str(id)
                return = self.result
            }
        }
    
    domain:
        var result = None
}
```

**Test Coverage**: 207/207 tests passing (100% success rate) including multiple async test cases.

## Error Handling (v0.49)

Frame provides comprehensive error handling support through try/except/finally/raise statements that generate idiomatic Python exception handling code.

```bnf
try_statement: 'try' '{' block '}' (except_clause)* (else_clause)? (finally_clause)?
except_clause: 'except' (exception_type (',' exception_type)* | '(' exception_type (',' exception_type)* ')') ('as' IDENTIFIER)? '{' block '}'
else_clause: 'else' '{' block '}'
finally_clause: 'finally' '{' block '}'
raise_statement: 'raise' expression?
```

### Error Handling Features

- **try/except blocks**: Catch and handle specific exceptions
- **Exception variable binding**: Access exception details with `as` syntax
- **Multiple exception types**: Handle different exception types in one clause
- **finally blocks**: Code that always executes for cleanup
- **raise statements**: Throw exceptions with custom messages
- **try/finally**: Finally blocks without except clauses (v0.49 fix)
- **Nested exception handling**: Full support for nested try/except blocks

### Basic Exception Handling

```frame
fn safe_division(a, b) {
    try {
        var result = a / b
        print("Result: " + str(result))
        return result
    }
    except ZeroDivisionError {
        print("Error: Cannot divide by zero!")
        return None
    }
}
```

### Exception Variable Binding

Access exception details using the `as` keyword:

```frame
fn parse_number(text) {
    try {
        var num = int(text)
        return num
    }
    except ValueError as e {
        print("Parse error: " + str(e))
        return 0
    }
}
```

### Multiple Exception Types

Handle different exception types in a single except clause:

```frame
fn process_data(data) {
    try {
        var num = int(data)
        var result = 100 / num
        return result
    }
    except (ValueError, ZeroDivisionError) as e {
        print("Processing failed: " + str(e))
        return None
    }
}
```

### Try/Except/Finally

The finally block always executes, regardless of exceptions:

```frame
fn read_file(filename) {
    var file = None
    try {
        file = open(filename, "r")
        var content = file.read()
        return content
    }
    except IOError as e {
        print("File error: " + str(e))
        return ""
    }
    finally {
        if file is not None {
            file.close()
            print("File closed")
        }
    }
}
```

### Try/Finally Without Except (v0.49)

Frame v0.49 supports try/finally blocks without except clauses for cleanup-only scenarios:

```frame
fn cleanup_example() {
    var resource = acquire_resource()
    try {
        # Do work with resource
        var result = process(resource)
        print("Processing complete")
    }
    finally {
        # Always cleanup, even if no exceptions
        release_resource(resource)
        print("Resource cleaned up")
    }
}
```

### Raise Statements

Throw custom exceptions with descriptive messages:

```frame
fn validate_age(age) {
    if age < 0 {
        raise ValueError("Age cannot be negative: " + str(age))
    }
    if age > 150 {
        raise ValueError("Age seems unrealistic: " + str(age))
    }
    return age
}

fn test_validation() {
    try {
        validate_age(-5)
    }
    except ValueError as e {
        print("Validation failed: " + str(e))
    }
}
```

### Nested Exception Handling

Frame supports arbitrary nesting of try/except blocks:

```frame
fn complex_operation() {
    try {
        print("Starting outer operation")
        
        try {
            print("Inner operation")
            raise ValueError("Inner failure")
        }
        except ValueError as inner_e {
            print("Caught inner exception: " + str(inner_e))
            # Re-raise as different exception
            raise RuntimeError("Operation failed in inner handler")
        }
        
    }
    except RuntimeError as outer_e {
        print("Caught outer exception: " + str(outer_e))
    }
    finally {
        print("Outer finally block executed")
    }
}
```

### System Error Handling

Error handling works seamlessly within Frame state machines:

```frame
system SafeProcessor {
    interface:
        processItem(item)
        handleError(error)
    
    machine:
        $Ready {
            processItem(item) {
                try {
                    var result = risky_operation(item)
                    self.result = result
                    -> $Complete
                }
                except Exception as e {
                    self.error = str(e)
                    -> $Error
                }
            }
        }
        
        $Error {
            $>() {
                self.handleError(self.error)
            }
            
            handleError(error) {
                print("System error: " + error)
                -> $Ready  # Reset to ready state
            }
        }
        
        $Complete {
            $>() {
                print("Processing successful: " + str(self.result))
            }
        }
    
    domain:
        var result = None
        var error = None
}
```

### Generated Python Code

Frame error handling generates idiomatic Python exception handling:

```frame
# Frame source:
try {
    var x = 10 / 0
}
except ZeroDivisionError as e {
    print("Error: " + str(e))
}
finally {
    print("Cleanup")
}
```

```python
# Generated Python:
try:
    x = 10 / 0
except ZeroDivisionError as e:
    print("Error: " + str(e))
finally:
    print("Cleanup")
```

### Implementation Status

**✅ Fully Implemented Features (v0.49)**:
- Try/except blocks with full exception type support
- Exception variable binding with `as` keyword  
- Multiple exception types in single except clause
- Finally blocks for cleanup code
- Raise statements with expression support
- Try/finally without except (v0.49 parser fix)
- Nested exception handling
- Perfect Python code generation
- System integration for state machine error handling

**✅ Test Coverage**: All 7 error handling scenarios verified working:
1. Basic try/except blocks
2. Exception variable binding (`as e`)
3. Multiple exception types  
4. Try/except/finally
5. Raise statements
6. Nested exception handling
7. Try/finally without except

## Module Variables (v0.31)

Module-level variables can be declared at the top level of a Frame module, making them accessible from all functions and systems in the module.

```bnf
module_var: 'var' IDENTIFIER type? '=' expr
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

fn increment() {
    counter = counter + 1  // Automatic 'global counter' in Python
    return counter
}

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

**v0.31 Default Return Values**: Interface methods can specify default return values using the syntax `: type = value`. This value is returned unless overridden by event handlers or system.return assignments.

## Machine Block

```bnf
machine_block: 'machine:' state*
state: '$' IDENTIFIER ('=>' '$' IDENTIFIER)? '{' event_handler* state_var* '}'
event_handler: ('async')? event_selector '{' stmt* terminator? '}'  // v0.37: async handlers
event_selector: IDENTIFIER '(' parameter_list? ')' (type ('=' expr)?)?
               | '$>' '(' parameter_list? ')'  // Enter handler
               | '<$' '(' parameter_list? ')'  // Exit handler
terminator: 'return' expr?
          | '=>'              // Forward/dispatch event  
          | '->' '$' IDENTIFIER  // Transition to named state (NOT '$^')
stmt: parent_dispatch_stmt | /* other statements */
parent_dispatch_stmt: '=>' '$^'  // Parent dispatch statement
state_var: 'var' IDENTIFIER type? '=' expr
```

## Domain Block

```bnf
domain_block: 'domain:' (domain_var | enum_decl)*
domain_var: 'var' IDENTIFIER type? '=' expr
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
                return = self.count  // self.variable in return assignment
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

**v0.31 System Return Restriction**: Operations cannot use `system.return` as they may be called from contexts without an interface (e.g., directly from outside or from functions). This is enforced at parse time.

**v0.36 Code Generation Architecture**: Event handlers can be generated as individual functions instead of inline code within state methods. This is controlled by the `event_handlers_as_functions` configuration flag. When enabled, each event handler becomes a separate function with state methods serving as dispatchers. Special events are automatically renamed for valid identifiers: `$>` → `_enter`, `<$` → `_exit`.

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

**v0.31 Default Values**: Actions can specify default return values for their return to the caller (not system.return). Actions can set system.return explicitly.

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
// For loops
for_stmt: 'for' (var_decl | identifier) 'in' expr ':' stmt
        | 'for' (var_decl | identifier) 'in' expr block
        | 'for' var_decl ';' expr ';' expr block  // C-style for loop with 'for' keyword

// While loops  
while_stmt: 'while' expr ':' stmt
          | 'while' expr block

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

### Syntax Enforcement

Similar to if/elif/else statements:
- After `:` only single statements are allowed (no `{` blocks)
- After condition/iterable without `:`, braces `{` are required

**Parser Note (v0.38)**: The parser uses lookahead to correctly distinguish for-in loops from binary `in` expressions. When it encounters an identifier followed by `in`, it recognizes this as a for-in loop pattern rather than treating it as a membership test expression.

## Statements

```bnf
stmt: expr_stmt
    | var_decl
    | assignment
    | if_stmt
    | for_stmt
    | while_stmt
    | loop_stmt
    | with_stmt        // v0.37: context managers
    | assert_stmt      // v0.47: assertions
    | return_stmt
    | return_assign_stmt
    | parent_dispatch_stmt
    | transition_stmt
    | state_stack_op
    | block_stmt
    | break_stmt
    | continue_stmt

expr_stmt: expr
var_decl: 'var' IDENTIFIER type? '=' expr
assignment: lvalue assignment_op expr
assignment_op: '=' | '+=' | '-=' | '*=' | '/=' | '%=' | '**='  // v0.39: Compound assignments
             | '&=' | '|=' | '<<=' | '>>='  // v0.39: Bitwise compound assignments
with_stmt: ('async')? 'with' expr ('as' IDENTIFIER)? '{' stmt* '}'  // v0.37
return_stmt: 'return' expr?
return_assign_stmt: 'return' '=' expr
parent_dispatch_stmt: '=>' '$^'
transition_stmt: '->' '$' IDENTIFIER
state_stack_op: '$$[' '+' ']' | '$$[' '-' ']'
block_stmt: '{' stmt* '}'
break_stmt: 'break'
continue_stmt: 'continue'
assert_stmt: 'assert' expr
```

### Assert Statement (v0.47)

Frame v0.47 adds support for assertions for debugging and validation:

```frame
fn validate_input(value) {
    # Basic assertion
    assert value > 0
    
    # Assert with complex expression
    assert value % 2 == 0 and value < 100
    
    # Assert in conditional logic
    if value > 50 {
        assert value < 75
    }
    
    print("All validations passed!")
}

# Assert in systems
system SafeCounter {
    domain:
        var count = 0
        var max = 10
    
    machine:
        $Ready {
            increment() {
                assert count < max  # Ensure we don't overflow
                count = count + 1
                return
            }
            
            decrement() {
                assert count > 0    # Ensure we don't go negative
                count = count - 1
                return
            }
        }
}
```

**Note**: Assert statements generate Python `assert` statements directly. Failed assertions raise `AssertionError` exceptions at runtime.

### With Statement (v0.37)

Frame v0.37 adds support for `with` and `async with` statements for context management:

```frame
// Regular with statement
fn readFile(filename) {
    with open(filename, "r") as f {
        var content = f.read()
        print("File content: " + content)
    }
}

// Async with statement
async fn fetchData(url) {
    async with aiohttp.ClientSession() as session {
        async with session.get(url) as response {
            var text = await response.text()
            return text
        }
    }
}

// With statements in systems
system FileProcessor {
    machine:
        $Ready {
            processFile(filename) {
                with open(filename, "r") as file {
                    self.content = file.read()
                    print("Read " + str(len(self.content)) + " bytes")
                }
                -> $Processing
            }
        }
}
```

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
    $$[+]          // Push current state onto stack
    -> $ModalState // Transition to new state
    return
}

// State stack pop - returns to saved state
closeModal() {
    -> $$[-]       // Pop and transition to previous state
    return
}
```

**State Stack Operators:**
- **`$$[+]`** - Push current state compartment onto stack (preserves variables)
- **`$$[-]`** - Pop state compartment from stack and use as transition target

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

Frame v0.31 introduces the `system.return` special variable for setting interface return values anywhere within event handlers or action methods:

```frame
// Setting interface return values with system.return
interface:
    validateInput(data: string): bool = false  // Default return value

machine:
    $ProcessingState {
        validateInput(data: string) {  // Can override with : bool = true
            if data == "" {
                system.return = false  // Set interface return value
                return                 // Exit event handler  
            }
            
            if checkFormat(data) {
                system.return = true   // Set interface return value
                return                 // Exit event handler
            }
            
            system.return = false      // Default case
            return
        }
    }

// Event handler default overrides interface default
machine:
    $Start {
        getStatus(): int = 99 {  // Override interface default
            // Implicit system.return = 99 on entry
            if someCondition {
                system.return = 200  // Further override
            }
            return
        }
    }

// Actions can also set system.return
actions:
    processData(input: string): string {
        if input == "error" {
            system.return = "failed"   // Set interface return value
            return "internal"   // Return value to caller (action method)
        }
        
        system.return = "success"      // Set interface return value
        return input            // Return value to caller (action method)
    }
```

## Expressions

```bnf
expr: binary_expr | unary_expr | primary_expr | call_expr | self_expr | fsl_expr 
    | list_expr | list_comprehension | unpack_expr | index_expr | slice_expr | lambda_expr  // v0.38: Added lambda_expr

binary_expr: expr operator expr
operator: '+' | '-' | '*' | '/' | '%' | '**'  // v0.38: Added exponent operator
        | '==' | '!=' | '<' | '>' | '<=' | '>='
        | 'and' | 'or'  // v0.38: Python logical operators (replaced && and ||)
        | 'in' | 'not in'  // v0.38: Membership operators
        | '&' | '|' | '<<' | '>>'  // v0.39: Bitwise operators
        | 'is' | 'is not'  // v0.39: Identity operators

unary_expr: ('-' | 'not' | '~') expr  // v0.38: 'not' replaces '!', v0.39: Added '~' bitwise NOT

primary_expr: IDENTIFIER | NUMBER | STRING | SUPERSTRING
            | 'true' | 'false' | 'None'
            | '(' expr ')' | '@'

self_expr: 'self' | 'self' '.' IDENTIFIER  // v0.31: self as standalone or dotted access

call_expr: IDENTIFIER '(' arg_list? ')' | '_' IDENTIFIER '(' arg_list? ')'
arg_list: expr (',' expr)*

// v0.38: Index and slice operations with nested support
index_expr: (IDENTIFIER | self_expr | index_expr) '[' expr ']'  // Nested indexing supported
slice_expr: (IDENTIFIER | self_expr | index_expr) '[' slice_notation ']'  // Slicing support
slice_notation: (expr)? ':' (expr)? (':' (expr)?)?  // [start:end:step]
          // v0.38: Chained indexing like dict[key1][key2] now fully supported

// v0.34: List expressions and comprehensions
list_expr: '[' list_elements? ']'
list_elements: list_element (',' list_element)*
list_element: expr | unpack_expr

list_comprehension: '[' expr 'for' IDENTIFIER 'in' expr ('if' expr)? ']'

unpack_expr: '*' expr  // v0.34: Unpacking operator for lists

// v0.38: Lambda expressions with full support in collections
lambda_expr: 'lambda' param_list? ':' expr
param_list: IDENTIFIER (',' IDENTIFIER)*

fsl_expr: fsl_conversion | fsl_property | fsl_method  // v0.33: Frame Standard Library
fsl_conversion: ('str' | 'int' | 'float' | 'bool') '(' expr ')'
fsl_property: expr '.' ('length' | 'is_empty' | 'name' | 'value')
fsl_list_method: expr '.' ('append' | 'pop' | 'clear' | 'insert' | 'remove' | 'extend' | 
                           'reverse' | 'sort' | 'copy' | 'index' | 'count') '(' arg_list? ')'
fsl_string_method: expr '.' ('trim' | 'upper' | 'lower' | 'replace' | 'split' | 
                             'contains' | 'substring') '(' arg_list? ')'
```

**Action Call Syntax**: Action calls use underscore prefix syntax `_actionName()` to distinguish them from interface method calls. This generates with proper `self._actionName()` syntax in Python target language.

### Index Operations (Full Support in v0.38)

✅ **Frame v0.38 has full support for nested dictionary and list indexing operations**.

#### Current Support (v0.38)

The parser now fully supports nested index operations including chained bracket notation:

**All patterns now working**:
```frame
// Simple indexing:
var item = mylist[0]                   // ✅ Simple list indexing
var value = self.data[index]           // ✅ Self variable indexing  
var element = array[i]                 // ✅ Variable as index

// Nested/chained indexing (NEW in v0.38):
matrix[i][j] = value                   // ✅ Chained indexing
dict["key1"]["key2"] = value           // ✅ Nested dictionary access
config["database"]["port"] = 3306      // ✅ Deep nesting
var val = data[section][field]         // ✅ Variable keys

// Complex expressions:
self.results[str(task_id)] = value     // ✅ Function call in index
dict["key"] = value                    // ✅ Literal keys
```

**Still unsupported patterns**:
```frame
// Method call followed by indexing:
var item = getArray()[0]               // ❌ Index after method call
```

#### Parser Implementation Details (v0.38)

The v0.38 parser enhancement handles consecutive bracket operations by:
1. Detecting multiple `[` tokens after an identifier
2. Creating synthetic `@chain_index` nodes for each bracket pair
3. Building a proper call chain with these synthetic nodes
4. The visitor recognizes synthetic nodes and generates correct Python without extra dots

This provides full support for nested dictionary and list operations.

#### Negative Indexing

For lists, negative indexing works in simple cases or with backticks:

```frame
var items = [10, 20, 30, 40, 50]
var last = items[-1]            // May work in v0.37
var last = items`[-1]`          // Always works: 50
var second_last = items`[-2]`   // Second to last: 40
```

#### Slicing Operations (v0.37) ✅

Frame now supports Python-style slicing for strings and lists:

```frame
fn slicingExamples() {
    var text = "Hello, World!"
    var numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    
    // Basic slicing
    var first5 = text[:5]          // "Hello"
    var last6 = text[7:]           // "World!"
    var middle = text[2:8]         // "llo, W"
    
    // List slicing
    var firstHalf = numbers[:5]    // [0, 1, 2, 3, 4]
    var secondHalf = numbers[5:]   // [5, 6, 7, 8, 9]
    var subset = numbers[3:7]      // [3, 4, 5, 6]
    
    // Step parameter
    var everyOther = numbers[::2]  // [0, 2, 4, 6, 8]
    var reversed = numbers[::-1]   // [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    var skipTwo = numbers[1:8:2]   // [1, 3, 5, 7]
}
```

**Slice Notation**:
- `[start:end]` - Basic slice from start (inclusive) to end (exclusive)
- `[:end]` - From beginning to end
- `[start:]` - From start to end of sequence
- `[start:end:step]` - With step parameter
- `[::step]` - Whole sequence with step
- `[::-1]` - Reverse the sequence

**Note**: Full native support for all index operation patterns is planned for future Frame versions.

### Operators (v0.38)

Frame v0.38 aligns with Python's operator syntax, replacing C-style operators with Python keywords:

#### Logical Operators
```frame
// Boolean AND (replaces &&)
if x > 0 and y > 0 {
    print("Both positive")
}

// Boolean OR (replaces ||)
if name == "admin" or hasPermission {
    allowAccess()
}

// Boolean NOT (replaces !)
if not isValid {
    print("Invalid input")
}

// Complex expressions
if (a and b) or (not c and d) {
    processData()
}
```

#### Membership Operators (NEW in v0.38)
```frame
// Check if item in collection
var fruits = ["apple", "banana", "orange"]
if "banana" in fruits {
    print("Found banana!")
}

// Check if key in dictionary
var config = {"debug": true, "port": 8080}
if "debug" in config {
    print("Debug mode: " + str(config["debug"]))
}

// Not in operator
if "grape" not in fruits {
    fruits.append("grape")
}

// Works with strings too
if "world" in "hello world" {
    print("Found substring")
}
```

#### Arithmetic Operators
```frame
# Standard arithmetic
var sum = a + b
var diff = a - b
var product = a * b
var quotient = a / b       # Regular division
var floor_div = a // b     # Floor division (NEW in v0.40)
var remainder = a % b

# Exponent operator (v0.38)
var square = x ** 2
var cube = x ** 3
var power = 2 ** 10  # 1024

# Right associativity for exponent
var tower = 2 ** 3 ** 2  # 512 (evaluates as 2 ** (3 ** 2))

# Floor division examples (v0.40)
var result = 10 // 3      # 3
var negative = -10 // 3   # -4 (Python floor division semantics)
```

#### Comparison Operators
```frame
// All comparison operators unchanged
if x == y { }  // Equal
if x != y { }  // Not equal
if x < y { }   // Less than
if x > y { }   // Greater than
if x <= y { }  // Less than or equal
if x >= y { }  // Greater than or equal
```

#### Breaking Changes from v0.37
- **Removed**: `&&`, `||`, `!` operators no longer supported
- **Migration**: Replace `&&` with `and`, `||` with `or`, `!` with `not`
- **Error Messages**: Scanner provides clear migration guidance for old operators

### Operators (v0.39 - NEW)

Frame v0.39 completes the Python operator alignment by adding compound assignments, bitwise operators, and identity operators:

#### Compound Assignment Operators
```frame
# Arithmetic compound assignments
var x = 10
x += 5       # x = x + 5 → 15
x -= 3       # x = x - 3 → 12
x *= 2       # x = x * 2 → 24
x /= 4       # x = x / 4 → 6
x //= 3      # x = x // 3 → 2 (floor division, NEW in v0.40)
x %= 4       # x = x % 4 → 2
x **= 3      # x = x ** 3 → 8

// Bitwise compound assignments
var a = 12   // 1100 in binary
a &= 5       // a = a & 5 → 4 (0100)
a |= 8       // a = a | 8 → 12 (1100)
a <<= 2      // a = a << 2 → 48
a >>= 1      // a = a >> 1 → 24

// Works with collections
var list1 = [1, 2, 3]
var list2 = [4, 5, 6]
list1 += list2  // list1 = [1, 2, 3, 4, 5, 6]

var text = "Hello"
text += " World"  // text = "Hello World"
```

#### Bitwise Operators
```frame
// Bitwise NOT (unary)
var x = 7     // 0111 in binary
var y = ~x    // -8 (two's complement)

// Bitwise AND
var a = 5     // 0101
var b = 3     // 0011
var c = a & b // 0001 = 1

// Bitwise OR (already existed for dict union, now also bitwise)
var d = a | b // 0111 = 7

// Left shift
var e = 5 << 2  // 20 (shift left by 2 positions)

// Right shift
var f = 20 >> 2 // 5 (shift right by 2 positions)

// Practical examples
var flags = 0b1010
if flags & 0b0010 {  // Check if second bit is set
    print("Flag 2 is set")
}

var mask = 0xFF
var lowByte = value & mask  // Extract low byte
```

#### Identity Operators
```frame
// 'is' operator - checks object identity
var x = None
if x is None {
    print("x is None")
}

// 'is not' operator - negated identity check
var y = 42
if y is not None {
    print("y has a value")
}

// Identity vs equality
var list1 = [1, 2, 3]
var list2 = [1, 2, 3]
var list3 = list1

// Equality checks values
if list1 == list2 {  // True - same values
    print("Equal values")
}

// Identity checks if same object
if list1 is list2 {  // False - different objects
    print("Same object")
}

if list1 is list3 {  // True - same object reference
    print("list1 and list3 are the same object")
}
```

#### Operator Precedence (Python-aligned)
Frame follows Python's operator precedence (highest to lowest):
1. `**` (exponentiation - right associative)
2. `~` (bitwise NOT), unary `-`, `not`
3. `*`, `/`, `%`
4. `+`, `-`
5. `<<`, `>>` (bitwise shifts)
6. `&` (bitwise AND)
7. `|` (bitwise OR)
8. `in`, `not in`, `is`, `is not`, `<`, `<=`, `>`, `>=`, `!=`, `==`
9. `and`
10. `or`
11. `=`, `+=`, `-=`, etc. (assignments - right associative)

### Lambda Expressions (v0.38)

Frame v0.38 has full support for Python-style lambda expressions, including use in collection literals:

#### Basic Lambda Syntax
```frame
// Simple lambda
var square = lambda x: x * x

// Multi-parameter lambda
var add = lambda a, b: a + b

// No-parameter lambda
var get_value = lambda: 42

// Using lambdas
print(str(square(5)))      // 25
print(str(add(3, 4)))      // 7
print(str(get_value()))    // 42
```

#### Lambda Assignment (FIXED in v0.38)
```frame
// Variable declaration with lambda
var square = lambda x: x * x

// Assignment to existing variable (now works!)
var func = lambda x: x + 1
func = lambda x: x * 2     // ✅ Reassignment now works
```

#### Lambdas in Collections (FULLY SUPPORTED in v0.38)
```frame
// Dictionary with lambda values
var operations = {
    "add": lambda x, y: x + y,
    "subtract": lambda x, y: x - y,
    "multiply": lambda x, y: x * y,
    "divide": lambda x, y: x / y
}

// Using dictionary lambdas
var result = operations["add"](10, 5)  // 15

// List of lambda functions
var transforms = [
    lambda n: n + 1,
    lambda n: n * 2,
    lambda n: n ** 2
]

// Using list lambdas
var value = transforms[0](5)  // 6
```

#### Lambda Closures
```frame
// Lambda capturing outer variables
var multiplier = 10
var scale = lambda x: x * multiplier

print(str(scale(5)))  // 50

// Function returning lambda
fn make_adder(n) {
    return lambda x: x + n
}

var add5 = make_adder(5)
print(str(add5(10)))  // 15
```

#### Lambda as Function Arguments
```frame
fn apply_operation(func, a, b) {
    return func(a, b)
}

var result = apply_operation(lambda x, y: x + y, 10, 20)  // 30
```

### Collection Constructors and Literals (v0.38)

Frame supports both literal syntax and constructor functions for creating collections:

#### Collection Literals
```frame
// Dictionary literal
var dict = {"key": "value", "count": 42}

// Set literal  
var set = {1, 2, 3}

// Empty set literal (v0.38)
var empty_set = {,}

// List literal
var list = [1, 2, 3]

// Tuple literal
var tuple = (1, 2, 3)
```

#### Collection Constructors (FIXED in v0.38)
```frame
// Set constructor - multiple arguments are automatically wrapped in a list
var s1 = set(1, 2, 3)              // Transpiles to: set([1, 2, 3])
var s2 = set([1, 2, 3])            // Also valid, passes list directly
var s3 = set()                     // Empty set

// List constructor
var l1 = list([1, 2, 3])           // Pass iterable
var l2 = list(range(10))           // From range
var l3 = list()                    // Empty list

// Tuple constructor  
var t1 = tuple([1, 2, 3])          // From list
var t2 = tuple()                   // Empty tuple

// Dict constructor
var d1 = dict([("key", "value")])  // From list of tuples
var d2 = dict()                    // Empty dict
```

**Parser Note**: The Frame transpiler automatically handles Python's requirement that collection constructors take a single iterable argument. When multiple arguments are provided to `set()`, `list()`, or `tuple()`, they are wrapped in a list for valid Python generation.

#### Nested Collections with Lambdas
```frame
// Complex data structure with lambdas
var config = {
    "validators": {
        "positive": lambda x: x > 0,
        "even": lambda x: x % 2 == 0
    },
    "formatters": [
        lambda s: s.upper(),
        lambda s: s.lower()
    ]
}

// Using nested lambdas
if config["validators"]["positive"](5) {
    print("Value is positive")
}
```

**Self Expression (v0.31)**: The `self` keyword can be used as a standalone expression (e.g., as a function argument) or with dotted access to reference instance members. Static methods cannot use `self` in any form.

**Call Chain Support**: Multi-node call chains like `sys.methodName()` correctly generate interface method calls on system instances without adding action prefixes.

**Frame Standard Library (v0.33)**: FSL provides native built-in operations guaranteed across all target languages, eliminating the need for backticks when using common operations like type conversions and collection methods.

## Frame Standard Library (FSL) - v0.34

The Frame Standard Library provides native built-in operations that work consistently across all target languages. 

**v0.34 Changes (FULLY IMPLEMENTED)**: 
- ✅ FSL operations require explicit import to prevent namespace conflicts
- ✅ FSL imports filtered from generated Python code (built into Python)
- ✅ Without import, operations treated as external function calls
- ✅ Seamless integration with new module system
- ✅ All FSL features work with qualified module names

```frame
// Import specific FSL operations
from fsl import str, int, float, bool

// Define modules that use FSL
module converter {
    fn toString(value) {
        return str(value)  // Uses FSL str() conversion
    }
    
    fn toNumber(text) {
        return int(text)   // Uses FSL int() conversion
    }
}

// Use FSL with qualified names
fn main() {
    var text = converter.toString(42)    // Module function using FSL
    var num = converter.toNumber("123")  // Module function using FSL
    var result = float("3.14")          // Direct FSL usage
}

// Without FSL import, all operations treated as external
fn withoutImport() {
    var s = str(42)  // Calls external str() function (not FSL)
}
```

**Test Results**: All FSL integration tests passing with 100% success rate.

### Phase 1: Type Conversion Operations ✅
```frame
fn example() {
    var x = 42
    var s = str(x)        // Convert to string: "42"
    var i = int("123")    // Convert to integer: 123
    var f = float("3.14") // Convert to float: 3.14
    var b = bool(0)       // Convert to boolean: false
}
```

### Phase 2: List Operations ✅

#### List Methods
```frame
fn listOperations() {
    var items = [1, 2, 3]
    
    // Basic operations
    items.append(4)           // Add to end: [1, 2, 3, 4]
    var last = items.pop()    // Remove and return last: 4
    items.clear()            // Remove all elements: []
    
    // Advanced operations
    items = [1, 2, 3, 4]
    items.insert(2, 99)      // Insert at index: [1, 2, 99, 3, 4]
    items.remove(99)         // Remove first occurrence: [1, 2, 3, 4]
    items.reverse()          // Reverse in place: [4, 3, 2, 1]
    items.sort()             // Sort in place: [1, 2, 3, 4]
    
    // Query operations
    var idx = items.index(3) // Find index: 2
    var cnt = items.count(2) // Count occurrences: 1
    
    // Copying
    var copy = items.copy()  // Shallow copy
    
    // Extending
    items.extend([5, 6])     // Add all from another list: [1, 2, 3, 4, 5, 6]
}
```

#### List Properties
```frame
fn listProperties() {
    var items = [1, 2, 3]
    var len = items.length    // Get list length: 3 (converts to len() in Python)
    
    var empty = []
    var is_empty = empty.is_empty  // Check if empty: true (converts to len() == 0)
}
```

#### List Comprehensions ✅ (v0.34)
```frame
fn listComprehensions() {
    // Basic list comprehension
    var squares = [x * x for x in range(10)]
    // Result: [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
    
    // With conditional filtering
    var numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    var evens = [x for x in numbers if x % 2 == 0]
    // Result: [2, 4, 6, 8, 10]
    
    // Nested comprehensions
    var matrix = [[i * j for j in range(3)] for i in range(3)]
    // Result: [[0, 0, 0], [0, 1, 2], [0, 2, 4]]
    
    // With complex expressions
    var words = ["hello", "world", "frame"]
    var uppercased = [word.upper() for word in words]
    // Result: ["HELLO", "WORLD", "FRAME"]
    
    // Combining with string operations
    var names = ["alice", "bob", "charlie"]
    var greetings = ["Hello " + name for name in names if len(name) > 3]
    // Result: ["Hello alice", "Hello charlie"]
}
```

#### Unpacking Operator ✅ (v0.34)
```frame
fn listUnpacking() {
    var list1 = [1, 2, 3]
    var list2 = [4, 5, 6]
    
    // Unpacking in list literals
    var combined = [*list1, *list2, 7, 8]
    // Result: [1, 2, 3, 4, 5, 6, 7, 8]
    
    // Multiple unpacking operations
    var a = [10, 20]
    var b = [30, 40]
    var c = [50, 60]
    var result = [0, *a, *b, *c, 70]
    // Result: [0, 10, 20, 30, 40, 50, 60, 70]
    
    // Unpacking with other expressions
    var base = [100, 200, 300]
    var extended = [50, *base, 400, 500]
    // Result: [50, 100, 200, 300, 400, 500]
}
```

#### Negative Indexing ✅
```frame
fn negativeIndexing() {
    var items = [10, 20, 30, 40, 50]
    var last = items[-1]      // Last element: 50
    var second_last = items[-2] // Second to last: 40
    items[-1] = 99           // Set last element: [10, 20, 30, 40, 99]
}
```

### Phase 3: String Operations ✅

#### Fully Supported String Methods
```frame
fn stringOperations() {
    var text = "  Hello World  "
    
    // Case conversion
    var upper = text.upper()     // "  HELLO WORLD  "
    var lower = text.lower()     // "  hello world  "
    
    // String manipulation
    var trimmed = text.trim()    // "Hello World" (converts to strip() in Python)
    var replaced = text.replace("World", "Frame")  // "  Hello Frame  "
    var parts = text.split(" ")  // ["", "", "Hello", "World", "", ""]
}
```

#### String Properties
```frame
fn stringProperties() {
    var text = "Hello"
    var len = text.length        // Get string length: 5 (converts to len() in Python)
}
```

#### Pending String Operations
The following operations are recognized by FSL but require additional visitor implementation:
- `contains(substring)` - Will transform to Python's `in` operator
- `substring(start, end)` - Will transform to Python's slice syntax `[start:end]`

### Enum Properties (v0.32) ✅
```frame
enum Status { Active, Inactive }

fn enumExample() {
    var s = Status.Active
    var name = s.name   // "Active"
    var value = s.value // 0
}
```

**Implementation Status:**
- ✅ Phase 1: Type conversions (str, int, float, bool) - Complete
- ✅ Phase 2: List operations and properties - Complete
- ⚠️ Phase 3: String operations - Partial (trim, upper, lower, replace, split working)
- 📋 Phase 4: Additional string operations (contains, substring) - Planned

## Comments (v0.40 Breaking Change)

**v0.40 Update**: Frame now uses Python-style comments to enable the floor division operator (`//`).

### Python-style Single-line Comments (v0.40)
```frame
# This is a Python-style comment
var x = 42  # Comment at end of line

# Multiple consecutive comments
# Line 1
# Line 2
# Line 3
```

### Frame Documentation Comments
```frame
{-- This is a Frame documentation comment
    It can span multiple lines
    and is typically used for documentation --}
```

### Migration from v0.39
- **Removed**: `//` single-line comments (now floor division)
- **Removed**: `/* */` C-style multiline comments
- **Added**: `#` Python-style single-line comments
- **Retained**: `{-- --}` Frame documentation comments

**Comment Rules**:
- Comments are preserved during parsing but not included in generated code
- `#` comments extend to end of line
- Frame documentation comments can span multiple lines
- Comments can appear anywhere whitespace is allowed

## Tokens

```bnf
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*
NUMBER: [0-9]+ ('.' [0-9]+)?
STRING: '"' (ESC | ~["])* '"'
SUPERSTRING: '`' ~[`]* '`' | '```' ~* '```'
COMMENT: '#' ~[\n]* | '{--' ~* '--}'
```

## Keywords

```
system interface machine actions operations domain
fn var return
if elif else for while loop in break continue
true false None
async await
```

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
   - **REMOVED**: `^=`
   - Use: `return = value`

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

## Target Languages

Frame transpiles to multiple target languages with full feature support:

### Currently Implemented
- **Python 3**: Primary target with 100% feature coverage
- **Graphviz**: For state machine visualization

### Planned Implementation
- **Rust**: First-class support with ownership inference and zero-cost abstractions
- **JavaScript/TypeScript**: ES6 modules and class-based systems
- **C#**: Full .NET integration with namespaces
- **Java**: Package-based module system
- **Go**: Struct-based systems with interfaces
- **C**: Function prefixing for module simulation
- **C++**: Class-based with namespace support

### Target Language Feature Matrix

| Feature | Python | Rust | JS/TS | C# | Java | Go | C |
|---------|--------|------|-------|-----|------|----|----|
| Modules | ✅ | ✅ mod | ✅ ES6 | ✅ namespace | ✅ package | ✅ package | 🔄 prefix |
| Systems | ✅ | ✅ struct | ✅ class | ✅ class | ✅ class | ✅ struct | ✅ struct |
| FSL | ✅ | ✅ crate | ✅ runtime | ✅ runtime | ✅ runtime | ✅ runtime | ✅ runtime |
| Enums | ✅ | ✅ enum | ✅ object | ✅ enum | ✅ enum | ✅ const | 🔄 define |
| Async | ✅ | ✅ tokio | ✅ promise | ✅ async | ✅ future | ✅ goroutine | ❌ |
| Generics | ✅ | ✅ native | ✅ TS | ✅ native | ✅ native | ✅ native | ❌ |

Legend:
- ✅ Full support
- 🔄 Requires transformation
- ❌ Not supported

