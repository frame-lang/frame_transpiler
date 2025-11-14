@target python
# @compile-expect: class S:

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { e() { pass } }
}
