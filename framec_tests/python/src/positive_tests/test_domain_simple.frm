# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system DomainTest {
    interface:
        run_test()
        
    machine:
        $Start {
            run_test() {
                # Current syntax (works)
                counter = 25
                print("Updated: " + str(self.counter))
                return
            }
        }
        
    domain:
        var counter: int = 10
}