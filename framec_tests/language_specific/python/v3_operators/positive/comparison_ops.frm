@target python

system S {
    machine:
        $A => $P {
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
        $P { }
}
