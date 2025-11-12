@target c

system S {
    machine:
        $P {
        }
        $A => $P {
            e() {
                // ok: $A declares a parent; not tested here
            }
        }
        $B {
            e() {
                => $^
            }
        }
}

