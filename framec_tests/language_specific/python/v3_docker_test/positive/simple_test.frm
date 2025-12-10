@target python_3

# @run-expect: SUCCESS
# @run-expect: Hello from Docker

system SimpleDocker {
    interface:
        run()
    
    machine:
        $Start {
            run() {
                print("SUCCESS: Hello from Docker")
                -> $End
            }
        }
        
        $End {}
}