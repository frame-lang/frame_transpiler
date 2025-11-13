@target rust
// @run-expect: FORWARD:PARENT
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $Child => $Parent {
            e() {
                => $^
                $$[+]
                -> $Next()
            }
        }
        $Next { }
        $Parent { }
}

