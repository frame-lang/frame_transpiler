# Multi-file test: Utility module
# This module provides utility functions for other modules

module MathUtils {
    var PI = 3.14159
    var E = 2.71828
    
    fn add(x, y) {
        return x + y
    }
    
    fn multiply(x, y) {
        return x * y
    }
    
    fn circleArea(radius) {
        return PI * radius * radius
    }
}

# Standalone utility functions
fn formatNumber(n) {
    return str(n)
}

fn isEven(n) {
    return n % 2 == 0
}