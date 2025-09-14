# Performance module 3

module Module3 {
    fn process(value) {
        return value ** 2 + value
    }
    
    fn benchmark() {
        var result = 0
        for i in range(1000) {
            result = result + process(i)
        }
        return result
    }
}