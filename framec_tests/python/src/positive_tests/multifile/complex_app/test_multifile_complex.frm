# Complex multi-file test - main entry point
# Tests: nested imports, multiple dependencies, module functions

import MathModule from "./test_multifile_math.frm"
import StringModule from "./test_multifile_strings.frm"
import { formatNumber, parseNumber } from "./test_multifile_formatters.frm"

system ComplexCalculator {
    interface:
        calculate(operation, a, b)
        formatResult(value)
    
    machine:
        $Ready {
            calculate(operation, a, b) {
                if operation == "add" {
                    system.return = MathModule.add(a, b)
                } elif operation == "multiply" {
                    system.return = MathModule.multiply(a, b)
                } elif operation == "power" {
                    system.return = MathModule.power(a, b)
                } else {
                    system.return = 0
                }
                return
            }
            
            formatResult(value) {
                system.return = formatNumber(value)
                return
            }
        }
}

fn main() {
    # Test math operations
    var calc = ComplexCalculator()
    
    var sum = calc.calculate("add", 10, 5)
    print("10 + 5 = " + calc.formatResult(sum))
    
    var product = calc.calculate("multiply", 7, 8)
    print("7 * 8 = " + calc.formatResult(product))
    
    var power = calc.calculate("power", 2, 8)
    print("2 ^ 8 = " + calc.formatResult(power))
    
    # Test string operations
    var greeting = StringModule.createGreeting("Frame")
    print(greeting)
    
    var reversed = StringModule.reverseString("Hello")
    print("Reversed: " + reversed)
    
    # Test formatter functions
    var formatted = formatNumber(3.14159)
    print("Formatted PI: " + formatted)
    
    var parsed = parseNumber("42.5")
    print("Parsed: " + str(parsed))
}

main()