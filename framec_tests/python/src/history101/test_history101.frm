# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
#[codegen.python.code.public_state_info:bool="true"]

system History101 {
    interface:
        gotoB()
        gotoC()
        gotoD()

    machine:
        $A {
            gotoB() {
                -> "B" $B
            }
            gotoC() {
                -> "C" $C
            }
        }

        $B {
            gotoD() {
                -> "D" $D
            }
        }

        $C {
            gotoD() {
                -> "D" $D
            }
        }

        $D {
        }
}