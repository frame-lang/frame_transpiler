@target rust

# @run-expect: SUCCESS
# @run-expect: Hello from Docker

system SimpleDocker {
    interface:
        run()
    
    machine:
        $Start {
            run() {
                println!("SUCCESS: Hello from Docker");
                -> $End
            }
        }
        
        $End {}
}