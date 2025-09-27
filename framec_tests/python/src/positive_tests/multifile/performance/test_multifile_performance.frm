# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Performance test - main file with many dependencies
# Tests compilation performance with multiple modules

import Module1 from "./test_perf_module1.frm"
import Module2 from "./test_perf_module2.frm"
import Module3 from "./test_perf_module3.frm"
import Module4 from "./test_perf_module4.frm"
import Module5 from "./test_perf_module5.frm"

fn main() {
    var results = []
    
    # Call functions from each module (using :: for module access)
    results.append(Module1::process(1))
    results.append(Module2::process(2))
    results.append(Module3::process(3))
    results.append(Module4::process(4))
    results.append(Module5::process(5))
    
    # Print results
    for r in results {
        print("Result: " + str(r))
    }
    
    print("Performance test completed with " + str(len(results)) + " modules")
}
