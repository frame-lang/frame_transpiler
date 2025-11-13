@target csharp
// @skip-if: csharp-toolchain-missing
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^
                var x = 42;
                -> $B()
            }
        }
        $B { }
        $P { }
}

