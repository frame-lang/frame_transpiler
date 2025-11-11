@target python

system S {
    machine:
        $A {
            e() {
                a = [1,2]
                a.append(3)
                => $^
                len(a)
            }
        }
}

