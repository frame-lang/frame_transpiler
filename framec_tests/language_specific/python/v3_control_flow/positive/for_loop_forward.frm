@target python

system S {
    machine:
        $A {
            e() {
                for i in range(3):
                    => $^
                    step(i)
                done()
            }
        }
}

