
#[codegen.python.code.public_state_info:bool="true"]

system SimpleHandlerCalls {
    interface:
        A()
        B()
        C()
        D()
        E()

    machine:
        $Init {
            A() {
                -> $A
                return
            }

            B() {
                -> $B
                return
            }

            C() {
                A()
                return
            }

            D() {
                B()
                -> $A
                return
            }

            E() {
                D()
                C()
                return
            }
        }

        $A {
        }
        
        $B {
        }
}
