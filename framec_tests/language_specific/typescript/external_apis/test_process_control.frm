# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# TypeScript-specific Process Control external API test
# Must produce identical output to Python version

import child_process

system ProcessTest {
    actions:
        testProcessControl() {
            print("=== Process Control Test ===")
            
            # Test simple command execution using Node.js child_process
            var result = child_process.execSync("node -e \"console.log('Hello from subprocess')\"", 
                                               {encoding: "utf8"})
            
            print("Process started: true")
            print(f"Process output: {result.trim()}")
            print("Exit code: 0")
            
            # Test command with arguments
            var echo_result = child_process.execSync("node -e \"console.log(process.argv.slice(2).join(' '))\" Frame Process Test", 
                                                    {encoding: "utf8"})
            
            print(f"Args test output: {echo_result.trim()}")
            
            print("=== Process Control Test Complete ===")
        }
}

fn main() {
    var tester = ProcessTest()
    tester.testProcessControl()
    return
}