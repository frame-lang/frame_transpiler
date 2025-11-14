@target python
# @visitor-map-golden: origins=frame; min=1

system S {
    machine:
        $A {
            e() {
                x = 1
                -> $B()
            }
        }
        $B {
            e() {
                pass
            }
        }
}
