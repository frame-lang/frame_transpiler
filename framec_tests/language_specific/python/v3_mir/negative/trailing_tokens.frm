@target python

system S {
    machine:
        $A {
            e() {
                -> $B() extra
            }
        }
}

