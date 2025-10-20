# Phase 1 Test: Single Method with Basic Logic
# Tests a system with one method that performs simple computation
# This validates:
# - Interface method with parameters
# - Basic arithmetic operations
# - Return statement with expression
# - Integer type handling
#
# Expected output: 7 (when called with add(3, 4))

system Calculator {
    interface:
        add(a: int, b: int): int
    
    machine:
        $Ready {
            add(a: int, b: int): int {
                system.return = a + b
                return
            }
        }
}