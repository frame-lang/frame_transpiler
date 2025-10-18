# TypeScript Frame system demonstrating capability-style patterns

system TypeScriptCapabilityDemo {
    interface:
        demonstrateCapabilities()
    
    operations:
        # Error handling operations (capability module style)
        createOk(value): dict {
            return {
                "isOk": true,
                "isError": false,
                "value": value,
                "error": None
            }
        }
        
        createError(errorMessage): dict {
            return {
                "isOk": false,
                "isError": true,
                "value": None,
                "error": errorMessage
            }
        }
        
        isOk(result): bool {
            return result["isOk"]
        }
        
        # File simulation operations
        simulateFileExists(path): bool {
            print("Checking if path exists: " + path)
            return true
        }
        
        simulateReadFile(path): str {
            print("Reading file: " + path)
            return "simulated file content from " + path
        }
        
        # HTTP simulation operations
        simulateHttpGet(url): dict {
            print("HTTP GET: " + url)
            return {
                "status": 200,
                "body": "simulated response from " + url,
                "headers": {}
            }
        }
    
    machine:
        $Start {
            demonstrateCapabilities() {
                print("=== TypeScript Capability Module Demo ===")
                
                # Manual error handling (simulating errors module)
                var okResult = self.createOk("success value")
                var errorResult = self.createError("test error")
                
                if self.isOk(okResult) {
                    print("✓ Error handling: OK result works")
                } else {
                    print("✗ Error handling: OK result failed")
                }
                
                # Manual collection operations (simulating collections module)
                var myList = []
                myList.append("item1")
                myList.append("item2")
                myList.append("item3")
                
                if len(myList) == 3 {
                    print("✓ Collections: List operations work")
                } else {
                    print("✗ Collections: List operations failed")
                }
                
                # Manual file simulation (simulating filesystem module)
                var fileExists = self.simulateFileExists("/test/path")
                var fileContent = self.simulateReadFile("/test/file.txt")
                
                if fileExists {
                    print("✓ FileSystem: File operations work")
                } else {
                    print("✗ FileSystem: File operations failed")
                }
                
                # Manual HTTP simulation (simulating async module)
                var response = self.simulateHttpGet("http://example.com")
                var status = response["status"]
                
                if status == 200 {
                    print("✓ AsyncSupport: HTTP operations work")
                } else {
                    print("✗ AsyncSupport: HTTP operations failed")
                }
                
                print("=== TypeScript Capability Demo Complete ===")
                print("This demonstrates that Frame systems work with capability-style patterns in TypeScript")
            }
        }
}