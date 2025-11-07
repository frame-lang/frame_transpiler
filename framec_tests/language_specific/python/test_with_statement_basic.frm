# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test file for with statement support in Frame v0.37 - Basic version
import tempfile
import os

# Test regular with statement for file operations
fn test_with_file() {
    # Create a temporary file for testing
    temp_file = tempfile.NamedTemporaryFile(mode="w+", delete=False, suffix=".txt")
    temp_file.write("Test content for with statement")
    temp_file.close()
    temp_path = temp_file.name
    
    print("Testing basic with statement...")
    
    # Test with statement
    with open(temp_path, "r") as f:
        content = f.read()
        print("File content: " + content)
    
    # Clean up
    os.unlink(temp_path)
    print("Basic with statement test passed!")
}

# Test nested with statements
fn test_nested_with() {
    print("Testing nested with statements...")
    
    # Create temp files
    temp1 = tempfile.NamedTemporaryFile(mode="w+", delete=False, suffix=".txt")
    temp1.write("input data for nested test")
    temp1.close()
    input_path = temp1.name
    
    temp2 = tempfile.NamedTemporaryFile(mode="w+", delete=False, suffix=".txt")
    temp2.close()
    output_path = temp2.name
    
    # Test nested with
    with open(input_path, "r") as input_file:
        with open(output_path, "w") as output_file:
            data = input_file.read()
            output_file.write(data.upper())
    
    # Verify result
    with open(output_path, "r") as result:
        output = result.read()
        print("Nested with result: " + output)
    
    # Clean up
    os.unlink(input_path)
    os.unlink(output_path)
    print("Nested with statement test passed!")
}

# Test with statement in a system
system FileProcessor {
    interface:
        processFile(content)
        getResult()
        
    machine:
        $Idle {
            processFile(content) {
                print("Processing in system with 'with' statement...")
                
                # Create temp file
                temp = tempfile.NamedTemporaryFile(mode="w+", delete=False, suffix=".txt")
                temp.write(content)
                temp.close()
                path = temp.name
                
                # Use with statement inside event handler
                with open(path, "r") as file:
                    self.content = file.read()
                    print("Read " + str(len(self.content)) + " bytes from file")
                
                # Clean up
                os.unlink(path)
                -> $Processing
            }
            getResult() {
                system.return = "Not processed yet"
            }
        }
        
        $Processing {
            $>() {
                print("Transforming content...")
                self.processed = self.content.upper()
                -> $Done
            }
            getResult() {
                system.return = "Processing: " + self.content
            }
        }
        $Done {
            $>() {
                print("Processing complete!")
            }
            getResult() {
                system.return = "Done: " + self.processed
            }
        }
        }
    domain:
        content = ""
        processed = ""
}

# Main test function
fn main() {
    print("=" * 50)
    print("Testing with statement support in Frame v0.37")
    print("=" * 50)
    
    # Test 1: Basic with statement
    print("\nTest 1: Basic with statement")
    print("-" * 30)
    test_with_file()
    
    # Test 2: Nested with statements
    print("\nTest 2: Nested with statements")
    print("-" * 30)
    test_nested_with()
    
    # Test 3: With statement in system
    print("\nTest 3: With statement in system")
    print("-" * 30)
    processor = FileProcessor()
    processor.processFile("test data from system")
    result = processor.getResult()
    print("Final result: " + result)
    
    print("\n" + "=" * 50)
    print("All with statement tests completed successfully!")
    print("=" * 50)
}

# Run the tests
