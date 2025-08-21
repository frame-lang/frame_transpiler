
#[codegen.python.code.public_state_info:bool="true"]

system TransitionSm {
    interface:
        transit()
        change()

    machine:
        $S0 {
            <$() {
                exit("S0")
                return
            }
            transit() {
                -> $S1
                return
            }
        }

        $S1 {
            $>() {
                enter("S1")
                return
            }
            change() {
                -> $S2
                return
            }
        }

        $S2 {
            <$() {
                exit("S2")
                return
            }
            transit() {
                -> $S3
                return
            }
        }

        $S3 {
            $>() {
                enter("S3")
                return
            }
            <$() {
                exit("S3")
                return
            }
            transit() {
                -> $S4
                return
            }
        }

        $S4 {
            $>() {
                enter("S4")
                -> $S0
                return
            }
        }

    actions:
        enter(state:str) {
        }
        exit(state:str) {
        }

    domain:
        var enters = `[]`
        var exits = `[]`
}
