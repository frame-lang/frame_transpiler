@target python

system S {
    machine:
        $A {
            e() {
                s = "-> $B() and => $^ ignored"
                a()
            }
        }
}

