@target csharp
// @skip-if: csharp-toolchain-missing
// @run-expect: FORWARD:PARENT

system S {
    machine:
        $A => $P {
            e() {
                if (true) {
                    => $^
                }
            }
        }
        $P { }
}

