# Math module for complex application

module MathModule {
    fn add(a, b) {
        return a + b
    }
    
    fn multiply(a, b) {
        return a * b
    }
    
    fn power(base, exponent) {
        var result = 1
        var counter = 0
        while counter < exponent {
            result = result * base
            counter = counter + 1
        }
        return result
    }
    
    fn subtract(a, b) {
        return a - b
    }
    
    fn divide(a, b) {
        if b == 0 {
            return 0
        }
        return a / b
    }
}