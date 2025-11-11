@target python

system S {
    machine:
        $A {
            e() {
                while cond:
                    => $^
                    step()
                done()
            }
        }
}

