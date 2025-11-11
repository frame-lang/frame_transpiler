@target cpp

system S {
    machine:
        $A => $P {
            e() {
                => $^ /* inline block ok */
            }
        }
        $P { }
}
