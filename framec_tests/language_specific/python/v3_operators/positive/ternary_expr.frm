@target python

system S {
    machine:
        $A {
            e() {
                x = 1 if cond else 2
                => $^
                str(x)
            }
        }
}

