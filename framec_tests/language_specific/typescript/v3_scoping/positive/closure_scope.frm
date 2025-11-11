@target typescript

system S {
    machine:
        $A {
            e() {
                function outer() {
                    let v = 1;
                    return function inner() { return v + 1; };
                }
                const f = outer();
                => $^; f();
            }
        }
}

