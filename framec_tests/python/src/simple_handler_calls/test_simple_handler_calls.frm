# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

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
            }

            B() {
                -> $B
            }

            C() {
                A()
                return
            }

            D() {
                B()
                -> $A
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
