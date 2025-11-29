@target python
# @expect: E301

system S {
    machine:
        $A {
            e() {
                -> $B(1, "x", func(2, 3)  # missing ')'
            }
        }
}
