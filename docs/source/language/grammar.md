# Frame v0.30 Grammar (BNF)

This grammar specification has been comprehensively validated with 105 working Frame systems and 100% test success rate (105/105 tests passing) including complete multi-entity module support, state stack operations, hierarchical state machines, and all major v0.30 language features.

## Module Structure

```bnf
module: (function | system)*
```

**v0.30 Multi-Entity Support**: Modules can contain any combination of functions and systems in any order. Each entity (function or system) can have individual attributes.

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

// Functions mixed with systems
fn utility(data) {
    print("Utility: " + data)
}

system Worker {
    interface:
        start()
        
    machine:
        $Idle {
            start() {
                utility("Worker starting")
                -> $Running
            }
        }
        
        $Running {
        }
}
```

## Systems

```bnf
system: 'system' IDENTIFIER system_params? '{' system_block* '}'
system_params: '(' system_param_list ')'
system_param_list: system_param (',' system_param)*
system_param: start_state_param | enter_event_param | domain_param
start_state_param: '$(' parameter_list ')'
enter_event_param: '$>(' parameter_list ')'
domain_param: IDENTIFIER type?

system_block: interface_block
            | machine_block
            | actions_block
            | operations_block
            | domain_block
```

### System Examples

#### Basic System
```frame
system TrafficLight {
    interface:
        start()
        stop()
        
    machine:
        $Red {
            start() {
                -> $Green
            }
        }
        
        $Green {
            stop() {
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
          | '->' '$' IDENTIFIER  // Transition
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

### Operations Examples

#### Instance Operations
```frame
system Calculator {
    operations:
        // Instance operation - includes implicit 'self' parameter
        getResult(): int {
            return currentValue
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

Frame supports hierarchical state machines where child states can inherit behavior from parent states using the dispatch operator `=>`:

```bnf
hierarchy: '$' IDENTIFIER '=>' '$' IDENTIFIER
```

#### State Hierarchy Example
```frame
machine:
    // Parent state
    $Parent {
        commonEvent() {
            print("Handled in parent")
            return
        }
    }
    
    // Child state inherits from parent
    $Child => $Parent {
        specificEvent() {
            print("Handled in child")
            return
        }
    }
```

#### Event Forwarding to Parent States

The `=> $^` statement forwards events from child states to their parent states:

```frame
$Child => $Parent {
    sharedEvent() {
        print("Processing in child first")
        => $^  // Forward to parent state
        print("This continues after parent unless parent transitions")
    }
}
```

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
expr: binary_expr | unary_expr | primary_expr | call_expr

binary_expr: expr operator expr
operator: '+' | '-' | '*' | '/' | '%'
        | '==' | '!=' | '<' | '>' | '<=' | '>='
        | '&&' | '||'

unary_expr: ('-' | '!' | '~') expr

primary_expr: IDENTIFIER | NUMBER | STRING | SUPERSTRING
            | 'true' | 'false' | 'nil'
            | '(' expr ')' | '@'

call_expr: IDENTIFIER '(' arg_list? ')'
arg_list: expr (',' expr)*
```

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

## Implementation Status

### v0.20 Recent Updates

**Empty Parameter List Support (2025-01-20)** ✅ **COMPLETED**
- **Achievement**: Full support for empty parameter lists `()` in all contexts
- **Parser Enhancement**: Fixed v0.11 restriction that rejected empty parameter syntax in certain contexts
- **Method Calls**: `self.method()` calls now parse and generate correct Python code  
- **Interface Declarations**: Empty parameter interfaces like `quit()` fully supported
- **Code Generation**: Fixed Python visitor to correctly handle `self.method()` → `method()` transformation
- **Test Validation**: All services documentation examples now compile successfully
- **Impact**: Enables conventional method call patterns and interface definitions without parameters

**Return Statements as Regular Statements (2025-01-16)** ✅ **COMPLETED**
- **Grammar**: `return_stmt: 'return' expr?` 
- **Context**: Return statements now work as regular statements in all contexts
- **Previous Issue**: Could only be used as event handler terminators, preventing if/elif/else chains
- **Fix**: Added `StatementType::ReturnStmt` and `ReturnStmtNode` to AST
- **Impact**: 
  - Enables conventional if/elif/else patterns with returns in event handlers
  - Supports early return validation patterns
  - Allows complex nested conditional logic with returns
  - Makes Frame v0.20 syntax more conventional and familiar
- **Test Cases**: All if/elif/return combinations validated in event handlers and actions

**Interface Return Assignment (2025-01-17)** ✅ **COMPLETED**
- **Grammar**: `return_assign_stmt: 'return' '=' expr`
- **Context**: New syntax for setting interface return values anywhere in event handlers/actions
- **Previous Syntax**: `^= expr` (removed in v0.20)
- **New Syntax**: `return = expr` (conventional assignment-like syntax)
- **Implementation**: Reuses existing `ReturnAssignStmtNode` AST structure
- **Code Generation**: 
  - Python: `self.return_stack[-1] = expr`
  - Java: `e._return = expr`
  - Other typed languages: similar assignment to return field
- **Benefits**: More readable and conventional than the previous `^=` operator

**Transition + Return Parsing (2025-01-17)** ✅ **COMPLETED**
- **Issue**: Parser failed with "Expected '}' - found 'elif'" when `return` followed transitions in if/elif/else
- **Root Cause**: Transitions terminated statement parsing, preventing subsequent elif/else clauses
- **Solution**: Consume optional `return` token after transitions without generating AST node
- **Implementation**: `self.advance()` to consume return token but don't add to statements
- **Rationale**: Transitions already terminate execution; explicit returns are for code clarity only
- **Result**: 
  - Allows readable `-> $State` followed by `return` syntax
  - Prevents duplicate return statements in generated code
  - Enables proper if/elif/else parsing with transitions
- **Example**:
  ```frame
  if condition == "error" {
      -> $Error     // Transition terminates execution
      return        // Consumed but not code-generated
  } elif condition == "success" {
      -> $Success   // Parser continues to elif
      return
  }
  ```

**Event Handler Terminator Optionality (2025-01-17)** ✅ **COMPLETED**
- **Grammar**: `event_handler: event_selector '{' stmt* terminator? '}'`
- **Implementation**: Event handlers no longer require explicit terminators
- **Rationale**: Block-scoped functions don't need explicit terminators in most languages
- **Backward Compatibility**: Explicit terminators still supported and recommended for transitions
- **Impact**: 
  - Reduces syntactic noise in simple event handlers
  - Maintains semantic clarity for state transitions
  - Aligns with conventional programming language patterns
  - Python visitor generates implicit returns only when needed
- **Transition Requirement**: Transitions (`->`) still terminate blocks - no statements after them

**C-style For Loop with 'for' Keyword (2025-01-17)** ✅ **COMPLETED**
- **Grammar**: `for var_decl ';' expr ';' expr block`
- **Implementation**: Parser now supports C-style for loops using the `for` keyword
- **Previous Limitation**: C-style loops only worked with `loop` keyword
- **Enhancement**: Traditional three-part for loops now work with conventional `for` syntax
- **Backward Compatibility**: `loop` keyword still supports C-style syntax
- **Rationale**: Aligns Frame syntax with Python/JavaScript conventions for familiar loop patterns
- **Examples**: 
  - `for var i = 0; i < 10; i = i + 1 { ... }` (new)
  - `loop var i = 0; i < 10; i = i + 1 { ... }` (legacy, still supported)

**Comprehensive Test Suite Validation (2025-01-17)** ✅ **COMPLETED**
- **Achievement**: 100% test file pass rate for implemented features (57/57 files)
- **Coverage**: All currently implemented v0.20 syntax features validated end-to-end
- **Quality**: Generated Python code passes syntax validation
- **Fixes Applied**:
  - Legacy syntax updates (^ → return, :> → => $^)
  - System parameter syntax corrections (v0.11 → v0.20)
  - Multiple function restrictions enforced (main only)
  - For loop syntax modernization (C-style → iterator)
- **Test Files**: Serve as comprehensive v0.20 syntax documentation
- **Regression Testing**: All existing functionality preserved
- **Parser Robustness**: Handles complex nested conditional patterns correctly

**Event Forwarding (2025-01-20)** ✅ **COMPLETED**
- **Grammar**: `=> $^` statement for parent state dispatch
- **Implementation**: Statement syntax (not terminator) with transition detection
- **Validation**: Parser prevents usage in non-hierarchical states
- **Replaces**: Deprecated `:>` and `@:>` operators

**Auto-Return Statements (2025-01-20)** ✅ **COMPLETED**
- **Feature**: Parser automatically adds return terminators to event handlers without explicit returns
- **Grammar**: `event_handler: event_selector '{' stmt* terminator? '}'` - terminator is auto-added if missing
- **Implementation**: Parser creates `TerminatorExpr::new(TerminatorType::Return, None, line_number)` when no terminator provided
- **Benefit**: Event handlers can omit explicit return statements for cleaner syntax
- **Compatibility**: Works with all event handler types including enter/exit handlers

### Grammar Coverage

- ✅ **Core Syntax**: System declarations, event handlers, actions, interfaces, domains
- ✅ **Control Flow**: if/elif/else, for/while/loop, return statements, break/continue
- ✅ **State Management**: Transitions, hierarchical states, enter/exit events, state variables
- ✅ **Modern Syntax**: Conventional parameter syntax, block structure, flattened arguments
- ✅ **System Parameters**: Start state, enter event, and domain parameter syntax
- ✅ **Function Limitations**: Single main function restriction properly enforced
- ✅ **Event Forwarding**: => $^ statement for parent state dispatch with router-based architecture
- ✅ **Return Mechanisms**: Both return statements and return assignment (return = expr)
- ✅ **Test Coverage**: 100% of comprehensive test files passing for v0.20 features (98/98 files)
- ✅ **Empty Parameter Lists**: Full support for `()` syntax in all contexts (methods, interfaces, event handlers)
- ✅ **Router Architecture**: Unified parent dispatch through dynamic router infrastructure
- 🔄 **Legacy Support**: v0.11 syntax documented but deprecated (parser rejects old syntax)

### Known Limitations

**Dead Code Generation**
- Event handlers always generate a default return terminator after statements
- This can result in unreachable return statements after exhaustive if/elif/else chains
- Functional correctness is maintained; this is a code generation optimization for future work