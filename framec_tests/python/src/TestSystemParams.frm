// Test file for v0.20 system parameter syntax
fn main() {
    // Test 1: No system parameters
    var sys1 = NoParamsSystem()
    
    // Test 2: Start state parameters only
    var sys2 = StartStateParameters("hello")
    
    // Test 3: Start state enter event parameters only  
    var sys3 = StartStateEnterParameters("world")
    
    // Test 4: Domain variable initialization only
    var sys4 = DomainVariables(1, 2)
    
    // Test 5: All parameter types together (flattened argument list)
    var sys5 = AllParameterTypes("hello", "world", 1, 2)
}

// System with no parameters
system NoParamsSystem {
    machine:
        $Start {
            $>() {
                print("NoParamsSystem started")
                return
            }
        }
}

// System with start state parameters
system StartStateParameters($(p1)) {
    machine:
        $S1(p1) { 
            $>() {
                print(p1)
                return
            }
        }
}

// System with start state enter event parameters
system StartStateEnterParameters($>(p1)) {
    machine:
        $S1 { 
            $>(p1) {
                print(p1)
                return
            }
        }
}

// System with domain variable initialization
system DomainVariables(a, c) {
    domain:
        var a = nil // a is set with a parameter value
        var b = nil 
        var c = nil // c is set with a parameter value
    
    machine:
        $Start {
            $>() {
                print(a + c)
                return
            }
        }
}

// System with all parameter types: start state, start state enter event, domain
system AllParameterTypes($(p1), $>(p2), a, c) {
    domain:
        var a = nil // a is set with a parameter value
        var b = nil 
        var c = nil // c is set with a parameter value
    
    machine:
        $S1(p1) { 
            $>(p2) {
                print(p1 + p2 + a + c)
                return
            }
        }
}