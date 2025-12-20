# Frame v4 State Machine Concepts

## Core State Machine Features

### 1. States and Event Handlers

States are the fundamental building blocks of Frame systems:

```python
@@target python

system TrafficLight {
    machine:
        $Red {
            # Event handler
            timer() {
                print("Red light timing out")
                -> $Green()  # Transition to Green state
            }
            
            # Event with parameters
            setDuration(seconds: int) {
                self.duration = seconds
            }
        }
        
        $Green {
            timer() {
                -> $Yellow()
            }
        }
        
        $Yellow {
            timer() {
                -> $Red()
            }
        }
}
```

### 2. Enter and Exit Handlers

States can have special handlers that execute on entry and exit:

```python
@@target python

system Door {
    machine:
        $Closed {
            $>() {  # Enter handler - runs when entering this state
                print("Door is now closed")
                self.lock()
            }
            
            $<() {  # Exit handler - runs when leaving this state
                print("Door is opening...")
                self.unlock()
            }
            
            open() {
                -> $Open()
            }
        }
        
        $Open {
            $>(duration: int) {  # Enter handler with parameters
                print(f"Door will remain open for {duration} seconds")
                self.auto_close_timer = duration
            }
            
            close() {
                -> $Closed()
            }
        }
}
```

### 3. State Parameters

States can receive parameters when transitioned to:

```python
@@target python

system Processor {
    machine:
        $Idle {
            start(mode: str, priority: int) {
                # Pass parameters to the target state
                -> $Processing(mode, priority)
            }
        }
        
        $Processing {
            # State parameters are received by the enter handler
            $>(mode: str, priority: int) {
                self.processing_mode = mode
                self.priority_level = priority
                print(f"Processing in {mode} mode with priority {priority}")
            }
            
            complete() {
                -> $Idle()
            }
        }
}
```

### 4. Event Forwarding

Events can be forwarded to parent states or the current state:

```python
@@target python

system Handler {
    machine:
        $Base => $Active {  # $Active inherits from $Base
            # Common handler in parent state
            error(msg: str) {
                self.log_error(msg)
                -> $Error()
            }
        }
        
        $Active {
            process(data) {
                if not self.validate(data):
                    # Forward to parent's error handler
                    => $^  
                
                # Process the data
                self.handle(data)
            }
        }
        
        $Error {
            reset() {
                -> $Active()
            }
        }
}
```

## Hierarchical State Machines (HSM)

### 5. State Hierarchy

States can inherit from parent states using `=>`:

```python
@@target typescript

system ATM {
    machine:
        // Base state for all operational states
        $Operational => $Ready {
            powerOff() {
                -> $Off()
            }
            
            maintenance() {
                -> $Maintenance()
            }
        }
        
        $Ready => $CardInserted {
            // Inherits powerOff() and maintenance() from $Operational
            // $CardInserted is initial child state of $Ready
        }
        
        $CardInserted {
            enterPin(pin: string) {
                if (this.validatePin(pin)) {
                    -> $Authenticated()
                } else {
                    -> $PinError()
                }
            }
        }
        
        $Authenticated {
            // Also inherits from $Operational through $Ready
            selectTransaction(type: string) {
                -> $Processing(type)
            }
        }
        
        $Off {
            powerOn() {
                -> $Ready()  // Goes to $CardInserted as initial child
            }
        }
}
```

### 6. State Stack Operations

Frame supports a state stack for complex navigation patterns:

```python
@@target python

system Wizard {
    machine:
        $Step1 {
            next() {
                $$[+]  # Push current state onto stack
                -> $Step2()
            }
            
            help() {
                $$[+]  # Push current state
                -> $Help()
            }
        }
        
        $Step2 {
            next() {
                $$[+]
                -> $Step3()
            }
            
            back() {
                $$[-]  # Pop state from stack and transition to it
            }
            
            help() {
                $$[+]
                -> $Help()
            }
        }
        
        $Help {
            $>(topic: str) {
                self.show_help(topic)
            }
            
            done() {
                $$[-]  # Return to wherever we came from
            }
        }
}
```

## Event System

### 7. Event Messages and Parameters

Events in Frame consist of a message and optional parameters:

```python
@@target python

system EventDemo {
    interface:
        handleEvent(msg: str, data: dict)
    
    machine:
        $Ready {
            handleEvent(msg: str, data: dict) {
                # Event message determines routing
                if msg == "start":
                    -> $Processing(data)
                elif msg == "configure":
                    self.configure(data)
                else:
                    print(f"Unknown event: {msg}")
            }
        }
        
        $Processing {
            $>(data: dict) {
                self.process_data = data
            }
            
            handleEvent(msg: str, data: dict) {
                if msg == "stop":
                    -> $Ready()
                elif msg == "pause":
                    $$[+]  # Save current state
                    -> $Paused()
            }
        }
        
        $Paused {
            handleEvent(msg: str, data: dict) {
                if msg == "resume":
                    $$[-]  # Return to previous state
            }
        }
}
```

### 8. Interface Default Values

Interface methods can specify default return values:

```python
@@target python

system Calculator {
    interface:
        getValue(): int = 0  # Default return value
        calculate(x: int, y: int): int
    
    machine:
        $Ready {
            getValue() {
                # Can override the default
                if self.has_value:
                    return self.current_value
                # Otherwise returns the interface default (0)
            }
            
            calculate(x: int, y: int) {
                result = x + y
                self.current_value = result
                self.has_value = True
                return result
            }
        }
}
```

## State Variables and Compartments

### 9. State-Local Variables (Compartments)

Frame uses "compartments" to maintain state-specific data:

```python
@@target python

system Counter {
    machine:
        $Counting {
            # State variables are stored in the state's compartment
            $>() {
                # Initialize state-local variable
                self._compartment.count = 0
            }
            
            increment() {
                self._compartment.count += 1
                print(f"Count: {self._compartment.count}")
                
                if self._compartment.count >= 10:
                    -> $Complete(self._compartment.count)
            }
        }
        
        $Complete {
            $>(final_count: int) {
                print(f"Completed with count: {final_count}")
            }
            
            reset() {
                -> $Counting()
            }
        }
}
```

## Control Flow

### 10. Transition Guards and Conditions

Transitions can be conditional:

```python
@@target python

system GuardDemo {
    machine:
        $Idle {
            request(amount: int) {
                if amount <= 0:
                    print("Invalid amount")
                    return
                
                if amount > self.limit:
                    -> $Denied(amount)
                else:
                    -> $Approved(amount)
            }
        }
        
        $Approved {
            $>(amount: int) {
                self.process_approval(amount)
                -> $Idle()  # Auto-transition back
            }
        }
        
        $Denied {
            $>(amount: int) {
                self.log_denial(amount)
                -> $Idle()
            }
        }
}
```

## Runtime Architecture

### 11. Frame Event and Compartment

Under the hood, Frame uses these runtime concepts:
- **FrameEvent**: Encapsulates event message and parameters
- **FrameCompartment**: Stores state-local variables and transition arguments
- **Return Stack**: Manages return values for reentrant calls

### 12. Static Operations

Operations can be marked as static:

```python
@@target python

system Utils {
    operations:
        # Instance operation (default)
        processData(data) {
            return self.transform(data)
        }
        
        # Static operation - cannot use self
        @staticmethod
        validateFormat(text) {
            import re
            return re.match(r'^[A-Z]+$', text) is not None
        }
}
```

## Best Practices

1. **Use Enter Handlers for Initialization**: Initialize state-specific data in `$>()`
2. **Use Exit Handlers for Cleanup**: Clean up resources in `$<()`
3. **Prefer State Parameters over Domain Variables**: Pass data through transitions
4. **Use State Stack for Modal Behavior**: Perfect for wizards, dialogs, help screens
5. **Keep Event Handlers Focused**: One responsibility per handler
6. **Use Event Forwarding for Common Behavior**: Implement once in parent state
7. **Document State Invariants**: What must be true in each state