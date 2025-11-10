@target c

system S {
    machine:
        $A {
            e() {
                => $^ /* inline block ok */
            }
        }
}

