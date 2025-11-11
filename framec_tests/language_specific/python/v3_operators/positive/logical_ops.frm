@target python

system S {
    machine:
        $A {
            e() {
                if a and b:
                    => $^
                elif a or b:
                    => $^
                else:
                    => $^
            }
        }
}

