@target typescript

system S {
    machine:
        $A => $P {
            e() {
                let t: [number, string] = [1, "a"];
                => $^; t[1].toUpperCase();
            }
        }
        $P { }
}
