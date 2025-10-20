# Phase 2: Basic state transitions

system SimpleStateMachine {
    interface:
        start(): string
        stop(): string

    machine:
        $Stopped {
            start() {
                system.return = "Starting..."
                -> $Running
            }
        }
        
        $Running {
            stop() {
                system.return = "Stopping..."
                -> $Stopped
            }
        }
}