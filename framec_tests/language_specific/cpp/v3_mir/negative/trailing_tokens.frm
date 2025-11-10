@target cpp

system S {
    machine:
        $A {
            e() {
                -> $B() extra
            }
        }
}

