@target rust
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

