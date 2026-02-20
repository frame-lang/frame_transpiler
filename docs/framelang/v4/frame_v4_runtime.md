# Frame V4 Runtime Specification

**Version:** 1.0
**Date:** February 2026
**Audience:** Implementation team
**Status:** Normative

---

## 1. Overview

The Frame runtime implements a **deferred transition** model where state changes are cached during event handling and processed by a central kernel after handler completion. This architecture prevents stack overflow in long-running services and enables powerful features like event forwarding.

---

## 2. Core Data Structures

### 2.1 Compartment

The **compartment** is Frame's central runtime data structure. Per official Frame documentation, it serves as "a closure concept for states that preserve the state itself, the data from the various scopes as well as runtime data needed for Frame machine semantics."

```
Compartment {
    state: string              # Current state identifier
    state_args: dict           # Arguments passed via $State(args)
    state_vars: dict           # State variables declared with $.varName
    enter_args: dict           # Arguments passed via -> (args) $State
    exit_args: dict            # Arguments passed via (args) -> $State
    forward_event: Event?      # Stashed event for -> => forwarding
    parent_compartment: Compartment?  # HSM parent reference (optional)
}
```

**Key invariants:**
- Every system instance maintains a `__compartment` field referencing the current state's compartment
- Every system instance maintains a `__next_compartment` field for deferred transitions (null when no transition pending)
- Compartments are **copied** when pushed to state stack, preserving all fields

### 2.2 FrameEvent

The **FrameEvent** carries event metadata through the runtime:

```
FrameEvent {
    _message: string           # Event type identifier ("$>", "$<", "methodName")
    _parameters: dict | null   # Event arguments (from interface call or enter/exit args)
    _return: any              # Return value (rarely used; prefer _return_value field)
}
```

**Special message values:**
- `"$>"` — Enter event (sent when entering a state)
- `"$<"` — Exit event (sent when exiting a state)

### 2.3 System Fields

Every generated system class contains:

```
__compartment: Compartment           # Current state's compartment
__next_compartment: Compartment?     # Pending transition target (null if none)
_state_stack: list[Compartment]      # State history stack
_return_value: any                   # Current interface return value
```

---

## 3. Runtime Methods

### 3.1 __kernel

The kernel is the central event processing loop. It:
1. Routes the event to the current state
2. Processes any pending transition (deferred model)
3. Repeats until no transitions remain

```python
def __kernel(self, __e: FrameEvent):
    # Step 1: Route event to current state
    self.__router(__e)

    # Step 2: Process deferred transitions
    while self.__next_compartment is not None:
        next_compartment = self.__next_compartment
        self.__next_compartment = None

        # Send exit event to current state
        exit_event = FrameEvent("<$", self.__compartment.exit_args)
        self.__router(exit_event)

        # Switch to new compartment
        self.__compartment = next_compartment

        # Send enter event OR forward event to new state
        if next_compartment.forward_event is None:
            # Normal enter
            enter_event = FrameEvent("$>", self.__compartment.enter_args)
            self.__router(enter_event)
        else:
            # Event forwarding (-> =>)
            forward_event = next_compartment.forward_event
            next_compartment.forward_event = None

            if forward_event._message == "$>":
                # Forwarding an enter event: just send it
                self.__router(forward_event)
            else:
                # Forwarding other event: send $> first, THEN forward
                enter_event = FrameEvent("$>", self.__compartment.enter_args)
                self.__router(enter_event)
                self.__router(forward_event)
```

**Critical:** The "send enter first, then forward" behavior ensures the target state is properly initialized before handling the forwarded event.

### 3.2 __router

The router dispatches events to state-specific handler methods:

```python
def __router(self, __e: FrameEvent):
    state_name = self.__compartment.state
    handler_name = f"_state_{state_name}"
    handler = getattr(self, handler_name, None)
    if handler:
        handler(__e)
```

**Pattern:** Dynamic dispatch via method name lookup, allowing unified state handling.

### 3.3 __transition

Caches the next compartment for deferred processing:

```python
def __transition(self, next_compartment: Compartment):
    self.__next_compartment = next_compartment
```

**Critical:** This does NOT immediately execute the transition. The kernel processes it after the current handler returns.

### 3.4 State Methods

Each state generates a unified handler method that dispatches by event message:

```python
def _state_MyState(self, __e: FrameEvent):
    if __e._message == "$>":
        # Initialize state variables
        self.__compartment.state_vars["count"] = 0
        # Execute enter handler body
        ...
    elif __e._message == "$<":
        # Execute exit handler body
        ...
    elif __e._message == "process":
        # Execute event handler body
        ...
    elif __e._message == "getData":
        # Handler with return value
        self._return_value = self.__compartment.state_vars["data"]
        __e._return = self._return_value
        return
    # Unhandled events: do nothing (no auto-forward)
```

---

## 4. Transitions

### 4.1 Simple Transition (`-> $State`)

```frame
-> $Target
```

Generated code:
```python
__compartment = SystemCompartment("Target")
self.__transition(__compartment)
return  # Handler exits; kernel processes transition
```

### 4.2 Transition with State Args (`-> $State(args)`)

```frame
-> $Target(value1, value2)
```

Generated code:
```python
__compartment = SystemCompartment("Target")
__compartment.state_args["param1"] = value1
__compartment.state_args["param2"] = value2
self.__transition(__compartment)
return
```

### 4.3 Transition with Enter Args (`-> (args) $State`)

```frame
-> (data, priority) $Target
```

Generated code:
```python
__compartment = SystemCompartment("Target")
__compartment.enter_args["0"] = data      # Positional keys
__compartment.enter_args["1"] = priority
self.__transition(__compartment)
return
```

The enter handler receives these via `__e._parameters`:
```python
def _state_Target(self, __e):
    if __e._message == "$>":
        data = __e._parameters["0"]
        priority = __e._parameters["1"]
```

### 4.4 Transition with Exit Args (`(args) -> $State`)

```frame
(cleanup_data) -> $Target
```

Generated code:
```python
self.__compartment.exit_args["0"] = cleanup_data  # Set on CURRENT compartment
__compartment = SystemCompartment("Target")
self.__transition(__compartment)
return
```

The exit handler of the current state receives these via `__e._parameters`.

### 4.5 Event Forwarding (`-> => $State`)

```frame
-> => $Target
```

Generated code:
```python
__compartment = SystemCompartment("Target")
__compartment.forward_event = __e  # Stash current event
self.__transition(__compartment)
return
```

**Kernel behavior:** After entering Target, the kernel sends the stashed event to Target's handler. If the forwarded event is NOT `$>`, the kernel sends `$>` first, then forwards.

### 4.6 Transition to Popped State (`-> pop$`)

```frame
-> pop$
```

Generated code:
```python
__compartment = self._state_stack.pop()
self.__transition(__compartment)
return
```

**Key behavior:** The popped compartment retains all its state variables — no reinitialization occurs.

---

## 5. State Stack Operations

### 5.1 Push (`push$`)

```frame
push$
```

Generated code:
```python
self._state_stack.append(self.__compartment.copy())
```

**Critical:** The compartment is COPIED, preserving:
- `state` — the state name
- `state_vars` — all state variable values
- `state_args` — state parameters
- All other fields

### 5.2 Pop (`pop$`)

```frame
pop$
```

As standalone statement (discard):
```python
self._state_stack.pop()  # Discard result
```

Combined with transition (`-> pop$`): See section 4.6.

### 5.3 Reentry vs History

| Transition Type | State Variable Behavior |
|----------------|------------------------|
| `-> $State` (normal) | State vars **reset** to initial values |
| `-> pop$` (history) | State vars **preserved** from saved compartment |

This distinction enables powerful patterns:
- Subroutine states that preserve caller context
- Undo/redo stacks with full state restoration
- Modal dialogs that return to previous state

---

## 6. Hierarchical State Machines (HSM)

### 6.1 Parent Declaration

```frame
$Child => $Parent {
    ...
}
```

This declares that `$Child` has `$Parent` as its parent state.

### 6.2 Explicit Parent Forward (`=> $^`)

**V4 uses explicit-only forwarding.** There is NO automatic forwarding of unhandled events.

```frame
$Child => $Parent {
    event_a() {
        // Handle event_a in child
        log("Child handled event_a")
    }

    event_b() {
        // Partially handle, then forward to parent
        log("Child pre-processing event_b")
        => $^  // Explicit forward to parent
    }

    // event_c is NOT handled — it's simply ignored, NOT forwarded
}
```

Generated code for `=> $^`:
```python
def _state_Child(self, __e):
    if __e._message == "event_a":
        log("Child handled event_a")
    elif __e._message == "event_b":
        log("Child pre-processing event_b")
        self._state_Parent(__e)  # Direct call to parent state method
    # No else clause — unhandled events are ignored
```

### 6.3 Default Forward (State-Level)

A state can include a bare `=> $^` as its last entry to forward ALL unhandled events:

```frame
$Child => $Parent {
    specific_event() { ... }
    => $^  // Forward everything else to parent
}
```

Generated code:
```python
def _state_Child(self, __e):
    if __e._message == "specific_event":
        ...
    else:
        self._state_Parent(__e)  # Default forward
```

### 6.4 Key HSM Principles

1. **No auto-forward:** Unhandled events are ignored unless `=> $^` is explicitly used
2. **`:>` is deprecated:** The old "continue" operator no longer exists
3. **`=> $^` anywhere:** Can be used at any point in a handler, not just at the end
4. **Direct method call:** Forwarding generates a direct call to parent's state method

---

## 7. System Return

### 7.1 Setting Return Value

```frame
system.return = value
```

Generated code:
```python
self._return_value = value
```

### 7.2 Return Sugar in Handlers

```frame
return expression
```

In event handlers (NOT actions), this is sugar for:
```python
self._return_value = expression
__e._return = self._return_value
return
```

### 7.3 Interface Method Return Chain

Interface methods:
1. Reset `_return_value` to None (or default)
2. Create event and send to kernel
3. Return `_return_value` to caller

```python
def get_status(self) -> str:
    self._return_value = None  # Or default value
    __e = FrameEvent("get_status", None)
    self.__kernel(__e)
    return self._return_value
```

### 7.4 Last Writer Wins

If multiple handlers in a transition chain set `system.return`, the **last** value wins:

```
Interface call → Handler sets "A"
              → Transition to State2
              → State2's enter handler sets "B"
              → Returns "B" (last writer)
```

---

## 8. State Variables

### 8.1 Declaration

```frame
$State {
    $.counter: int = 0
    $.data = {}

    $>() { ... }
}
```

### 8.2 Storage

State variables are stored in `compartment.state_vars`:

```python
# Access
value = self.__compartment.state_vars["counter"]

# Assignment
self.__compartment.state_vars["counter"] = value + 1
```

### 8.3 Initialization

State variables initialize when the `$>` handler runs:

```python
def _state_MyState(self, __e):
    if __e._message == "$>":
        # State var initialization
        self.__compartment.state_vars["counter"] = 0
        self.__compartment.state_vars["data"] = {}
        # Then execute $>() handler body
        ...
```

### 8.4 Lifecycle

| Event | State Var Behavior |
|-------|-------------------|
| `-> $State` | Variables initialized to declared values |
| `-> pop$` | Variables restored from saved compartment |
| Within state | Variables persist until state exits |

---

## 9. Actions and Operations

### 9.1 Actions

Private helper methods that CAN access:
- Domain variables
- State variables via `$.varName`
- `system.return`

Actions CANNOT:
- Trigger transitions
- Use `push$` or `pop$`

```python
def __my_action(self, param):
    # Can access state vars
    count = self.__compartment.state_vars["count"]
    # Can access domain vars
    self.domain_var = count
    # CANNOT use: -> $State, push$, pop$
```

### 9.2 Operations

Public methods that bypass the state machine entirely:
- Direct access to domain variables
- No access to state variables
- No access to `system.return`
- Pure native code

```python
def my_operation(self, param) -> int:
    return self.domain_var + param
```

---

## 10. Persistence

### 10.1 Save

`@@persist` generates a `save_state()` method that serializes:
- Current compartment (state, state_args, state_vars)
- Domain variables
- State stack

### 10.2 Restore

`restore_state()` deserializes and creates a new instance:
- Sets `__compartment` directly (NO enter event)
- Restores domain variables
- Restores state stack

**Key:** Restore does NOT invoke the enter handler — the state is being restored, not entered.

---

## 11. Implementation Checklist

### 11.1 Kernel Implementation

- [ ] Route event to current state via router
- [ ] Deferred transition loop
- [ ] Exit event with exit_args from current compartment
- [ ] Compartment switch
- [ ] Enter event OR forward event handling
- [ ] Forward event: send $> first if forwarded event is not $>

### 11.2 HSM Implementation

- [ ] Parse `$Child => $Parent` syntax
- [ ] Store parent state name in StateAst
- [ ] Generate `=> $^` as direct parent method call
- [ ] Generate default forward (else clause) when state has bare `=> $^`
- [ ] Do NOT auto-forward unhandled events

### 11.3 Compartment Implementation

- [ ] 6-field structure (7 with parent_compartment)
- [ ] `copy()` method for state stack
- [ ] State vars stored in `state_vars` dict
- [ ] Push copies entire compartment
- [ ] Pop restores entire compartment

### 11.4 Forward Event Implementation

- [ ] `forward_event` field on compartment
- [ ] `-> =>` sets `forward_event = __e`
- [ ] Kernel checks `forward_event`
- [ ] If forward is `$>`, send directly
- [ ] If forward is other, send `$>` first, then forward
- [ ] Clear `forward_event` after processing
