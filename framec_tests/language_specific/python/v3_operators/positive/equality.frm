@target python

system S {
    machine:
        $A {
            e() {
                if a == b:
                    => $^
                else:
                    => $^
            }
        }
}

