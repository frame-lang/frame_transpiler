@target python
# @visitor-map-golden: origins=frame; min=2
# @debug-manifest-expect: system=S; states=A,B,C

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B {
            e() {
                y = 1
                -> $C(1, 2)
            }
            f() {
                pass
            }
        }
        $C {
            e() { pass }
        }
}

