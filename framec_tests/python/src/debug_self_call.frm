fn main() {
    var service = DebugService()
}

system DebugService {

    operations:
        test_op() {
            self.test_method()
        }
        
        test_method() {
            print("called")
        }

    machine:

    $Start {
        $>() {
            self.test_op()
        }
    }
}