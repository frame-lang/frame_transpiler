@target rust

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

