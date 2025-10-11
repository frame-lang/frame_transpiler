# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
#[codegen.python.code.public_state_info:bool="true"]

system History102 {
    interface:
        gotoB()
        gotoC()
        gotoD()
        ret()

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
                -> "D" $D("B")
            }
        }

        $C {
            gotoD() {
                -> "D" $D("C")
            }
        }

        $D(previous_state) {
            ret() {
                if previous_state == "B" {
                    -> "ret" $B
                } elif previous_state == "C" {
                    -> "ret" $C
                }
            }
        }
}