# Frame to Rust Translation Guide

## Overview

This guide helps LLMs and developers translate Frame specifications to Rust implementations. The Python output from the Frame transpiler serves as the canonical reference for Frame semantics.

## **CRITICAL REQUIREMENT: Deterministic Translation**

**Frame specifications MUST generate working Python code that translates deterministically to Rust.**

- **No suggestive comments** - Frame must contain actual working implementations
- **No mock functions** - All functions must have real, executable logic  
- **Ownership-aware design** - Use Rust's ownership system safely while maintaining Frame semantics
- **Identical semantics** - Same state machine behavior, error handling, and control flow as Python
- **Working code only** - Frame generates production-ready Python that Rust mirrors exactly

### **Memory Management Requirements**

All state management and data sharing MUST use:
- **Python**: Objects and dictionaries with dynamic references
- **Rust**: `Rc<RefCell<T>>` for interior mutability or owned data where possible

This ensures safe memory management while preserving Frame's dynamic dispatch semantics.

## Core Concepts and Mappings

| Frame Concept | Python Implementation | Rust Translation |
|---------------|----------------------|------------------|
| System | Class | `struct` with `impl` blocks |
| State | String variable in compartment | `enum` with variants |
| Event | Dictionary with `_message` and `_parameters` | `struct` or `enum` with typed parameters |
| Event Handler | Method in state dispatcher | Method implementation with pattern matching |
| Event Message | Method call creating FrameEvent | Method call with typed event |
| Transition | Sets `__next_compartment` | Sets `next_compartment` |
| Actions | Direct method calls | Method calls |
| Interface | Public methods creating events | Public methods with event dispatch |
| **Interface Method Call** | `system.method()` → `self.method()` | `system.method()` → `self.method()` |
| **Exception Variables** | `except Exception as e: print(e)` | `match result { Err(e) => println!("{}", e) }` |
| **Async Function** | `async def func()` | `async fn func() -> Result<T, E>` |
| **Async Call** | `await func()` | `func().await` |

## Critical Translation Patterns

### Interface Method Calls

**Frame Syntax**: `system.interfaceMethod()`  
**CRITICAL**: This must be translated differently for each target language:

**Python Translation (CORRECT)**:
```python
# Frame: system.getValue()
# Python: self.getValue()
system.getValue()  # → self.getValue()
```

**Rust Translation (MUST MATCH)**:
```rust
// Frame: system.getValue()  
// Rust: self.getValue()
system.getValue();  // → self.getValue();
```

### Exception Variable Handling

**Frame Syntax**: `except Exception as e { print(f"Error: {e}") }`

**Python Translation (CORRECT)**:
```python
try:
    risky_operation()
except Exception as e:
    print(f"Error: {e}")  # Local variable 'e'
```

**Rust Translation (MUST MATCH)**:
```rust
match risky_operation() {
    Ok(_) => {},
    Err(e) => {
        println!("Error: {}", e);  // Local variable 'e' - NOT self.e
    }
}
```

**CRITICAL ERROR**: Exception variables must remain as local variables, not be treated as struct fields (`self.e`).

## Frame Runtime Kernel Features

The Frame runtime kernel provides critical features that MUST be preserved in Rust implementations:

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

**Rust Implementation:**
```rust
fn kernel(&mut self, event: FrameEvent) {
    // Send event to current state
    self.router(event);
    
    // Loop until no transitions occur
    while let Some(next) = self.next_compartment.take() {
        // Exit current state
        let exit_event = FrameEvent::new("<$", self.compartment.exit_args.clone());
        self.router(exit_event);
        
        // Change state
        self.compartment = next;
        
        // Handle enter event
        if let Some(forward_event) = &self.compartment.forward_event {
            // Handle forwarded event...
            self.router(forward_event.clone());
        } else {
            let enter_event = FrameEvent::new("$>", self.compartment.enter_args.clone());
            self.router(enter_event);
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

**Rust Implementation:**
```rust
fn transition(&mut self, next_compartment: FrameCompartment) {
    self.next_compartment = Some(next_compartment);
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

**Rust:**
```rust
fn handle_state_a_process(&mut self, e: FrameEvent) {
    self.prepare();
    let next = FrameCompartment::new_with_forward("StateB", Some(e));
    self.transition(next);
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

**Rust Implementation:**
```rust
pub fn interface_method(&mut self, param: i32) -> Option<String> {
    self.return_stack.push(None);
    let mut params = HashMap::new();
    params.insert("param".to_string(), param.to_string());
    let event = FrameEvent::new("interface_method", Some(params));
    self.kernel(event);
    self.return_stack.pop().flatten()
}
```

## Python Reference Pattern Analysis

The Frame Python transpiler generates a specific pattern that Rust should follow:

### Python Event Handler Pattern:
```python
# 1. Individual event handler methods
def _handle_ready_add(self, __e, compartment):
    a = __e._parameters.get("a") if __e._parameters else None
    b = __e._parameters.get("b") if __e._parameters else None
    self.return_stack[-1] = a + b
    return

# 2. State dispatcher methods that call handlers
def __calculator_state_Ready(self, __e, compartment):
    if __e._message == "add":
        return self._handle_ready_add(__e, compartment)
```

### Required Rust Translation Pattern:
```rust
// 1. Individual event handler methods (matching Python)
fn handle_ready_add(&mut self, e: &FrameEvent, compartment: &FrameCompartment) {
    if let Some(params) = &e.parameters {
        if let (Some(a_str), Some(b_str)) = (params.get("a"), params.get("b")) {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                // Set return value on the stack (matching Python pattern)
                if let Some(last) = self.return_stack.last_mut() {
                    *last = Some((a + b).to_string());
                }
            }
        }
    }
}

// 2. State dispatcher methods (matching Python)
fn calculator_state_ready(&mut self, e: &FrameEvent, compartment: &FrameCompartment) {
    match e.message.as_str() {
        "add" => self.handle_ready_add(e, compartment),
        "$>" => {}, // Enter handler
        "<$" => {}, // Exit handler  
        _ => {} // Unhandled event
    }
}
```

**CRITICAL REQUIREMENT**: The Rust visitor MUST generate separate methods for each event handler, exactly like Python does, rather than inline code within match statements.

## Complete Translation Example: MinimalDebugProtocol

### Frame Source
```frame
system MinimalDebugProtocol {
    interface:
        initialize(port: int)
        connect()
        disconnect()
    
    machine:
        $Disconnected {
            initialize(port: int) {
                debugPort = port
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
        var debugPort: int = 0
}
```

### Rust Implementation

```rust
use std::collections::HashMap;

// Event type definitions
#[derive(Debug, Clone)]
pub struct FrameEvent {
    pub message: String,
    pub parameters: Option<HashMap<String, String>>,
}

impl FrameEvent {
    pub fn new(message: &str, parameters: Option<HashMap<String, String>>) -> Self {
        Self {
            message: message.to_string(),
            parameters,
        }
    }
}

// State compartment
#[derive(Debug, Clone)]
pub struct FrameCompartment {
    pub state: String,
    pub forward_event: Option<FrameEvent>,
    pub exit_args: Option<HashMap<String, String>>,
    pub enter_args: Option<HashMap<String, String>>,
}

impl FrameCompartment {
    pub fn new(state: &str) -> Self {
        Self {
            state: state.to_string(),
            forward_event: None,
            exit_args: None,
            enter_args: None,
        }
    }
    
    pub fn new_with_forward(state: &str, forward_event: Option<FrameEvent>) -> Self {
        Self {
            state: state.to_string(),
            forward_event,
            exit_args: None,
            enter_args: None,
        }
    }
}

// State enum
#[derive(Debug, Clone, PartialEq)]
pub enum DebugState {
    Disconnected,
    Connecting,
    Connected,
}

impl DebugState {
    pub fn as_str(&self) -> &'static str {
        match self {
            DebugState::Disconnected => "Disconnected",
            DebugState::Connecting => "Connecting",
            DebugState::Connected => "Connected",
        }
    }
}

// Main system struct
pub struct MinimalDebugProtocol {
    compartment: FrameCompartment,
    next_compartment: Option<FrameCompartment>,
    return_stack: Vec<Option<String>>,
    
    // Domain variables
    debug_port: i32,
}

impl MinimalDebugProtocol {
    pub fn new() -> Self {
        let mut instance = Self {
            compartment: FrameCompartment::new("Disconnected"),
            next_compartment: None,
            return_stack: vec![None],
            debug_port: 0,
        };
        
        // Send system start event
        let start_event = FrameEvent::new("$>", None);
        instance.kernel(start_event);
        instance
    }
    
    // ==================== Interface Block ==================
    
    pub fn initialize(&mut self, port: i32) -> Option<String> {
        self.return_stack.push(None);
        let mut params = HashMap::new();
        params.insert("port".to_string(), port.to_string());
        let event = FrameEvent::new("initialize", Some(params));
        self.kernel(event);
        self.return_stack.pop().flatten()
    }
    
    pub fn connect(&mut self) -> Option<String> {
        self.return_stack.push(None);
        let event = FrameEvent::new("connect", None);
        self.kernel(event);
        self.return_stack.pop().flatten()
    }
    
    pub fn disconnect(&mut self) -> Option<String> {
        self.return_stack.push(None);
        let event = FrameEvent::new("disconnect", None);
        self.kernel(event);
        self.return_stack.pop().flatten()
    }
    
    // ==================== State Handlers ==================
    
    fn handle_disconnected(&mut self, e: &FrameEvent) {
        match e.message.as_str() {
            "initialize" => {
                if let Some(params) = &e.parameters {
                    if let Some(port_str) = params.get("port") {
                        if let Ok(port) = port_str.parse::<i32>() {
                            println!("Initializing with port {}", port);
                            self.debug_port = port;
                            let next = FrameCompartment::new("Connecting");
                            self.transition(next);
                        }
                    }
                }
            }
            "connect" => {
                println!("Cannot connect - not initialized");
            }
            _ => {}
        }
    }
    
    fn handle_connecting(&mut self, e: &FrameEvent) {
        match e.message.as_str() {
            "$>" => { // Enter event
                println!("Attempting connection");
            }
            "connect" => {
                let next = FrameCompartment::new("Connected");
                self.transition(next);
            }
            _ => {}
        }
    }
    
    fn handle_connected(&mut self, e: &FrameEvent) {
        match e.message.as_str() {
            "disconnect" => {
                let next = FrameCompartment::new("Disconnected");
                self.transition(next);
            }
            _ => {}
        }
    }
    
    // ==================== Runtime Kernel ==================
    
    fn kernel(&mut self, event: FrameEvent) {
        // Send event to current state
        self.router(&event);
        
        // Loop until no transitions occur
        while let Some(next) = self.next_compartment.take() {
            // Exit current state
            let exit_event = FrameEvent::new("<$", self.compartment.exit_args.clone());
            self.router(&exit_event);
            
            // Change state
            self.compartment = next;
            
            // Handle enter event or forward event
            if let Some(forward_event) = &self.compartment.forward_event.clone() {
                if forward_event.message == "$>" {
                    self.router(forward_event);
                } else {
                    let enter_event = FrameEvent::new("$>", self.compartment.enter_args.clone());
                    self.router(&enter_event);
                    self.router(forward_event);
                }
            } else {
                let enter_event = FrameEvent::new("$>", self.compartment.enter_args.clone());
                self.router(&enter_event);
            }
        }
    }
    
    fn router(&mut self, event: &FrameEvent) {
        match self.compartment.state.as_str() {
            "Disconnected" => self.handle_disconnected(event),
            "Connecting" => self.handle_connecting(event),
            "Connected" => self.handle_connected(event),
            _ => {}
        }
    }
    
    fn transition(&mut self, next_compartment: FrameCompartment) {
        self.next_compartment = Some(next_compartment);
    }
}

// Main function for testing
fn main() {
    let mut protocol = MinimalDebugProtocol::new();
    
    // Test the state machine
    protocol.initialize(8080);
    protocol.connect();
    protocol.disconnect();
}
```

## Rust-Specific Patterns

### 1. Ownership and Borrowing

```rust
// Use references for event handling to avoid clones
fn router(&mut self, event: &FrameEvent) {
    // Borrow event instead of taking ownership
}

// Clone only when necessary for transitions
fn transition(&mut self, next: FrameCompartment) {
    self.next_compartment = Some(next); // Takes ownership
}
```

### 2. Error Handling with Result Types

```rust
pub fn risky_operation(&mut self) -> Result<String, String> {
    // Frame exception handling maps to Result
    match some_operation() {
        Ok(value) => Ok(value),
        Err(e) => {
            println!("Error: {}", e); // Local variable e
            Err(e.to_string())
        }
    }
}
```

### 3. Type-Safe State Enums

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Disconnected,
    Connecting,
    Connected,
}

// Use in pattern matching
match current_state {
    State::Disconnected => self.handle_disconnected(event),
    State::Connecting => self.handle_connecting(event),
    State::Connected => self.handle_connected(event),
}
```

### 4. Interior Mutability for Complex Sharing

```rust
use std::rc::Rc;
use std::cell::RefCell;

// For complex shared state (when needed)
pub struct ComplexSystem {
    state: Rc<RefCell<SystemState>>,
    // ...
}

// Access with borrow
fn update_state(&mut self) {
    let mut state = self.state.borrow_mut();
    state.update();
}
```

## Implementation Checklist

When translating Frame to Rust (following Python pattern):

1. ✅ Define FrameEvent struct with message and parameters
2. ✅ Define FrameCompartment struct with state and metadata  
3. ✅ Create state enum for type safety
4. ✅ Implement system struct with domain variables
5. ✅ Add interface methods that create events and use return stack
6. ✅ Implement kernel with non-recursive transition loop
7. ✅ Create router to dispatch events to state handlers
8. ✅ **CRITICAL**: Generate separate methods for each event handler (like Python `_handle_state_event`)
9. ✅ **CRITICAL**: Generate state dispatcher methods that call event handler methods (like Python `__system_state_State`)
10. ✅ Add transition method that sets next_compartment
11. ✅ Handle enter/exit events in kernel loop
12. ✅ Support event forwarding with Option types
13. ✅ Use return stack for interface method returns
14. ✅ Handle ownership correctly with references and clones
15. ✅ Map Frame exceptions to Rust Result types

### Method Generation Pattern (REQUIRED):
- **Event Handler Methods**: `fn handle_{state}_{event}(&mut self, e: &FrameEvent, compartment: &FrameCompartment)`
- **State Dispatcher Methods**: `fn {system}_state_{state}(&mut self, e: &FrameEvent, compartment: &FrameCompartment)`
- **Router calls dispatcher**: `"{State}" => self.{system}_state_{state}(event, compartment)`
- **Dispatcher calls handler**: `"event" => self.handle_{state}_{event}(e, compartment)`

## Critical Differences from Naive Implementation

### ❌ WRONG: Recursive Transitions
```rust
// This will cause stack overflow!
fn transition(&mut self, next_state: State) {
    self.exit_current_state();
    self.state = next_state;
    self.enter_new_state(); // If this triggers another transition...
}
```

### ✅ CORRECT: Deferred Transitions
```rust
// Kernel loop handles transitions iteratively
fn transition(&mut self, next: FrameCompartment) {
    self.next_compartment = Some(next); // Just schedule it
}
```

### ❌ WRONG: Direct State Change
```rust
// Breaks enter/exit semantics
self.state = State::Next;
```

### ✅ CORRECT: Through Transition
```rust
// Ensures enter/exit events fire
let next = FrameCompartment::new("Next");
self.transition(next);
```

### ❌ WRONG: Unnecessary Cloning
```rust
// Inefficient - clones every time
fn router(&mut self, event: FrameEvent) {
    self.handle_state(event); // Takes ownership unnecessarily
}
```

### ✅ CORRECT: Borrowing
```rust
// Efficient - borrows the event
fn router(&mut self, event: &FrameEvent) {
    self.handle_state(event); // Borrows, clones only when needed
}
```

## Testing Your Implementation

Verify these Frame runtime behaviors:

1. **Infinite Transitions**: States with enter handlers that transition should not overflow
2. **Event Forwarding**: `>> $State` should forward the current event  
3. **Return Values**: Interface methods should return handler values via return stack
4. **Enter/Exit Order**: Exit old state → Change state → Enter new state
5. **Multiple Transitions**: Single event can trigger chain of transitions
6. **Memory Safety**: No use-after-free, no data races, proper ownership
7. **Type Safety**: Compile-time guarantees about state and event validity