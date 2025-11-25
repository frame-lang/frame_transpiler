@target python
# @compile-expect: Compiled 1 module

system P {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B {
            e() {
                pass
            }
        }
}

