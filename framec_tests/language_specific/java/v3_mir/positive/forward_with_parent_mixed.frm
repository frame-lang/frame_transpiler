@target java

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

