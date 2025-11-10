@target java

system S {
    machine:
        $A {
            e() {
                => $^ // inline ok
            }
        }
}

