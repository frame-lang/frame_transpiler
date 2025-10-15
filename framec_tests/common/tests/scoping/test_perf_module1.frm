# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Performance module 1

module Module1 {
    fn process(value) {
        return value * 2 + 1
    }
    
    fn helper(base) {
        var result = 0
        for i in range(base) {
            result = result + i * 2
        }
        return result
    }
    
    fn compute_sum(numbers) {
        var total = 0
        for num in numbers {
            total = total + num
        }
        return total
    }
}