# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Performance module 4

module Module4 {
    fn process(value) {
        return (value + 5) * 2
    }
    
    fn benchmark() {
        var result = 0
        for i in range(1000) {
            result = result + process(i)
        }
        return result
    }
}