@target java

system S {
    machine:
        $A {
            e() {
                String s = "inside -> $B() and => $^ should be ignored";
                native();
            }
        }
}

