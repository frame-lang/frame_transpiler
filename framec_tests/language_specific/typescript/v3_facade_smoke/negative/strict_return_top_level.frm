@target typescript

system S {
    machine:
        $A {
            e() {
                // Native parse (SWC) should flag a malformed if-statement
                if (
                => $^;
            }
        }
}
