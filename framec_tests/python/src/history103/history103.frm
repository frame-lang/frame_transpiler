#[codegen.python.code.public_state_info:bool="true"]

system History103 {
    interface:
        gotoC()
        ret()

    machine:
        $A {
            gotoC() {
                $$[+]
                -> "$$[+]" $C
                return
            }
        }

        $B {
            gotoC() {
                $$[+]
                -> "$$[+]" $C
                return
            }
        }

        $C {
            ret() {
                -> "$$[-]" $$[-]
                return
            }
        }
}