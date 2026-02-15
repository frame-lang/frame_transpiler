# Frame v4 Quick Reference

## Typography Convention
- **Frame syntax** in regular font: `@@system Name { }`, `-> $State()`, `$StateName { }`
- *Native code* in italic: *`variable = value`*, *`if condition`*, *`array[index]`*

## File Structure

```
@@target language           # Required pragma
*imports...*                # Native language imports

@@system Name(*param: type*) {  # System definition with optional parameters
    operations:            # Optional - must come first if present
    interface:             # Optional - public API
    machine:               # Optional - state machine
    actions:               # Optional - private methods  
    domain:                # Optional - instance variables
}
```

## Frame Annotations (all use @@)

| Annotation | Purpose | Example |
|------------|---------|---------|
| `@@target` | Specify target language | `@@target python` |
| `@@system Name { }` | Define a system | `@@system TrafficLight { ... }` |
| `@@system var = Name()` | Instantiate a system | `@@system light = TrafficLight("config.json", True)` |
| `@@persist` | Enable persistence | `@@persist @@system SaveableSystem { }` |
| `@@persist(...)` | Selective persistence | `@@persist(domain=[x, y]) @@system Partial { }` |

## State Machine Syntax

| Construct | Syntax | Description |
|-----------|--------|-------------|
| State | `$StateName { }` | Define a state |
| Event Handler | `eventName(params) { }` | Handle an event |
| Enter Handler | `$>(params) { }` | Run on state entry |
| Exit Handler | `$<() { }` | Run on state exit |
| Transition | `-> $Target()` | Change state |
| Transition with Args | `-> $Target(arg1, arg2)` | Pass data to enter handler |
| Forward Event | `=> $^` | Forward to parent state |
| Push State | `$$[+]` | Push current state to stack |
| Pop State | `$$[-]` | Pop and transition to state from stack |
| State Hierarchy | `$Parent => $Child { }` | Child inherits from parent |

## Return Semantics

| Context | `return` behavior | `system.return` |
|---------|-------------------|-----------------|
| Event Handler | Returns to interface | Sets interface return value |
| Action | Returns to caller | Available to set interface return |
| Operation | Returns to caller | Available to set interface return |

## Interface

```python
interface:
    methodName(param: type): returnType      # Method signature
    getValue(): int = 42                     # With default return
```

## Domain Variables

```python
# Python
domain:
    counter = 0
    name: str = "default"    # With type hint
    items = []

# TypeScript  
domain:
    let counter: number = 0
    let name: string = "default"
    let items: string[] = []
```

## Actions vs Operations

| Aspect | Actions | Operations |
|--------|---------|------------|
| Visibility | Private | Helper methods |
| Location | `actions:` block | `operations:` block |
| Access to self | Yes | Yes (unless @staticmethod) |
| Can use system.return | Yes | Yes |
| Typical use | State machine helpers | Utility functions |

## File Extensions

| Extension | Language | Required `@@target` |
|-----------|----------|---------------------|
| `.frm` | Universal | Yes - determines language |
| `.fpy` | Python | `@@target python` |
| `.frts` | TypeScript | `@@target typescript` |
| `.frs` | Rust | `@@target rust` |
| `.fc` | C | `@@target c` |
| `.fcpp` | C++ | `@@target cpp` |
| `.fjava` | Java | `@@target java` |
| `.frcs` | C# | `@@target csharp` |

## Code Blocks

Within all code blocks (event handlers, actions, operations):
- Use **native language syntax**
- Native variables, functions, control flow
- Native imports and libraries
- Frame statements (`->`, `=>`, `$$[+]`, etc.) remain Frame syntax

## System Parameters and Initialization

### System with Parameters
```
system Server(*host: string, port: number, debug: boolean = false*) {
    domain:
        *host = host*
        *port = port*
        *debug = debug*
}

@@system *server = Server("localhost", 8080, true)*
```

### Default Interface Returns
```
interface:
    getValue(): *int* = *0*          # Default return value
    isReady(): *boolean* = *false*   # Default boolean
    getName(): *string* = *"unnamed"* # Default string
```

## Common Patterns

### Persistence (Save/Restore)

```python
@@persist system Saveable {
    # System definition
}

# Usage
*system = Saveable()*
*snapshot = Saveable.save_to_json(system)*
*restored = Saveable.restore_from_json(snapshot)*
```

### Modal State with Stack

```python
$MainState {
    openModal() {
        $$[+]  # Save current state
        -> $ModalState()
    }
}

$ModalState {
    close() {
        $$[-]  # Return to previous state
    }
}
```

### State with Initialization

```python
$Processing {
    $>(data: dict) {  # Enter handler with parameters
        self.data = data
        self.start_time = datetime.now()
    }
    
    $<() {  # Exit handler
        self.cleanup()
    }
}
```

### Conditional Transition

```python
process(value: int) {
    if value < 0:
        -> $Error("negative value")
    elif value == 0:
        -> $Idle()
    else:
        result = self.calculate(value)
        -> $Complete(result)
}
```

### Event Forwarding Pattern

```python
$BaseState => $ChildState {
    handleError(msg) {
        self.log(msg)
        -> $ErrorState()
    }
}

$ChildState {
    process(data) {
        if not self.validate(data):
            => $^  # Forward to parent's handleError
        # Continue processing...
    }
}
```

## Rules and Constraints

1. **Block Order**: When present, blocks must appear in order: operations, interface, machine, actions, domain
2. **State Names**: Must start with `$`
3. **Enter/Exit Handlers**: Maximum one of each per state
4. **@@target**: Required, determines language (not file extension)
5. **Imports**: Use native language syntax
6. **Code Blocks**: Use native language syntax
7. **Frame Statements**: Universal across languages (`->`, `=>`, `$$[+]`, etc.)