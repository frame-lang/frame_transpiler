@target typescript

system S {
    machine:
        $A {
            e() {
                let t: [number, string] = [1, "a"];
                => $^; t[1].toUpperCase();
            }
        }
}

