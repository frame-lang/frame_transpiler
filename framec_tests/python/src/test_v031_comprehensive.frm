function main() {
    var test = TestSystem()
    test.run_test()
    return
}

system TestSystem {
    operations:
        do_work()
        calculate(x: int): int
        
    interface:
        run_test()
        get_value(): string
        
    machine:
        $Start {
            run_test() {
                // Test self.operation() calls
                self.do_work()
                var result = self.calculate(42)
                
                // Test self.domainVar access
                print("Counter: " + str(self.counter))
                
                // Test system.interface() calls
                var msg = system.get_value()
                print("Got: " + msg)
                
                return
            }
            
            get_value(): string {
                return = "Test value from interface"
            }
        }
        
    actions:
        finish_work()
        
    domain:
        counter: int = 10
}