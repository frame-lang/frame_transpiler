@target c

system S {
    machine:
        $A {
            e() {
                => $^ extra
            }
        }
}

