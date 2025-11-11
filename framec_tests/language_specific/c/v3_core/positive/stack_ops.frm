@target c

system S {
    machine:
        $A {
            e() {
                $$[+]
                x();
                $$[-]
                y();
            }
        }
}

