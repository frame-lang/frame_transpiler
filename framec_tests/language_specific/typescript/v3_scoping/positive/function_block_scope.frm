@target typescript
// @exec-ok

system S {
    machine:
        $A => $P {
            e() {
                function f() { const y = 2; }
                let y = 3;
                => $^; y.toString();
            }
        }
        $P { }
}
