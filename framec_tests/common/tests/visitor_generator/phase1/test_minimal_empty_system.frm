# Phase 1 Test: Minimal Empty System
# Tests the most basic Frame system with a single method that returns a constant.
# This validates:
# - System declaration
# - Interface method definition
# - Machine state definition  
# - Basic return statement
# - String literal handling
#
# Expected output: "SUCCESS: Empty system working"

system MinimalEmpty {
    interface:
        test(): string
    
    machine:
        $Start {
            test(): string {
                system.return = "SUCCESS: Empty system working"
                return
            }
        }
}