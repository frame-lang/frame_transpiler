// Test for => $^ parent dispatch with transition detection
// This test specifically validates that code after => $^ doesn't execute
// when the parent state triggers a transition

fn main() {
    var hsm = TransitionDetectionTest()
    
    print("=== Testing parent transition detection ===")
    
    // This should trigger a transition in the parent
    // Code after => $^ should NOT execute
    hsm.triggerParentTransition()
    
    // Verify we're now in the new state
    hsm.checkCurrentState()
}

system TransitionDetectionTest {
    
    interface:
        triggerParentTransition()
        checkCurrentState()
    
    machine:
        
        // Child state that dispatches to parent - THIS IS THE START STATE (first state listed)
        $Child => $Parent {
            triggerParentTransition() {
                print("Child: Before parent dispatch")
                => $^  // This should trigger transition in parent
                print("ERROR: This line should NOT execute due to parent transition!")
                return
            }
            
            checkCurrentState() {
                print("ERROR: This should not be called - we should be in NewState")
                return
            }
        }
        
        // Parent state that will transition when triggerParentTransition is called
        $Parent {
            triggerParentTransition() {
                print("Parent: Triggering transition to NewState")
                -> $NewState
                return
            }
            
            checkCurrentState() {
                print("ERROR: This should not be called - we should be in NewState")
                return
            }
        }
        
        // Target state after transition
        $NewState {
            checkCurrentState() {
                print("SUCCESS: We are correctly in NewState after parent transition")
                return
            }
            
            triggerParentTransition() {
                print("NewState: triggerParentTransition called (no action)")
                return
            }
        }
}