# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Multi-file test: Main entry point
# This is the main module that uses the calculator

import Calculator from "./test_multifile_calculator.frm"
import MathUtils from "./test_multifile_utils.frm"

fn main() {
    # Create calculator instance
    var calc = Calculator()
    
    # Test basic operations (instance methods use .)
    print("5 + 3 = " + calc.add(5, 3))
    print("4 * 7 = " + calc.multiply(4, 7))
    
    # Test circle area
    print("Area of circle with radius 5 = " + calc.getCircleArea(5))
    
    # Test even check
    if calc.checkEven(10) {
        print("10 is even")
    }
    
    if not calc.checkEven(7) {
        print("7 is odd")
    }
    
    # Direct module access (use :: for module functions)
    print("Direct calculation: " + str(MathUtils::add(100, 200)))
    print("PI value: " + str(MathUtils::PI))
}

# Call main function
main()