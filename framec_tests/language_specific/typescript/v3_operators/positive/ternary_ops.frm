@target typescript

system S {
    machine:
        $A {
            e() {
                let x = cond ? 1 : 2;
                => $^; x.toString();
            }
        }
}

