# Validation system test — Python native bodies

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
                if command == "pause":
                    return True
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

