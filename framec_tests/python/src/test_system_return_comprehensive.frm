// Comprehensive test for system return semantics

system SystemReturnTest {
    interface:
        // Method with default return value
        getDefault() : int = 42
        
        // Method with handler override
        getOverride() : int = 10
        
        // Method that uses action to set return
        getFromAction() : string = "default"
        
        // Method without default (should be None/null)
        getNoDefault() : int
        
    machine:
        $Start {
            // Uses interface default (42)
            getDefault() {
                return
            }
            
            // Overrides with event handler value
            getOverride() : int = 99 {
                // Handler default overrides interface default
                return
            }
            
            // Action sets system.return
            getFromAction() {
                callAction()
                return
            }
            
            // No default anywhere
            getNoDefault() {
                return
            }
        }
        
    actions:
        callAction() {
            // Action explicitly sets system.return
            system.return = "from_action"
            return
        }
}

fn main() {
    var tester = SystemReturnTest()
    
    // Test 1: Interface default
    var result1 = tester.getDefault()
    print("Test 1 - Interface default: " + str(result1))
    if result1 == 42 {
        print("  PASS: Got interface default 42")
    } else {
        print("  FAIL: Expected 42, got " + str(result1))
    }
    
    // Test 2: Handler override
    var result2 = tester.getOverride()
    print("Test 2 - Handler override: " + str(result2))
    if result2 == 99 {
        print("  PASS: Got handler override 99")
    } else {
        print("  FAIL: Expected 99, got " + str(result2))
    }
    
    // Test 3: Action sets return
    var result3 = tester.getFromAction()
    print("Test 3 - Action sets return: " + str(result3))
    if result3 == "from_action" {
        print("  PASS: Got action value 'from_action'")
    } else {
        print("  FAIL: Expected 'from_action', got " + str(result3))
    }
    
    // Test 4: No default
    var result4 = tester.getNoDefault()
    print("Test 4 - No default: " + str(result4))
    if result4 == null {
        print("  PASS: Got null/None as expected")
    } else {
        print("  FAIL: Expected null/None, got " + str(result4))
    }
    
    print("\n=== Test Summary ===")
    print("All tests completed")
}