@@target python_3

@@system SimpleFSM {
    interface:
        next()
        prev()

    machine:
        $A {
            next() {
                -> $B
            }
        }

        $B {
            next() {
                -> $C
            }
            prev() {
                -> $A
            }
        }

        $C {
            prev() {
                -> $B
            }
        }
}
