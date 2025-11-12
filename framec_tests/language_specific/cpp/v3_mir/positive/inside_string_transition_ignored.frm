@target cpp

system S {
    machine:
        $A {
            e() {
                const char* s = "inside -> $B() and => $^ should be ignored";
                native();
            }
        }
}

