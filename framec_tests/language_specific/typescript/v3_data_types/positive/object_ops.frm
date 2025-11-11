@target typescript

system S {
    machine:
        $A {
            e() {
                let o = { a: 1, b: "x" };
                o.a = 2;
                => $^; o.b.toUpperCase();
            }
        }
}

