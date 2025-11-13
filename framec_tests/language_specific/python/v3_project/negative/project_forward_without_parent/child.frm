@target python
# @expect: E403

system S {
    machine:
        $A {
            e() {
                => $^
            }
        }
}

