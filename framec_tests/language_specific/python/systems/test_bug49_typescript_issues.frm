// Test case for Bug #49: TypeScript Generation Issues
// This test reproduces the issues described in the bug report

system DebugAdapter {
    interface:
        initialize() -> Bool
        launch() -> Bool
        setBreakpoints() -> Bool
        continue()
        step()
        evaluate(expression: string) -> string
        disconnect()
    
    machine:
        $Idle {
            initialize() {
                if self.checkConfiguration():
                    -> $Configuring
                else:
                    return False
                }
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
            }
            
            continue() {
                self.resumeExecution()
            }
            
            step() {
                self.stepExecution()
            }
            
            evaluate(expression: string) {
                result = self.evaluateExpression(expression)
                return result
            }
            
            disconnect() {
                self.cleanup()
                -> $Idle
            }
        }
    
    actions:
        checkConfiguration() -> Bool
        startServer() -> Bool  
        validateBreakpoints() -> Bool
        resumeExecution()
        stepExecution()
        evaluateExpression(expr: string) -> string
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
