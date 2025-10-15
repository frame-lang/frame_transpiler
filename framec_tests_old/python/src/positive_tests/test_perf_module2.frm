# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Performance test module 2

module Module2 {
    fn process(x) {
        var result = x * 2
        for j in range(20) {
            result = result + j
        }
        return result
    }
    
    fn helper2(y) {
        return y ** 2
    }
}
