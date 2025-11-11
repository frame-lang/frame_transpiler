@target java

system S {
    machine:
        $A {
            e() {
                -> $B();
                List<String x; // missing '>'
            }
        }
        $B {
        }
}

