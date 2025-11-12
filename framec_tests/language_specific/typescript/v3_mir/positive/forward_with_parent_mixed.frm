@target typescript

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

