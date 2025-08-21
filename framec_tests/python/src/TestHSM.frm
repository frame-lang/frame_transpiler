fn main() {
    var sys = ContinueTerminatorDemo()
    sys.passMe1()
    sys.passMe2()
}

system ContinueTerminatorDemo {

    interface:
        passMe1()
        passMe2()

    machine:
        // Dispatch operator (=>) defines state hierarchy

        $Child => $Parent {
            // Continue operator sends events to $Parent

            passMe1() {
                => $^
            }
            passMe2() {
                print("handled in $Child")
                => $^
            }
        }

        $Parent {
            passMe1() {
                print("handled in $Parent")
                return
            }
            passMe2() {
                print("handled in $Parent")
                return
            }
        }
}