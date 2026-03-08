# Frame to TypeScript Translation Guide

## Overview

This guide helps LLMs and developers translate Frame specifications to TypeScript implementations. The Python output from the Frame transpiler serves as the canonical reference for Frame semantics.

## **CRITICAL REQUIREMENT: Deterministic Translation**

**Frame specifications MUST generate working Python code that translates deterministically to TypeScript.** 

- **No suggestive comments** - Frame must contain actual working implementations
- **No mock functions** - All functions must have real, executable logic  
- **Async-first design** - Use `asyncio` in Python that maps cleanly to TypeScript Promises
- **Identical semantics** - Same concurrency model, error handling, and control flow in both languages
- **Working code only** - Frame generates production-ready Python that TypeScript mirrors exactly

### **Async/Concurrency Requirements**

All socket operations, I/O, and concurrent operations MUST use:
- **Python**: `asyncio` with `async`/`await`
- **TypeScript**: Promises with `async`/`await`

This ensures clean, deterministic mapping between languages with identical execution semantics.

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
| **Interface Method Call** | `system.method()` → `self.method()` | `system.method()` → `this.method()` |
| **Exception Variables** | `except Exception as e: print(e)` | `catch (e) { console.log(e); }` |
| **Async Function** | `async def func()` | `async func(): Promise<T>` |
| **Async Call** | `await func()` | `await func()` |
| **Socket Connection** | `asyncio.open_connection()` | `net.createConnection()` + Promise |
| **Socket Read** | `await reader.readexactly()` | `socket.on('data')` + Promise |
| **Socket Write** | `writer.write(); await writer.drain()` | `socket.write()` + Promise |
| **Background Task** | `asyncio.create_task()` | `Promise.resolve().then()` |

## Critical Translation Patterns

**🎉 Update v0.85.6**: Both interface method calls and exception variable handling are now **FULLY RESOLVED** in the Frame transpiler. The translations below are now automatically generated correctly.

### Interface Method Calls (✅ FIXED v0.85.6)

**Frame Syntax**: `system.interfaceMethod()`  
**CRITICAL**: This must be translated differently for each target language:

**Python Translation (CORRECT)**:
```python
# Frame: system.getValue()
# Python: self.getValue()
system.getValue()  # → self.getValue()
```

**TypeScript Translation (MUST MATCH)**:
```typescript
// Frame: system.getValue()  
// TypeScript: this.getValue()
system.getValue();  // → this.getValue();
```

### Exception Variable Handling (✅ FIXED v0.85.6)

**Frame Syntax**: `except Exception as e { print(f"Error: {e}") }`

**Python Translation (CORRECT)**:
```python
try:
    risky_operation()
except Exception as e:
    print(f"Error: {e}")  # Local variable 'e'
```

**TypeScript Translation (MUST MATCH)**:
```typescript
try {
    risky_operation();
} catch (e) {
    console.log(`Error: ${e}`);  // Local variable 'e' - NOT this.e
}
```

**CRITICAL ERROR**: Exception variables must remain as local variables, not be treated as instance properties (`this.e`).

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

## **Async Socket Translation Patterns**

### **Connection Establishment**

**Frame Python (asyncio):**
```python
async def connectSocket(self, host, port):
    try:
        self.reader, self.writer = await asyncio.open_connection(host, port)
        asyncio.create_task(self.processMessages())
        event = FrameEvent("onConnected", None)
        self.__kernel(event)
    except Exception as e:
        event = FrameEvent("onConnectionError", {"error": str(e)})
        self.__kernel(event)
```

**TypeScript Translation:**
```typescript
async connectSocket(host: string, port: number): Promise<void> {
    try {
        this.socket = await this.createConnection(host, port);
        this.startMessageProcessing();
        const event = new FrameEvent("onConnected", null);
        this.__kernel(event);
    } catch (error) {
        const event = new FrameEvent("onConnectionError", { error: error.message });
        this.__kernel(event);
    }
}

private createConnection(host: string, port: number): Promise<net.Socket> {
    return new Promise((resolve, reject) => {
        const socket = net.createConnection(port, host);
        socket.on('connect', () => resolve(socket));
        socket.on('error', reject);
    });
}
```

### **Message Processing**

**Frame Python (asyncio):**
```python
async def processMessages(self):
    while not self.terminated:
        try:
            message = await self.receiveMessage()
            if message:
                self.handleMessage(message)
        except Exception as e:
            print(f"Message processing error: {e}")
            break

async def receiveMessage(self):
    length_bytes = await self.reader.readexactly(4)
    length = int.from_bytes(length_bytes, 'little')
    data = await self.reader.readexactly(length)
    return json.loads(data.decode('utf-8'))
```

**TypeScript Translation:**
```typescript
private startMessageProcessing(): void {
    this.socket.on('data', (data: Buffer) => {
        try {
            this.processIncomingData(data);
        } catch (error) {
            console.error('Message processing error:', error);
        }
    });
}

private processIncomingData(data: Buffer): void {
    // Handle length-prefixed message protocol
    if (data.length >= 4) {
        const length = data.readUInt32LE(0);
        const messageData = data.slice(4, 4 + length);
        const message = JSON.parse(messageData.toString('utf-8'));
        this.handleMessage(message);
    }
}
```

### **Data Transmission**

**Frame Python (asyncio):**
```python
async def sendSocketData(self, data):
    if self.writer:
        self.writer.write(data)
        await self.writer.drain()

def encodeMessage(self, message):
    data = json.dumps(message).encode('utf-8')
    length = len(data).to_bytes(4, 'little')
    return length + data
```

**TypeScript Translation:**
```typescript
private sendSocketData(data: Buffer): Promise<void> {
    return new Promise((resolve, reject) => {
        if (this.socket) {
            this.socket.write(data, (error) => {
                if (error) reject(error);
                else resolve();
            });
        } else {
            reject(new Error('No socket connection'));
        }
    });
}

private encodeMessage(message: any): Buffer {
    const data = Buffer.from(JSON.stringify(message), 'utf-8');
    const length = Buffer.allocUnsafe(4);
    length.writeUInt32LE(data.length, 0);
    return Buffer.concat([length, data]);
}
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

## Persistence (`@@persist`)

TypeScript supports the `@@persist` annotation for serializing and restoring system state.

### Generated Methods

When `@@persist` is present, the generated TypeScript class includes:

```typescript
// Instance method - save current state
public saveState(): any {
    return {
        _compartment: this.__compartment.copy(),
        _state_stack: this._state_stack.map(c => c.copy()),
        // ... domain variables
    };
}

// Static method - restore from saved state
public static restoreState(data: any): MySystem {
    const instance = Object.create(MySystem.prototype);
    instance.__compartment = new MySystemCompartment(data._compartment.state);
    instance.__compartment.state_args = {...(data._compartment.state_args || {})};
    instance.__compartment.state_vars = {...(data._compartment.state_vars || {})};
    instance.__compartment.enter_args = {...(data._compartment.enter_args || {})};
    instance.__compartment.exit_args = {...(data._compartment.exit_args || {})};
    instance.__compartment.forward_event = data._compartment.forward_event;
    instance.__next_compartment = null;
    instance._state_stack = (data._state_stack || []).map((c: any) => {
        const comp = new MySystemCompartment(c.state);
        // ... restore compartment fields
        return comp;
    });
    instance._context_stack = [];
    // ... restore domain variables
    return instance;
}
```

### What Gets Persisted

| Field | Description |
|-------|-------------|
| `_compartment.state` | Current state name |
| `_compartment.state_args` | State arguments |
| `_compartment.state_vars` | State variables |
| `_compartment.enter_args` | Enter handler arguments |
| `_compartment.exit_args` | Exit handler arguments |
| `_compartment.forward_event` | Forwarded event (if any) |
| `_state_stack` | Stack of compartments for push$/pop$ |
| Domain variables | All fields from the `domain:` section |

### What Gets Reinitialized

| Field | Initialized To |
|-------|---------------|
| `_context_stack` | Empty array `[]` |
| `__next_compartment` | `null` |

### Usage Example

```typescript
@@target typescript

@@persist
@@system SessionManager {
    interface:
        login(user: string)
        logout()
        getUser(): string

    domain:
        currentUser: string = ""
        loginCount: number = 0

    machine:
        $LoggedOut {
            login(user: string) {
                this.currentUser = user;
                this.loginCount = this.loginCount + 1;
                -> $LoggedIn
            }
            getUser(): string { return ""; }
        }

        $LoggedIn {
            logout() { -> $LoggedOut }
            getUser(): string { return this.currentUser; }
        }
}

// Usage
const session = new SessionManager();
session.login("alice");

// Save state
const savedData = session.saveState();
console.log(JSON.stringify(savedData));
// {"_compartment":{"state":"LoggedIn",...},"currentUser":"alice","loginCount":1}

// Restore to new instance
const restored = SessionManager.restoreState(savedData);
console.log(restored.getUser()); // "alice"
```

### Dependencies

TypeScript persistence requires no external dependencies. It uses:
- Native JavaScript objects for the save format
- `Object.create()` for prototype-based restoration
- Standard spread operators for shallow copying

### JSON Serialization

The saved state is a plain JavaScript object that can be serialized to JSON:

```typescript
// Save to JSON string
const jsonString = JSON.stringify(session.saveState());

// Restore from JSON string
const restored = SessionManager.restoreState(JSON.parse(jsonString));
```

## Testing Your Implementation

Verify these Frame runtime behaviors:

1. **Infinite Transitions**: States with enter handlers that transition should not overflow
2. **Event Forwarding**: `>> $State` should forward the current event  
3. **Return Values**: Interface methods should return handler values via return stack
4. **Enter/Exit Order**: Exit old state → Change state → Enter new state
5. **Multiple Transitions**: Single event can trigger chain of transitions