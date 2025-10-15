# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    var service = TestService()
}

system TestService {

    operations:
        signal_handler(sig, frame) {
            print("handler")
        }

    interface:
        quit()

    machine:

    $Start {
        quit() {
            print("Test")
        }
    }
}