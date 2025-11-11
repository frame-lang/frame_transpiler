@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    service = TestService()
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
