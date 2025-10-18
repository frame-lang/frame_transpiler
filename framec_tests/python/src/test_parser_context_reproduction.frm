system FrameDebugAdapter {
    interface:
        terminate()
        onRuntimeConnected()
        onRuntimeReady()
        onRuntimeStopped()
        onRuntimeOutput()
        onRuntimeTerminated()
        
    machine:
        $Connecting {
            onData(data) {
                try {
                    if data["event"] == "connected" {
                        system.onRuntimeConnected()
                    } elif data["event"] == "ready" {
                        system.onRuntimeReady()
                    } elif data["event"] == "stopped" {
                        system.onRuntimeStopped()
                    } elif data["event"] == "output" {
                        system.onRuntimeOutput()
                    } elif data["event"] == "terminated" {
                        system.onRuntimeTerminated()
                    }
                } except Exception as e {
                    print(f"Failed to parse runtime message: {e}")
                }
            }
            
            onClose() {
                print("Runtime connection closed")
            }
            
            onError(error) {
                print(f"Runtime connection error: {error}")
            }
            
            onTimeout() {
                print("Connection timeout - retrying...")
            }
            
            terminate() {
                print("Terminating")
            }
        }
    
    actions:
        onRuntimeConnected() {}
        onRuntimeStopped(reason, threadId, text) {}
        onRuntimeOutput(output, category) {}
        onRuntimeTerminated(exitCode) {}
}