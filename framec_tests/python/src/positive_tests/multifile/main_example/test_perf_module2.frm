# Performance module 2

module Module2 {
    fn process(value) {
        return value * 3 - 2
    }
    
    fn helper2(base) {
        var result = 1
        for i in range(1, base + 1) {
            result = result * 2
        }
        return result
    }
    
    fn find_max(numbers) {
        if len(numbers) == 0 {
            return 0
        }
        var maximum = numbers[0]
        for num in numbers {
            if num > maximum {
                maximum = num
            }
        }
        return maximum
    }
}