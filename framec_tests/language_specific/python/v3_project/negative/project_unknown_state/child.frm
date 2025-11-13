@target python
# @expect: E402

system S {
    machine:
        $A {
            e() {
                -> $Z()
            }
        }
}

