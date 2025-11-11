@target python

system S {
    machine:
        $A {
            e() {
                t = (1, "a")
                => $^
                str(t[1])
            }
        }
}

