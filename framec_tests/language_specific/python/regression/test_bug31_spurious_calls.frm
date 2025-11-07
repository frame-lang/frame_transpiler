# Test case for Bug #31: Spurious interface method calls in event handlers
# This demonstrates unreachable getCurrentState() calls appearing in the
# wrong handlers (canExecuteCommand) after all return statements

system Bug31Test {
    
    interface:
        canExecuteCommand(command)
        getCurrentState()
        handleAction()
    
    machine:
        $Idle {
            canExecuteCommand(command) {
                if command == "start":
                    return True
                else:
                    return False
            }
            
            getCurrentState() {
                return "idle"
            }
            
            handleAction() {
                print("Starting...")
                -> $Running
            }
        }
        
        # BUG #31: This state generates spurious getCurrentState() call
        # inside the canExecuteCommand handler (unreachable code)
        $Running {
            canExecuteCommand(command) {
                if command == "continue":
                    return False  # Already running
                elif command == "step":
                    return False  # Can't step while running  
                elif command == "pause":
                    return True
                else:
                    return False
                # BUG: Transpiler adds unreachable getCurrentState() call here
            }
            
            getCurrentState() {
                return "running"
            }
            
            handleAction() {
                print("Processing...")
            }
        }
        
        # BUG #31: This state also generates spurious getCurrentState() call
        $Paused {
            canExecuteCommand(command) {
                if command in ["continue", "step"]:
                    return True
                elif command == "pause":
                    return False  # Already paused
                else:
                    return True
                # BUG: Transpiler adds unreachable getCurrentState() call here
            }
            
            getCurrentState() {
                return "paused"
            }
            
            handleAction() {
                print("Resuming...")
                -> $Running
            }
        }
    
    domain:
        status = "ready"
}

##

# EXPECTED: canExecuteCommand handlers should NOT contain getCurrentState() calls
# ACTUAL BUG: Unreachable getCurrentState() calls appear after return statements
#
# To verify Bug #31:
# framec -l python_3 test_bug31_spurious_calls.frm > test_bug31_spurious_calls.py
# 
# Look for spurious calls (these should NOT exist):
# grep -n "getCurrentState()" test_bug31_spurious_calls.py | grep -v "def "
# 
# The spurious calls appear as unreachable code after the else clause:
#   else:
#       self.return_stack[-1] = False
#       return
#   getCurrentState()  # <-- SPURIOUS/UNREACHABLE
#   return
