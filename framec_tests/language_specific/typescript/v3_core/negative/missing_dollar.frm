@target typescript
// @expect: E300

system S {
    machine:
        $A {
            e() {
                -> $ (1)
            }
        }
}
