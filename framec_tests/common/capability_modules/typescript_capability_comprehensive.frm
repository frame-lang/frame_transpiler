# Comprehensive TypeScript Frame system demonstrating capability-style patterns

system TypeScriptCapabilityComprehensive {
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
        
        isError(result): bool {
            return result["isError"]
        }
        
        unwrap(result): str {
            return result["value"]
        }
        
        # Collection operations
        createList(): list {
            return []
        }
        
        appendToList(lst, item): list {
            lst.append(item)
            return lst
        }
        
        listLength(lst): int {
            return len(lst)
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
            $>() {
                print("=== TypeScript Comprehensive Capability Demo ===")
                
                # Test error handling
                var okResult = self.createOk("success value")
                var errorResult = self.createError("test error")
                
                if self.isOk(okResult) {
                    print("✓ Error handling: OK result works - " + self.unwrap(okResult))
                } else {
                    print("✗ Error handling: OK result failed")
                }
                
                if self.isError(errorResult) {
                    print("✓ Error handling: Error result works")
                } else {
                    print("✗ Error handling: Error result failed")
                }
                
                # Test collections
                var myList = self.createList()
                self.appendToList(myList, "TypeScript")
                self.appendToList(myList, "capability")
                self.appendToList(myList, "modules")
                
                if self.listLength(myList) == 3 {
                    print("✓ Collections: List operations work - length: " + str(self.listLength(myList)))
                } else {
                    print("✗ Collections: List operations failed")
                }
                
                # Test file operations
                var fileExists = self.simulateFileExists("/typescript/test/path")
                var fileContent = self.simulateReadFile("/typescript/test/file.ts")
                
                if fileExists {
                    print("✓ FileSystem: File operations work")
                } else {
                    print("✗ FileSystem: File operations failed")
                }
                
                # Test HTTP operations
                var response = self.simulateHttpGet("http://typescript.example.com/api")
                var status = response["status"]
                
                if status == 200 {
                    print("✓ AsyncSupport: HTTP operations work - status: " + str(status))
                } else {
                    print("✗ AsyncSupport: HTTP operations failed")
                }
                
                # Complex operation combining multiple capability patterns
                var results = self.createList()
                var paths = ["/app/main.ts", "/app/utils.ts", "/app/components.ts"]
                
                var i = 0
                while i < len(paths) {
                    var path = paths[i]
                    if self.simulateFileExists(path) {
                        var content = self.simulateReadFile(path)
                        var result = self.createOk("Processed: " + path)
                        self.appendToList(results, result)
                    } else {
                        var result = self.createError("File not found: " + path)
                        self.appendToList(results, result)
                    }
                    i = i + 1
                }
                
                print("Processing complete - handled " + str(self.listLength(results)) + " files")
                
                print("=== TypeScript Capability Demo Complete ===")
                print("SUCCESS: Frame capability patterns work perfectly in TypeScript!")
            }
        }
}