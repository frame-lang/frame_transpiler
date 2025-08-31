fn main() {
    var test = TestSystem()
    test.run_test()
    return
}

system UtilitySystem {
    operations:
        @staticmethod
        calculate(x: int): int
        
        @staticmethod  
        format_message(msg: string): string
}

system TestSystem {
    interface:
        run_test()
        
    machine:
        $Start {
            run_test() {
                // Test static operation calls
                var result = UtilitySystem.calculate(42)
                print("Result: " + str(result))
                
                var msg = UtilitySystem.format_message("Hello")
                print("Message: " + msg)
                
                return
            }
        }
}