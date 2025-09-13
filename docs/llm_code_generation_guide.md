# LLM Code Generation Guide for Frame

## Overview

This guide explains how to translate Frame source code and its Python reference implementation into other target languages. The Frame transpiler generates Python code as the reference implementation. LLMs should use both the Frame source and Python output to generate semantically equivalent code in other languages.

## Translation Process

### Input Materials
1. **Frame source file** (`.frm`) - The original Frame specification
2. **Generated Python code** (`.py`) - Reference implementation from Frame transpiler
3. **This guide** - Semantic rules and patterns for target languages

### Output
Semantically equivalent implementation in the target language that:
- Preserves Frame's state machine semantics
- Maintains the same external interface
- Follows target language idioms and conventions

## Core Frame Concepts

### 1. Systems (State Machines)

#### Frame Source
```frame
system TrafficLight {
    interface:
        start()
        stop()
    
    machine:
        $Red {
            start() => $Green
            stop() => $Off
        }
        
        $Green {
            stop() => $Off
        }
        
        $Off {
            start() => $Red
        }
}
```

#### Python Reference
```python
class TrafficLight:
    def __init__(self):
        self.__compartment = FrameCompartment(self.__red_state)
    
    def start(self):
        __e = FrameEvent("start", None)
        self.__router(__e)
    
    def __red_state(self, __e):
        if __e._message == "start":
            self.__transition_to_green_state()
            return
```

#### Translation Rules
1. **Class/Struct**: System becomes a class or struct
2. **State Methods**: Each state is a method/function
3. **Router Pattern**: Implement message routing to current state
4. **Compartment**: Track current state and state-specific data

### Target Language Patterns

#### Rust
```rust
pub struct TrafficLight {
    compartment: Compartment,
}

impl TrafficLight {
    pub fn new() -> Self {
        TrafficLight {
            compartment: Compartment::new(State::Red),
        }
    }
    
    pub fn start(&mut self) {
        let e = FrameEvent::new("start", None);
        self.router(&e);
    }
    
    fn red_state(&mut self, e: &FrameEvent) {
        match e.message.as_str() {
            "start" => self.transition_to_green_state(),
            "stop" => self.transition_to_off_state(),
            _ => {}
        }
    }
}
```

#### Go
```go
type TrafficLight struct {
    compartment *Compartment
}

func NewTrafficLight() *TrafficLight {
    tl := &TrafficLight{}
    tl.compartment = NewCompartment(tl.redState)
    return tl
}

func (tl *TrafficLight) Start() {
    e := NewFrameEvent("start", nil)
    tl.router(e)
}

func (tl *TrafficLight) redState(e *FrameEvent) {
    switch e.Message {
    case "start":
        tl.transitionToGreenState()
    case "stop":
        tl.transitionToOffState()
    }
}
```

### 2. Modules and Namespaces

#### Frame Source
```frame
module utils {
    fn helper(x) {
        return x * 2
    }
    
    module math {
        fn add(a, b) {
            return a + b
        }
    }
}
```

#### Python Pattern
```python
class utils:
    @staticmethod
    def helper(x):
        return x * 2
    
    class math:
        @staticmethod
        def add(a, b):
            return a + b
```

#### Translation Mapping

| Frame | Python | Rust | Go | Java/C# | JavaScript |
|-------|--------|------|----|---------|-----------| 
| `module` | `class` with static methods | `mod` | package functions | `static class` | object literal |
| `module.function()` | `class.method()` | `module::function()` | `package.Function()` | `Class.method()` | `object.method()` |

### 3. Native Python Operations

Frame supports native Python operations directly. These operations map to language built-ins:

| Operation | Python | Rust | Go | Java/C# | JavaScript |
|---------------|--------|------|----|---------|------------|
| `str(x)` | `str(x)` | `x.to_string()` | `fmt.Sprint(x)` | `x.ToString()` | `String(x)` |
| `int(s)` | `int(s)` | `s.parse::<i32>()` | `strconv.Atoi(s)` | `int.Parse(s)` | `parseInt(s)` |
| `list.append(x)` | `list.append(x)` | `vec.push(x)` | `slice = append(slice, x)` | `list.Add(x)` | `array.push(x)` |

### 4. State Transitions

#### Frame Pattern
```frame
$StateA {
    event() => $StateB
}
```

#### Implementation Pattern
1. Exit current state (if has exit handler)
2. Transition to new state
3. Enter new state (if has enter handler)
4. Update compartment

```python
def __transition_to_state_b(self):
    self.__exit_state_a()  # If exists
    self.__compartment = FrameCompartment(self.__state_b)
    self.__enter_state_b()  # If exists
```

### 5. Hierarchical States

#### Frame Pattern
```frame
$Child => $Parent {
    unhandled() @:>  // Forward to parent
}
```

#### Implementation
- Child state has reference to parent state
- Unhandled events forwarded to parent
- Parent state handles forwarded events

## Common Patterns

### Event Handling
```python
# Python pattern
def __state_handler(self, __e):
    if __e._message == "event_name":
        # Handle event
        return
    # Forward to parent or ignore
```

### State Variables
```python
# State-scoped variables in compartment
self.__compartment.state_vars["counter"] = 0
```

### Domain Variables
```python
# Instance variables
self.count = 0  # Domain variable
```

## Translation Checklist

- [ ] **Preserve State Machine Structure**: States, transitions, events
- [ ] **Maintain Interface**: Public methods match Frame interface
- [ ] **Handle Events**: Route events to current state
- [ ] **State Transitions**: Exit, transition, enter sequence
- [ ] **Variable Scoping**: Domain, state, local variables
- [ ] **Native Operations**: Map to language built-ins
- [ ] **Module Structure**: Preserve namespace hierarchy
- [ ] **Error Handling**: Maintain Frame error semantics
- [ ] **Concurrency**: Handle if Frame system is marked thread-safe

## Validation

To validate translation:

1. **Interface Test**: Call all interface methods
2. **State Coverage**: Ensure all states are reachable
3. **Transition Test**: Verify all transitions work
4. **Variable State**: Check domain and state variables
5. **Edge Cases**: Test error conditions and boundaries

## Example Translation

Given Frame + Python, generate Rust:

1. **Analyze Frame**: Understand system structure, states, interface
2. **Study Python**: See how Frame concepts map to code
3. **Apply Patterns**: Use Rust patterns from this guide
4. **Preserve Semantics**: Ensure behavior matches exactly
5. **Follow Idioms**: Use Rust conventions (ownership, error handling)
6. **Test Thoroughly**: Validate against Frame specification

## Language-Specific Considerations

### Rust
- Use `enum` for state representation
- Leverage pattern matching for event handling
- Consider ownership for compartment management
- Use `Result` for error handling

### Go
- Use interfaces for system contracts
- Function receivers for state methods
- Channels for async communication if needed
- Error as return value pattern

### Java/C#
- Use inheritance for state hierarchies
- Interfaces for system contracts
- Enums for state constants
- Properties/getters/setters for domain variables

### JavaScript/TypeScript
- Prototype or class-based patterns
- Arrow functions for handlers
- Async/await for async operations
- TypeScript for type safety

## Resources

- Frame Language Documentation: [frame-lang.org](https://frame-lang.org)
- Frame Grammar: `docs/source/language/grammar.md`
- Python Operations Reference: Python built-in functions documentation
- Test Examples: `framec_tests/python/src/`