@target typescript
// @run-expect: STACK:PUSH
// @run-expect: STACK:POP
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                for (let i = 0; i < 1; i++) {
                    $$[+];
                    let j = 0;
                    while (j < 1) {
                        j += 1;
                    }
                    $$[-];
                }
                -> $B();
            }
        }
        $B { }
}

