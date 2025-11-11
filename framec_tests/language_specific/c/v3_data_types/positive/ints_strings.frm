@target c

system S {
    machine:
        $A {
            e() {
                int n = 42;
                const char* s = "hello";
                => $^; n = n + 1;
            }
        }
}

