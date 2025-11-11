@target typescript

system S {
    machine:
        $A {
            e() {
                function f() { const y = 2; }
                let y = 3;
                => $^; y.toString();
            }
        }
}

