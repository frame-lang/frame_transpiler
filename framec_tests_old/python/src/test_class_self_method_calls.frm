# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test for self.method() calls in class methods working correctly
# This test specifically validates the fix for preserving self. prefix in class method calls

class Calculator {
    fn add(a, b) {
        return a + b
    }
    
    fn multiply(a, b) {
        return a * b
    }
    
    fn calculate(a, b, op) {
        if op == "add" {
            return self.add(a, b)
        } else {
            return self.multiply(a, b)
        }
    }
}

fn main() {
    var calc = Calculator()
    var sum = calc.calculate(5, 3, "add")
    print("5 + 3 = " + str(sum))
    var product = calc.calculate(5, 3, "multiply")
    print("5 * 3 = " + str(product))
}