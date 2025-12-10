@target typescript

# @run-expect: SUCCESS
# @run-expect: Hello from Docker

system SimpleDocker {
    interface:
        run(): void
    
    machine:
        $Start {
            run() {
                console.log("SUCCESS: Hello from Docker")
                -> $End
            }
        }
        
        $End {}
}