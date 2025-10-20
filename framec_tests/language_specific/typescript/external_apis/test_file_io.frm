# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# TypeScript-specific File I/O external API test  
# Must produce identical output to Python version

import fs

system FileIOTest {
    actions:
        testFileOperations() {
            print("=== File I/O Test ===")
            
            # Create test input file first
            fs.writeFileSync("test_input.txt", "Frame File I/O Test Data")
            
            # Test file existence using Node.js fs
            var exists = fs.existsSync("test_input.txt")
            print(f"File exists: {exists}")
            
            # Read file content using Node.js idioms
            if exists {
                var content = fs.readFileSync("test_input.txt", "utf8").trim()
                print(f"File content: {content}")
            } else {
                print("File content: [file not found]")
            }
            
            # Write file using Node.js idioms
            fs.writeFileSync("test_output.txt", "Hello from Frame TypeScript!")
            
            # Verify write succeeded
            var write_exists = fs.existsSync("test_output.txt")
            print(f"Write successful: {write_exists}")
            
            print("=== File I/O Test Complete ===")
        }
}

fn main() {
    var tester = FileIOTest()
    tester._action_testFileOperations()
    return
}