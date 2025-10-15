# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
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
            }
        }

        $B {
            gotoC() {
                $$[+]
                -> "$$[+]" $C
            }
        }

        $C {
            ret() {
                -> "$$[-]" $$[-]
            }
        }
}