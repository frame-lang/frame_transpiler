fn main() {
    var service = TestService()
}

system TestService {

    operations:
        signal_handler(sig, frame) {
            self.quit()
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