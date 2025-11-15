@target typescript
// @compile-expect: from 'frame_runtime_ts'

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { e() { } }
}

