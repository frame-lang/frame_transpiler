fn main() {
    var service = TestService()
}

system TestService {

    operations:
        simple_op() {
            print("simple")
        }

    machine:

    $Start {
        $>() {
            self.simple_op()
        }
    }
}