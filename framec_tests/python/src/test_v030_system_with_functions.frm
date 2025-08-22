// Test: System with functions (v0.30)
// Validates interaction between systems and functions

fn main() {
    var sys = TestSystem()
    sys.start()
    utility("test")
}

fn utility(msg) {
    print("Utility: " + msg)
}

system TestSystem {
    interface:
        start()
        stop()
        
    machine:
        $Idle {
            start() {
                utility("Starting system")
                -> $Running
            }
        }
        
        $Running {
            stop() {
                -> $Idle
            }
        }
}