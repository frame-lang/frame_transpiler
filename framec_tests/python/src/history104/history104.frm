#[codegen.python.code.public_state_info:bool="true"]

system History104 {
    interface:
        gotoB()
        retToB()
        gotoC()
        retToC()
        gotoD()

    machine:
        $A {
            $>() {
                print("In $A")
                return
            }
            gotoB() {
                -> "B" $B
                return
            }
        }

        $B {
            var b = 0

            // upon reentry using a transition, b == 0
            $>() {
                print("Entering $B. b = " + str(b))
                return
            }

            gotoC() {
                print("--------------")
                print("Going to $C.")
                print("--------------")
                -> "C" $C
                return
            }
            gotoD() {
                b = 1
                print("Going to $D. b = " + str(b))
                -> "D" $D
                return
            }
        }

        $C {
            var c = 0

            // upon reentry using history pop, c == 1
            $>() {
                print("Entering $C. c = " + str(c))
                return
            }

            gotoD() {
                c = 1
                print("Going to $D. c = " + str(c))
                $$[+]
                -> "D" $D
                return
            }
        }

        $D {
            $>() {
                print("In $D")
                return
            }
            retToB() {
                print("Returning to $B")
                -> "retToB" $B
                return
            }
            retToC() {
                print("Returning to $C")
                -> "retToC" $$[-]
                return
            }
        }
}