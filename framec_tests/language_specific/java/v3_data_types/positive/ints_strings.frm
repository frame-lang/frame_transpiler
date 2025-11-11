@target java

system S {
    machine:
        $A => $P {
            e() { int n=1; String s="x"; => $^; s.toUpperCase(); }
        }
        $P { }
}
