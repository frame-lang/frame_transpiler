# Phase 3: Comprehensive test with all features

system CompleteSystem {
    operations:
        getCurrentStatus(): string {
            return status + " - Count: " + str(processed_count)
        }
    
    interface:
        start(): bool
        process(input: string): string
        getStatus(): string
        stop(): bool
    
    machine:
        $Idle {
            start() {
                status = "starting"
                initializeSystem()
                system.return = true
                -> $Running
            }
        }
        
        $Running {
            process(input: string) {
                processed_count = processed_count + 1
                var result = processData(input)
                last_result = result
                system.return = result
                return
            }
            
            getStatus() {
                system.return = getCurrentStatus()
                return
            }
            
            stop() {
                status = "stopping"
                cleanupSystem()
                system.return = true
                -> $Idle
            }
        }
    
    actions:
        initializeSystem() {
            status = "initialized"
            processed_count = 0
        }
        
        processData(input: string): string {
            return "Processed: " + input + " (#" + str(processed_count) + ")"
        }
        
        cleanupSystem() {
            status = "cleaned"
            last_result = ""
        }
    
    domain:
        var status: string = "idle"
        var processed_count: int = 0
        var last_result: string = ""
}