# Python-specific: domain native assignment
system DomainTest {
    interface:
        run_test()
    machine:
        $Start {
            run_test() {
                counter = 25
                print("Updated: " + str(self.counter))
                return
            }
        }
    domain:
        counter: int = 10
}

