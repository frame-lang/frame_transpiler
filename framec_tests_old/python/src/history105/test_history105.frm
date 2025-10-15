# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
#[codegen.python.code.public_state_info:bool="true"]

system History105 {
    interface:
        gotoB()
        gotoC()
        ret()

    machine:
        $A {
            var a = 0

            $>() {
                print("In $A. a = " + str(a))
                return
            }

            gotoB() {
                print("Transitioning to $B")
                -> $B
            }

            gotoC() {
                # When we return, a == 1
                a = a + 1
                print("Incrementing a to " + str(a))
                $$[+]
                -> $C
            }
        }

        $B {
            var b = 0

            $>() {
                print("In $B. b = " + str(b))
                return
            }

            gotoC() {
                # When we return, b == 1
                b = b + 1
                print("Incrementing b to " + str(b))
                $$[+]
                -> $C
            }
        }

        $C {
            $>() {
                print("In $C")
                return
            }

            ret() {
                print("Return to previous state")
                -> $$[-]
            }
        }
}