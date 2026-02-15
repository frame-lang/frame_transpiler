# Frame v4 Return Semantics

## Overview

Frame v4 uses contextual return semantics that do what developers naturally expect. The `return` statement behaves differently based on context, while `system.return` provides explicit control over interface return values.

## Return Statement Behavior by Context

### 1. Event Handlers (in machine states)

In event handlers, `return` is a Frame statement that returns control to the interface method that dispatched the event:

```python
@@target python

system Calculator {
    interface:
        add(x: int, y: int): int
    
    machine:
        $Ready {
            add(x: int, y: int) {
                # Calculate result
                result = x + y
                
                # These are equivalent:
                return result           # Syntactic sugar
                # OR
                system.return = result  # Explicit form
                return                  # Return control
            }
        }
}
```

**Key Points:**
- `return value` in event handlers is syntactic sugar for `system.return = value; return`
- Plain `return` exits the handler and returns control to the interface
- The interface method returns the value in `system.return` (or its default)

### 2. Actions (private methods)

In actions, `return` uses native language semantics and returns to the direct caller:

```python
@@target python

system DataProcessor {
    interface:
        process(data: str): dict
    
    machine:
        $Active {
            process(data: str) {
                # Action can set interface return
                result = self.validateAndTransform(data)
                return result  # Returns to interface
            }
        }
    
    actions:
        validateAndTransform(data: str) {
            # Native return to caller (the event handler)
            if not data:
                return {"error": "empty data"}
            
            # Can also set interface return explicitly
            transformed = data.upper()
            system.return = {"processed": transformed}
            
            # Return different value to direct caller
            return {"status": "success", "length": len(transformed)}
        }
}
```

**Key Points:**
- `return` in actions returns to the direct caller (native behavior)
- Actions can read and write `system.return` to participate in interface returns
- Useful when action needs different return values for caller vs interface

### 3. Operations (helper methods)

Operations use purely native return semantics:

```python
@@target python

system StringUtils {
    operations:
        formatName(first: str, last: str) {
            # Native return to caller
            return f"{first} {last}".title()
        }
        
        calculateHash(text: str) {
            # Operations can also access system.return if needed
            import hashlib
            hash_val = hashlib.md5(text.encode()).hexdigest()
            system.return = {"hash": hash_val}  # Set interface return
            return hash_val  # Return to direct caller
        }
}
```

**Key Points:**
- Operations use native `return` statements
- Can access `system.return` when called in interface context
- Typically used for pure helper functions

## The system.return Variable

`system.return` provides explicit control over interface return values from any context:

```typescript
@@target typescript

system StateMachine {
    interface:
        execute(command: string): Result
    
    machine:
        $Processing {
            execute(command: string): Result {
                if (command === "abort") {
                    // Set return value before transition
                    system.return = {status: "aborted", data: null}
                    -> $Idle()  // Transition happens, interface still returns the value
                }
                
                const result = this.processCommand(command)
                return result  // Syntactic sugar for system.return = result; return
            }
        }
    
    actions:
        processCommand(cmd: string): any {
            // Action can set interface return
            system.return = {status: "completed", data: cmd.toUpperCase()}
            
            // But return different value to caller
            return true  // Just success/failure to event handler
        }
}
```

## Reentrancy and the Return Stack

Frame maintains a return stack to handle reentrant calls correctly:

```python
@@target python

system Recursive {
    interface:
        factorial(n: int): int
    
    machine:
        $Calculate {
            factorial(n: int) {
                if n <= 1:
                    return 1
                
                # Recursive call - each level gets its own return context
                result = self.factorial(n - 1)
                return n * result
            }
        }
}
```

**How it works:**
- Each interface method call pushes a new return context onto the stack
- `system.return` always refers to the current call's context
- When the method completes, its context is popped
- Supports recursive and reentrant system calls

## Summary Table

| Context | `return` behavior | `return value` behavior | Can use `system.return`? |
|---------|------------------|------------------------|--------------------------|
| Event Handler | Returns to interface | Sets `system.return` and returns | Yes |
| Action | Returns to caller | Returns value to caller | Yes |
| Operation | Returns to caller | Returns value to caller | Yes |
| Interface | N/A | N/A | Read-only (returns final value) |

## Best Practices

1. **Use natural return patterns** - Let context determine behavior
   ```python
   # In event handler
   add(x, y) {
       return x + y  # Natural and correct
   }
   ```

2. **Use system.return for explicit control**
   ```python
   # When action needs to set interface return
   validateData(data) {
       system.return = {"valid": true}
       return len(data)  # Different value to caller
   }
   ```

3. **Handle transitions with returns**
   ```python
   # Set return before transition
   process(data) {
       if not data:
           system.return = {"error": "no data"}
           -> $Error()  # Interface still gets the return value
       return {"processed": data}
   }
   ```

4. **Remember returns are contextual**
   - Event handlers: return to interface
   - Actions/Operations: return to caller
   - system.return: always sets interface value

## Migration Notes

When migrating from Frame v3:
- The return semantics remain the same
- `system.return` continues to work as before
- Main difference: code blocks now use native language syntax
- Return patterns are unchanged, just the surrounding code is native