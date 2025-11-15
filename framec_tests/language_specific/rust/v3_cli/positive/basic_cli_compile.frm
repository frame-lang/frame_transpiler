@target rust
// @rust-state-enum
// @compile-expect: enum StateId \{
// @compile-expect: state: StateId

system S1 {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { e() { } }
}
