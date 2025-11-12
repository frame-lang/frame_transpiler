@target python

system S {
    machine:
        $P {
        }
        $A => $P {
            e() {
                => $^
            }
        }
        $B {
        }
}

