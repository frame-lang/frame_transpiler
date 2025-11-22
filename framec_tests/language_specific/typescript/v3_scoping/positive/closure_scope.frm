@target typescript
// @exec-ok

system S {
    machine:
        $A => $P {
            e() {
                function outer() {
                    let v = 1;
                    return function inner() { return v + 1; };
                }
                const f = outer();
                => $^; f();
            }
        }
        $P { }
}
