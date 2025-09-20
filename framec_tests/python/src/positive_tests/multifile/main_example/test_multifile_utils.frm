# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Math utilities for main example

module MathUtils {
    var PI = 3.14159
    
    fn add(a, b) {
        return a + b
    }
    
    fn multiply(a, b) {
        return a * b
    }
    
    fn circleArea(radius) {
        return PI * radius * radius
    }
    
    fn isEven(number) {
        return number % 2 == 0
    }
    
    fn factorial(n) {
        if n <= 1 {
            return 1
        }
        return n * factorial(n - 1)
    }
}