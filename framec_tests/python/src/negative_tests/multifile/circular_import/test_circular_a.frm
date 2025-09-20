# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# @test-expect: error
# @error-pattern: Circular dependency detected
# @error-type: Multi-file compilation failed
#
# Circular dependency test - Module A
# This should fail with a circular dependency error

import ModuleB from "./test_circular_b.frm"

module ModuleA {
    fn functionA() {
        return "A calls B: " + ModuleB.functionB()
    }
}