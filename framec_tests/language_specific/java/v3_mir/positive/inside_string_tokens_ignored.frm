@target java

system S {
    machine:
        $A {
            e() {
                String s = "-> $B() and => $^ ignored";
                a();
            }
        }
}

