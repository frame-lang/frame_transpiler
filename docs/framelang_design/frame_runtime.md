# Frame Runtime Abstract Specification

## Overview

The Frame runtime is a language-agnostic specification that defines the execution model and semantics for Frame state machines. This document describes the abstract runtime functionality that must be supported by any target language implementation, establishing the invariants and behavioral contracts that ensure Frame programs execute consistently across all platforms.

## Core Runtime Components

### 1. System Lifecycle Management

Every Frame runtime must support the complete system lifecycle:

#### 1.1 System Instantiation
- **Parameter Resolution**: Systems accept three categories of parameters that must be processed in order:
  - State parameters: Initial state configuration values
  - Enter parameters: Arguments for the start state's enter event
  - Domain parameters: System-level member variable initializers
  
- **Initialization Sequence**: The runtime must guarantee this initialization order:
  1. Create initial state compartment
  2. Initialize runtime state tracking structures
  3. Process and store state parameters
  4. Initialize domain parameters as system member variables
  5. Initialize domain variables with their default values (BEFORE start event)
  6. Send system start event with enter parameters

#### 1.2 Start Event Transmission
The system start event (`$>`) must be the first event processed by the state machine, sent automatically after all initialization is complete. This event triggers the enter handler of the initial state.

### 2. State Compartmentalization

The Frame runtime implements states through a separation of concerns:

#### 2.1 State Functionality
State behavior is defined by state handler functions/methods that process events. These handlers are stateless in themselves - all state-specific data resides in compartments.

#### 2.2 State Compartment Data Structure
Each state compartment must maintain:
- **State identifier**: Reference to the state handler function
- **State arguments**: Parameters passed during state entry
- **Enter arguments**: Arguments for the enter event
- **Exit arguments**: Arguments for the exit event
- **Forward event**: Event to be forwarded after state entry
- **Parent compartment**: Reference for hierarchical state machines

#### 2.3 Compartment Lifecycle
- Compartments are created during transitions
- Compartments persist for the duration of their state's activation
- Compartments are replaced (not modified) during transitions

### 3. Event Processing Model

#### 3.1 Event Structure
Events must contain:
- **Message**: String identifier for the event type
- **Parameters**: Optional payload data
- **Return value**: Mechanism for interface method returns

#### 3.2 Event Routing Pipeline
The runtime must implement a three-stage routing pipeline:

1. **Event Kernel**: Main event loop that:
   - Routes events to the current state
   - Processes pending transitions
   - Manages enter/exit event generation
   - Handles event forwarding

2. **State Router**: Dispatcher that:
   - Maps compartment state identifiers to handler functions
   - Supports hierarchical routing for parent dispatch
   - Maintains routing consistency during transitions

3. **State Handlers**: Event processors that:
   - Match event messages to handler blocks
   - Execute handler logic
   - Trigger transitions or parent dispatch
   - Set return values for interface methods

#### 3.3 Event Processing Invariants
- Events are processed atomically - no partial handling
- Transition events execute in deterministic order: exit → transition → enter
- Forwarded events are processed after enter events
- Event processing completes before the next event begins

### 4. Transition Mechanics

#### 4.1 Transition Execution
Transitions must follow this exact sequence:
1. Create new state compartment with target state identifier
2. Set transition pending flag/reference
3. Return from current event handler
4. Process exit event for current state
5. Replace current compartment with new compartment
6. Process enter event for new state
7. Process any forwarded events

#### 4.2 Transition Parameters
Transitions may specify:
- Enter arguments for the target state
- State arguments for the target state compartment
- Events to forward after entry

#### 4.3 Transition Atomicity
The runtime must ensure transitions are atomic - either the complete transition succeeds or the system remains in the original state.

### 5. Hierarchical State Machine Support

#### 5.1 Parent-Child Relationships
States may have parent states, forming a hierarchy. The runtime must:
- Maintain parent references in compartments
- Support child-to-parent event delegation
- Preserve hierarchy during transitions

#### 5.2 Parent Dispatch Semantics
When a child state performs parent dispatch (`=> $^`):
1. Event is first processed by the child state handler
2. Event is then forwarded to parent state handler
3. Parent processes the same event instance
4. Both handlers may trigger transitions independently

#### 5.3 Hierarchical Transitions
When transitioning between hierarchical states:
- Exit events propagate from child to parent
- Enter events propagate from parent to child
- Intermediate states in the hierarchy are properly initialized

### 6. Interface Method Binding

#### 6.1 Method-to-Event Mapping
Interface methods must:
- Generate events with method name as message
- Package method parameters as event parameters
- Route events through the standard kernel
- Extract and return event return values

#### 6.2 Return Value Handling
The runtime must provide a mechanism (stack, field, or context) for:
- Storing return values set by handlers at arbitrary points in execution
- Preserving return values across multiple state transitions
- Supporting nested/reentrant interface method calls
- Retrieving values after complete event processing
- Maintaining type safety for returns

##### Return Value Invariants
- **Persistence**: Return values must survive state transitions triggered during interface call processing
- **Isolation**: Nested interface calls must maintain separate return contexts
- **Accessibility**: Any handler in the call chain can set the return value
- **Atomicity**: Return value setting and retrieval must be atomic operations
- **Default Behavior**: Unset return values must have predictable default behavior

### 7. Domain Variable Management

#### 7.1 Initialization Timing
Domain variables with default values MUST be initialized:
- After system structural initialization
- Before the start event is sent
- In declaration order

#### 7.2 Scope and Access
Domain variables must be:
- Accessible from all states
- Accessible from all actions
- Modifiable during system execution
- Preserved across transitions

### 8. Actions and Operations

#### 8.1 Actions (Private Methods)
Actions are internal methods that:
- Can only be called from within the system
- Have access to domain variables
- Can modify system state
- Follow naming conventions to prevent external access

#### 8.2 Operations (Public Methods)
Operations are external methods that:
- Can be called from outside the system
- Have access to domain variables
- Can be static or instance methods
- Do not directly trigger state transitions

### 9. State Stack Operations

#### 9.1 State History
The runtime must support state stack operations:
- **Push**: Save current state compartment to stack
- **Pop**: Restore state compartment from stack
- **Swap**: Atomically push and transition

#### 9.2 Stack Invariants
- Stack operations preserve complete compartment data
- Stack depth is unbounded (within system limits)
- Empty stack operations are handled gracefully

### 10. Error Handling

#### 10.1 Runtime Errors
The runtime must handle:
- Unhandled events (no matching handler)
- Invalid transitions (non-existent states)
- Stack underflow conditions
- Null compartment access

#### 10.2 Error Propagation
Errors should:
- Not corrupt system state
- Provide diagnostic information
- Allow for graceful degradation
- Maintain system invariants

## Runtime Behavioral Contracts

### Determinism Contract
Given the same initial conditions and event sequence, a Frame system must produce identical behavior across all runtime implementations.

### Atomicity Contract  
State transitions, event handling, and stack operations must be atomic - they either complete fully or have no effect.

### Ordering Contract
The runtime must preserve:
- Event ordering (FIFO within priority level)
- Initialization ordering (as specified above)
- Transition phase ordering (exit → transition → enter)

### Isolation Contract
Systems must be isolated from each other - no shared mutable state except through explicit interfaces.

### Completeness Contract
Every Frame language feature must have a complete runtime implementation:
- All event types must be routable
- All state types must be enterable
- All transitions must be executable
- All parameters must be accessible

## Implementation Requirements

Any target language runtime must:

1. **Preserve Semantics**: Implement all behavioral contracts exactly
2. **Maintain Type Safety**: Respect Frame's type system within target language constraints
3. **Ensure Thread Safety**: Support concurrent system instances (not concurrent events within a system)
4. **Provide Diagnostics**: Enable debugging and tracing of state machine execution
5. **Optimize Performance**: Minimize overhead while maintaining correctness

## Validation Criteria

A runtime implementation is considered compliant when:

1. All Frame language constructs execute correctly
2. All behavioral contracts are maintained
3. Standard test suites pass completely
4. Performance meets minimum benchmarks
5. Error handling follows specified patterns

This abstract specification serves as the canonical reference for Frame runtime behavior, ensuring consistency and portability across all target language implementations.