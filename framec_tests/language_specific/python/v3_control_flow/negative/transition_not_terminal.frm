@target python
# @expect: E400

system S {
    machine:
        $A {
            e() {
                x()
                -> $B()
                y()  # should violate terminal rule
            }
        }
        $B { }
}
