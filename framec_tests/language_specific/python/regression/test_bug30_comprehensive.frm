# Frame Protocol - Minimal Proof of Concept
# This will be transpiled to Python to test the approach

system MinimalDebugProtocol {
    
    interface:
        # Basic lifecycle
        initialize(port)
        connect()
        disconnect()
        
        # Debug commands
        handleContinue()
        handleStep()
        handleBreakpoint(line)
        
        # Query state
        canExecuteCommand(command)
        getCurrentState()
    
    machine:
        $Disconnected {
            initialize(port) {
                print(f"Initializing with port {port}")
                self.debugPort = port
                -> $Connecting
            }
            
            connect() {
                print("Cannot connect - not initialized")
                # Stay in $Disconnected
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
                # Entry action - attempt connection
                print(f"Attempting to connect to port {self.debugPort}")
                # In real implementation, would start socket connection
                self.connectionAttempts = self.connectionAttempts + 1
            }
            
            connect() {
                # Simulate successful connection
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
                # Would send breakpoints, source maps, etc.
            }
            
            handleContinue() {
                print("Starting execution")
                -> $Running
            }
            
            handleBreakpoint(line) {
                print(f"Adding breakpoint at line {line}")
                self.breakpoints.append(line)
                # Stay in $Initializing
            }
            
            getCurrentState() {
                return "initializing"
            }
        }
        
        $Running {
            handleContinue() {
                print("Already running - ignoring continue")
                # Stay in $Running
            }
            
            handleStep() {
                print("Cannot step while running")
                return False
            }
            
            handleBreakpoint(line) {
                if line in self.breakpoints:
                    print(f"Hit breakpoint at line {line}")
                    self.currentLine = line
                    -> $Paused
                else:
                    print(f"Line {line} is not a breakpoint")
            }
            
            canExecuteCommand(command) {
                if command == "continue":
                    return False  # Already running
                elif command == "step":
                    return False  # Can't step while running
                elif command == "pause":
                    return True
                else:
                    return False
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
                print(f"Paused at line {self.currentLine}")
            }
            
            handleContinue() {
                print("Resuming execution")
                -> $Running
            }
            
            handleStep() {
                print("Stepping to next line")
                # In real implementation, would set step mode
                -> $Stepping
            }
            
            canExecuteCommand(command) {
                if command in ["continue", "step", "stepOver", "stepOut"]:
                    return True
                elif command == "pause":
                    return False  # Already paused
                else:
                    return True  # Most commands valid when paused
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
                # Simulate step completion
                self.currentLine = self.currentLine + 1
            }
            
            handleBreakpoint(line) {
                # Step complete, now paused
                self.currentLine = line
                -> $Paused
            }
            
            handleContinue() {
                print("Step interrupted by continue")
                -> $Running
            }
            
            canExecuteCommand(command) {
                return False  # No commands during step
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
        # Helper methods that don't change state
        
        addBreakpoint(line) {
            if line not in self.breakpoints:
                self.breakpoints.append(line)
                print(f"Breakpoint added at line {line}")
        }
        
        removeBreakpoint(line) {
            if line in self.breakpoints:
                self.breakpoints.remove(line)
                print(f"Breakpoint removed from line {line}")
        }
        
        getBreakpoints() {
            return self.breakpoints
        }
    
    domain:
        # State variables
        debugPort = 0
        breakpoints = []
        currentLine = 0
        connectionAttempts = 0
}
