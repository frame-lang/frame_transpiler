
#[codegen.python.code.public_state_info:bool="true"]

system StateStack {
    interface:
        to_a()
        to_b()
        to_c()
        push()
        pop()


    machine:
        $A {
            $>() {
                log("A:>")
                return
            }
            <$() {
                log("A:<")
                return
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
            $>() {
                log("B:>")
                return
            }
            <$() {
                log("B:<")
                return
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
            $>() {
                log("C:>")
                return
            }
            <$() {
                log("C:<")
                return
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
