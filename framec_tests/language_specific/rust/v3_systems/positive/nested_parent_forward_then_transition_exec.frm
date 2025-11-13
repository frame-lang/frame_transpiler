@target rust
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $Child => $Parent {
            e() {
                // Nested block to simulate structured control
                {
                    => $^
                }
                -> $Next()
            }
        }
        $Next { }
        $Parent { }
}

