@target java

system S {
    machine:
        $A => $P {
            e() {
                => $^ /* inline block ok */
            }
        }
        $P { }
}
