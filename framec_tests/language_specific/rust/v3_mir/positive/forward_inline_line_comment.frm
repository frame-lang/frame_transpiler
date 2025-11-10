@target rust

system S {
    machine:
        $A {
            e() {
                => $^ // inline ok
            }
        }
}

