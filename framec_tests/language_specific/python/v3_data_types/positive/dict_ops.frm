@target python

system S {
    machine:
        $A {
            e() {
                d = {"k": 1}
                d["k"] = 2
                => $^
                d.get("k")
            }
        }
}

