@target java

system S {
    machine:
        $A {
            e() {
                if (true) {
                    $$[+]
                    $$[-]
                } else {
                    $$[+]
                    $$[-]
                }
                -> $B()
            }
        }
        $B { e() { } }
}

