// Simple validation test with all blocks
fn main() {
    ValidationTest()
}

system ValidationTest {
    operations:
        do_work() {
            print("Operations: do_work called")
        }

    interface:
        test_interface()

    machine:
    $Start {
        $>() {
            print("Machine: Start state entered")
            self.do_work()
            print("Actions: calling finish_work")  
            self.finish_work()
        }
        
        test_interface() {
            print("Machine: test_interface called")
        }
    }

    actions:
        finish_work() {
            print("Actions: finish_work called")
            print("Domain: counter = " + str(self.counter))
        }

    domain:
        var counter : int = 42
}