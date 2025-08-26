# Frame Python Runtime Implementation

## Overview

This document describes the Python-specific implementation of the Frame runtime, detailing how Python language features and idioms are used to realize the abstract Frame runtime specification. All implementation choices are driven by the behavioral contracts defined in [frame_runtime.md](frame_runtime.md).

## Python Runtime Architecture

### Class-Based System Implementation

Frame systems are implemented as Python classes, leveraging object-oriented features to encapsulate state machine behavior:

```python
class SystemName:
    def __init__(self, ...):
        # System initialization
    
    def interface_method(self, ...):
        # Public interface
    
    def __state_handler(self, __e, compartment):
        # Private state handler
```

This design provides natural encapsulation and allows multiple system instances to coexist without interference, satisfying the **Isolation Contract** from the abstract specification.

## Core Component Implementations

### 1. System Lifecycle Implementation

#### Constructor Generation Pattern

The Python runtime implements the initialization sequence specified in the abstract runtime through a carefully ordered constructor:

```python
def __init__(self, arg0, arg1, arg2, arg3, arg4, arg5):
    # Step 1: Create initial state compartment
    self.__compartment = FrameCompartment('__system_state_Start', None, None, None, None)
    
    # Step 2: Initialize runtime state tracking
    self.__next_compartment = None
    self.return_stack = [None]
    
    # Step 3: Process state parameters
    self.__compartment.state_args = {"A": arg0, "B": arg1}
    
    # Step 4: Initialize domain parameters
    self.E = arg4
    self.F = arg5
    
    # Step 5: Initialize domain variables (BEFORE start event)
    self.hello_txt = "Hello"
    self.world_txt = "World!"
    
    # Step 6: Send system start event
    enter_params = {"C": arg2, "D": arg3}
    frame_event = FrameEvent("$>", enter_params)
    self.__kernel(frame_event)
```

This implementation ensures the **Ordering Contract** is maintained exactly as specified in the abstract runtime.

### 2. State Compartment Implementation

#### FrameCompartment Class

The Python runtime implements compartments as a simple data class:

```python
class FrameCompartment:
    def __init__(self, state, forward_event=None, exit_args=None, 
                 enter_args=None, parent_compartment=None):
        self.state = state                    # State method name string
        self.forward_event = forward_event    # FrameEvent instance
        self.exit_args = exit_args           # Dictionary of arguments
        self.enter_args = enter_args         # Dictionary of arguments
        self.parent_compartment = parent_compartment  # Parent FrameCompartment
```

Python's dynamic typing eliminates the need for complex type hierarchies while maintaining the complete compartment data structure required by the specification.

#### State Method Naming Convention

State handlers follow a double-underscore prefix pattern for privacy:

```python
def __systemname_state_StateName(self, __e, compartment):
    # State handler implementation
```

This naming convention:
- Prevents external access (Python name mangling)
- Provides consistent state identification
- Enables reliable state routing

### 3. Event Processing Implementation

#### FrameEvent Class

Events are implemented as a lightweight class:

```python
class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message        # String event identifier
        self._parameters = parameters  # Dictionary of parameters
```

#### Event Kernel Implementation

The kernel implements the three-stage pipeline through method composition:

```python
def __kernel(self, __e):
    # Stage 1: Route to current state
    self.__router(__e)
    
    # Stage 2: Process transitions
    while self.__next_compartment != None:
        next_compartment = self.__next_compartment
        self.__next_compartment = None
        
        # Exit current state
        self.__router(FrameEvent("<$", self.__compartment.exit_args))
        
        # Change state
        self.__compartment = next_compartment
        
        # Enter new state
        if next_compartment.forward_event is None:
            self.__router(FrameEvent("$>", self.__compartment.enter_args))
        else:
            # Handle forwarded events
            if next_compartment.forward_event._message == "$>":
                self.__router(next_compartment.forward_event)
            else:
                self.__router(FrameEvent("$>", self.__compartment.enter_args))
                self.__router(next_compartment.forward_event)
            next_compartment.forward_event = None
```

This implementation ensures the **Atomicity Contract** by completing all transition phases before processing the next event.

#### State Router Implementation

The router uses string comparison for state dispatch:

```python
def __router(self, __e, compartment=None):
    target_compartment = compartment or self.__compartment
    
    if target_compartment.state == '__systemname_state_Start':
        self.__systemname_state_Start(__e, target_compartment)
    elif target_compartment.state == '__systemname_state_Next':
        self.__systemname_state_Next(__e, target_compartment)
    # ... additional states
```

The optional compartment parameter enables hierarchical routing for parent dispatch.

### 4. Transition Implementation

#### Transition Method

Transitions are triggered through a simple compartment assignment:

```python
def __transition(self, next_compartment):
    self.__next_compartment = next_compartment
```

#### Transition Statement Generation

Frame transition statements generate compartment creation and transition calls:

```python
# Frame: -> $NextState
next_compartment = FrameCompartment('__systemname_state_NextState', 
                                    None, None, None, None)
self.__transition(next_compartment)
```

This deferred execution model ensures the **Atomicity Contract** - the transition only occurs after the current event handler completes.

### 5. Hierarchical State Machine Implementation

#### Parent Dispatch in Python

Parent dispatch leverages the compartment's parent reference:

```python
def __systemname_state_Child(self, __e, compartment):
    if __e._message == "eventName":
        # Process in child
        self.doSomething()
        
        # Forward to parent (=> $^)
        self.__router(__e, compartment.parent_compartment)
        return
```

The router's compartment parameter enables seamless parent state execution without call stack manipulation.

### 6. Interface Method Implementation

#### The Return Stack Architecture

The return stack is a critical innovation in the Frame Python runtime that solves multiple complex problems:

1. **Return Value Persistence Across Transitions**: Interface calls may trigger multiple state transitions before completing. The return value must survive these transitions.
2. **Arbitrary Return Point**: Any handler at any depth in the call chain can set the return value.
3. **Reentrant Call Support**: Systems may call their own interface methods, requiring nested return contexts.

#### Method-to-Event Bridge

Interface methods use a return stack for value passing:

```python
def interface_method(self, param1, param2):
    self.return_stack.append(None)  # Push default return value
    
    # Create and send event
    __e = FrameEvent("interface_method", {"param1": param1, "param2": param2})
    self.__kernel(__e)
    
    # Pop and return result (whatever the handlers set)
    return self.return_stack.pop(-1)
```

#### Return Stack Mechanics

The return stack enables complex return scenarios:

```python
# Scenario 1: Simple return from current state
def __system_state_Start(self, __e, compartment):
    if __e._message == "getValue":
        self.return_stack[-1] = 42  # Set return value
        return

# Scenario 2: Return after transition
def __system_state_Start(self, __e, compartment):
    if __e._message == "process":
        # Transition to Processing state
        next_compartment = FrameCompartment('__system_state_Processing', None, None, None, None)
        self.__transition(next_compartment)
        # Return value will be set by Processing state
        return

def __system_state_Processing(self, __e, compartment):
    if __e._message == "$>":  # Enter event
        # Calculate result after transition
        result = self.complex_calculation()
        self.return_stack[-1] = result  # Set return for original interface call
        
        # Transition to Done
        next_compartment = FrameCompartment('__system_state_Done', None, None, None, None)
        self.__transition(next_compartment)
        return

# Scenario 3: Return from deeply nested transitions
def __system_state_Start(self, __e, compartment):
    if __e._message == "complexOperation":
        # Start a chain of transitions
        # Start -> A -> B -> C -> Done
        # Return value can be set at ANY point in this chain
        next_compartment = FrameCompartment('__system_state_A', None, None, None, None)
        self.__transition(next_compartment)
        return
```

#### Reentrant Call Support

The stack structure specifically supports reentrant calls where a system calls its own interface methods:

```python
def __system_state_Active(self, __e, compartment):
    if __e._message == "recursive":
        depth = __e._parameters.get("depth", 0)
        
        if depth < 3:
            # System calling its own interface method
            result = self.recursive(depth + 1)  # This pushes a new return context
            
            # Combine with our calculation
            self.return_stack[-1] = result * 2  # Set our return value
        else:
            # Base case
            self.return_stack[-1] = 1
        return
```

In this reentrant scenario:
1. Each `recursive()` call pushes a new return slot
2. Each handler sets its return value at `return_stack[-1]`
3. Each call pops its own return value
4. The stack maintains proper return isolation

#### Return Stack Invariants

The Python runtime maintains these invariants:

1. **Stack Balance**: Every interface method call pushes exactly one entry and pops exactly one entry
2. **Top Access**: Handlers always modify `return_stack[-1]` (the top of the stack)
3. **Default Values**: Stack entries initialize to `None` for predictable behavior
4. **Transition Safety**: The return stack is unaffected by state transitions
5. **Exception Safety**: Stack unwinding occurs even if handlers raise exceptions

#### Complex Return Patterns

The return stack enables sophisticated patterns:

```python
# Pattern 1: Conditional return based on state
def interface_method(self):
    self.return_stack.append(None)
    __e = FrameEvent("interface_method", None)
    self.__kernel(__e)
    
    # Different states may or may not set a return value
    result = self.return_stack.pop(-1)
    return result if result is not None else "default"

# Pattern 2: Accumulating return across transitions
def __system_state_Collector(self, __e, compartment):
    if __e._message == "collect":
        # Start with initial value
        self.return_stack[-1] = []
        
        # Transition through collecting states
        next_compartment = FrameCompartment('__system_state_Collect1', None, None, None, None)
        self.__transition(next_compartment)
        return

def __system_state_Collect1(self, __e, compartment):
    if __e._message == "$>":
        # Add to accumulated return
        self.return_stack[-1].append("item1")
        
        next_compartment = FrameCompartment('__system_state_Collect2', None, None, None, None)
        self.__transition(next_compartment)
        return

# Pattern 3: Early return from nested calls
def __system_state_Process(self, __e, compartment):
    if __e._message == "validate":
        if not self.is_valid():
            self.return_stack[-1] = "error"
            return  # Early return without transition
        
        # Continue with transitions if valid
        next_compartment = FrameCompartment('__system_state_Execute', None, None, None, None)
        self.__transition(next_compartment)
        return
```

This return stack architecture is essential for Frame's expressive power, allowing interface methods to trigger complex state machine behaviors while maintaining clean return semantics.

### 7. Domain Variable Implementation

#### Instance Variable Storage

Domain variables become standard Python instance variables:

```python
# In constructor:
self.domain_var = initial_value

# Access from any method:
value = self.domain_var
self.domain_var = new_value
```

Python's object model naturally provides the required scope and access patterns specified in the abstract runtime.

### 8. Actions and Operations Implementation

#### Actions (Private Methods)

Actions use the `_do` suffix convention:

```python
def action_name_do(self, param1, param2):
    # Action implementation with full system access
    self.domain_var = param1
    return result
```

#### Operations (Public Methods)

Operations are standard public methods:

```python
def operation_name(self, param1, param2):
    # Public operation
    return self.domain_var + param1
```

#### Static Operations

Static operations use Python's `@staticmethod` decorator:

```python
@staticmethod
def static_operation(param1, param2):
    # No access to self
    return param1 + param2
```

### 9. State Stack Implementation

The Python runtime implements state stacks as a list of compartments:

```python
# Initialize in constructor
self.__state_stack = []

# Push operation ($$[+])
self.__state_stack.append(self.__compartment)

# Pop operation ($$[-])
if self.__state_stack:
    restored_compartment = self.__state_stack.pop()
    self.__transition(restored_compartment)
```

Python's list operations provide the unbounded stack depth required by the specification.

### 10. Error Handling Implementation

#### Graceful Degradation

The Python runtime handles errors through defensive programming:

```python
def __router(self, __e, compartment=None):
    target_compartment = compartment or self.__compartment
    
    # Unhandled state - silent no-op
    if target_compartment.state not in self.__state_handlers:
        return
    
    # Route to handler
    self.__state_handlers[target_compartment.state](__e, target_compartment)
```

#### Diagnostic Support

Debug comments are embedded for development visibility:

```python
# DEBUG_EXPR_TYPE: Discriminant(4)
# DEBUG: TransitionStmt
```

## Python-Specific Optimizations

### 1. Name Mangling for Privacy

The double-underscore prefix triggers Python's name mangling:
- `__method` becomes `_ClassName__method`
- Provides strong encapsulation
- Prevents accidental external access

### 2. Dictionary-Based Parameters

Using dictionaries for parameters provides flexibility:
```python
parameters = {"key1": value1, "key2": value2}
```
- Dynamic parameter sets
- Optional parameters
- Easy parameter forwarding

### 3. Dynamic Method Resolution

String-based state identification enables runtime flexibility:
```python
state_name = '__systemname_state_' + dynamic_state
getattr(self, state_name)(__e, compartment)
```

## Performance Considerations

### Memory Management
- Compartments are lightweight objects
- No deep copying unless necessary
- Garbage collection handles cleanup

### Execution Efficiency
- Direct method calls (no reflection)
- String comparison for routing
- Minimal object allocation

### Scalability
- O(1) state routing via if/elif chains
- O(1) transition triggering
- O(n) state stack operations

## Thread Safety

The Python implementation provides instance-level isolation:
- Each system instance has independent state
- No shared mutable class variables
- Thread-safe for multiple instances

Note: Individual instances are NOT thread-safe for concurrent event processing, maintaining the Frame semantic that events are processed sequentially.

## Testing and Validation

### Compliance Verification

The Python runtime passes all Frame test suites:
1. Basic state machine operations
2. Hierarchical state machines
3. State parameters and contexts
4. Domain variables and initialization
5. Interface methods and returns

### Debug Support

Generated code includes debugging aids:
- State names in compartments
- Debug comments for expressions
- Clear method naming

## Conclusion

The Python runtime implementation fully realizes the Frame abstract runtime specification while leveraging Python's strengths:
- Dynamic typing for flexibility
- Object-oriented encapsulation for isolation
- Native data structures for simplicity
- Duck typing for extensibility

This implementation serves as a reference for how the abstract Frame runtime can be efficiently realized in a high-level, dynamically-typed language while maintaining all behavioral contracts and semantic requirements.