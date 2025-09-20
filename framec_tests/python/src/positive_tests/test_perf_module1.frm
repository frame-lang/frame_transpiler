# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Performance test module 1

module Module1 {
    fn process(x) {
        var result = x * 2
        for i in range(10) {
            result = result + i
        }
        return result
    }
    
    fn helper(y) {
        return y * y
    }
}