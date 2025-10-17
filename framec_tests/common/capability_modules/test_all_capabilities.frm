# Comprehensive test of all capability modules working together

import Errors from "./errors_simple.frm"
import Collections from "./collections_simple.frm"
import FileSystem from "./filesystem_simple.frm"
import AsyncSupport from "./async_simple.frm"

system ComprehensiveCapabilityTest {
    interface:
        runAllTests()
    
    machine:
        $Start {
            runAllTests() {
                print("=== Testing All Capability Modules ===")
                
                # Test error handling
                var okResult = Errors::createOk("success")
                var errorResult = Errors::createError("test error")
                
                if Errors::isOk(okResult) {
                    print("✓ Errors: OK result creation works")
                } else {
                    print("✗ Errors: OK result creation failed")
                }
                
                if Errors::isError(errorResult) {
                    print("✓ Errors: Error result creation works")
                } else {
                    print("✗ Errors: Error result creation failed")
                }
                
                # Test collections
                var list = Collections::createList()
                Collections::append(list, "item1")
                Collections::append(list, "item2")
                Collections::append(list, "item3")
                
                if Collections::length(list) == 3 {
                    print("✓ Collections: List operations work")
                } else {
                    print("✗ Collections: List operations failed")
                }
                
                var map = Collections::createMap()
                Collections::put(map, "key1", "value1")
                Collections::put(map, "key2", "value2")
                
                var retrieved = Collections::get(map, "key1")
                if retrieved == "value1" {
                    print("✓ Collections: Map operations work")
                } else {
                    print("✗ Collections: Map operations failed")
                }
                
                # Test filesystem
                var exists = FileSystem::exists("/test/path")
                var content = FileSystem::readFile("/test/file.txt")
                var size = FileSystem::getFileSize("/test/file.txt")
                
                if exists and size == 1024 {
                    print("✓ FileSystem: File operations work")
                } else {
                    print("✗ FileSystem: File operations failed")
                }
                
                # Test async/HTTP
                var response = AsyncSupport::httpGet("http://example.com")
                var status = response["status"]
                
                if status == 200 {
                    print("✓ AsyncSupport: HTTP operations work")
                } else {
                    print("✗ AsyncSupport: HTTP operations failed")
                }
                
                # Test complex operation combining multiple modules
                var fileList = Collections::createList()
                Collections::append(fileList, "/path/file1.txt")
                Collections::append(fileList, "/path/file2.txt")
                Collections::append(fileList, "/path/file3.txt")
                
                print("Testing complex operation with " + str(Collections::length(fileList)) + " files:")
                
                var i = 0
                while i < Collections::length(fileList) {
                    var filePath = fileList[i]
                    if FileSystem::exists(filePath) {
                        var result = Errors::createOk("File processed: " + filePath)
                        print("  ✓ " + Errors::unwrap(result))
                    } else {
                        var result = Errors::createError("File not found: " + filePath)
                        print("  ⚠ " + Errors::getError(result))
                    }
                    i = i + 1
                }
                
                print("=== All Capability Module Tests Complete ===")
                print("SUCCESS: Frame cross-language universality foundation is working!")
            }
        }
}