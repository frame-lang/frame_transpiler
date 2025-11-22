@target typescript
// @exec-ok

system S {
    machine:
        $A => $P {
            e() {
                let x = 1;
                {
                    let x = 2;
                    => $^; x.toString();
                }
                x = 3;
            }
        }
        $P { }
}
