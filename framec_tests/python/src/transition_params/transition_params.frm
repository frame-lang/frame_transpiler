
#[codegen.python.code.public_state_info:bool="true"]

system TransitParams {
    interface:
        Next()

    machine:
        $Init {
            Next() {
                -> ("hi A") $A
                return
            }
        }

        $A {
            $>(msg:str) {
                log(msg)
                return
            }

            <$() {
                log("bye A")
                return
            }

            Next() {
                -> ("hi B", 42) $B
                return
            }
        }

        $B {
            $>(msg:str, val:int) {
                log(msg)
                log(str(val))
                return
            }

            <$(val:bool, msg:str) {
                log(str(val))
                log(msg)
                return
            }

            Next() {
                (true, "bye B") -> ("hi again A") $A
                return
            }
        }

    actions:
        log(msg:str) {
        }

    domain:
        var tape = `[]`
}
