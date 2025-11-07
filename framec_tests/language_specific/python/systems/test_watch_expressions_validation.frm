# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Watch Expressions and Conditional Breakpoints Validation Test
# This file tests Frame expression evaluation, watch expressions, and conditional breakpoints

system WatchTestSystem {
    interface:
        start()
        increment()
        decrement()
        processItems()
        reset()
        setState(newState)
    
    machine:
        $Start {
            $>() {
                print("System starting - counter initialized to " + str(self.counter))
                self.lastResult = "started"
            }
            
            start() {
                print("Starting system with counter = " + str(self.counter))
                -> $Running
            }
            
            setState(newState) {
                if newState == "running":
                    -> $Running
                elif newState == "processing":
                    -> $Processing
                else:
                    print("Invalid state: " + newState)
                }
            }
        }
        
        $Running {
            $>() {
                print("Entering Running state")
                self.lastResult = "running"
            }
            
            increment() {
                print("Incrementing counter from " + str(self.counter))
                self.counter = self.counter + 1
                print("Counter is now " + str(self.counter))
                
                # Test conditional expressions
                if self.counter >= 5:
                    print("Counter reached threshold!")
                    -> $Processing
                else:
                    -> $Running
                }
            }

            decrement() {
                print("Decrementing counter from " + str(self.counter))
                if self.counter > 0:
                    self.counter = self.counter - 1
                    print("Counter is now " + str(self.counter))
                else:
                    print("Counter cannot go below zero")
                }
                -> $Running
            }
            
            processItems() {
                print("Starting to process items")
                -> $Processing
            }
            
            reset() {
                print("Resetting system")
                self.counter = 0
                self.lastResult = "reset"
                -> $Start
            }
        }
        
        $Processing {
            $>() {
                print("Entering Processing state")
                self.lastResult = "processing"
            }
            
            processItems() {
                print("Processing " + str(len(self.items)) + " items")
                total = 0
                
                for item in self.items:
                    total = total + item
                    print("Processing item: " + str(item) + ", total so far: " + str(total))
                
                self.lastResult = total
                print("Processing complete, total: " + str(total))
                
                if total > 10:
                    self.counter = total
                    -> $Complete
                else:
                    -> $Running
                }
            }
            
            increment() {
                print("Cannot increment while processing")
                -> $Processing
            }
            
            reset() {
                print("Resetting from processing state")
                self.counter = 0
                self.lastResult = "reset"
                -> $Start
            }
        }
        
        $Complete {
            $>() {
                print("System completed successfully")
                self.lastResult = "complete"
            }
            
            reset() {
                print("Resetting completed system")
                self.counter = 0
                self.lastResult = "reset"
                self.isActive = true
                -> $Start
            }
            
            setState(newState) {
                if newState == "start":
                    -> $Start
                else:
                    print("Can only reset to start from complete state")
                }
            }
        }
    
    domain:
        counter = 0
        name = "TestSystem"
        items = [1, 2, 3, 4, 5]
        isActive = true
        lastResult = ""
}

# Test helper function
fn validateCounter(test_system, expectedValue) {
    print("Validating counter - expected: " + str(expectedValue))
    # This will be useful for watch expressions
    return expectedValue
}

fn main() {
    print("=== Watch Expressions and Conditional Breakpoints Test ===")
    
    watch_system = WatchTestSystem()
    
    # Test sequence 1: Basic increment with watch expressions
    print("\n--- Test 1: Basic Increment Sequence ---")
    watch_system.start()
    
    # These calls will test conditional breakpoints
    watch_system.increment()  # counter = 1
    watch_system.increment()  # counter = 2  
    watch_system.increment()  # counter = 3
    watch_system.increment()  # counter = 4
    watch_system.increment()  # counter = 5 (should trigger state transition)
    
    # Test sequence 2: Processing with watch expressions
    print("\n--- Test 2: Processing Sequence ---")
    watch_system.processItems()
    
    # Test sequence 3: Reset and repeat
    print("\n--- Test 3: Reset and Repeat ---")
    watch_system.reset()
    
    # Test conditional expressions
    watch_system.start()
    watch_system.increment()  # counter = 1
    watch_system.increment()  # counter = 2
    watch_system.decrement()  # counter = 1
    watch_system.increment()  # counter = 2
    watch_system.increment()  # counter = 3
    
    # Test state transitions
    print("\n--- Test 4: State Management ---")
    watch_system.setState("processing")
    watch_system.processItems()
    
    # Final reset
    watch_system.reset()
    
    print("\n=== Test Complete ===")
    print("Final system state and counter values ready for inspection")
}

# Manual Test Instructions for VS Code Debugging:
#
# 1. WATCH EXPRESSIONS TO TEST:
#    - counter                    (should show current counter value)
#    - currentState              (should show $Start, $Running, $Processing, $Complete)
#    - counter > 2               (boolean expression)
#    - currentState == '$Running' (state comparison)
#    - len(items)                (function call)
#    - items[0]                  (array access)
#    - name + " status"          (string concatenation)
#    - counter >= 5 and isActive (complex boolean expression)
#    - lastResult                (track result changes)
#
# 2. CONDITIONAL BREAKPOINTS TO TEST:
#    - Line with increment(): counter >= 3
#    - Line with processItems(): currentState == '$Processing'
#    - Line with state transition: counter == 5
#    - Line in for loop: item > 3
#    - Line with reset(): lastResult == "complete"
#
# 3. HIT COUNT BREAKPOINTS TO TEST:
#    - Line with increment(): hit count = 3 (break on 3rd call)
#    - Line with increment(): hit count >= 2 (break on 2nd call and after)
#    - Line with print in for loop: % 2 == 0 (break every 2nd iteration)
#    - Line with system.increment() in main: hit count = 5
#
# 4. EXPECTED BEHAVIOR:
#    - Watch expressions update in real-time during debugging
#    - Conditional breakpoints only trigger when conditions are true
#    - Hit count breakpoints follow specified patterns
#    - Complex expressions (and, or, function calls) work correctly
#    - State transitions are reflected in currentState watch expression
#    - Variable modifications are visible in watch expressions
#
# 5. DEBUGGING SEQUENCE:
#    a) Set watch expressions listed above
#    b) Set conditional breakpoint on increment() with condition: counter >= 3
#    c) Set hit count breakpoint on increment() with hit count: 3
#    d) Start debugging and step through execution
#    e) Verify watch expressions update correctly
#    f) Verify conditional breakpoint only triggers when counter >= 3
#    g) Verify hit count breakpoint triggers on 3rd call to increment()
#    h) Test expression evaluation in Debug Console
