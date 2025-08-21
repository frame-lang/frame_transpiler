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
                return
            }
            gotoC() {
                -> "C" $C
                return
            }
        }

        $B {
            gotoD() {
                -> "D" $D
                return
            }
        }

        $C {
            gotoD() {
                -> "D" $D
                return
            }
        }

        $D {
        }
}