@target python
# @visitor-map-golden: origins=frame; min=1

system S {
    machine:
        $A {
            e() {
                if True:
                    x = 1; y = 2  # inline sep
                else:
                    y = 3; x = 4  # inline sep
                -> $B()
            }
        }
        $B { e() { pass } }
}

