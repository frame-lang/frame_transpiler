fn main() {
    var test = DomainTest()
    test.run_test()
    return
}

system DomainTest {
    interface:
        run_test()
        
    machine:
        $Start {
            run_test() {
                // Test domain variable read access (already working)
                print("Initial counter: " + str(self.counter))
                
                // Test domain variable assignment (needs implementation)
                self.counter = 25
                print("Updated counter: " + str(self.counter))
                
                return
            }
        }
        
    domain:
        counter: int = 10
}