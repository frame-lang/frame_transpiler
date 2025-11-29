@target rust
// Transition is not the final statement in the block; expect E400.
// @expect: E400

system S {
    machine:
        $A {
            e() {
                x();
                -> $B();
                y(); // should violate terminal rule
            }
        }
        $B { }
}
