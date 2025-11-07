# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    var service = TestService()
}

system TestService {

    operations:
        test_method() {
            test_func()
        }

    machine:

    $Start {
        $>() {
            print("test")
        }
    }
}

fn test_func() {
    print("external")
}