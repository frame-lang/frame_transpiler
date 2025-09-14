# Performance module 5

module Module5 {
    fn process(value) {
        if value % 2 == 0 {
            return value / 2
        } else {
            return value * 3 + 1
        }
    }
    
    fn benchmark() {
        var result = 0
        for i in range(1000) {
            result = result + process(i)
        }
        return result
    }
}