@target java

system S {
    machine:
        $A => $P {
            e() {
                => $^
            }
        }
        $P { }
}
