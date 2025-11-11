@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

#[codegen.python.code.public_state_info:bool="true"]


system EventHandler {
    interface:
        LogIt(x:int)
        LogAdd(a:int, b:int)
        LogReturn(a:int, b:int) : int
        PassAdd(a:int, b:int)
        PassReturn(a:int, b:int) : int

    machine:
        $S1 {
            LogIt(x:int) {
                log("x", x)
                return
            }

            LogAdd(a:int, b:int) {
                log("a", a)
                log("b", b)
                log("a+b", a+b)
                return
            }

            LogReturn(a:int, b:int) : int {
                log("a", a)
                log("b", b)
                r = a + b
                log("r", r)
                return r
            }

            PassAdd(a:int, b:int) {
                -> $S2(a+b)
            }

            PassReturn(a:int, b:int): int {
                r = a + b
                log("r", r)
                -> $S2(r)
            }
        }

        $S2(p:int) {
            $>() {
                log("p", p)
                return
            }
            LogIt(x:int) { return }
            LogAdd(a:int, b:int) { return }
            LogReturn(a:int, b:int) : int { return 0 }
            PassAdd(a:int, b:int) { return }
            PassReturn(a:int, b:int) : int { return 0 }
        }

    actions:
        log(msg:str, val:int) {
        }

    domain:
        tape = []
}
