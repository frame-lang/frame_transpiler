@target c

system S {
    machine:
        $A {
            e() {
                const char* s = "inside -> $B() and => $^ should be ignored";
                native();
            }
        }
}

