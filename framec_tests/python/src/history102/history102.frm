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
                return
            }
            gotoC() {
                -> "C" $C
                return
            }
        }

        $B {
            gotoD() {
                -> "D" $D("B")
                return
            }
        }

        $C {
            gotoD() {
                -> "D" $D("C")
                return
            }
        }

        $D(previous_state) {
            ret() {
                if previous_state == "B" {
                    -> "ret" $B
                    return
                } elif previous_state == "C" {
                    -> "ret" $C
                    return
                }
                return
            }
        }
}