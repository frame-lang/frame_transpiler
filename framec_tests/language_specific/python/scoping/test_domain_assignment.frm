# Python-specific: domain native assignment
fn main() {
    test = DomainTest()
    test.run_test()
    return
}

system DomainTest {
    interface:
        run_test()
    machine:
        $Start {
            run_test() {
                print("Initial counter: " + str(self.counter))
                self.counter = 25
                print("Updated counter: " + str(self.counter))
                return
            }
        }
    domain:
        counter: int = 10
}

