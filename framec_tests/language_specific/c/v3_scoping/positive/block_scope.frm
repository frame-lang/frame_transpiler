@target c

system S {
    machine:
        $A {
            e() {
                int x = 1;
                { int x = 2; => $^; }
                x = 3;
            }
        }
}

