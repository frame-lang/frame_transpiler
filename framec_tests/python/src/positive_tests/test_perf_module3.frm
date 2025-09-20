# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Performance test module 3

module Module3 {
    fn process(x) {
        var result = x * 3
        for j in range(30) {
            result = result + j
        }
        return result
    }
    
    fn helper3(y) {
        return y ** 3
    }
}
