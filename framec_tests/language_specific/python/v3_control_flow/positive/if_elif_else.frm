@target python

system S {
    machine:
        $A {
            e() {
                if a:
                    x()
                elif b:
                    y()
                else:
                    z()
                => $^
            }
        }
}

