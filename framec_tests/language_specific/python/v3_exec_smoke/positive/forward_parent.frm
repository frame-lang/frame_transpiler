@target python

system S {
    machine:
        $P {
            e() {
                # no-op in parent
            }
        }
        $C => $P {
            e() {
                => $^
            }
        }
}
