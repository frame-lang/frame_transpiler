@target java

system S {
    machine:
        $A {
            e() { if (a) { => $^; } else { => $^; } }
        }
}

