# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    var service = TestService()
}

system TestService {

    interface:
        quit()

    machine:

    $Start {
        quit() {
            print("Test")
        }
    }
}