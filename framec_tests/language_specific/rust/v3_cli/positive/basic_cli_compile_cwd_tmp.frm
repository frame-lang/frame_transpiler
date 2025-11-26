@target rust
// @meta: rs_compile
# @cwd: tmp

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { e() { } }
}
