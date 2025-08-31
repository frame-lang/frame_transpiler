# Frame Language Grammar (v0.31)

This document provides the formal grammar specification for the Frame language using BNF notation, along with examples for each language construct.

## Module Structure

```bnf
module: (import_stmt | function | system)*
```

**v0.31 Import Support**: Modules can now include native import statements at the top level, supporting Python module imports without requiring backticks.

**v0.30 Multi-Entity Support**: Modules can contain any combination of functions and systems in any order. Each entity (function or system) can have individual attributes.

## Import Statements (v0.31)

Frame v0.31 introduces native import statement support, primarily targeting Python but designed for future multi-language support.

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
        var msg = nil
        
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
        var E = nil
        var F = nil
    
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
interface_method: IDENTIFIER '(' parameter_list? ')' type?
```

## Machine Block

```bnf
machine_block: 'machine:' state*
state: '$' IDENTIFIER ('=>' '$' IDENTIFIER)? '{' event_handler* state_var* '}'
event_handler: event_selector '{' stmt* terminator? '}'
event_selector: IDENTIFIER '(' parameter_list? ')' type?
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
domain_block: 'domain:' domain_var*
domain_var: 'var' IDENTIFIER type? '=' expr
```

## Operations Block

```bnf
operations_block: 'operations:' operation*
operation: attribute* IDENTIFIER '(' parameter_list? ')' type? '{' stmt* '}'
attribute: '@' IDENTIFIER  // Python-style attributes (e.g., @staticmethod)
```

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
action: IDENTIFIER '(' parameter_list? ')' type? action_body
action_body: '{' stmt* '}'
parameter_list: parameter (',' parameter)*
parameter: IDENTIFIER type?
type: ':' IDENTIFIER
```

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

### Interface Return Assignment

Frame v0.20 introduces the `return = expr` syntax for setting interface return values anywhere within event handlers or action methods:

```frame
// Setting interface return values in event handlers
machine:
    $ProcessingState {
        validateInput(data: string): bool {
            if data == "" {
                return = false  // Set interface return value
                return          // Exit event handler  
            }
            
            if checkFormat(data) {
                return = true   // Set interface return value
                return          // Exit event handler
            }
            
            return = false      // Default case
            return
        }
    }

// Setting interface return values in action methods
actions:
    processData(input: string): string {
        if input == "error" {
            return = "failed"   // Set interface return value
            return "internal"   // Return value to caller (action method)
        }
        
        return = "success"      // Set interface return value
        return input            // Return value to caller (action method)
    }
```

## Expressions

```bnf
expr: binary_expr | unary_expr | primary_expr | call_expr | self_expr

binary_expr: expr operator expr
operator: '+' | '-' | '*' | '/' | '%'
        | '==' | '!=' | '<' | '>' | '<=' | '>='
        | '&&' | '||'

unary_expr: ('-' | '!' | '~') expr

primary_expr: IDENTIFIER | NUMBER | STRING | SUPERSTRING
            | 'true' | 'false' | 'nil'
            | '(' expr ')' | '@'

self_expr: 'self' | 'self' '.' IDENTIFIER  // v0.31: self as standalone or dotted access

call_expr: IDENTIFIER '(' arg_list? ')' | '_' IDENTIFIER '(' arg_list? ')'
arg_list: expr (',' expr)*
```

**Action Call Syntax**: Action calls use underscore prefix syntax `_actionName()` to distinguish them from interface method calls. This generates with proper `self._actionName()` syntax in Python target language.

**Self Expression (v0.31)**: The `self` keyword can be used as a standalone expression (e.g., as a function argument) or with dotted access to reference instance members. Static methods cannot use `self` in any form.

**Call Chain Support**: Multi-node call chains like `sys.methodName()` correctly generate interface method calls on system instances without adding action prefixes.

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
true false nil
```

## Deprecated Features (v0.11 → v0.20)

The following syntax from Frame v0.11 is deprecated in v0.20:

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

5. **Return token**: 
   - Old: `^` and `^(value)`
   - New: `return` and `return value`

6. **Parameter lists**: 
   - Old: `[param1, param2]`
   - New: `(param1, param2)`

7. **Event selectors**: 
   - Old: `|eventName|`
   - New: `eventName()`

8. **Function declaration**: 
   - Old: `fn main {`
   - New: `fn main() {`

9. **Enter/Exit events**:
   - Old: `|>|` and `|<|`
   - New: `$>()` and `<$()`

10. **Event forwarding to parent**:
   - Old: `:>` (v0.11-v0.19), `@:>` (early v0.20)
   - New: `=> $^` (v0.20)

11. **Attributes**:
   - Old: `#[static]` (Rust-style)
   - New: `@staticmethod` (Python-style)

12. **Current event reference**:
   - Old: `@` for current event
   - New: `$@` for current event (single `@` now reserved for attributes)

13. **Empty parameter lists**:
   - Old: v0.11 rejected `()` in certain parsing contexts
   - New: v0.20 fully supports empty parameter lists `()` in all method calls, interface declarations, and event handlers

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

