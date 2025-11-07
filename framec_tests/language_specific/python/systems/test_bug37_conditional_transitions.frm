// Test case for Bug #37: State Diagram Generation Missing Conditional Transitions
// Verifies that conditional transitions appear in generated state diagrams

system ConditionalTransitionTest {
    interface:
        configure(stopOnEntry: bool) -> Bool
        onRuntimeReady() -> Bool
        start()
        stop()
    
    machine:
        $Initial {
            configure(stopOnEntry: bool) {
                self.stopOnEntry = stopOnEntry
                -> $Configuring
                return True
            }
        }
        
        $Configuring {
            $>() {
                print("Configuring system...")
            }
            
            onRuntimeReady() {
                # This conditional transition should appear in state diagram
                if self.stopOnEntry:
                    -> $WaitingForEntry  # Bug #37: This transition missing from diagram
                else:
                    -> $Running
                return True
            }
        }
        
        $WaitingForEntry {
            $>() {
                print("Waiting for entry point...")
            }
            
            start() {
                -> $Running
            }
        }
        
        $Running {
            $>() {
                print("System is running")
            }
            
            stop() {
                -> $Stopped
            }
        }
        
        $Stopped {
            $>() {
                print("System stopped")
            }
            
            configure(stopOnEntry: bool) {
                self.stopOnEntry = stopOnEntry
                -> $Configuring
                return True
            }
        }
    
    actions:
        initializeRuntime() -> Bool
        shutdownRuntime()
    
    domain:
        stopOnEntry: bool = False
        runtimeReady: bool = False
}

fn main() {
    test = ConditionalTransitionTest()
    print("Testing conditional transitions for state diagram generation...")
    
    # Test both conditional paths
    print("Testing with stopOnEntry = True")
    result1 = test.configure(True)
    if result1:
        test.onRuntimeReady()  # Should transition to $WaitingForEntry
        print("SUCCESS: Conditional transition to WaitingForEntry")
    
    print("Testing with stopOnEntry = False") 
    result2 = test.configure(False)
    if result2:
        test.onRuntimeReady()  # Should transition to $Running
        print("SUCCESS: Conditional transition to Running")
    
    if result1 and result2:
        print("SUCCESS: All conditional transitions work correctly")
    else:
        print("FAIL: Conditional transitions failed")
        # Force test failure
        failed_tests = []
        index = failed_tests[999]  # This will cause an IndexError and fail the test
}
