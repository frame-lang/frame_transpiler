@target java

system S {
    machine:
        $A {
            e() {
                => $^ extra
            }
        }
}

