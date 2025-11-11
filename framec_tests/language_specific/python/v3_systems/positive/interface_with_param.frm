@target python

system S {
    machine:
        $A {
            ev(n:int) {
                => $^
                str(n)
            }
        }
}
