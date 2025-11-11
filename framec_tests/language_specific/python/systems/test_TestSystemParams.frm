@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test file for v0.20 system parameter syntax
fn main() {
    # Test 1: No system parameters
    sys1 = NoParamsSystem()
    
    # Test 2: Start state parameters only
    sys2 = StartStateParameters("hello")
    
    # Test 3: Start state enter event parameters only  
    sys3 = StartStateEnterParameters("world")
    
    # Test 4: Domain variable initialization only
    sys4 = DomainVariables(1, 2)
    
    # Test 5: All parameter types together (flattened argument list)
    sys5 = AllParameterTypes(10, 20, 1, 2)
}

# System with no parameters
system NoParamsSystem {
    machine:
        $Start {
            $>() {
                print("NoParamsSystem started")
                return
            }
        }
}

# System with start state parameters
system StartStateParameters($(p1)) {
    machine:
        $S1(p1) { 
            $>() {
                print(p1)
                return
            }
        }
}

# System with start state enter event parameters
system StartStateEnterParameters($>(p1)) {
    machine:
        $S1 { 
            $>(p1) {
                print(p1)
                return
            }
        }
}

# System with domain variable initialization
system DomainVariables(a, c) {
    machine:
        $Start {
            $>() {
                print(a + c)
                return
            }
        }
    
    domain:
        a = None # a is set with a parameter value
        b = None 
        c = None # c is set with a parameter value
}

# System with all parameter types: start state, start state enter event, domain
system AllParameterTypes($(p1), $>(p2), a, c) {
    machine:
        $S1(p1) { 
            $>(p2) {
                print(p1 + p2 + a + c)
                return
            }
        }
    
    domain:
        a = None # a is set with a parameter value
        b = None 
        c = None # c is set with a parameter value
}
