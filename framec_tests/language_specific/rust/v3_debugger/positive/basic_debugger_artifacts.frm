@target rust
// @debug-manifest-expect: system=S; states=A,B

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { e() { } }
}

