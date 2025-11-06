# Minimal Debug Protocol — TypeScript native bodies

system MinimalDebugProtocol {
    interface:
        initialize(port)
        connect()
        disconnect()
        handleContinue()
        handleStep()
        handleBreakpoint(line)
        canExecuteCommand(command)
        getCurrentState()

    machine:
        $Disconnected {
            initialize(port) {
                print("Initializing with port " + str(port))
                self.debugPort = port
                -> $Connecting
            }

            connect() {
                print("Cannot connect - not initialized")
            }

            handleContinue() {
                print("Cannot continue - not connected")
            }

            getCurrentState() {
                return "disconnected"
            }
        }

        $Connecting {
            $>() {
                // Entry action - attempt connection
                print("Attempting to connect to port " + str(self.debugPort))
                self.connectionAttempts = self.connectionAttempts + 1
            }

            connect() {
                print("Connection established")
                -> $Initializing
            }

            disconnect() {
                print("Aborting connection attempt")
                -> $Disconnected
            }

            getCurrentState() {
                return "connecting"
            }
        }

        $Initializing {
            $>() {
                print("Sending initialization data")
            }

            handleContinue() {
                print("Starting execution")
                -> $Running
            }

            handleBreakpoint(line) {
                print("Adding breakpoint at line " + str(line))
                self.breakpoints.append(line)
            }

            getCurrentState() {
                return "initializing"
            }
        }

        $Running {
            handleContinue() {
                print("Already running - ignoring continue")
            }

            handleStep() {
                print("Cannot step while running")
                return False
            }

            handleBreakpoint(line) {
                if (self.breakpoints.includes(line)) {
                    print("Hit breakpoint at line " + str(line))
                    self.currentLine = line
                    -> $Paused
                } else {
                    print("Line " + str(line) + " is not a breakpoint")
                }
            }

            canExecuteCommand(command) {
                if (command === "continue") {
                    return False
                } else if (command === "step") {
                    return False
                } else if (command === "pause") {
                    return True
                } else {
                    return False
                }
            }

            getCurrentState() {
                return "running"
            }

            disconnect() {
                -> $Disconnecting
            }
        }

        $Paused {
            $>() {
                print("Paused at line " + str(self.currentLine))
            }

            handleContinue() {
                print("Resuming execution")
                -> $Running
            }

            handleStep() {
                print("Stepping to next line")
                -> $Stepping
            }

            canExecuteCommand(command) {
                if (["continue", "step", "stepOver", "stepOut"].includes(command)) {
                    return True
                } else if (command === "pause") {
                    return False
                } else {
                    return True
                }
            }

            getCurrentState() {
                return "paused"
            }

            disconnect() {
                -> $Disconnecting
            }
        }

        $Stepping {
            $>() {
                print("Executing step operation")
                self.currentLine = self.currentLine + 1
            }

            handleBreakpoint(line) {
                self.currentLine = line
                -> $Paused
            }

            handleContinue() {
                print("Step interrupted by continue")
                -> $Running
            }

            canExecuteCommand(command) {
                return False
            }

            getCurrentState() {
                return "stepping"
            }
        }

        $Disconnecting {
            $>() {
                print("Closing connection")
                self.debugPort = 0
                self.breakpoints = []
                self.currentLine = 0
            }

            disconnect() {
                print("Cleanup complete")
                -> $Disconnected
            }

            getCurrentState() {
                return "disconnecting"
            }
        }

    actions:
        addBreakpoint(line) {
            if (!self.breakpoints.includes(line)) {
                self.breakpoints.append(line)
                print("Breakpoint added at line " + str(line))
            }
        }

        removeBreakpoint(line) {
            if (self.breakpoints.includes(line)) {
                self.breakpoints.remove(line)
                print("Breakpoint removed from line " + str(line))
            }
        }

        getBreakpoints() {
            return self.breakpoints
        }

    domain:
        var debugPort = 0
        var breakpoints = []
        var currentLine = 0
        var connectionAttempts = 0
}

