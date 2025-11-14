@target rust
// @expect: E403

system S {
    machine:
        $A {
            e() {
                // Forward without a declared parent should fail validation
                => $^
                // Inline after forward (non-terminal) then a transition
                let z = 10;
                -> $B()
            }
        }
        $B { e() { } }
}

