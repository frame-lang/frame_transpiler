// Bug #50 Regression Test: Large Frame File Parser Validation
// 
// This test validates that the parser correctly handles large Frame files
// with complex machine blocks containing multiple states and event handlers.
// 
// Bug #50 originally manifested as:
// 1. Parser issues with large files (900+ lines)
// 2. "Module-level function calls" error for valid interface methods
// 3. Token synchronization issues in event handler parsing
//
// This test ensures the parser fix remains stable for production-sized Frame files.

system FrameDebugAdapterRegression {
    
    interface:
        // Basic DAP command interface - should not trigger "module-level function calls" error
        initialize(args)
        launch(args)
        setBreakpoints(source, lines)
        configurationDone()
        continueExecution(threadId)
        nextStep(threadId)
        stepInto(threadId)
        stepOutOf(threadId)
        pause(threadId)
        disconnect()
        terminate()  // This method specifically triggered Bug #50
        restart()
        
        // VS Code integration methods
        setVSCodeSession(session)
        
        // Runtime events - complex interface for parser stress testing
        onRuntimeConnected()
        onRuntimeReady()
        onRuntimeStopped(reason, threadId, text)
        onRuntimeOutput(output, category)
        onRuntimeTerminated(exitCode)
        onRuntimeError(message)
        
        // Socket event handlers
        onData(data)
        onClose()
        onError(error)
        
    machine:
        // Large machine block with multiple states to stress test parser
        // Original bug occurred around this complexity level
        
        $Initializing {
            initialize(args) {
                print("Frame Debug adapter initializing...")
                self.clientId = args
                -> $Ready
            }
            
            terminate() {
                print("Terminating from Initializing")
                -> $Terminated
            }
            
            onError(error) {
                print("Initialization error")
                -> $Error
            }
        }
        
        $Ready {
            launch(args) {
                print("Launching Frame program...")
                self.program = args
                -> $Launching
            }
            
            setBreakpoints(source, lines) {
                print("Setting breakpoints")
                return True
            }
            
            terminate() {
                print("Terminating from Ready")
                -> $Terminated
            }
            
            onError(error) {
                print("Ready state error")
                -> $Error
            }
        }
        
        $Launching {
            configurationDone() {
                print("Configuration complete, starting execution...")
                -> $Running
            }
            
            terminate() {
                print("Terminating from Launching")
                -> $Terminated
            }
            
            onError(error) {
                print("Launch error")
                -> $Error
            }
        }
        
        $Running {
            continueExecution(threadId) {
                print("Continue execution")
                return True
            }
            
            nextStep(threadId) {
                print("Next step")
                return False
            }
            
            stepInto(threadId) {
                print("Step into")
                return False
            }
            
            stepOutOf(threadId) {
                print("Step out")
                return False
            }
            
            pause(threadId) {
                print("Pause execution")
                return True
            }
            
            onRuntimeStopped(reason, threadId, text) {
                print("Runtime stopped")
                -> $Stopped
            }
            
            onRuntimeTerminated(exitCode) {
                print("Runtime terminated")
                -> $Terminated
            }
            
            terminate() {
                print("Terminating from Running")
                -> $Terminated
            }
            
            onError(error) {
                print("Runtime error")
                -> $Error
            }
        }
        
        $Stopped {
            continueExecution(threadId) {
                print("Resume execution")
                -> $Running
            }
            
            nextStep(threadId) {
                print("Next step from stopped")
                -> $Running
            }
            
            stepInto(threadId) {
                print("Step into from stopped")
                -> $Running
            }
            
            stepOutOf(threadId) {
                print("Step out from stopped")
                -> $Running
            }
            
            onRuntimeTerminated(exitCode) {
                print("Runtime terminated from stopped")
                -> $Terminated
            }
            
            terminate() {
                print("Terminating from Stopped")
                -> $Terminated
            }
            
            onError(error) {
                print("Stopped state error")
                -> $Error
            }
        }
        
        $Error {
            terminate() {
                print("Terminating from Error")
                -> $Terminated
            }
            
            restart() {
                print("Restarting from Error")
                -> $Initializing
            }
        }
        
        $Terminated {
            // Final state - no transitions allowed
            // This tests parser handling of terminal states
        }
        
    actions:
        // Complex actions block to increase file size and parser complexity
        
        initializeTranspiler() {
            print("Initialize transpiler")
        }
        
        setupTranspilerConfig() {
            print("Setup transpiler config")
        }
        
        sendInitializeResponse() {
            print("Send initialize response")
        }
        
        sendResponse(command, body) {
            print("Send response: " + command)
        }
        
        sendEvent(event, body) {
            print("Send event: " + event)
        }
        
        sendDebugConsole(message) {
            print("Debug console: " + message)
        }
        
        sendStoppedEvent(reason, threadId, text) {
            print("Send stopped event")
        }
        
        sendTerminatedEvent() {
            print("Send terminated event")
        }
        
        setupBreakpoints() {
            print("Setup breakpoints")
        }
        
        validateBreakpoints(source, lines) {
            print("Validate breakpoints")
        }
        
        startPythonRuntime() {
            print("Start Python runtime")
        }
        
        setupRuntimeCallbacks() {
            print("Setup runtime callbacks")
        }
        
        stopPythonRuntime() {
            print("Stop Python runtime")
        }
        
        sendRuntimeCommand(command, args) {
            print("Send runtime command: " + command)
        }
        
        logError(message) {
            print("ERROR: " + message)
        }
        
        cleanup() {
            print("Cleanup resources")
        }
        
        clearBreakpoints() {
            print("Clear breakpoints")
        }
        
        reset() {
            print("Reset adapter")
        }
        
    domain:
        // Complex domain block with multiple variables
        var clientId: str
        var adapterID: str
        var program: str
        var seq: int
}