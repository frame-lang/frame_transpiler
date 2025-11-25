@target python
# @compile-expect: class S:
# @import-call: class=S; method=e
# @py-compile

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { e() { pass } }
}
