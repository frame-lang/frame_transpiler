# Math module for complex multi-file test

module MathModule {
    var operations_count = 0
    
    fn add(a, b) {
        operations_count = operations_count + 1
        return a + b
    }
    
    fn multiply(a, b) {
        operations_count = operations_count + 1
        return a * b
    }
    
    fn power(base, exp) {
        operations_count = operations_count + 1
        return base ** exp
    }
    
    fn getOperationsCount() {
        return operations_count
    }
}