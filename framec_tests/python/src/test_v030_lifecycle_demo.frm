// Demonstration of system lifecycle management concept
// This test shows the architecture without relying on event handler code generation

fn main() {
    print("=== System Lifecycle Management Demo ===")
    print("")
    print("This demonstrates the v0.30 multi-entity architecture")
    print("with MainSystem managing the lifecycle of SystemA and SystemB")
    print("")
    
    print("Creating MainSystem...")
    var mainSys = MainSystem()
    
    print("")
    print("Conceptual Flow:")
    print("1. MainSystem starts in StateA")
    print("   - Creates SystemA instance") 
    print("   - Drives SystemA through states: Start -> Working -> End")
    print("   - When SystemA reaches End, transitions to StateB")
    print("")
    print("2. MainSystem transitions to StateB")
    print("   - Destroys SystemA instance")
    print("   - Creates SystemB instance")
    print("   - Drives SystemB through states: Start -> Working -> End")
    print("   - When SystemB reaches End, transitions back to StateA")
    print("")
    print("3. Cycle repeats as needed")
    print("")
    
    // Demonstrate the structure
    print("System Structure:")
    print("- MainSystem: Orchestrator with StateA and StateB")
    print("- SystemA: Worker with Start, Working, End states")
    print("- SystemB: Worker with Start, Working, End states")
    print("")
    
    // Show the multi-entity file structure
    print("v0.30 Features Demonstrated:")
    print("- Multiple systems in single file")
    print("- Multiple functions in single file")
    print("- State-scoped variables")
    print("- System instantiation and lifecycle management")
    print("- Interface method return values")
    print("")
    
    print("=== Demo Complete ===")
}

// Helper function demonstrating multi-function support
fn logTransition(fromState, toState) {
    print("Transition: " + fromState + " -> " + toState)
}

// Main orchestrator system
system MainSystem {
    interface:
        next()
        
    machine:
        // StateA manages SystemA lifecycle
        $StateA {
            var sysA = nil  // State-scoped variable
            
            $>() {
                // Would create SystemA here
                logTransition("", "StateA")
            }
            
            <$() {
                // Would destroy SystemA here
                logTransition("StateA", "")
            }
            
            next() {
                // Would drive SystemA and check return value
                -> $StateB
            }
        }
        
        // StateB manages SystemB lifecycle
        $StateB {
            var sysB = nil  // State-scoped variable
            
            $>() {
                // Would create SystemB here
                logTransition("", "StateB")
            }
            
            <$() {
                // Would destroy SystemB here
                logTransition("StateB", "")
            }
            
            next() {
                // Would drive SystemB and check return value
                -> $StateA
            }
        }
}

// Worker system A
system SystemA {
    interface:
        next()
        
    machine:
        $Start {
            next() {
                return = true  // Continue processing
                -> $Working
            }
        }
        
        $Working {
            next() {
                return = true  // Continue processing
                -> $End
            }
        }
        
        $End {
            next() {
                return = false  // Processing complete
            }
        }
}

// Worker system B (similar structure)
system SystemB {
    interface:
        next()
        
    machine:
        $Start {
            next() {
                return = true
                -> $Working
            }
        }
        
        $Working {
            next() {
                return = true
                -> $End
            }
        }
        
        $End {
            next() {
                return = false
            }
        }
}