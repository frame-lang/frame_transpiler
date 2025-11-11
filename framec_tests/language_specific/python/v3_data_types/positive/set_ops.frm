@target python

system S {
    machine:
        $A {
            e() {
                s = {1,2}
                s.add(3)
                => $^
                len(s)
            }
        }
}

