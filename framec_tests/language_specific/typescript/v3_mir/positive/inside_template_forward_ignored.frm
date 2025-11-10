@target typescript

system S {
    machine:
        $A {
            e() {
                const t = `inside ${"string"} => $^ ignored`;
                a();
            }
        }
}

