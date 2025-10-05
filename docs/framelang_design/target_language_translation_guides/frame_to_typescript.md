# Frame to TypeScript Translation Guide

## Overview

This guide helps LLMs and developers translate Frame specifications to TypeScript implementations. The Python output from the Frame transpiler serves as the canonical reference for Frame semantics.

## Core Concepts and Mappings

| Frame Concept | Python Implementation | TypeScript Translation |
|---------------|----------------------|------------------------|
| System | Class | Class |
| State | String variable in compartment | Enum or string literal type |
| Event | Dictionary with `_message` and `_parameters` | Interface with discriminated union |
| Event Handler | Method in state dispatcher | Method implementation |
| Event Message | Method call creating FrameEvent | Method call with typed event |
| Transition | Sets `__next_compartment` | Sets nextCompartment |
| Actions | Direct method calls | Method calls |
| Interface | Public methods creating events | Public methods with event dispatch |

## Frame Runtime Kernel Features

The Frame runtime kernel provides critical features that MUST be preserved in TypeScript implementations:

### 1. Non-Recursive Transition Loop

Frame handles state transitions iteratively to prevent stack overflow during infinite transitions:

**Python Reference (from transpiler):**
```python
def __kernel(self, __e):
    # send event to current state
    self.__router(__e)
    
    # loop until no transitions occur
    while self.__next_compartment != None:
        next_compartment = self.__next_compartment
        self.__next_compartment = None
        
        # exit current state
        self.__router(FrameEvent("<$", self.__compartment.exit_args))
        # change state
        self.__compartment = next_compartment
        
        # handle enter event
        if next_compartment.forward_event is None:
            self.__router(FrameEvent("$>", self.__compartment.enter_args))
        else:
            # handle forwarded event...
```

**TypeScript Implementation:**
```typescript
private kernel(event: FrameEvent): void {
    // Send event to current state
    this.router(event);
    
    // Loop until no transitions occur
    while (this.nextCompartment !== null) {
        const next = this.nextCompartment;
        this.nextCompartment = null;
        
        // Exit current state
        this.router(new FrameEvent("<$", this.compartment.exitArgs));
        // Change state
        this.compartment = next;
        
        // Handle enter event
        if (next.forwardEvent === null) {
            this.router(new FrameEvent("$>", this.compartment.enterArgs));
        } else {
            // Handle forwarded event...
        }
    }
}
```

### 2. Deferred Transition Mechanism

Transitions don't immediately change state - they schedule the transition for the kernel loop:

**Python Reference:**
```python
def __transition(self, next_compartment):
    self.__next_compartment = next_compartment
```

**TypeScript Implementation:**
```typescript
private transition(nextCompartment: FrameCompartment): void {
    this.nextCompartment = nextCompartment;
}
```

### 3. Event Forwarding

Events can be forwarded to the new state after transition:

**Frame:**
```frame
$StateA {
    process() {
        self.prepare()
        >> $StateB  # Forward current event to StateB
    }
}
```

**TypeScript:**
```typescript
private handleStateA_process(e: FrameEvent): void {
    this.prepare();
    const next = new FrameCompartment('StateB', e); // Forward event
    this.transition(next);
}
```

### 4. Return Value Stack

Interface methods use a return stack to pass values from event handlers:

**Python Reference:**
```python
def interface_method(self, param):
    self.return_stack.append(None)
    __e = FrameEvent("interface_method", {"param": param})
    self.__kernel(__e)
    return self.return_stack.pop(-1)
```

**TypeScript Implementation:**
```typescript
public interfaceMethod(param: any): any {
    this.returnStack.push(null);
    const event = new FrameEvent("interface_method", { param });
    this.kernel(event);
    return this.returnStack.pop();
}
```

## Complete Translation Example: MinimalDebugProtocol

### Frame Source
```frame
system MinimalDebugProtocol {
    interface:
        initialize(port)
        connect()
        disconnect()
    
    machine:
        $Disconnected {
            initialize(port) {
                self.debugPort = port
                -> $Connecting
            }
            connect() {
                print("Cannot connect - not initialized")
            }
        }
        
        $Connecting {
            $>() {
                print("Attempting connection")
            }
            connect() {
                -> $Connected
            }
        }
        
        $Connected {
            disconnect() {
                -> $Disconnected
            }
        }
    
    domain:
        var debugPort = 0
}
```

### TypeScript Implementation

```typescript
// Event type definitions
type FrameEventType = 
    | "initialize" 
    | "connect" 
    | "disconnect"
    | "$>"  // Enter
    | "<$"; // Exit

interface FrameEventParameters {
    [key: string]: any;
}

class FrameEvent {
    constructor(
        public message: FrameEventType,
        public parameters: FrameEventParameters | null = null
    ) {}
}

// State compartment
class FrameCompartment {
    constructor(
        public state: string,
        public forwardEvent: FrameEvent | null = null,
        public exitArgs: FrameEventParameters | null = null,
        public enterArgs: FrameEventParameters | null = null
    ) {}
}

// State enum
enum DebugState {
    Disconnected = "Disconnected",
    Connecting = "Connecting",
    Connected = "Connected"
}

// Main system class
class MinimalDebugProtocol {
    private compartment: FrameCompartment;
    private nextCompartment: FrameCompartment | null = null;
    private returnStack: any[] = [null];
    
    // Domain variables
    private debugPort: number = 0;
    
    constructor() {
        // Initialize start state
        this.compartment = new FrameCompartment(DebugState.Disconnected);
        
        // Send system start event
        const startEvent = new FrameEvent("$>");
        this.kernel(startEvent);
    }
    
    // ==================== Interface Block ==================
    
    public initialize(port: number): any {
        this.returnStack.push(null);
        const event = new FrameEvent("initialize", { port });
        this.kernel(event);
        return this.returnStack.pop();
    }
    
    public connect(): any {
        this.returnStack.push(null);
        const event = new FrameEvent("connect");
        this.kernel(event);
        return this.returnStack.pop();
    }
    
    public disconnect(): any {
        this.returnStack.push(null);
        const event = new FrameEvent("disconnect");
        this.kernel(event);
        return this.returnStack.pop();
    }
    
    // ==================== State Handlers ==================
    
    private handleDisconnected(e: FrameEvent): void {
        switch (e.message) {
            case "initialize":
                const port = e.parameters?.port;
                console.log(`Initializing with port ${port}`);
                this.debugPort = port;
                const next = new FrameCompartment(DebugState.Connecting);
                this.transition(next);
                break;
                
            case "connect":
                console.log("Cannot connect - not initialized");
                break;
        }
    }
    
    private handleConnecting(e: FrameEvent): void {
        switch (e.message) {
            case "$>": // Enter event
                console.log("Attempting connection");
                break;
                
            case "connect":
                const next = new FrameCompartment(DebugState.Connected);
                this.transition(next);
                break;
        }
    }
    
    private handleConnected(e: FrameEvent): void {
        switch (e.message) {
            case "disconnect":
                const next = new FrameCompartment(DebugState.Disconnected);
                this.transition(next);
                break;
        }
    }
    
    // ==================== Runtime Kernel ==================
    
    private kernel(event: FrameEvent): void {
        // Send event to current state
        this.router(event);
        
        // Loop until no transitions occur
        while (this.nextCompartment !== null) {
            const next = this.nextCompartment;
            this.nextCompartment = null;
            
            // Exit current state
            this.router(new FrameEvent("<$", this.compartment.exitArgs));
            
            // Change state
            this.compartment = next;
            
            // Handle enter event or forward event
            if (next.forwardEvent === null) {
                this.router(new FrameEvent("$>", this.compartment.enterArgs));
            } else {
                if (next.forwardEvent.message === "$>") {
                    this.router(next.forwardEvent);
                } else {
                    this.router(new FrameEvent("$>", this.compartment.enterArgs));
                    this.router(next.forwardEvent);
                }
                next.forwardEvent = null;
            }
        }
    }
    
    private router(event: FrameEvent): void {
        switch (this.compartment.state) {
            case DebugState.Disconnected:
                this.handleDisconnected(event);
                break;
            case DebugState.Connecting:
                this.handleConnecting(event);
                break;
            case DebugState.Connected:
                this.handleConnected(event);
                break;
        }
    }
    
    private transition(nextCompartment: FrameCompartment): void {
        this.nextCompartment = nextCompartment;
    }
}
```

## TypeScript-Specific Patterns

### 1. Type-Safe Events with Discriminated Unions

```typescript
type FrameEvent = 
    | { type: 'initialize'; port: number }
    | { type: 'connect' }
    | { type: 'disconnect' }
    | { type: '$>'; args?: any }  // Enter
    | { type: '<$'; args?: any }; // Exit

// Type guards for safe dispatch
function isInitializeEvent(e: FrameEvent): e is { type: 'initialize'; port: number } {
    return e.type === 'initialize';
}
```

### 2. State Enum for Compile-Time Safety

```typescript
enum State {
    Disconnected = "Disconnected",
    Connecting = "Connecting",
    Connected = "Connected"
}

// Use in compartment
class FrameCompartment {
    constructor(public state: State, ...) {}
}
```

### 3. Async Event Handlers

For async operations, maintain Frame's semantics:

```typescript
private async handleStateAsync(e: FrameEvent): Promise<void> {
    // Async work
    await this.doAsyncWork();
    
    // Transition still uses deferred mechanism
    this.transition(new FrameCompartment(State.Next));
}
```

### 4. Hierarchical State Support

```typescript
class FrameCompartment {
    constructor(
        public state: string,
        public parentCompartment: FrameCompartment | null = null,
        // ... other fields
    ) {}
}

// Forward to parent with => $^
private forwardToParent(event: FrameEvent): void {
    if (this.compartment.parentCompartment) {
        this.router(event, this.compartment.parentCompartment);
    }
}
```

## Implementation Checklist

When translating Frame to TypeScript:

1. ✅ Define event types/interfaces with discriminated unions
2. ✅ Define state enum or string literal types  
3. ✅ Create FrameEvent and FrameCompartment classes
4. ✅ Implement system class with domain variables
5. ✅ Add interface methods that create events
6. ✅ Implement kernel with non-recursive transition loop
7. ✅ Create router to dispatch events to states
8. ✅ Implement state handlers for each state/event combination
9. ✅ Add transition method that sets nextCompartment
10. ✅ Handle enter/exit events in kernel loop
11. ✅ Support event forwarding if needed
12. ✅ Use return stack for interface method returns

## Critical Differences from Naive Implementation

### ❌ WRONG: Recursive Transitions
```typescript
// This will cause stack overflow!
private transition(nextState: State): void {
    this.exitCurrentState();
    this.state = nextState;
    this.enterNewState(); // If this triggers another transition...
}
```

### ✅ CORRECT: Deferred Transitions
```typescript
// Kernel loop handles transitions iteratively
private transition(next: FrameCompartment): void {
    this.nextCompartment = next; // Just schedule it
}
```

### ❌ WRONG: Direct State Change
```typescript
// Breaks enter/exit semantics
this.state = State.Next;
```

### ✅ CORRECT: Through Transition
```typescript
// Ensures enter/exit events fire
this.transition(new FrameCompartment(State.Next));
```

## Testing Your Implementation

Verify these Frame runtime behaviors:

1. **Infinite Transitions**: States with enter handlers that transition should not overflow
2. **Event Forwarding**: `>> $State` should forward the current event  
3. **Return Values**: Interface methods should return handler values via return stack
4. **Enter/Exit Order**: Exit old state → Change state → Enter new state
5. **Multiple Transitions**: Single event can trigger chain of transitions