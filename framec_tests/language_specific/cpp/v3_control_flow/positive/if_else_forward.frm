@target cpp

system S {
    machine:
        $A {
            e() { if (a) { => $^; } else { => $^; } }
        }
}

