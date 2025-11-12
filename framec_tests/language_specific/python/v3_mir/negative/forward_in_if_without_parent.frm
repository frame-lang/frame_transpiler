@target python

system S {
    machine:
        $A {
            e() {
                if True:
                    => $^
            }
        }
}

