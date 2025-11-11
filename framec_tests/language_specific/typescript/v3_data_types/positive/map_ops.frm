@target typescript

system S {
    machine:
        $A {
            e() {
                let m = new Map<string, number>();
                m.set("k", 1);
                => $^; m.get("k");
            }
        }
}

