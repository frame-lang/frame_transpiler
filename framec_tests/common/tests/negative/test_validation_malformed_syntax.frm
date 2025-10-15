# NEGATIVE TEST - Malformed syntax with missing closing braces
# This test should fail with: "Expected '}'"

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
                # Missing closing brace here - should be caught by validation
            }
        }

        $Paused {
            getCurrentState() {
                return "Paused"  
            }
            
            canExecuteCommand(command) {
                return True
            # Missing closing brace here too - should be caught by validation
        }
    }

    actions:
        log(msg) {
            print(msg)
        }
}