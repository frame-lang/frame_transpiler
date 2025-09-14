# Performance module 4

module Module4 {
    fn process(value) {
        return (value + 5) * 2
    }
    
    fn helper4(base) {
        var result = []
        for i in range(base) {
            result.append(i + base)
        }
        return len(result)
    }
    
    fn fibonacci(n) {
        if n <= 1 {
            return n
        }
        return fibonacci(n - 1) + fibonacci(n - 2)
    }
}