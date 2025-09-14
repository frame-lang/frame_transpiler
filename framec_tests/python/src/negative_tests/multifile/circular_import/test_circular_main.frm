# @test-expect: error
# @error-pattern: Circular dependency detected
# @error-type: Multi-file compilation failed
#
# Main file to test circular dependency detection

import ModuleA from "./test_circular_a.frm"

fn main() {
    var result = ModuleA.functionA()
    print(result)
}

main()