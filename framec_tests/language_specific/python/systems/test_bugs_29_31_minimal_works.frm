@target python

# Minimal test showing that simple files work CORRECTLY
# This proves the bugs are triggered by file complexity, not syntax

system MinimalWorking {
    interface:
        canExecuteCommand(command)
        getCurrentState()
    
    machine:
        $Running {
            canExecuteCommand(command) {
                if command == "pause":
                    return True
                else:
                    return False
            }
            
            getCurrentState() {
                return "running"
            }
        }
        
        $Paused {
            canExecuteCommand(command) {
                if command == "continue":
                    return True
                else:
                    return False
            }
            
            getCurrentState() {
                return "paused"
            }
        }
}

##

# THIS SIMPLE VERSION WORKS CORRECTLY!
# 
# framec -l python_3 test_bugs_29_31_minimal_works.frm > test_bugs_29_31_minimal_works.py
#
# Verify it works:
# echo "=== Checking minimal test (should work) ==="
# grep "def __handle_running_getCurrentState" test_bugs_29_31_minimal_works.py && echo "✓ Handler exists!"
# grep -A5 "__minimalworking_state_Running" test_bugs_29_31_minimal_works.py | grep getCurrentState && echo "✓ Routing exists!"
# grep -n "getCurrentState()" test_bugs_29_31_minimal_works.py | grep -v "def " || echo "✓ No spurious calls!"
#
# This proves the bugs only occur in more complex files
