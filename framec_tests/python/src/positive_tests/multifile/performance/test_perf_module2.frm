# Performance module 2

module Module2 {
    fn process(value) {
        return value * 3 - 2
    }
    
    fn benchmark() {
        var result = 0
        for i in range(1000) {
            result = result + process(i)
        }
        return result
    }
}