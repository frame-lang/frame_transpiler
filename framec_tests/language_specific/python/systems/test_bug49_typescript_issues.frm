// Test case for Bug #49: TypeScript Generation Issues
// This test reproduces the issues described in the bug report

system DebugAdapter {
    interface:
        initialize(): bool
        launch(): bool
        setBreakpoints(): bool
        continue()
        step()
        evaluate(expression: str): str
        disconnect()
    
    machine:
        $Idle {
            initialize() {
                if self.checkConfiguration():
                    -> $Configuring
                else:
                    return False
                return True
            }
        }
        
        $Configuring {
            launch() {
                self.startServer()
                -> $Running
                return True
            }
        }
        
        $Running {
            setBreakpoints() {
                if self.validateBreakpoints():
                    return True
                else:
                    return False
            }
            
            continue() {
                self.resumeExecution()
            }
            
            step() {
                self.stepExecution()
            }
            
            evaluate(expression: str) {
                result = self.evaluateExpression(expression)
                return result
            }
            
            disconnect() {
                self.cleanup()
                -> $Idle
            }
        }
    
    actions:
        checkConfiguration(): bool
        startServer(): bool  
        validateBreakpoints(): bool
        resumeExecution()
        stepExecution()
        evaluateExpression(expr: str): str
        cleanup()
    
    domain:
        adapterID: string = "frame-debug-adapter"
        serverPort: int = 0
        isRunning: bool = False
        breakpoints: list = []
}

fn main() {
    adapter = DebugAdapter()
    print("Debug adapter created")
    print("Adapter ID: " + adapter.adapterID)
}
