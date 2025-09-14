# Large multi-file compilation test
# Tests performance with many modules and cross-dependencies

import Module1 from "./test_perf_module1.frm"
import Module2 from "./test_perf_module2.frm"
import Module3 from "./test_perf_module3.frm"
import Module4 from "./test_perf_module4.frm"
import Module5 from "./test_perf_module5.frm"

fn process_all() {
    var total = 0
    
    # Use all imported modules with :: for module access
    for i in range(100) {
        total = total + Module1::process(i)
        total = total + Module2::process(i)
        total = total + Module3::process(i)
        total = total + Module4::process(i)
        total = total + Module5::process(i)
    }
    
    # Use helpers with :: for module functions
    var helper_sum = Module1::helper(10)
    helper_sum = helper_sum + Module2::helper2(10)
    helper_sum = helper_sum + Module3::helper3(10)
    helper_sum = helper_sum + Module4::helper4(10)
    helper_sum = helper_sum + Module5::helper5(10)
    
    return total + helper_sum
}

fn complex_calculation(n) {
    var results = []
    
    # Complex nested module usage
    for i in range(n) {
        var temp = 0
        if i % 5 == 0 {
            temp = Module5::process(i)
        } elif i % 4 == 0 {
            temp = Module4::process(i)
        } elif i % 3 == 0 {
            temp = Module3::process(i)
        } elif i % 2 == 0 {
            temp = Module2::process(i)
        } else {
            temp = Module1::process(i)
        }
        results.append(temp)
    }
    
    return results
}

fn benchmark_large_compilation() {
    print("Starting large compilation benchmark...")
    
    var result = process_all()
    print("Process all result: " + str(result))
    
    var complex_result = complex_calculation(20)
    print("Complex calculation count: " + str(len(complex_result)))
    
    print("Benchmark complete!")
}

fn main() {
    benchmark_large_compilation()
}