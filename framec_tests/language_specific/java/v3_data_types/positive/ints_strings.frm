@target java

system S {
    machine:
        $A {
            e() { int n=1; String s="x"; => $^; s.toUpperCase(); }
        }
}

