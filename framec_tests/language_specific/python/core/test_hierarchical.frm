# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

#[codegen.python.code.public_state_info:bool="true"]

system Hierarchical {
    interface:
        A()
        B()
        C()

    machine:
        $I {
            $>() {
                -> $S
            }
        }

        $S {
            $>() {
                enter("S")
                return
            }
            <$() {
                exit("S")
                return
            }
            A() {
                log("S.A")
                -> $S0
            }
            B() {
                log("S.B")
                -> $S1
            }
        }

        $S0 => $S {
            $>() {
                enter("S0")
                => $^
                return
            }
            <$() {
                exit("S0")
                => $^
                return
            }
            A() {   # override parent handler
                log("S0.A")
                -> $T
            }
            B() {   # do this, then parent handler
                log("S0.B")
                => $^
                return
            }
            C() {   # extend parent handler
                log("S0.C")
                -> $S2
            }
        }

        $S1 => $S {
            $>() {
                enter("S1")
                return
            }
            <$() {
                exit("S1")
                return
            }
            # defer to parent for A
            B() {   # do this, then parent, which transitions here
                log("S1.B")
                => $^
                return
            }
            C() {   # propagate message not handled by parent
                log("S1.C")
                => $^
                return
            }
        }

        $S2 => $S0 {
            $>() {
                enter("S2")
                => $^
                return
            }
            <$() {
                exit("S2")
                => $^
                return
            }
            B() {   # will propagate to S0 and S
                log("S2.B")
                => $^
                return
            }
            C() {
                log("S2.C")
                -> $T
            }
        }

        $S3 => $S1 {
            $>() {
                enter("S3")
                => $^
                return
            }
            <$() {
                exit("S3")
                => $^
                return
            }
            # defer to grandparent for A
            B() {   # override and move to sibling
                log("S3.B")
                -> $S2
            }
        }

        $T {
            $>() {
                enter("T")
                return
            }
            <$() {
                exit("T")
                return
            }
            A() {
                log("T.A")
                -> $S
            }
            B() {
                log("T.B")
                -> $S2
            }
            C() {
                log("T.C")
                -> $S3
            }
        }

    actions:
        enter(msg:str) {
        }
        exit(msg:str) {
        }
        log(msg:str) {
        }

    domain:
        enters = []
        exits = []
        tape = []
}
