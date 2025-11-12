@target rust
# @expect: E301

system S {
    machine:
        $A {
            e() {
                -> $B(1, 2    // missing closing ')'
            }
        }
        $B { }
}

