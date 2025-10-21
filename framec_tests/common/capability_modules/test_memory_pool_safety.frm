# Test for memory pool constructor safety improvements

import Memory from "./memory.frm"
import Errors from "./errors.frm"

fn testMemoryPoolSafety() {
    print("=== Testing Memory Pool Constructor Safety ===")
    
    # Test 1: Valid pool creation
    var result1 = Memory::createMemoryPool(1024, 10)
    if Errors::isOk(result1) {
        print("SUCCESS: Valid pool creation works")
        var pool = Errors::unwrap(result1)
        print("Pool created with size: " + str(pool["poolSize"]))
    } else {
        print("FAIL: Valid pool creation failed: " + Errors::getError(result1))
        return False
    }
    
    # Test 2: Invalid item size (negative)
    var result2 = Memory::createMemoryPool(-1, 10)
    if Errors::isError(result2) {
        print("SUCCESS: Negative item size correctly rejected")
        print("Error: " + Errors::getError(result2))
    } else {
        print("FAIL: Negative item size should be rejected")
        return False
    }
    
    # Test 3: Invalid pool size (zero)
    var result3 = Memory::createMemoryPool(1024, 0)
    if Errors::isError(result3) {
        print("SUCCESS: Zero pool size correctly rejected")
        print("Error: " + Errors::getError(result3))
    } else {
        print("FAIL: Zero pool size should be rejected")
        return False
    }
    
    # Test 4: Pool allocation and return
    if Errors::isOk(result1) {
        var pool = Errors::unwrap(result1)
        
        # Allocate from pool
        var allocResult = Memory::allocateFromPool(pool)
        if Errors::isOk(allocResult) {
            print("SUCCESS: Pool allocation works")
            var item = Errors::unwrap(allocResult)
            
            # Return to pool
            var returnResult = Memory::returnToPool(pool, item)
            if Errors::isOk(returnResult) {
                print("SUCCESS: Pool return works")
            } else {
                print("FAIL: Pool return failed: " + Errors::getError(returnResult))
                return False
            }
            
            # Test double return (should fail)
            var doubleReturnResult = Memory::returnToPool(pool, item)
            if Errors::isError(doubleReturnResult) {
                print("SUCCESS: Double return correctly rejected")
                print("Error: " + Errors::getError(doubleReturnResult))
            } else {
                print("FAIL: Double return should be rejected")
                return False
            }
        } else {
            print("FAIL: Pool allocation failed: " + Errors::getError(allocResult))
            return False
        }
    }
    
    print("=== All Memory Pool Safety Tests Passed ===")
    return True
}

system MemoryPoolTest {
    interface:
        test()
    
    machine:
        $Start {
            test() {
                var success = testMemoryPoolSafety()
                if success {
                    print("=== Memory Pool Safety Test PASSED ===")
                } else {
                    print("=== Memory Pool Safety Test FAILED ===")
                    # Force test failure by raising an exception
                    var failed_tests = []
                    var index = failed_tests[999]  # This will cause an IndexError and fail the test
                }
                return
            }
        }
}