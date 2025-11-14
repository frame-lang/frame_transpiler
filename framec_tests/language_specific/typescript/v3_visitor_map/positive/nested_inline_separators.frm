@target typescript
// @visitor-map-golden: origins=frame; min=1

system S {
    machine:
        $A {
            e() {
                if (true) {
                    let x = 1; let y = 2; // inline sep
                } else {
                    let y = 3; let x = 4; // inline sep
                }
                -> $B()
            }
        }
        $B { e() { } }
}

