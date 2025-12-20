# Frame v4 Grammar

## Overview

Frame v4 uses a native-first approach: native language syntax within code blocks, with Frame providing the structural framework for state machines and transitions.

## File Structure

```
file            = prolog imports? system+
prolog          = "@@target" language_identifier
imports         = native_import+
system          = "system" identifier "{" system_body "}"
```

### File Extensions
The `@@target` pragma determines the target language, not the file extension. Extensions are purely conventional:
- `.frm` - Universal extension (works with any `@@target`)
- `.fpy` - Convention for Python files
- `.frts` - Convention for TypeScript files
- `.frs` - Convention for Rust files
- `.fc` - Convention for C files
- `.fcpp` - Convention for C++ files
- `.fjava` - Convention for Java files
- `.frcs` - Convention for C# files

## System Structure

```
system_body     = operations? interface? machine? actions? domain?
operations      = "operations:" operation_method+
interface       = "interface:" interface_method+
machine         = "machine:" state+
actions         = "actions:" action_method+
domain          = "domain:" domain_declaration+
```

### Block Ordering
When multiple blocks are present, they must appear in this canonical order:
1. `operations:` - Internal helper methods
2. `interface:` - Public API methods
3. `machine:` - State machine definition
4. `actions:` - Private implementation methods
5. `domain:` - Private state variables

Each block is optional, but when present must respect this order.

## State Machine Constructs

```
state           = "$" identifier "{" handler* "}"
handler         = event_handler | enter_handler | exit_handler
event_handler   = identifier "(" params? ")" "{" native_code_block "}"
enter_handler   = "$>(" params? ")" "{" native_code_block "}"
exit_handler    = "$<(" params? ")" "{" native_code_block "}"
```

## Native Code Blocks

Code within handler, action, and operation blocks uses the target language's native syntax:

```
native_code_block = native_statement*
native_statement  = native_language_statement
                  | frame_transition
                  | frame_forward
                  | frame_stack_operation

# Native statements follow target language syntax
# Frame-specific statements remain consistent across languages
```

## Frame State Machine Statements

```
transition      = "->" "$" identifier "(" args? ")"
forward         = "=>" "$^"
stack_push      = "$$[+]"
stack_pop       = "$$[-]"
return_statement = "return" expression?  // Contextual behavior
system_return   = "system.return" "=" expression
```

### Return Statement Context
- **In event handlers**: `return expr` is sugar for `system.return = expr; return`
- **In actions/operations**: `return expr` returns to direct caller
- **system.return**: Always sets interface return value

## System Annotation (New in v4)

```
system_instantiation = "@@system" identifier "=" identifier "(" args? ")"
```

## Actions and Operations

```
action_method   = identifier "(" params? ")" "{" native_code_block "}"
operation_method = identifier "(" params? ")" "{" native_code_block "}"
```

## Examples

### Python Example
```python
@@target python

system Controller {
    interface:
        process(data: str): str
        reset()
    
    machine:
        $Active {
            process(data: str) {
                # Native Python in blocks
                result = self.processData(data)
                if len(result) > 10:
                    print("Processing: " + result)
                    self.output = result
                    -> $Done(result)
                else:
                    print("Too short, forwarding")
                    => $^
                return result
            }
            
            reset() {
                self.output = ""
                -> $Active()
            }
        }
        
        $Done {
            $>(result: str) {
                print(f"Completed with: {result}")
                self.output = result
            }
            
            process(data: str) {
                print("Already done, ignoring")
                return self.output
            }
            
            reset() {
                self.output = ""
                -> $Active()
            }
        }
    
    actions:
        processData(input) {
            # Private helper method
            cleaned = input.strip()
            return cleaned.upper()
        }
    
    domain:
        output = ""
}
```

### TypeScript Example
```typescript
@@target typescript

system Controller {
    interface:
        process(data: string): string
        reset(): void
    
    machine:
        $Active {
            process(data: string): string {
                // Native TypeScript in blocks
                const result = this.processData(data)
                if (result.length > 10) {
                    console.log("Processing: " + result)
                    this.output = result
                    -> $Done(result)
                } else {
                    console.log("Too short, forwarding")
                    => $^
                }
                return result
            }
            
            reset(): void {
                this.output = ""
                -> $Active()
            }
        }
        
        $Done {
            $>(result: string) {
                console.log(`Completed with: ${result}`)
                this.output = result
            }
            
            process(data: string): string {
                console.log("Already done, ignoring")
                return this.output
            }
            
            reset(): void {
                this.output = ""
                -> $Active()
            }
        }
    
    actions:
        processData(input: string): string {
            // Private helper method
            const cleaned = input.trim()
            return cleaned.toUpperCase()
        }
    
    domain:
        let output: string = ""
}
```

## Interface Methods

Interface methods declare the public API of a system:

```
interface_method = identifier "(" params? ")" return_type?
```

The syntax for parameters and return types follows the native language conventions.

## Domain Declarations

Domain declarations follow native language variable declaration syntax:

### Python
```python
domain:
    counter = 0
    name = "default"
    items = []
    # Or with type annotations:
    # counter: int = 0
    # name: str = "default"
    # items: list = []
```

### TypeScript
```typescript
domain:
    let counter: number = 0
    let name: string = "default"
    let items: string[] = []
```

## Frame-Specific Constructs

While code blocks use native language syntax, Frame provides these universal constructs for state machine behavior:

1. **State transitions** - `-> $State()`, `=> $^`
2. **Stack operations** - `$$[+]`, `$$[-]`
3. **State machine structure** - `$StateName { }`, event handlers
4. **System structure** - `system Name { }`, section headers with colons
5. **Special handlers** - `$>()` for enter, `$<()` for exit

## System Instantiation and Validation

The `@@system` annotation enables Frame to track system instances and validate their usage:

```python
@@target python

system TrafficLight {
    interface:
        timer()
        getColor(): str
    
    machine:
        $Red {
            timer() {
                -> $Green()
            }
            getColor() {
                return "red"
            }
        }
        $Green {
            timer() {
                -> $Green()
            }
            getColor() {
                return "green"
            }
        }
}

system Manager {
    interface:
        runDemo()
    
    machine:
        $Init {
            runDemo() {
                # Declares light as a TrafficLight system instance
                @@system light = TrafficLight()
                
                # Frame validates these calls at compile time
                color1 = light.getColor()    # ✓ Valid - returns "red"
                light.timer()                 # ✓ Valid - transitions to green
                color2 = light.getColor()    # ✓ Valid - returns "green"
                
                print(f"Before: {color1}, After: {color2}")
                # light.privateAction()       # ✗ Would error - not in interface
            }
        }
}
```

## Comments

Comments follow the native language conventions:
- Python: `#` for line comments
- TypeScript/JavaScript: `//` for line, `/* */` for block
- Rust: `//` for line, `/* */` for block
- C/C++: `//` for line, `/* */` for block

## String Literals

String literals follow the native language conventions, including escape sequences, interpolation, and multi-line strings.