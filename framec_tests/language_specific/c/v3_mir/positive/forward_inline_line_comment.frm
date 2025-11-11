@target c

system S {
    machine:
        $A => $P {
            e() {
                => $^ // inline ok
            }
        }
        $P { }
}
