@target c

system S {
    machine:
        $A {
            e() {
                => $^
                int x = ; // malformed native statement
            }
        }
}

