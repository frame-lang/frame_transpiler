
#[codegen.python.code.public_state_info:bool="true"]

system StateContextStack {
    interface:
        to_a()
        to_b()
        to_c()
        inc()
        value() : int
        push()
        pop()

    machine:
        $A {
            var x:int = 0
            $>() {
                log("A:>")
                return
            }
            <$() {
                log("A:<")
                return
            }
            inc() {
                x = x + 1
                return
            }
            value() : int {
                return x
            }
            to_a() {
                -> $A
                return
            }
            to_b() {
                -> $B
                return
            }
            to_c() {
                -> $C
                return
            }
            push() {
                $$[+]
                return
            }
            pop() {
                -> $$[-]
                return
            }
        }

        $B {
            var y:int = 0
            $>() {
                log("B:>")
                return
            }
            <$() {
                log("B:<")
                return
            }
            inc() {
                y = y + 5
                return
            }
            value() : int {
                return y
            }
            to_a() {
                -> $A
                return
            }
            to_b() {
                -> $B
                return
            }
            to_c() {
                -> $C
                return
            }
            push() {
                $$[+]
                return
            }
            pop() {
                -> $$[-]
                return
            }
        }

        $C {
            var z:int = 0
            $>() {
                log("C:>")
                return
            }
            <$() {
                log("C:<")
                return
            }
            inc() {
                z = z + 10
                return
            }
            value() : int {
                return z
            }
            to_a() {
                -> $A
                return
            }
            to_b() {
                -> $B
                return
            }
            to_c() {
                -> $C
                return
            }
            push() {
                $$[+]
                return
            }
            pop() {
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
