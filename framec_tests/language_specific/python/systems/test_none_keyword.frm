# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test for None keyword standardization

fn testNoneValue() {
    var x = None
    if x == None {
        print("x is None")
    }
    return None
}

system NoneChecker {
    interface:
        check(value)
        getValue(): int
        
    machine:
        $Start {
            check(value) {
                if value == None {
                    print("Received None")
                } else {
                    print("Received value: " + str(value))
                }
                return
            }
            
            getValue(): int {
                # Return None instead of a number
                system.return = None
                return
            }
        }
        
    domain:
        data = None
}

fn main() {
    # Test function with None
    var result = testNoneValue()
    print("here")
    if result == None {
        print("Function returned None")
    }
    
    # Test system with None
    var checker = NoneChecker()
    checker.check(None)
    checker.check(42)
    
    var val = checker.getValue()
    if val == None {
        print("System returned None")
    }
    
    print("Test complete")
}
