# Frame V4 Grammar

This document defines the complete grammar for Frame V4.

---

## Module Level

### @@target Directive

Required at the start of every Frame module. Specifies the target language for code generation.

```
@@target <language>
```

**Supported languages:** `python_3`, `typescript`, `rust`

**Example:**
```frame
@@target python_3
```

### @@system Declaration

Declares a state machine system.

```
@@system <Identifier> {
    ...
}
```

**Example:**
```frame
@@system TrafficLight {
    interface:
        next()

    machine:
        $Red {
            next() {
                -> $Green
            }
        }
}
```

---

## System Annotations

Annotations modify how the Framepiler generates a system. They appear before `@@system`.

### @@persist

Generates JSON serialization and deserialization methods.

```
@@persist
@@persist(domain=[<field_list>])
@@persist(exclude=[<field_list>])
```

| Form | Behavior |
|------|----------|
| `@@persist` | Serialize all domain variables |
| `@@persist(domain=[f1, f2])` | Serialize only listed fields |
| `@@persist(exclude=[f1, f2])` | Serialize all except listed fields |

**Example:**
```frame
@@persist
@@system SessionManager {
    domain:
        user_id = ""
        session_token = ""
}
```

---

## System Structure

A system contains up to five sections in this order:

```
@@system <SystemName> {
    interface:
        <method_declarations>

    machine:
        <state_definitions>

    actions:
        <action_definitions>

    operations:
        <operation_definitions>

    domain:
        <variable_declarations>
}
```

### interface:

Declares public methods that dispatch events to the state machine.

```frame
interface:
    start()
    stop()
    getValue(): int
    process(data: str): bool
```

### machine:

Defines states and their event handlers.

```frame
machine:
    $Idle {
        start() {
            -> $Running
        }
    }

    $Running {
        stop() {
            -> $Idle
        }
    }
```

### actions:

Private methods callable from handlers. May not contain Frame statements.

```frame
actions:
    logEvent(msg: str) {
        print(f"Event: {msg}")
    }
```

### operations:

Utility methods not routed through the state machine. May not contain Frame statements.

```frame
operations:
    calculateHash(data: str): str {
        import hashlib
        return hashlib.md5(data.encode()).hexdigest()
    }
```

### domain:

Instance variables with optional initialization.

```frame
domain:
    count = 0
    name = "default"
    items = []
```

---

## State Declaration

### Basic State

```
$<StateName> {
    <handlers>
}
```

**Example:**
```frame
$Idle {
    start() {
        -> $Running
    }
}
```

### Hierarchical State (HSM)

A state can declare a parent using `=>`:

```
$<StateName> => $<ParentState> {
    <handlers>
}
```

**Example:**
```frame
$Active => $Base {
    // Inherits unhandled events from $Base
    specificEvent() {
        -> $Done
    }
}
```

### Default Forward (HSM)

Forward all unhandled events to parent:

```frame
$Child => $Parent {
    handled_event() {
        // explicitly handled
    }

    => $^   // forward everything else to parent
}
```

---

## Handlers

### Event Handler

```
<event_name>(<params>) {
    <body>
}
```

With return type:

```
<event_name>(<params>): <return_type> {
    <body>
}
```

**Example:**
```frame
process(data: str): bool {
    result = validate(data)
    if result:
        -> $Valid
    return result
}
```

### Enter Handler

Called when entering a state. Receives parameters from transitions.

```
$>() {
    // enter code
}

$>(<params>) {
    // enter code with parameters
}
```

**Example:**
```frame
$Processing {
    $>(task_id: str) {
        self.current_task = task_id
        print(f"Starting task: {task_id}")
    }
}
```

### Exit Handler

Called when leaving a state.

```
$<() {
    // exit code
}
```

**Example:**
```frame
$Connected {
    $<() {
        self.cleanup_connection()
    }
}
```

---

## Frame Statements

Frame V4 recognizes exactly 4 runtime statement types within handler bodies.

### 1. Transition

Transitions to a target state, invoking exit handler on current state and enter handler on target state.

**Basic:**
```
-> $<TargetState>
```

**With state parameters:**
```
-> $<TargetState>(<state_params>)
```

**With exit params** (passed to current state's exit handler):
```
(<exit_params>) -> $<TargetState>
```

**With enter params** (passed to target state's enter handler):
```
-> (<enter_params>) $<TargetState>
```

**Full form:**
```
(<exit_params>) -> (<enter_params>) $<TargetState>(<state_params>)
```

**Transition to popped state:**
```
-> pop$
```

**Examples:**
```frame
-> $Idle                      // Simple transition
-> $Active(user_id)           // With state parameter
(cleanup) -> $Shutdown        // With exit params
-> (init_val) $Running        // With enter params
-> pop$                       // Transition to saved state
```

### 2. Forward to Parent

Forwards the current event to the parent state in a hierarchical state machine.

```
=> $^
```

**Prerequisites:** The current state must have a parent declared with `$Child => $Parent`.

**Example:**
```frame
$Child => $Parent {
    knownEvent() {
        // handle locally
    }

    unknownEvent() {
        => $^   // let parent handle it
    }

    => $^       // forward all other events to parent
}
```

### 3. Stack Push

Pushes the current state onto the state stack.

```
push$
```

**Example:**
```frame
handleInterrupt() {
    push$              // Save current state
    -> $Interrupt      // Go handle interrupt
}
```

### 4. Stack Pop

Pops state from the stack. Two forms:

**As statement** (changes state without enter/exit lifecycle):
```
pop$
```

**As transition target** (full lifecycle with exit/enter):
```
-> pop$
```

**Examples:**
```frame
// Return to saved state without lifecycle
pop$

// Return to saved state with full lifecycle
-> pop$
```

---

## Native Code

Everything that is not one of the 4 Frame statements is native code and passes through unchanged.

**Example (Python target):**
```frame
process() {
    # This is native Python code
    result = compute_value()
    if result > threshold:
        -> $HighState
    else:
        log("Staying in current state")
}
```

The Frame statement (`-> $HighState`) is expanded. The Python code passes through unchanged.

---

## Statement Termination

Frame statements may optionally end with a semicolon:

```frame
-> $Next;      // Optional semicolon
-> $Next       // Also valid
push$;         // Optional semicolon
push$          // Also valid
```

---

## Complete Example

```frame
@@target python_3

from datetime import datetime

@@persist
@@system TaskProcessor {
    interface:
        submit(task: str)
        cancel()
        getStatus(): str

    machine:
        $Idle {
            $>() {
                self.started_at = None
            }

            submit(task: str) {
                self.current_task = task
                -> $Processing
            }

            getStatus(): str {
                return "idle"
            }
        }

        $Processing {
            $>() {
                self.started_at = datetime.now()
                print(f"Processing: {self.current_task}")
            }

            $<() {
                print("Stopping processor")
            }

            cancel() {
                -> $Idle
            }

            getStatus(): str {
                return f"processing: {self.current_task}"
            }
        }

    actions:
        logState(msg: str) {
            print(f"[{datetime.now()}] {msg}")
        }

    domain:
        current_task = ""
        started_at = None
}

if __name__ == '__main__':
    p = TaskProcessor()
    print(p.getStatus())
    p.submit("task-001")
    print(p.getStatus())
    p.cancel()
```

---

## Summary Tables

### Runtime Statements

| Statement | Syntax | Purpose |
|-----------|--------|---------|
| Transition | `-> $State` | Change state with enter/exit lifecycle |
| Transition (pop) | `-> pop$` | Transition to state from stack |
| Forward | `=> $^` | Delegate event to parent state |
| Stack Push | `push$` | Save current state to stack |
| Stack Pop | `pop$` | Change to state from stack (no lifecycle) |

### Token Conventions

| Token | Meaning |
|-------|---------|
| `@@` | Frame directive or annotation |
| `$` | State reference |
| `->` | Transition operator |
| `=>` | Forward operator / HSM parent declaration |

### System Sections

| Section | Purpose | Frame Statements Allowed |
|---------|---------|-------------------------|
| `interface:` | Public API | No (declarations only) |
| `machine:` | State handlers | Yes |
| `actions:` | Private methods | No |
| `operations:` | Utility methods | No |
| `domain:` | Instance variables | No (declarations only) |
