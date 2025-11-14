@target typescript
// @visitor-map-golden: origins=frame; min=1
// @debug-manifest-expect: system=S; states=A,B

system S {
    machine:
        $A {
            e() {
                let x = 1;
                -> $B()
            }
        }
        $B { e() { } }
}

