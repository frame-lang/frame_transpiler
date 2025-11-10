@target rust

system S {
    machine:
        $A {
            e() {
                => $^ /* inline block ok */
            }
        }
}

