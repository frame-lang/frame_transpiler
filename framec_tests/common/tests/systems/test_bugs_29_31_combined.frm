# Combined test case for Bug #29 and Bug #31
# This demonstrates that the SAME methods that are:
# 1. Missing their handlers (Bug #29)
# 2. Appearing as spurious calls in wrong handlers (Bug #31)
#
# This suggests the transpiler is misplacing getCurrentState code

system CombinedBugTest {
    
    interface:
        # These methods should each get their own handler
        canExecuteCommand(command)
        getCurrentState()
    
    machine:
        # Control state - works correctly
        $Idle {
            canExecuteCommand(command) {
                return command == "start"
            }
            
            getCurrentState() {
                return "idle"
            }
        }
        
        # BOTH BUGS MANIFEST HERE:
        # Bug #29: getCurrentState handler NOT generated
        # Bug #31: getCurrentState() call appears in canExecuteCommand
        $Running {
            canExecuteCommand(command) {
                if command == "pause" {
                    return True
                } else {
                    return False
                }
                # BUG #31: Transpiler incorrectly adds getCurrentState() here
            }
            
            # BUG #29: This handler is NOT generated in the Python output
            getCurrentState() {
                return "running"
            }
        }
        
        # BOTH BUGS MANIFEST HERE TOO:
        $Paused {
            canExecuteCommand(command) {
                if command == "continue" {
                    return True
                } else {
                    return False  
                }
                # BUG #31: Transpiler incorrectly adds getCurrentState() here
            }
            
            # BUG #29: This handler is NOT generated in the Python output
            getCurrentState() {
                return "paused"
            }
        }
}

##

# VERIFICATION SCRIPT:
# framec -l python_3 test_bugs_29_31_combined.frm > test_bugs_29_31_combined.py
#
# Check Bug #29 (missing handlers):
# echo "=== BUG #29: Missing handlers ==="
# grep "def __handle_running_getCurrentState" test_bugs_29_31_combined.py || echo "MISSING!"
# grep "def __handle_paused_getCurrentState" test_bugs_29_31_combined.py || echo "MISSING!"
#
# Check Bug #29 (missing routing):
# echo "=== BUG #29: Missing routing ==="
# grep -A8 "def __combinedbugtest_state_Running" test_bugs_29_31_combined.py | grep getCurrentState || echo "ROUTING MISSING!"
#
# Check Bug #31 (spurious calls):
# echo "=== BUG #31: Spurious calls ==="
# grep -n "getCurrentState()" test_bugs_29_31_combined.py | grep -v "def "
#
# HYPOTHESIS: The transpiler is trying to process getCurrentState but:
# 1. Fails to generate the handler method (Bug #29)
# 2. Instead places the call in the previous handler (Bug #31)