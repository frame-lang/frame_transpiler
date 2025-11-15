@target typescript
// @compile-expect: from 'frame_runtime_ts'
// @cwd: tmp

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { e() { } }
}

