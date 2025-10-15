# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Performance module 1

module Module1 {
    fn process(value) {
        return value * 2 + 1
    }
    
    fn benchmark() {
        var result = 0
        for i in range(1000) {
            result = result + process(i)
        }
        return result
    }
}