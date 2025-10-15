# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Performance test module 4

module Module4 {
    fn process(x) {
        var result = x * 4
        for j in range(40) {
            result = result + j
        }
        return result
    }
    
    fn helper4(y) {
        return y ** 4
    }
}
