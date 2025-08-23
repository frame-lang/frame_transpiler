// Test: System lifecycle management with state-driven instantiation/destruction
// Tests v0.30 multi-entity support with complex system interactions

fn main() {
    print("=== Starting System Lifecycle Test ===")
    var mainSys = MainSystem()
    
    // Drive 2 complete cycles: StateA -> StateB -> StateA -> StateB
    print("\n--- Cycle 1: StateA ---")
    mainSys.next()  // Start -> Working
    mainSys.next()  // Working -> End
    mainSys.next()  // End returns false, transition to StateB
    
    print("\n--- Cycle 1: StateB ---")
    mainSys.next()  // Start -> Working
    mainSys.next()  // Working -> End
    mainSys.next()  // End returns false, transition to StateA
    
    print("\n--- Cycle 2: StateA ---")
    mainSys.next()  // Start -> Working
    mainSys.next()  // Working -> End
    mainSys.next()  // End returns false, transition to StateB
    
    print("\n--- Cycle 2: StateB ---")
    mainSys.next()  // Start -> Working
    mainSys.next()  // Working -> End
    mainSys.next()  // End returns false, done
    
    print("\n=== System Lifecycle Test Complete ===")
}

system MainSystem {
    interface:
        next()
        
    machine:
        $StateA {
            var sysA = nil
            
            $>() {
                print("MainSystem: Entering StateA")
                sysA = SystemA()
                print("MainSystem: Created SystemA instance")
            }
            
            <$() {
                print("MainSystem: Exiting StateA")
                sysA = nil
                print("MainSystem: Destroyed SystemA instance")
            }
            
            next() {
                var continueProcessing = sysA.next()
                if (!continueProcessing) {
                    print("MainSystem: SystemA complete, transitioning to StateB")
                    -> $StateB
                }
            }
        }
        
        $StateB {
            var sysB = nil
            
            $>() {
                print("MainSystem: Entering StateB")
                sysB = SystemB()
                print("MainSystem: Created SystemB instance")
            }
            
            <$() {
                print("MainSystem: Exiting StateB")
                sysB = nil
                print("MainSystem: Destroyed SystemB instance")
            }
            
            next() {
                var continueProcessing = sysB.next()
                if (!continueProcessing) {
                    print("MainSystem: SystemB complete, transitioning to StateA")
                    -> $StateA
                }
            }
        }
}

system SystemA {
    interface:
        next()
        
    machine:
        $Start {
            $>() {
                print("SystemA: Entering Start state")
            }
            
            <$() {
                print("SystemA: Exiting Start state")
            }
            
            next() {
                print("SystemA: Start.next() -> Working (returning true)")
                return = true
                -> $Working
            }
        }
        
        $Working {
            $>() {
                print("SystemA: Entering Working state")
            }
            
            <$() {
                print("SystemA: Exiting Working state")
            }
            
            next() {
                print("SystemA: Working.next() -> End (returning true)")
                return = true
                -> $End
            }
        }
        
        $End {
            $>() {
                print("SystemA: Entering End state")
            }
            
            <$() {
                print("SystemA: Exiting End state")
            }
            
            next() {
                print("SystemA: End.next() - complete (returning false)")
                return = false
            }
        }
}

system SystemB {
    interface:
        next()
        
    machine:
        $Start {
            $>() {
                print("SystemB: Entering Start state")
            }
            
            <$() {
                print("SystemB: Exiting Start state")
            }
            
            next() {
                print("SystemB: Start.next() -> Working (returning true)")
                return = true
                -> $Working
            }
        }
        
        $Working {
            $>() {
                print("SystemB: Entering Working state")
            }
            
            <$() {
                print("SystemB: Exiting Working state")
            }
            
            next() {
                print("SystemB: Working.next() -> End (returning true)")
                return = true
                -> $End
            }
        }
        
        $End {
            $>() {
                print("SystemB: Entering End state")
            }
            
            <$() {
                print("SystemB: Exiting End state")
            }
            
            next() {
                print("SystemB: End.next() - complete (returning false)")
                return = false
            }
        }
}