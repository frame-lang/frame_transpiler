@target c

system S {
    machine:
        $A {
            e() {
                const char* s = "-> $B() and => $^ ignored";
                a();
            }
        }
}

