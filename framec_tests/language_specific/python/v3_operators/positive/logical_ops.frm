@target python

system S {
    machine:
        $A => $P {
            e() {
                if a and b:
                    => $^
                elif a or b:
                    => $^
                else:
                    => $^
            }
        }
        $P { }
}
