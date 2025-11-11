@target java

system S {
    machine:
        $A {
            e() {
                => $^
                int x = ; // malformed native statement
            }
        }
}

