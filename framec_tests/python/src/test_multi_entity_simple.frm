// Frame v0.30 Simple Multi-Entity Demo
// This demonstrates multiple functions and systems working together
// without domain variables (to avoid current code generation bug)

// Helper function
fn helper(msg) {
    print("Helper says: " + msg)
    return "processed"
}

// Logger function  
fn log_event(info) {
    print("[LOG] " + info)
}

// Main entry point
fn main() {
    print("=== Simple Multi-Entity Demo ===")
    
    // Test helper function
    var result = helper("hello")
    print("Result: " + result)
    
    // Create and test toggle system
    var toggle = ToggleSwitch()
    toggle.flip()
    toggle.flip()
    toggle.flip()
    
    // Create and test state machine
    var machine = SimpleStateMachine()
    machine.advance()
    machine.advance()
    machine.advance()
    
    print("=== Demo Complete ===")
}

// Simple Toggle Switch System
system ToggleSwitch {
    
    interface:
        flip()
    
    machine:
        $Off {
            flip() {
                log_event("Switch: OFF -> ON")
                -> $On
                return
            }
            
            $>() {
                print("Switch initialized to OFF")
                return
            }
        }
        
        $On {
            flip() {
                log_event("Switch: ON -> OFF")
                -> $Off
                return
            }
            
            $>() {
                print("Now ON")
                return
            }
        }
}

// Simple State Machine with Three States
system SimpleStateMachine {
    
    interface:
        advance()
    
    machine:
        $StateA {
            advance() {
                print("State A -> B")
                -> $StateB
                return
            }
            
            $>() {
                print("Starting in State A")
                return
            }
        }
        
        $StateB {
            advance() {
                print("State B -> C")
                -> $StateC
                return
            }
            
            $>() {
                print("Entered State B")
                return
            }
        }
        
        $StateC {
            advance() {
                print("State C -> A (cycling back)")
                -> $StateA
                return
            }
            
            $>() {
                print("Entered State C")
                return
            }
        }
}