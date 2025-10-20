# Phase 1 Test: Validation System
# Tests a system that validates its own functionality
# This validates:
# - Boolean logic and conditionals
# - System method calls within handlers
# - String operations and formatting
# - Cross-language behavioral consistency
#
# Expected output: "SUCCESS: All tests passed" or "FAIL: [reason]"

system Validator {
    interface:
        runTest(): string
    
    machine:
        $Testing {
            runTest(): string {
                var result = performCalculation()
                if result == 42 {
                    system.return = "SUCCESS: All tests passed"
                } else {
                    system.return = "FAIL: Expected 42, got " + str(result)
                }
                return
            }
        }
    
    actions:
        performCalculation(): int {
            return 6 * 7
        }
}