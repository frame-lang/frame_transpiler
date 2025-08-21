fn main() {
    FibonacciSystemParamsDemo(0, 1)
}

system FibonacciSystemParamsDemo ($(zero), $>(one)) {
    interface:
        next()

    machine:
        $Setup(zero) {
            $>(one) {
                print(zero)
                print(one)

                // initialize $PrintNextFibonacciNumber state parameters
                -> $PrintNextFibonacciNumber(zero, one)
                return
            }
        }
        
        // params (a,b) = (0,1)
        $PrintNextFibonacciNumber(a, b) {
            next() {
                var sum = a + b
                print(sum)
                a = b
                b = sum
                return
            }
        }
}