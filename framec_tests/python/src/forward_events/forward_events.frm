
#[codegen.python.code.public_state_info:bool="true"]

system ForwardEvents {
    interface:
        GotoS1()
        GotoS2()
        ReturnFromS1()
        ReturnFromS2()

    machine:
        $S0 {
            $>() {
                log("Enter $S0")
                return
            }
            <$() {
                log("Exit $S0")
                return
            }
            GotoS1() {
                log("Recieved GotoS1")
                -> $S1
                return
            }
            GotoS2() {
                log("Recieved GotoS2")
                $$[+]
                -> $S2
                return
            }
            ReturnFromS1() {
                log("ReturnFromS1 Forwarded")
                return
            }
            ReturnFromS2() {
                log("ReturnFromS2 Forwarded")
                return
            }
        }

        $S1 {
            $>() {
                log("Enter $S1")
                return
            }
            <$() {
                log("Exit $S1")
                return
            }
            ReturnFromS1() {
                -> $S0
                return
            }
        }

        $S2 {
            $>() {
                log("Enter $S2")
                return
            }
            <$() {
                log("Exit $S2")
                return
            }
            ReturnFromS2() {
                -> $$[-]
                return
            }
        }

    actions:
        log(msg:str) {
        }

    domain:
        var tape = `[]`
}
