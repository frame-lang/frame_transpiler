// Test: Multiple simple systems for v0.30 validation
// Systems with basic state transitions

system Parent {
    interface:
        start()
        process(data)
        
    machine:
        $Root {
            start() {
                -> $Active
            }
            
            process(data) {
                print("Parent processing: " + data)
            }
        }
        
        $Active {
            process(data) {
                print("Parent Active processing: " + data)
            }
        }
}

system AnotherParent {
    interface:
        begin()
        handle(msg)
        
    machine:
        $Start {
            begin() {
                -> $Running
            }
        }
        
        $Running {
            handle(msg) {
                print("Running: " + msg)
            }
        }
}