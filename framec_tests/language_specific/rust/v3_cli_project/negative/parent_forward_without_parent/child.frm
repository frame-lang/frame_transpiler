@target rust

system Child {
    machine:
        $A {
            e() { => $^ }
        }
}

