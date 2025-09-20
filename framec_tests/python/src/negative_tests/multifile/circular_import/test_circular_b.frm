# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# @test-expect: error
# @error-pattern: Circular dependency detected
# @error-type: Multi-file compilation failed
#
# Circular dependency test - Module B
# This creates a circular dependency with Module A

import ModuleA from "./test_circular_a.frm"

module ModuleB {
    fn functionB() {
        return "B calls A: " + ModuleA.functionA()
    }
}