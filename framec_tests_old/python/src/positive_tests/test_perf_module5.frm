# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Performance test module 5

module Module5 {
    fn process(x) {
        var result = x * 5
        for j in range(50) {
            result = result + j
        }
        return result
    }
    
    fn helper5(y) {
        return y ** 5
    }
}
