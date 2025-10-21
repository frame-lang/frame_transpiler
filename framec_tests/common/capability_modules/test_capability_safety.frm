# Comprehensive test for capability module safety improvements

import Memory from "./memory.frm"
import FileSystem from "./filesystem.frm"
import Errors from "./errors.frm"

fn testCapabilitySafety() {
    print("=== Testing Capability Module Safety Improvements ===")
    
    var allTestsPassed = True
    
    # Test 1: Memory Pool Safety
    print("\n--- Memory Pool Safety Tests ---")
    var memoryResult = testMemoryPoolSafety()
    if not memoryResult {
        allTestsPassed = False
    }
    
    # Test 2: Safe File Operations
    print("\n--- Safe File Operations Tests ---")
    var fileResult = testSafeFileOperations()
    if not fileResult {
        allTestsPassed = False
    }
    
    # Test 3: Error Handling Consistency
    print("\n--- Error Handling Consistency Tests ---")
    var errorResult = testErrorHandlingConsistency()
    if not errorResult {
        allTestsPassed = False
    }
    
    return allTestsPassed
}

fn testMemoryPoolSafety() {
    print("Testing memory pool constructor safety...")
    
    # Test valid pool creation
    var poolResult = Memory::createMemoryPool(1024, 5)
    if Errors::isError(poolResult) {
        print("FAIL: Valid memory pool creation failed: " + Errors::getError(poolResult))
        return False
    }
    
    var pool = Errors::unwrap(poolResult)
    print("SUCCESS: Memory pool created with " + str(pool["poolSize"]) + " items")
    
    # Test invalid inputs
    var invalidResult1 = Memory::createMemoryPool(-1, 5)
    if Errors::isOk(invalidResult1) {
        print("FAIL: Negative item size should be rejected")
        return False
    }
    print("SUCCESS: Negative item size correctly rejected")
    
    var invalidResult2 = Memory::createMemoryPool(1024, 0)
    if Errors::isOk(invalidResult2) {
        print("FAIL: Zero pool size should be rejected")
        return False
    }
    print("SUCCESS: Zero pool size correctly rejected")
    
    # Test allocation and return
    var allocResult = Memory::allocateFromPool(pool)
    if Errors::isError(allocResult) {
        print("FAIL: Pool allocation failed: " + Errors::getError(allocResult))
        return False
    }
    
    var item = Errors::unwrap(allocResult)
    print("SUCCESS: Item allocated from pool")
    
    var returnResult = Memory::returnToPool(pool, item)
    if Errors::isError(returnResult) {
        print("FAIL: Return to pool failed: " + Errors::getError(returnResult))
        return False
    }
    print("SUCCESS: Item returned to pool")
    
    # Test double-return protection
    var doubleReturnResult = Memory::returnToPool(pool, item)
    if Errors::isOk(doubleReturnResult) {
        print("FAIL: Double return should be prevented")
        return False
    }
    print("SUCCESS: Double return correctly prevented")
    
    print("All memory pool safety tests passed!")
    return True
}

fn testSafeFileOperations() {
    print("Testing safe file operations...")
    
    # Test safe file read
    var readResult = FileSystem::safeReadFile("test.txt")
    if Errors::isOk(readResult) {
        var content = Errors::unwrap(readResult)
        print("SUCCESS: Safe file read returned: " + content)
    } else {
        print("Note: File read failed as expected: " + Errors::getError(readResult))
    }
    
    # Test safe file write
    var writeResult = FileSystem::safeWriteFile("output.txt", "Hello, World!")
    if Errors::isError(writeResult) {
        print("FAIL: Safe file write failed: " + Errors::getError(writeResult))
        return False
    }
    print("SUCCESS: Safe file write completed")
    
    # Test safe file append
    var appendResult = FileSystem::safeAppendFile("output.txt", " More content.")
    if Errors::isError(appendResult) {
        print("FAIL: Safe file append failed: " + Errors::getError(appendResult))
        return False
    }
    print("SUCCESS: Safe file append completed")
    
    # Test RAII pattern with custom operation
    var customResult = FileSystem::withFile("custom.txt", "w", lambda handle: {
        print("Custom operation on file: " + handle["path"])
        return "custom result"
    })
    
    if Errors::isError(customResult) {
        print("FAIL: Custom file operation failed: " + Errors::getError(customResult))
        return False
    }
    
    var customValue = Errors::unwrap(customResult)
    print("SUCCESS: Custom operation returned: " + customValue)
    
    print("All safe file operation tests passed!")
    return True
}

fn testErrorHandlingConsistency() {
    print("Testing error handling consistency...")
    
    # Test Result type structure consistency
    var okResult = Errors::createOk("test value")
    if not Errors::isOk(okResult) {
        print("FAIL: createOk should create Ok result")
        return False
    }
    
    var errorResult = Errors::createError("test error")
    if not Errors::isError(errorResult) {
        print("FAIL: createError should create Error result")
        return False
    }
    
    # Test unwrap behavior
    var unwrappedValue = Errors::unwrap(okResult)
    if unwrappedValue != "test value" {
        print("FAIL: unwrap should return original value")
        return False
    }
    
    # Test unwrapOr behavior
    var defaultValue = Errors::unwrapOr(errorResult, "default")
    if defaultValue != "default" {
        print("FAIL: unwrapOr should return default on error")
        return False
    }
    
    # Test andThen chaining
    var chainResult = Errors::andThen(okResult, lambda value: {
        return Errors::createOk(value + " processed")
    })
    
    if Errors::isError(chainResult) {
        print("FAIL: andThen should chain successful results")
        return False
    }
    
    var chainedValue = Errors::unwrap(chainResult)
    if chainedValue != "test value processed" {
        print("FAIL: andThen should process value correctly")
        return False
    }
    
    print("SUCCESS: Error handling consistency verified")
    return True
}

system CapabilitySafetyTest {
    interface:
        test()
    
    machine:
        $Start {
            test() {
                var success = testCapabilitySafety()
                if success {
                    print("\n=== ALL CAPABILITY SAFETY TESTS PASSED ===")
                } else {
                    print("\n=== CAPABILITY SAFETY TESTS FAILED ===")
                    # Force test failure by raising an exception
                    var failed_tests = []
                    var index = failed_tests[999]  # This will cause an IndexError and fail the test
                }
                return
            }
        }
}