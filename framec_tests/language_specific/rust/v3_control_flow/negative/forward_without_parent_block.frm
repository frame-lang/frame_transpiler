@target rust
# @expect: E403

system S {
    machine:
        $A {
            e() {
                => $^
            }
        }
}

