fn main() {
    var service = TestService()
}

system TestService {

    operations:
        test_method() {
            print("test")
        }

    machine:

    $Start {
        $>() {
            self.test_method()
        }
    }
}