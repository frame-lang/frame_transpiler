# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Performance module 3

module Module3 {
    fn process(value) {
        return value ** 2 + value
    }
    
    fn helper3(base) {
        var result = 0
        for i in range(base) {
            result = result + (i * i)
        }
        return result
    }
    
    fn factorial(n) {
        if n <= 1 {
            return 1
        }
        return n * factorial(n - 1)
    }
}