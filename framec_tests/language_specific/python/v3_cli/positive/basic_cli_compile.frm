@target python
# @compile-expect: class S:
# @import-call: class=S; method=e

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { e() { pass } }
}
