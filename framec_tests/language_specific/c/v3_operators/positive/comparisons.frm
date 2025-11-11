@target c

system S {
    machine:
        $A {
            e() {
                if (a > b) { => $^; }
                else if (a < b) { => $^; }
                else { => $^; }
            }
        }
}

