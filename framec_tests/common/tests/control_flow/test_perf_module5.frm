# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Performance module 5

module Module5 {
    fn process(value) {
        if value % 2 == 0 {
            return value / 2
        } else {
            return value * 3 + 1
        }
    }
    
    fn helper5(base) {
        var result = 0
        var temp = base
        while temp > 1 {
            if temp % 2 == 0 {
                temp = temp / 2
            } else {
                temp = temp * 3 + 1
            }
            result = result + 1
        }
        return result
    }
    
    fn reverse_list(items) {
        var reversed = []
        for i in range(len(items) - 1, -1, -1) {
            reversed.append(items[i])
        }
        return reversed
    }
}