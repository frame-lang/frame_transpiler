@target rust
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                $$[+]; // inline separator before transition on next line
                -> $B()
            }
        }
        $B { }
}

