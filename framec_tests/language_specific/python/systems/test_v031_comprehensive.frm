# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    var test = TestSystem()
    test.run_test()
    return
}

system TestSystem {
    operations:
        do_work() {
            print("Doing work...")
        }
        calculate(x: int): int {
            return x * 2
        }
        
    interface:
        run_test()
        get_value(): string
        
    machine:
        $Start {
            run_test() {
                # Test self.operation() calls
                self.do_work()
                var result = self.calculate(42)
                
                # Test self.domainVar access
                print("Counter: " + str(self.counter))
                
                # Test interface method calls (with system. prefix)
                var msg = system.get_value()
                print("Got: " + msg)
                
                return
            }
            
            get_value(): string {
                system.return = "Test value from interface"
            }
        }
        
    actions:
        finish_work() {
            print("Finishing work...")
        }
        
    domain:
        counter: int = 10
}