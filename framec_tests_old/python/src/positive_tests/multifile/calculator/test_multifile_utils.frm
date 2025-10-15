# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Math utilities module for calculator test

module MathUtils {
    fn add(a, b) {
        return a + b
    }
    
    fn multiply(a, b) {
        return a * b
    }
    
    fn circleArea(radius) {
        return 3.14159 * radius * radius
    }
}

fn formatNumber(num) {
    return "Result: " + str(num)
}

fn isEven(number) {
    return number % 2 == 0
}