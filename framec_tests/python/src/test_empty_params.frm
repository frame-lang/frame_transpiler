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