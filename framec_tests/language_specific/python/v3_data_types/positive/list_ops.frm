@target python

system S {
    machine:
        $A => $P {
            e() {
                a = [1,2]
                a.append(3)
                => $^
                len(a)
            }
        }
        $P { }
}
