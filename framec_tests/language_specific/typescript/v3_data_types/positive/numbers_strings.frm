@target typescript

system S {
    machine:
        $A {
            e() {
                let n = 42;
                let s = "hello";
                => $^; n.toString();
            }
        }
}

