// Test examples from updated systems.rst documentation

fn main() {
    // System with no parameters
    var sys1 = NoParameters()
    
    // Start state parameters
    var sys2 = StartStateParameters("StartStateParameters started")
    
    // Start state enter parameters  
    var sys3 = StartStateEnterParameters(">StartStateEnterParameters started")
    
    // Domain parameters
    var sys4 = SystemDomainParameters("SystemDomainParameters started")
    
    // All parameter types
    var sys5 = SystemInitializationDemo("a","b","c","d","e","f")
}

// System with no parameters
system NoParameters {
    machine:
        $Start {
            $>() {
                print("NoParameters started")
                return
            }
        }
}

// Start state parameters  
system StartStateParameters ($(msg)) {
    machine:
        $Start(msg) {
            $>() {
                print(msg)
                return
            }
        }
}

// Start state enter parameters
system StartStateEnterParameters ($>(msg)) {
    machine:
        $Start {
            $>(msg) {
                print(msg)
                return
            }
        }
}

// Domain parameters
system SystemDomainParameters (msg) {
    machine:
        $Start {
            $>() {
                print(msg)
                return
            }
        }
    
    domain:
        var msg = nil 
}

// All parameter types together
system SystemInitializationDemo ($(A,B), $>(C,D), E,F) {
    machine:
        $Start(A,B) {
            $>(C,D) {
                print(A + B + C + D + E + F)
                return
            }
        }
    
    domain:
        var E = nil
        var F = nil 
}