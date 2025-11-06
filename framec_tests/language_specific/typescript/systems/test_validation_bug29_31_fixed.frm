# Validation system test — TypeScript native bodies

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
            
            getCurrentState() {
                return "Init"
            }
        }

        $Running {
            getCurrentState() {
                return "Running"
            }

            canExecuteCommand(command) {
                if (command === "pause") {
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
