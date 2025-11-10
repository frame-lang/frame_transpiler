@target cpp

system S {
    machine:
        $A {
            e() {
                => $^ // inline ok
            }
        }
}

