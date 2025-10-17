# Test Frame system that uses capability modules
# This validates that capability modules can be imported and used

import Errors from "./errors_simple.frm"
import Collections from "./collections_simple.frm"
import FileSystem from "./filesystem_simple.frm"
import AsyncSupport from "./async_simple.frm"

system CapabilityTester {
    interface:
        testCapabilities()
    
    machine:
        $Start {
            testCapabilities() {
                print("=== Testing Capability Modules ===")
                
                # Test errors module
                var okResult = Errors::createOk("test value")
                var errorResult = Errors::createError("test error")
                
                if Errors::isOk(okResult) {
                    print("SUCCESS: Errors module OK result works")
                } else {
                    print("FAIL: Errors module OK result failed")
                }
                
                if Errors::isError(errorResult) {
                    print("SUCCESS: Errors module error result works")
                } else {
                    print("FAIL: Errors module error result failed")
                }
                
                # Test collections module
                var list = Collections::createList()
                Collections::append(list, "item1")
                Collections::append(list, "item2")
                
                if Collections::length(list) == 2 {
                    print("SUCCESS: Collections module list operations work")
                } else {
                    print("FAIL: Collections module list operations failed")
                }
                
                # Test filesystem module  
                var exists = FileSystem::exists("/test/path")
                var content = FileSystem::readFile("/test/file.txt")
                
                if exists {
                    print("SUCCESS: FileSystem module operations work")
                } else {
                    print("FAIL: FileSystem module operations failed")
                }
                
                # Test async module
                var response = AsyncSupport::httpGet("http://example.com")
                var status = response["status"]
                
                if status == 200 {
                    print("SUCCESS: AsyncSupport module operations work")
                } else {
                    print("FAIL: AsyncSupport module operations failed")
                }
                
                print("=== Capability Module Testing Complete ===")
            }
        }
    
    operations:
        # No operations needed
        
    actions:
        # No actions needed
    
    domain:
        var testCount = 0
}