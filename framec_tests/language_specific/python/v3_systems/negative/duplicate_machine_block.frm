@target python

# System with duplicate machine: blocks. Expect E114.
# @expect: E114

system S {
    machine:
        $A {
            e() { x() }
        }
    machine:
        $B {
            e() { x() }
        }
}
