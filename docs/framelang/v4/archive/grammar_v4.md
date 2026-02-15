# Frame V4 Grammar

This document defines the grammar for Frame V4.

## Overview

Frame V4 uses an "oceans model" where native code (the ocean) flows through unchanged, and Frame constructs (islands) are recognized and expanded into target language constructs.

### Design Principles

**`@@` means "Frame is talking."** The `@@` token marks any Frame construct — directives, declarations, annotations. It appears at module level (`@@target`), before systems (`@@persist`), on states, or anywhere Frame needs to communicate with the Framepiler.

**Inside code blocks, `$`, `->`, and `=>` are the Frame markers.** The four runtime statements use syntax that cannot collide with any target language. No `@@` token is needed — they are recognizable on their own.

**Frame recognizes exactly 4 runtime statements.** Everything else within a handler body is native code in the target language and passes through unchanged.

---

## Module Level

### Target Directive

```
@@target <language>
```

Required at the start of every Frame module. Specifies the target language for code generation.

**Supported languages:** `python_3`, `typescript`, `rust`

### System Declaration

```
@@system <Identifier> {
    ...
}
```

Declares a state machine system.

---

## System Annotations

Annotations modify how the Framepiler generates a system. They appear before `@@system` and follow the pattern `@@keyword` or `@@keyword(params)`. Multiple annotations can be stacked.

```frame
@@persist
@@system SessionManager {
    ...
}
```

With parameters:

```frame
@@persist(exclude=[temp_cache, debug_mode])
@@system SessionManager {
    ...
}
```

### @@persist

Generates JSON serialization and deserialization methods for the system. The Framepiler knows the complete inventory of states, domain variables, and stack structure, and generates idiomatic save/restore code for the target language.

**Syntax:**

```
@@persist
@@persist(domain=[<field_list>])
@@persist(exclude=[<field_list>])
```

**Forms:**

| Form | Behavior |
|------|----------|
| `@@persist` | Serialize all domain variables |
| `@@persist(domain=[f1, f2])` | Serialize only listed fields |
| `@@persist(exclude=[f1, f2])` | Serialize all except listed fields |

**Snapshot format:**

All targets produce and consume the same language-neutral JSON schema:

```json
{
    "schemaVersion": 1,
    "systemName": "Counter",
    "state": "Active",
    "stateParams": {},
    "domain": {
        "count": 5,
        "label": "main"
    },
    "stack": []
}
```

| Field | Type | Description |
|-------|------|-------------|
| `schemaVersion` | number | Format version for migration support |
| `systemName` | string | Name of the Frame system |
| `state` | string | Current state name (without `$`) |
| `stateParams` | object | Parameters passed to current state |
| `domain` | object | Domain variable values |
| `stack` | array | Saved states from push operations |

**Generated methods by target:**

Python:
```python
def _save(self) -> str        # Returns JSON string
@classmethod
def _restore(cls, data: str)  # Returns new instance from JSON
```

TypeScript:
```typescript
save(): string                        // Returns JSON string
static restore(data: string): System  // Returns new instance from JSON
```

Rust:
```rust
fn save(&self) -> String                          // Returns JSON string
fn restore(data: &str) -> Result<Self, Error>     // Returns new instance from JSON
```

---

## System Structure

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

---

## State Declaration

```
$<StateName> {
    <handlers>
}
```

With parent (HSM):

```
$<StateName> => $<ParentState> {
    <handlers>
}
```

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

### Enter/Exit Handlers

```
$>(<params>) {
    // enter handler
}

$<() {
    // exit handler
}
```

### Default Forward (HSM)

A state with a parent can forward all unhandled events by placing a bare forward statement at the state level:

```
$Child => $Parent {
    handled_event() {
        // explicitly handled
    }

    => $^   // forward everything else to parent
}
```

---

## Frame Statements

Frame V4 recognizes exactly 4 runtime statement types within handler bodies.

### 1. Transition

Transitions to a target state, invoking the exit handler on the current state and the enter handler on the target state.

**Syntax:**

```
-> $<TargetState>
```

With state parameters:

```
-> $<TargetState>(<state_params>)
```

With exit params (passed to current state's exit handler):

```
(<exit_params>) -> $<TargetState>
```

With enter params (passed to target state's enter handler):

```
-> (<enter_params>) $<TargetState>
```

Full form with all parameter positions:

```
(<exit_params>) -> (<enter_params>) $<TargetState>(<state_params>)
```

**Transition to popped state:**

```
-> pop$
```

Pops the top state from the stack and transitions to it with full lifecycle (exit handler on current state, enter handler on popped state).

**Examples:**

```frame
-> $Idle                      // Simple transition
-> $Active(user_id)           // With state parameter
(cleanup) -> $Shutdown        // With exit params
-> (init_val) $Running        // With enter params
-> pop$                       // Transition to saved state
```

### 2. Forward to Parent

Forwards the current event to the parent state in a hierarchical state machine (HSM).

**Syntax:**

```
=> $^
```

**Prerequisites:** The current state must have a parent declared with `$Child => $Parent`.

**Example:**

```frame
$Child => $Parent {
    evt1() {
        // event is handled here, no forwarding
    }

    evt2() {
        if x:
            => $^             // conditionally forward to parent
    }

    => $^                     // forward all other events to parent
}
```

### 3. Stack Push

Pushes the current state (with its compartment) onto the state stack. Used for temporary state excursions.

**Syntax:**

```
push$
```

**Example:**

```frame
handle_interrupt() {
    push$              // Save current state
    -> $Interrupt      // Go handle interrupt
}
```

**Generated code (Python):**

```python
self._state_stack.append((self._state, self._compartment))
```

### 4. Stack Pop

Pops the top state from the stack.

**Syntax:**

As a statement (pops and changes state without enter/exit lifecycle):

```
pop$
```

As part of a transition (pops and transitions with full lifecycle):

```
-> pop$
```

`pop$` is a restricted expression — it is only valid in these two forms.

**Examples:**

```frame
// Change to saved state (no enter/exit)
pop$

// Transition to saved state (with enter/exit)
-> pop$
```

**Generated code (Python):**

```python
# pop$ alone
self._state, self._compartment = self._state_stack.pop()

# -> pop$
saved_state, saved_compartment = self._state_stack.pop()
self._transition(saved_state, saved_compartment)
```

---

## Native Code

Everything that is not one of the 4 Frame statements is native code and passes through unchanged. Native code is written in the target language.

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

## Summary

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
| `@@` | Frame is talking — directives, declarations, annotations |
| `$` | State — state names, enter/exit, push/pop |
| `->` | Transition operator |
| `=>` | Forward operator / HSM parent declaration |

### Annotations

| Annotation | Level | Purpose |
|------------|-------|---------|
| `@@target` | Module | Specify target language |
| `@@system` | Module | Declare a state machine |
| `@@persist` | System | Generate JSON serialization/deserialization |
