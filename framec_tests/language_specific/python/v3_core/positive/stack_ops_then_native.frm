@target python

system S {
    machine:
        $A {
            e() {
                $$[+]
                x()
                $$[-]
                y()
            }
        }
}

