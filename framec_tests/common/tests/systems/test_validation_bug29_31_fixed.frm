# Test case for validation system - Bug #29/#31 style issues - FIXED VERSION
# This file has the missing braces added to show validation passing

system DebuggerValidationTest {
    interface:
        start()
        getCurrentState()
        canExecuteCommand(command)

    machine:
        $Init {
            start() {
                -> $Running
            }
        }

        $Running {
            getCurrentState() {
                return "Running"
            }
            
            canExecuteCommand(command) {
                if command == "pause" {
                    return True
                }
            }
        }

        $Paused {
            getCurrentState() {
                return "Paused"  
            }
            
            canExecuteCommand(command) {
                return True
            }
        }
    }

    actions:
        log(msg) {
            print(msg)
        }
}