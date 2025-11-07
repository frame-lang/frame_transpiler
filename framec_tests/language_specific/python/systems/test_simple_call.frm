# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    service = TestService()
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