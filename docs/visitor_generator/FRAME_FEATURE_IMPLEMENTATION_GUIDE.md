# Frame Feature Implementation Guide for Multi-Language Transpiler

## Overview

This document provides comprehensive guidance for implementing Frame language features across all target languages. Historical notes draw on the retired type-safe Rust visitor as well as the current Python/TypeScript visitors to keep behavior aligned if new backends are added.

## Target Languages

1. **Python** (python_3) - Current implementation
2. **TypeScript** - Current implementation  
3. **Rust** - Historical type-safe implementation with thread-safety options
4. **C** - Planned implementation
5. **C++** - Planned implementation  
6. **C#** - Historical implementation traces
7. **Java** - Historical implementation traces
8. **Go** - Historical implementation traces

## Core Architecture Patterns

### State Machine Runtime Architecture

All implementations follow the Frame state machine runtime pattern with these core components:

#### 1. **Frame Event System**
- **Frame Event Type**: Central message passing mechanism
- **Event Parameters**: Typed parameter passing
- **Event Routing**: Dispatch to current state handlers

**Implementation Patterns:**
- **Python**: Dictionary-based events with dynamic typing
- **TypeScript**: Interface-based events with structural typing
- **Rust**: Enum-based events with pattern matching, `Send + Sync` for thread safety
- **C**: Struct-based events with function pointers
- **C++**: Class hierarchy or variant-based events  
- **C#**: Class hierarchy with interfaces
- **Java**: Interface hierarchy with inheritance
- **Go**: Interface-based events with embedding

#### 2. **State Management**
- **Current State**: Track active state
- **State Context**: State-specific data and behavior
- **State Transitions**: Safe state changes with validation

**Implementation Patterns:**
- **Python**: String-based state IDs with dynamic dispatch
- **TypeScript**: Enum-based states with type guards
- **Rust**: Enum states with pattern matching, `Arc<Mutex<State>>` for thread safety
- **C**: Enum states with function pointer tables
- **C++**: Enum class or state pattern with virtual dispatch
- **C#**: Enum states with delegate dispatch
- **Java**: Enum states with strategy pattern
- **Go**: Interface-based states with method dispatch

#### 3. **Memory Management Strategy**

**Thread-Safe vs Non-Thread-Safe Design (from Rust analysis):**

The Rust implementation revealed sophisticated patterns for memory management:

```rust
// Thread-safe pattern
Arc<Mutex<T>>          // Shared ownership, thread-safe access
Box<dyn Fn() + Send>   // Thread-safe callbacks

// Non-thread-safe pattern  
Rc<RefCell<T>>         // Shared ownership, single-threaded
Box<dyn Fn()>          // Single-threaded callbacks
```

**Language Adaptations:**
- **Python**: GIL provides implicit thread safety, use threading.Lock for explicit coordination
- **TypeScript**: Single-threaded by default, use Workers for concurrency
- **Rust**: Explicit thread-safety with `Send + Sync` traits
- **C**: Manual memory management with optional pthread mutexes
- **C++**: RAII with std::shared_ptr, std::mutex for thread safety
- **C#**: Automatic memory management with lock statements
- **Java**: Automatic memory management with synchronized blocks
- **Go**: Garbage collection with goroutines and channels

## Frame Language Feature Mapping

### 1. **System Declaration**

**Frame Syntax:**
```frame
system SystemName {
    interface:
        methodName(param: type): returnType
    
    machine:
        $StateName { ... }
    
    actions:
        actionName() { ... }
    
    operations:
        operationName(): type { ... }
    
    domain:
        var x = 0
}
```

**Implementation Strategies:**

| Language | System Container | Interface | Machine | Actions | Operations | Domain |
|----------|------------------|-----------|---------|---------|------------|--------|
| Python | Class | Methods | State methods | Private methods | Public methods | Instance vars |
| TypeScript | Class | Public methods | Private methods | Private methods | Public methods | Private fields |
| Rust | Struct + impl | Trait impl | State enum + match | Associated fns | Associated fns | Struct fields |
| C | Struct + functions | Function pointers | Function table | Static functions | Function pointers | Struct members |
| C++ | Class | Virtual methods | State pattern | Private methods | Public methods | Private members |
| C# | Class | Interface impl | State pattern | Private methods | Public methods | Private fields |
| Java | Class | Interface impl | State pattern | Private methods | Public methods | Private fields |
| Go | Struct + methods | Interface impl | Method dispatch | Unexported methods | Exported methods | Struct fields |

### 2. **Event Handling**

**Frame Syntax:**
```frame
$StateName {
    eventName(param: type) {
        // handler code
        actionCall()
        -> $NextState
    }
}
```

**Implementation Patterns:**

| Language | Event Dispatch | Parameter Passing | State Transition | Action Calls |
|----------|----------------|-------------------|------------------|--------------|
| Python | Dynamic method lookup | **kwargs | State string assignment | Method calls |
| TypeScript | Switch statements | Object destructuring | Enum assignment | Method calls |
| Rust | Pattern matching | Struct destructuring | Enum variant change | Associated fn calls |
| C | Function pointer table | Struct passing | Enum assignment | Function calls |
| C++ | Virtual dispatch or visitor | Reference/value passing | Enum assignment | Method calls |
| C# | Switch expressions | Object parameters | Enum assignment | Method calls |
| Java | Switch statements | Object parameters | Enum assignment | Method calls |
| Go | Type switch | Interface parameters | Type assignment | Method calls |

### 3. **State Enter/Exit Handlers**

**Frame Syntax:**
```frame
$StateName {
    $>() {  // Enter handler
        // enter code
    }
    
    $<() {  // Exit handler  
        // exit code
    }
}
```

**Implementation Strategy:**
- All languages implement enter/exit as special methods called during state transitions
- Enter handlers execute when transitioning TO the state
- Exit handlers execute when transitioning FROM the state
- Must be called in correct order: exit current state, then enter new state

### 4. **Actions and Operations**

**Actions**: Internal system behavior, no return values
**Operations**: External interface, can return values

**Frame Syntax:**
```frame
actions:
    actionName(param: type) {
        // implementation
    }

operations:
    operationName(param: type): returnType {
        // implementation
        ^ return value
    }
```

**Return Value Handling:**
- **Frame Pattern**: `@@:return = value; return`
- **Language Adaptations**: Each language uses native return mechanisms

### 5. **Domain Variables**

**Frame Syntax:**
```frame
domain:
    var counter: int = 0
    var name: string = "default"
```

**Scope and Lifetime:**
- Domain variables are system-scoped (instance variables)
- Accessible from all states, actions, and operations
- Persist across state transitions
- Initialized when system is created

### 6. **Type System Integration**

**Frame Type Mapping:**

| Frame Type | Python | TypeScript | Rust | C | C++ | C# | Java | Go |
|------------|--------|------------|------|---|-----|----|----- |----|
| int | int | number | i32/i64 | int | int | int | int | int |
| string | str | string | String | char* | std::string | string | String | string |
| bool | bool | boolean | bool | bool | bool | bool | boolean | bool |
| float | float | number | f32/f64 | float | float | float | float | float64 |
| list<T> | List[T] | T[] | Vec<T> | T* + size | std::vector<T> | List<T> | List<T> | []T |
| dict<K,V> | Dict[K,V] | Map<K,V> | HashMap<K,V> | Custom struct | std::map<K,V> | Dictionary<K,V> | Map<K,V> | map[K]V |

## Advanced Features

### 7. **Hierarchical State Machines (HSM)**

**Frame Syntax:**
```frame
$ParentState {
    $ChildState {
        event() {
            // child handler
        }
    }
    
    event() {
        // parent handler (fallback)
    }
}
```

**Implementation Strategy:**
- Child states handle events first
- If no handler found, bubble up to parent state
- Requires state hierarchy traversal mechanism

### 8. **State Parameters**

**Frame Syntax:**
```frame
$StateWithParams(param1: type, param2: type) {
    $>() {
        // can access param1, param2
    }
}
```

**Implementation Strategy:**
- State constructors accept parameters
- Parameters stored with state context
- Accessible throughout state lifetime

### 9. **Event Forwarding**

**Frame Syntax:**
```frame
eventHandler() {
    >> $OtherState
}
```

**Implementation Strategy:**
- Change state without executing current event handler
- Deliver same event to new state
- Useful for event delegation patterns

### 10. **Multi-System Integration**

**Frame Syntax:**
```frame
system A {
    operations:
        callB(): string
}

system B {
    interface:
        getResult(): string
}
```

**Implementation Strategy:**
- Systems can reference other systems
- Cross-system method calls
- Dependency injection patterns

## Thread Safety Considerations

### Rust Type-Safe Patterns (from main branch analysis)

The historical Rust implementation used sophisticated patterns:

```rust
// Configuration-driven thread safety
pub struct RustConfig {
    pub features: RustFeatures {
        pub thread_safe: bool,
        pub use_arc_mutex: bool,
    }
}

// Conditional type selection
fn callback_type(&self) -> String {
    if self.config.features.thread_safe {
        "Box<dyn Fn(FrameEvent) -> () + Send + Sync>"
    } else {
        "Box<dyn Fn(FrameEvent) -> ()>"
    }
}

fn state_container_type(&self) -> String {
    if self.config.features.thread_safe {
        "Arc<Mutex<State>>"
    } else {
        "Rc<RefCell<State>>"
    }
}
```

**Adaptation for Other Languages:**

| Language | Thread-Safe Option | Single-Threaded Option |
|----------|-------------------|------------------------|
| Python | threading.Lock | No locks |
| TypeScript | Worker threads | Single thread |
| Rust | Arc<Mutex<T>> | Rc<RefCell<T>> |
| C | pthread_mutex_t | No locks |
| C++ | std::mutex | No thread protection |
| C# | lock statements | No locks |
| Java | synchronized | No synchronization |
| Go | Mutexes + channels | No coordination |

## Code Generation Patterns

### Template Structure (legacy Rust visitor reference)

All visitors should follow this structure:

1. **Header Generation**: Imports, includes, using statements
2. **Type Definitions**: Event types, state enums, parameter structs
3. **System Class/Struct**: Main container
4. **Constructor**: Initialize state machine
5. **Interface Methods**: Public API implementation
6. **State Machine Logic**: Event dispatch and state management
7. **Action/Operation Implementation**: Business logic
8. **Helper Methods**: Utility functions

### CodeBuilder Integration

All visitors must use the CodeBuilder API for source mapping:

```rust
// Correct CodeBuilder usage
self.code.writeln(&format!("class {}", system_name));
self.code.indent();
self.code.writeln("def __init__(self):");
self.code.dedent();

// Generate final output
let result = self.code.build();
```

**Key Points:**
- Use `writeln()` for code lines
- Use `indent()`/`dedent()` for structure
- Use `build()` for final output
- Maintain source mapping for debugging

## Testing Strategy

### Cross-Language Functional Equivalence

All implementations must pass identical test suites:

1. **Phase 1**: Basic system creation and method calls
2. **Phase 2**: State transitions and event handling
3. **Phase 3**: Actions, operations, and domain variables
4. **Phase 4**: Complex event handling and parameters
5. **Phase 5**: Hierarchical state machines
6. **Phase 6**: Advanced features (async, threading)
7. **Phase 7**: Multi-system integration
8. **Phase 8**: Performance and edge cases

### Validation Requirements

Each language implementation must:
- Generate syntactically correct target code
- Compile without errors (for compiled languages)
- Execute with identical runtime behavior
- Pass all functional equivalence tests
- Meet performance benchmarks (where applicable)

## Implementation Priority

### Phase 1 (Immediate)
1. **Rust** - Implement fresh type-safe design using historical patterns as reference
2. **C** - Foundation for systems programming
3. **Go** - Modern concurrent programming

### Phase 2 (Short-term)
4. **C++** - Object-oriented systems programming
5. **C#** - Enterprise development platform

### Phase 3 (Medium-term)
6. **Java** - Enterprise and Android development

This guide ensures that all Frame language visitors maintain functional equivalence while leveraging each target language's strengths and idioms.
