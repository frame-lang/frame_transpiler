@target python
# @compile-expect: class S
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

