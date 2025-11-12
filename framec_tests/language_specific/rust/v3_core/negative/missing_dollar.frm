@target rust
# @expect: E300

system S {
    machine:
        $A {
            e() {
                -> $ (1)   // invalid state name start after '$'
            }
        }
}
