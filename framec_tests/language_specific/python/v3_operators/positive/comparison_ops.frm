@target python

system S {
    machine:
        $A {
            e() {
                if a > b:
                    => $^
                elif a < b:
                    => $^
                elif a != b:
                    => $^
                else:
                    => $^
            }
        }
}

