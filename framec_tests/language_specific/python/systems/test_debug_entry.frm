# Test Stop on Entry - Line 41 should be the first executable line in main

fn testFunction(value) {
    print("  In testFunction with value: " + str(value))
    result = value * 2
    return result
}

system SimpleSystem {
    interface:
        start()
        process(value)
        
    machine:
        $Start {
            start() {
                print("  System starting")
                -> $Running
            }
            
            process(value) {
                print("  Cannot process in Start state")
                return
            }
        }
        
        $Running {
            start() {
                print("  Already running")
                return
            }
            
            process(value) {
                print("  Processing value: " + str(value))
                result = value + 10
                system.return = result
                return
            }
        }
        
    domain:
        data = 0
}

fn main() {
    print("Line 47: Starting main function")  # This should be line 47
    print("Line 48: Setting up initial state")
    
    x = 100
    y = 200
    print("Line 52: x = " + str(x) + ", y = " + str(y))
    
    # Test function call
    result = testFunction(x)
    print("Line 56: Function returned: " + str(result))
    
    # Test system
    sys = SimpleSystem()
    sys.start()
    sys_result = sys.process(42)
    print("Line 62: System returned: " + str(sys_result))
    
    # Some control flow
    if x > 50:
        print("Line 66: x is greater than 50")
    else:
        print("Line 68: x is not greater than 50")
    }
    
    # Loop
    i = 0
    while i < 3:
        print("Line 74: Loop iteration " + str(i))
        i = i + 1
    }
    
    print("Line 78: Program ending")
    return
}
