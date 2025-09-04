# Frame Language Grammar (v0.37)

**Last Updated**: 2025-09-04  
**Status**: Event-handlers-as-functions architecture implemented with 99.5% test success rate (211/212 tests passing).

This document provides the formal grammar specification for the Frame language using BNF notation, along with examples for each language construct.

## Module Structure

```bnf
module: (import_stmt | module_decl | enum_decl | var_decl | function | system)*

module_decl: 'module' IDENTIFIER '{' module_content '}'
module_content: (module_decl | enum_decl | var_decl | function | system)*
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

### Current Implementation Status

**✅ Implemented Features**:
- Async function declarations (`async fn name() { }`)
- Async interface method declarations (`async methodName()`)
- Await expression parsing (`await expression`)
- Async function code generation (Python `async def`)
- Async interface method code generation
- State handler async propagation (when handling async interface events)

**⚠️ Architectural Limitation**:
Frame's event-driven state machine runtime is synchronous by design. While async interface methods and functions work correctly, complex async state handlers with await expressions may require runtime architecture changes for full compatibility.

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
event_handler: event_selector '{' stmt* terminator? '}'
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

## Statements

```bnf
stmt: expr_stmt
    | var_decl
    | assignment
    | if_stmt
    | for_stmt
    | while_stmt
    | loop_stmt
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
assignment: lvalue '=' expr
return_stmt: 'return' expr?
return_assign_stmt: 'return' '=' expr
parent_dispatch_stmt: '=>' '$^'
transition_stmt: '->' '$' IDENTIFIER
state_stack_op: '$$[' '+' ']' | '$$[' '-' ']'
block_stmt: '{' stmt* '}'
break_stmt: 'break'
continue_stmt: 'continue'
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
    | list_expr | list_comprehension | unpack_expr  // v0.34: Added list features

binary_expr: expr operator expr
operator: '+' | '-' | '*' | '/' | '%'
        | '==' | '!=' | '<' | '>' | '<=' | '>='
        | '&&' | '||'

unary_expr: ('-' | '!' | '~') expr

primary_expr: IDENTIFIER | NUMBER | STRING | SUPERSTRING
            | 'true' | 'false' | 'None'
            | '(' expr ')' | '@'

self_expr: 'self' | 'self' '.' IDENTIFIER  // v0.31: self as standalone or dotted access

call_expr: IDENTIFIER '(' arg_list? ')' | '_' IDENTIFIER '(' arg_list? ')'
arg_list: expr (',' expr)*

// v0.34: List expressions and comprehensions
list_expr: '[' list_elements? ']'
list_elements: list_element (',' list_element)*
list_element: expr | unpack_expr

list_comprehension: '[' expr 'for' IDENTIFIER 'in' expr ('if' expr)? ']'

unpack_expr: '*' expr  // v0.34: Unpacking operator for lists

fsl_expr: fsl_conversion | fsl_property | fsl_method  // v0.33: Frame Standard Library
fsl_conversion: ('str' | 'int' | 'float' | 'bool') '(' expr ')'
fsl_property: expr '.' ('length' | 'is_empty' | 'name' | 'value')
fsl_list_method: expr '.' ('append' | 'pop' | 'clear' | 'insert' | 'remove' | 'extend' | 
                           'reverse' | 'sort' | 'copy' | 'index' | 'count') '(' arg_list? ')'
fsl_string_method: expr '.' ('trim' | 'upper' | 'lower' | 'replace' | 'split' | 
                             'contains' | 'substring') '(' arg_list? ')'
```

**Action Call Syntax**: Action calls use underscore prefix syntax `_actionName()` to distinguish them from interface method calls. This generates with proper `self._actionName()` syntax in Python target language.

### Index Operations (Limited Support)

⚠️ **Important**: Frame currently has limited support for index operations (subscript notation with square brackets).

#### Current Limitations

Frame's parser does not fully support native index operations like `array[index]` or `dict[key]`. When such syntax is used, the parser may incorrectly interpret it as separate expressions, leading to malformed generated code.

**Problematic patterns**:
```frame
// These patterns may generate incorrect code:
self.results[str(task_id)] = value     // May split across lines
var item = array[index]                // May not parse correctly
dict[key] = new_value                  // May generate invalid syntax
```

#### Workarounds

For dictionary and list access, use backtick expressions:

```frame
// Use backticks for index operations:
var urls = self.config`["urls"]`       // Dictionary access
var item = self.items`[0]`             // List access by index
self.data`[str(key)]` = value          // Dictionary assignment

// For complex operations, use full backtick blocks:
`
    self.results[str(task_id)] = value
    item = my_list[index]
    my_dict[key] = new_value
`
```

#### Negative Indexing

For lists, negative indexing works within backtick expressions:

```frame
var items = [10, 20, 30, 40, 50]
var last = items`[-1]`          // Last element: 50
var second_last = items`[-2]`   // Second to last: 40
```

**Note**: This is a known limitation in Frame v0.37. Future versions may add full native support for index operations.

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

## Tokens

```bnf
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*
NUMBER: [0-9]+ ('.' [0-9]+)?
STRING: '"' (ESC | ~["])* '"'
SUPERSTRING: '`' ~[`]* '`' | '```' ~* '```'
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

