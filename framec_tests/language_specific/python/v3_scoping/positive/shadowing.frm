@target python

system S {
    machine:
        $A {
            e() {
                x = 1
                if True:
                    x = 2
                    => $^
                x = 3
            }
        }
}

