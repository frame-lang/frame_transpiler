# Test case for Bug #29: Missing interface method event routing
# This test demonstrates that getCurrentState handlers are missing 
# from the generated Python code for certain states in complex files

system Bug29Test {
    
    interface:
        # Basic lifecycle
        initialize(port)
        connect()
        disconnect()
        
        # Debug commands
        handleContinue()
        handleStep()
        handleBreakpoint(line)
        
        # Query state - THIS IS THE PROBLEM METHOD
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
                print(f"Attempting to connect to port {self.debugPort}")
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
                print(f"Adding breakpoint at line {line}")
                self.breakpoints.append(line)
            }
            
            getCurrentState() {
                return "initializing"
            }
        }
        
        # BUG #29: getCurrentState handler SHOULD be generated but ISN'T
        $Running {
            handleContinue() {
                print("Already running - ignoring continue")
            }
            
            handleStep() {
                print("Cannot step while running")
                return False
            }
            
            handleBreakpoint(line) {
                if line in self.breakpoints {
                    print(f"Hit breakpoint at line {line}")
                    self.currentLine = line
                    -> $Paused
                } else {
                    print(f"Line {line} is not a breakpoint")
                }
            }
            
            canExecuteCommand(command) {
                if command == "continue" {
                    return False
                } elif command == "step" {
                    return False
                } elif command == "pause" {
                    return True
                } else {
                    return False
                }
            }
            
            # THIS HANDLER SHOULD BE GENERATED BUT ISN'T
            getCurrentState() {
                return "running"
            }
            
            disconnect() {
                -> $Disconnecting
            }
        }
        
        # BUG #29: getCurrentState handler SHOULD be generated but ISN'T
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
                -> $Stepping
            }
            
            canExecuteCommand(command) {
                if command in ["continue", "step", "stepOver", "stepOut"] {
                    return True
                } elif command == "pause" {
                    return False
                } else {
                    return True
                }
            }
            
            # THIS HANDLER SHOULD BE GENERATED BUT ISN'T
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
            if line not in self.breakpoints {
                self.breakpoints.append(line)
                print(f"Breakpoint added at line {line}")
            }
        }
        
        removeBreakpoint(line) {
            if line in self.breakpoints {
                self.breakpoints.remove(line)
                print(f"Breakpoint removed from line {line}")
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

##

# EXPECTED: All states should have getCurrentState routing in their dispatchers
# ACTUAL BUG: Running and Paused states are missing getCurrentState routing
# 
# To verify:
# framec -l python_3 test_bug29_missing_routing.frm > test_bug29_missing_routing.py
# grep -A10 "__bug29test_state_Running" test_bug29_missing_routing.py | grep getCurrentState
# ^ Should find routing but doesn't
#
# grep "def __handle_running_getCurrentState" test_bug29_missing_routing.py  
# ^ Should find handler but doesn't