system TestParserContext {
    interface:
        terminate()
        
    machine:
        $Start {
            onData(data) {
                try {
                    if data["event"] == "connected" {
                        self.onRuntimeReady()
                    } elif data["event"] == "ready" {
                        self.onRuntimeReady()
                    } elif data["event"] == "stopped" {
                        self.onRuntimeStopped()
                    } elif data["event"] == "output" {
                        self.onRuntimeOutput()
                    } elif data["event"] == "terminated" {
                        self.onRuntimeTerminated()
                    }
                } except Exception as e {
                    print(f"Failed to parse: {e}")
                }
            }
            
            onClose() {
                print("Connection closed")
            }
            
            onError(error) {
                print(f"Error: {error}")
            }
            
            terminate() {
                self.sendTerminateCommand()
                -> $Terminating
            }
        }
        
        $Terminating {
            $>() {
                print("Terminating")
            }
        }
    
    actions:
        sendTerminateCommand() {}
        onRuntimeReady() {}
        onRuntimeStopped() {}
        onRuntimeOutput() {}
        onRuntimeTerminated() {}
}