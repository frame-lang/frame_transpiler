# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test file for validating proper scope isolation
# Functions should NOT be able to access system internals (actions/operations)
# Systems should NOT be able to access other systems' internals

import math

# Module-level function - accessible from anywhere
fn add(a: int, b: int): int {
    return a + b
}

# Test function that should NOT be able to call system actions
fn testFunctionIsolation() {
    print("Testing function scope isolation")
    
    # This should work - calling module-level function
    var result = add(5, 3)
    print("add(5, 3) = " + str(result))
    
    # This should work - using built-in
    print("math.pi = " + str(math.pi))
    
    # These should NOT work if scope isolation is properly implemented
    # Functions cannot call system actions or operations
    # The parser should treat these as external/undeclared calls
    # _testAction()  // Would be error if uncommented
    # testOperation() // Would be error if uncommented
}

# First system with its own actions and operations
system SystemA {
    operations:
        publicOperation() {
            print("SystemA publicOperation")
        }
        
    interface:
        test()
        
    machine:
        $Start {
            test() {
                print("SystemA test()")
                _privateAction()
                publicOperation()
                return
            }
        }
        
    actions:
        privateAction() {
            print("SystemA privateAction")
        }
}

# Second system - should NOT access SystemA's internals
system SystemB {
    operations:
        ownOperation() {
            print("SystemB ownOperation")
        }
        
    interface:
        test()
        
    machine:
        $Start {
            test() {
                print("SystemB test()")
                _ownAction()
                ownOperation()
                
                # These would be errors if uncommented - cannot access other system's internals
                # _privateAction()  // SystemA's action - not accessible
                # publicOperation() // SystemA's operation - not accessible  
                return
            }
        }
        
    actions:
        ownAction() {
            print("SystemB ownAction")
        }
}

# Test that functions can call system interface methods on instances
fn testSystemInteraction() {
    print("Testing system interaction from function")
    
    # This should work - creating system instances
    var sysA = SystemA()
    var sysB = SystemB()
    
    # This should work - calling interface methods
    sysA.test()
    sysB.test()
    
    # This should work - calling static operations with class prefix
    # SystemA.publicOperation() // Would need @staticmethod to work
    
    # This should NOT work - cannot call actions
    # sysA._privateAction() // Error - actions are private
}

# Main entry point
fn main() {
    print("=== Scope Isolation Test ===")
    
    # Test function isolation
    testFunctionIsolation()
    
    # Test system isolation
    var systemA = SystemA()
    var systemB = SystemB()
    systemA.test()
    systemB.test()
    
    # Test system interaction from function
    testSystemInteraction()
    
    print("=== Test Complete ===")
}