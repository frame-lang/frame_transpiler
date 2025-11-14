@target python

system C {
    machine:
        $A {
            e() { => $^ }
        }
}

