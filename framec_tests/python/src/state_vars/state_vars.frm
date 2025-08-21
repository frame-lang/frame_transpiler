
#[codegen.python.code.public_state_info:bool="true"]
#[codegen.python.code.public_compartment:bool="true"]

system StateVars {
    interface:
        X()
        Y()
        Z()

    machine:
        $Init {
            $>() {
                -> $A
                return
            }
        }

        $A {
            var x:int = 0
            X() {
                x = x + 1
                return
            }
            Y() {
                -> $B
                return
            }
            Z() {
                -> $B
                return
            }
        }

        $B {
            var y:int = 10
            var z:int = 100
            X() {
                -> $A
                return
            }
            Y() {
                y = y + 1
                return
            }
            Z() {
                z = z + 1
                return
            }
        }

    actions:

    domain:
}
