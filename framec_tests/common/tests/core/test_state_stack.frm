# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

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
            }
            to_b() {
                -> $B
            }
            to_c() {
                -> $C
            }
            push() {
                $$[+]
                return
            }
            pop() {
                -> $$[-]
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
            }
            to_b() {
                -> $B
            }
            to_c() {
                -> $C
            }
            push() {
                $$[+]
                return
            }
            pop() {
                -> $$[-]
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
            }
            to_b() {
                -> $B
            }
            to_c() {
                -> $C
            }
            push() {
                $$[+]
                return
            }
            pop() {
                -> $$[-]
            }
        }

    actions:
        log(msg:str) {
        }

    domain:
        var tape = []
}
