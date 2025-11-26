@target python
# @expect: E113

system S {
    interface:
        e()

    domain:
        x = 1

    machine:
        $A {
            e() { pass }
        }
}

