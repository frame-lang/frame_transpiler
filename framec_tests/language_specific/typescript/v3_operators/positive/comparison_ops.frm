@target typescript

system S {
    machine:
        $A {
            e() {
                if (a > b) { => $^; gt(); }
                else if (a < b) { => $^; lt(); }
                else if (a !== b) { => $^; ne(); }
                else { => $^; eq(); }
            }
        }
}

