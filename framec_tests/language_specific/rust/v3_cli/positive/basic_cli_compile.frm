@target rust
// @compile-expect: pub fn e\(\) \{

system S1 {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { e() { } }
}
