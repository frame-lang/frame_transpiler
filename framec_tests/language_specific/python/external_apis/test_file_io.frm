# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Python-specific File I/O external API test
# Must produce identical output to TypeScript version

import os.path

system FileIOTest {
    actions:
        testFileOperations() {
            print("=== File I/O Test ===")
            
            # Create test input file first
            f = open("test_input.txt", "w")
            f.write("Frame File I/O Test Data")
            f.close()
            
            # Test file existence using Python os.path
            exists = os.path.exists("test_input.txt")
            print(f"File exists: {exists}")
            
            # Read file content using Python idioms
            if exists:
                f_read = open("test_input.txt", "r")
                content = f_read.read().strip()
                f_read.close()
                print(f"File content: {content}")
            } else {
                print("File content: [file not found]")
            
            # Write file using Python idioms
            f_write = open("test_output.txt", "w")
            f_write.write("Hello from Frame Python!")
            f_write.close()
            
            # Verify write succeeded
            write_exists = os.path.exists("test_output.txt")
            print(f"Write successful: {write_exists}")
            
            print("=== File I/O Test Complete ===")
}

fn main() {
    tester = FileIOTest()
    tester._action_testFileOperations()
    return
}