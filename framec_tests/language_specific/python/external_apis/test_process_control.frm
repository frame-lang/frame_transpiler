# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Python-specific Process Control external API test
# Must produce identical output to TypeScript version

import subprocess
import sys

system ProcessTest {
    actions:
        testProcessControl() {
            print("=== Process Control Test ===")
            
            # Test simple command execution using Python subprocess
            result = subprocess.run([sys.executable, "-c", "print('Hello from subprocess')"], 
                                       capture_output=True, text=True)
            
            print("Process started: true")
            print(f"Process output: {result.stdout.strip()}")
            print(f"Exit code: {result.returncode}")
            
            # Test command with arguments
            echo_result = subprocess.run([sys.executable, "-c", "import sys; print(' '.join(sys.argv[1:]))", "Frame", "Process", "Test"], 
                                           capture_output=True, text=True)
            
            print(f"Args test output: {echo_result.stdout.strip()}")
            
            print("=== Process Control Test Complete ===")

fn main() {
    testProcessControl()
    return
}
