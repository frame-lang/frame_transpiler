@target typescript
// @expect: E200

system S {
    machine:
        $A {
            e() {
                => $B  // invalid forward head
            }
        }
}
